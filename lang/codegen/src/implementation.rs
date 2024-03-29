// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use crate::{
    implementations::*,
    internal,
    internal::*,
};
use proc_macro2::TokenStream;
use quote::{
    quote,
    ToTokens,
};
use std::collections::HashMap;
use syn::{
    Item,
    Path,
};

pub fn generate(attrs: TokenStream, input: TokenStream) -> TokenStream {
    if internal::skip() {
        return quote! {}
    }
    // map attribute args to default contract names
    let args = syn::parse2::<AttributeArgs>(attrs)
        .expect("No default contracts to implement provided")
        .iter()
        .map(|arg| {
            match arg {
                NestedMeta::Path(method) => method.to_token_stream().to_string().replace(' ', ""),
                _ => panic!("Can't parse naming of default contract to implement"),
            }
        })
        .collect::<Vec<String>>();

    let mut module = syn::parse2::<syn::ItemMod>(input).expect("Can't parse contract module");
    let (braces, items) = match module.clone().content {
        Some((brace, items)) => (brace, items),
        None => {
            panic!(
                "{}",
                "out-of-line openbrush modules are not supported, use `#[implementation] mod name {{ ... }}`",
            )
        }
    };

    // name of struct for which we will implement the traits
    let ident = extract_storage_struct_name(&items);
    // we will look for overriden functions and remove them from the mod
    let (map, mut items) = consume_overriders(items);

    // to save importing of stuff by users
    let mut imports = HashMap::<&str, syn::ItemUse>::default();
    // if multiple contracts are using the same trait implemented differently we override it this way
    let mut overriden_traits = HashMap::<&str, syn::Item>::default();

    let mut impl_args = ImplArgs::new(&map, &mut items, &mut imports, &mut overriden_traits, ident);
    let is_capped = args.contains(&"PSP22Capped".to_string());

    for to_implement in args.clone() {
        match to_implement.as_str() {
            "PSP22" => impl_psp22(&mut impl_args, is_capped),
            "PSP22Mintable" => impl_psp22_mintable(&mut impl_args),
            "PSP22Burnable" => impl_psp22_burnable(&mut impl_args),
            "PSP22Permit" => impl_psp22_permit(&mut impl_args),
            "PSP22Metadata" => impl_psp22_metadata(&mut impl_args),
            "PSP22Capped" => impl_psp22_capped(&mut impl_args),
            "PSP22Wrapper" => impl_psp22_wrapper(&mut impl_args),
            "PSP22Votes" => impl_psp22_votes(&mut impl_args),
            "Flashmint" => impl_flashmint(&mut impl_args),
            "PSP22TokenTimelock" => impl_token_timelock(&mut impl_args),
            "PSP22Pallet" => impl_psp22_pallet(&mut impl_args),
            "PSP22PalletBurnable" => impl_psp22_pallet_burnable(&mut impl_args),
            "PSP22PalletMetadata" => impl_psp22_pallet_metadata(&mut impl_args),
            "PSP22PalletMintable" => impl_psp22_pallet_mintable(&mut impl_args),
            "PSP34" => impl_psp34(&mut impl_args),
            "PSP34Burnable" => impl_psp34_burnable(&mut impl_args),
            "PSP34Mintable" => impl_psp34_mintable(&mut impl_args),
            "PSP34Metadata" => impl_psp34_metadata(&mut impl_args),
            "PSP34Enumerable" => impl_psp34_enumerable(&mut impl_args),
            "PSP37" => impl_psp37(&mut impl_args),
            "PSP37Batch" => impl_psp37_batch(&mut impl_args),
            "PSP37Burnable" => impl_psp37_burnable(&mut impl_args),
            "PSP37Metadata" => impl_psp37_metadata(&mut impl_args),
            "PSP37Mintable" => impl_psp37_mintable(&mut impl_args),
            "PSP37Enumerable" => impl_psp37_enumerable(&mut impl_args),
            "Ownable" => impl_ownable(&mut impl_args),
            "PaymentSplitter" => impl_payment_splitter(&mut impl_args),
            "AccessControl" => impl_access_control(&mut impl_args),
            "AccessControlEnumerable" => impl_access_control_enumerable(&mut impl_args),
            "Pausable" => impl_pausable(&mut impl_args),
            "TimelockController" => impl_timelock_controller(&mut impl_args),
            "Proxy" => impl_proxy(&mut impl_args),
            "Diamond" => impl_diamond(&mut impl_args),
            "DiamondLoupe" => impl_diamond_loupe(&mut impl_args),
            "Upgradeable" => impl_upgradeable(&mut impl_args),
            "Governor" => impl_governor(&mut impl_args),
            "GovernorSettings" => impl_governor_settings(&mut impl_args),
            "GovernorVotes" => impl_governor_votes(&mut impl_args),
            "GovernorQuorum" => impl_governor_quorum(&mut impl_args),
            "GovernorCounting" => impl_governor_counting(&mut impl_args),
            "Nonces" => impl_nonces(&mut impl_args),
            "PSP61" => impl_psp61(&mut impl_args, args.clone()),
            _ => panic!("openbrush::implementation({to_implement}) not implemented!"),
        }
    }

    cleanup_imports(impl_args.imports);

    // add the imports
    impl_args
        .items
        .append(&mut impl_args.imports.values().cloned().map(syn::Item::Use).collect());

    // add overriden traits
    impl_args
        .items
        .append(&mut impl_args.overriden_traits.values().cloned().collect());

    module.content = Some((braces, items));

    quote! {
        #module
    }
}

