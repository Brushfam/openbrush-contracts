// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse2,
    spanned::Spanned,
    Data,
};

pub fn storage_derive(item: TokenStream) -> TokenStream {
    let derive: syn::DeriveInput = parse2(item).expect("Expected DeriveInput");

    let struct_ident = derive.ident;
    let (impls, types, where_clause) = derive.generics.split_for_impl();

    let fields: Vec<_> = match &derive.data {
        Data::Struct(st) => st.fields.iter().collect(),
        Data::Enum(en) => en.variants.iter().flat_map(|v| v.fields.iter()).collect(),
        Data::Union(un) => un.fields.named.iter().collect(),
    };

    let impls = fields
        .iter()
        .filter(|field| field.attrs.iter().any(|a| a.path.is_ident("storage_field")))
        .map(|field| {
            let field_ident = field.ident.clone();
            let ty = field.ty.clone();
            let span = field.span();

            quote::quote_spanned!(span=>
                impl #impls ::openbrush::traits::Storage<#ty> for #struct_ident #types #where_clause {
                    fn get(&self) -> &#ty {
                        &self.#field_ident
                    }

                    fn get_mut(&mut self) -> &mut #ty {
                        &mut self.#field_ident
                    }
                }
            )
        });

    quote! {
        #(#impls)*
    }
}
