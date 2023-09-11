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
    internal,
    metadata::LockedTrait,
};
use proc_macro2::TokenStream;
use quote::{
    quote,
};
use syn::Item;

pub fn generate(_attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    if internal::skip() {
        return quote! {}
    }
    let input: TokenStream = ink_module;
    let attrs: TokenStream = _attrs;
    let mut module = syn::parse2::<syn::ItemMod>(input).expect("Can't parse contract module");
    let (braces, mut items) = match module.content {
        Some((brace, items)) => (brace, items),
        None => {
            panic!(
                "{}",
                "out-of-line openbrush modules are not supported, use `#[openbrush::contract] mod name {{ ... }}`",
            )
        }
    };

    // First, we need to consume all traits and update metadata file.
    // After, we can consume all other stuff.
    items = consume_traits(items);

    let generated_items = generate_impls(items);

    module.content = Some((braces, generated_items));

    quote! {
        #[::ink::contract(#attrs)]
        #module
    }
}

fn consume_traits(items: Vec<syn::Item>) -> Vec<syn::Item> {
    let mut result: Vec<syn::Item> = vec![];
    items.into_iter().for_each(|item| {
            result.push(item);
    });

    result
}

fn generate_impls(mut items: Vec<syn::Item>) -> Vec<syn::Item> {
    let mut generated_items: Vec<syn::Item> = vec![];
    items.iter_mut().for_each(|mut item| {
        if let Item::Impl(item_impl) = &mut item {
            if let Some((_, trait_path, _)) = item_impl.trait_.clone() {
                let trait_ident = trait_path.segments.last().expect("Trait path is empty").ident.clone();
                let trait_lock = LockedTrait::new(trait_ident.to_string());
                if let Some(trait_definition) = &trait_lock.trait_definition {
                    let mut generated_impls =
                        internal::impl_external_trait(item_impl.clone(), &trait_path, trait_definition);
                    generated_items.append(&mut generated_impls);
                    return
                }
            }

            generated_items.push(syn::Item::from(item_impl.clone()));
        } else {
            generated_items.push(item.clone());
        }
    });

    generated_items
}