fn cleanup_imports(imports: &mut HashMap<&str, syn::ItemUse>) {
    // we will remove unnecessary imports
    let psp22_impls = vec![
        "PSP22Mintable",
        "PSP22Burnable",
        "PSP22Capped",
        "PSP22Metadata",
        "PSP22Wrapper",
        "PSP22Permit",
        "Flashmint",
    ];
    check_and_remove_import("PSP22", psp22_impls, imports);

    let psp22_pallet_impls = vec!["PSP22PalletMintable", "PSP22PalletBurnable", "PSP22PalletMetadata"];
    check_and_remove_import("PSP22Pallet", psp22_pallet_impls, imports);

    let psp34_impls = vec!["PSP34Mintable", "PSP34Burnable", "PSP34Metadata", "PSP34Enumerable"];
    check_and_remove_import("PSP34", psp34_impls, imports);

    let psp37_impls = vec![
        "PSP37Batch",
        "PSP37Burnable",
        "PSP37Metadata",
        "PSP37Mintable",
        "PSP37Enumerable",
    ];
    check_and_remove_import("PSP37", psp37_impls, imports);

    let access_impls = vec!["AccessControlEnumerable", "TimelockController"];
    check_and_remove_import("AccessControl", access_impls, imports);

    check_and_remove_import("Diamond", vec!["DiamondLoupe"], imports);
}

fn check_and_remove_import(name_to_check: &str, to_check: Vec<&str>, imports: &mut HashMap<&str, syn::ItemUse>) {
    if to_check.iter().any(|name| imports.contains_key(name)) {
        imports.remove(name_to_check);
    }
}

// this method consumes override annotated methods and returns them mapped to code and the mod without them
// we will later override the methods
fn consume_overriders(items: Vec<syn::Item>) -> (OverridenFnMap, Vec<syn::Item>) {
    let mut map = HashMap::new();
    let mut result: Vec<syn::Item> = vec![];
    items.into_iter().for_each(|mut item| {
        if let Item::Fn(item_fn) = &mut item {
            if is_attr(&item_fn.attrs, "overrider") || is_attr(&item_fn.attrs, "default_impl") {
                let attr_name = if is_attr(&item_fn.attrs, "overrider") {
                    "overrider"
                } else {
                    "default_impl"
                };
                let fn_name = item_fn.sig.ident.to_string();
                let code = item_fn.block.clone();
                let mut attributes = item_fn.attrs.clone();

                // we will remove the overrider attribute since some other attributes might be interesting to us
                let to_remove_idx = attributes
                    .iter()
                    .position(|attr| is_attr(&[attr.clone()], attr_name))
                    .expect("No {attr_name} attribute found!");
                let overrider_attribute = attributes.remove(to_remove_idx);

                let trait_name = overrider_attribute
                    .parse_args::<Path>()
                    .expect("Expected overriden trait identifier")
                    .to_token_stream()
                    .to_string()
                    .replace(' ', "");

                let mut vec = map.get(&trait_name).unwrap_or(&vec![]).clone();
                vec.push((fn_name, (code, attributes, attr_name == "default_impl")));
                map.insert(trait_name, vec.to_vec());
            } else {
                result.push(item);
            }
        } else {
            result.push(item);
        }
    });

    (map, result)
}

fn extract_storage_struct_name(items: &[syn::Item]) -> String {
    let contract_storage_struct = items
        .iter()
        .find(|item| {
            if let Item::Struct(structure) = item {
                let ink_attr_maybe = structure
                    .attrs
                    .iter()
                    .cloned()
                    .find(|attr| is_attr(&[attr.clone()], "ink"));

                if let Some(ink_attr) = ink_attr_maybe {
                    if let Ok(path) = ink_attr.parse_args::<Path>() {
                        return path.to_token_stream().to_string() == "storage"
                    }
                }
                false
            } else {
                false
            }
        })
        .expect("Contract storage struct not found!");
    match contract_storage_struct {
        Item::Struct(structure) => structure.ident.to_string(),
        _ => unreachable!("Only Item::Struct allowed here"),
    }
}
