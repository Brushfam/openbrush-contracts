// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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
    Block,
    Item,
    Path,
};

pub fn generate(attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    if internal::skip() {
        return (quote! {}).into()
    }
    let input: TokenStream = ink_module.into();

    // map attribute args to default contract names
    let args = syn::parse2::<AttributeArgs>(attrs)
        .expect("No default contracts to implement provided")
        .iter()
        .map(|arg| {
            match arg {
                NestedMeta::Path(method) => method.to_token_stream().to_string().replace(" ", ""),
                _ => panic!("Expected names of OpenBrush traits to implement in the contract!"),
            }
        })
        .collect::<Vec<String>>();

    let mut module = syn::parse2::<syn::ItemMod>(input.clone()).expect("Can't parse contract module");
    let (braces, items) = match module.clone().content {
        Some((brace, items)) => (brace, items),
        None => {
            panic!(
                "{}",
                "out-of-line openbrush modules are not supported, use `#[implementation] mod name {{ ... }}`",
            )
        }
    };

    // we will look for overriden functions and remove them from the mod
    let (map, mut items) = consume_overriders(items);

    // to save importing of stuff by users
    let mut imports = HashMap::<&str, syn::ItemUse>::default();

    for to_implement in args {
        match to_implement.as_str() {
            "PSP22" => impl_psp22(&map, &mut items, &mut imports),
            "PSP22Mintable" => impl_psp22_mintable(&map, &mut items, &mut imports),
            "PSP22Burnable" => impl_psp22_burnable(&map, &mut items, &mut imports),
            "PSP22Metadata" => impl_psp22_metadata(&map, &mut items, &mut imports),
            "PSP22Capped" => impl_psp22_capped(&map, &mut items, &mut imports),
            "PSP22Wrapper" => impl_psp22_wrapper(&map, &mut items, &mut imports),
            "Flashmint" => impl_flashmint(&map, &mut items, &mut imports),
            "PSP22TokenTimelock" => impl_token_timelock(&map, &mut items, &mut imports),
            _ => panic!("openbrush::implementation({to_implement}) not implemented!"),
        }
    }

    cleanup_imports(&mut imports);

    // add the imports
    items.append(
        &mut imports
            .values()
            .cloned()
            .map(|item_use| syn::Item::Use(item_use))
            .collect(),
    );

    module.content = Some((braces.clone(), items));

    let result = quote! {
        #module
    };
    result.into()
}

fn cleanup_imports(imports: &mut HashMap<&str, syn::ItemUse>) {
    // we will remove unnecessary imports
    let psp22_impls = vec![
        "PSP22Mintable",
        "PSP22Burnable",
        "PSP22Capped",
        "PSP22Metadata",
        "PSP22Wrapper",
        "Flashmint",
    ];
    check_and_remove_import("PSP22", psp22_impls, imports)
}

fn check_and_remove_import(name_to_check: &str, to_check: Vec<&str>, imports: &mut HashMap<&str, syn::ItemUse>) {
    if to_check.iter().any(|name| imports.contains_key(name)) {
        imports.remove(name_to_check).expect(
            format!("Trying to implement a {name_to_check} extension without a {name_to_check} implementation!")
                .as_str(),
        );
    }
}

// this method consumes override annotated methods and returns them mapped to code and the mod without them
// we will later override the methods
fn consume_overriders(items: Vec<syn::Item>) -> (HashMap<String, Vec<(String, Box<Block>)>>, Vec<syn::Item>) {
    let mut map = HashMap::new();
    let mut result: Vec<syn::Item> = vec![];
    items.into_iter().for_each(|mut item| {
        if let Item::Fn(item_fn) = &mut item {
            if is_attr(&item_fn.attrs, "overrider") {
                let fn_name = item_fn.sig.ident.to_string();
                let code = item_fn.block.clone();

                let trait_name = item_fn
                    .attrs
                    .clone()
                    .into_iter()
                    .find(|attr| is_attr(&vec![attr.clone()], "overrider"))
                    .expect("No overrider attribute found!")
                    .parse_args::<Path>()
                    .expect("Expected overriden trait identifier")
                    .to_token_stream()
                    .to_string()
                    .replace(" ", "");

                let mut vec = map.get(&trait_name).unwrap_or(&vec![]).clone();
                vec.push((fn_name, code));
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
