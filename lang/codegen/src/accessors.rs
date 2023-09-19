// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
    quote_spanned,
};
use syn::{
    spanned::Spanned,
    Data,
    DataStruct,
    Field,
    Fields,
};

pub fn accessors(trait_ident: TokenStream, s: synstructure::Structure) -> TokenStream {
    let struct_ident = s.ast().ident.clone();

    let item = match s.ast().data.clone() {
        Data::Struct(struct_item) => generate_struct(&s, struct_item),
        _ => panic!("Only structs are supported"),
    };

    let fields: Vec<_> = extract_fields(s.clone(), "get");

    let get_impls = fields.iter().map(|field| {
        let field_ident = field.ident.clone().unwrap();
        let method_ident = format_ident!("get_{}", field_ident);
        let field_type = field.ty.clone();
        let span = field.span();

        quote_spanned! {span =>
            #[ink(message)]
            fn #method_ident(&self) -> #field_type {
                self.data().#field_ident
            }
        }
    });

    let fields: Vec<_> = extract_fields(s.clone(), "set");

    let set_impls = fields.iter().map(|field| {
        let field_ident = field.ident.clone().unwrap();
        let method_ident = format_ident!("set_{}", field_ident);
        let field_type = field.ty.clone();
        let span = field.span();

        quote_spanned! {span =>
            #[ink(message)]
            fn #method_ident(&mut self, value: #field_type) {
                self.data().#field_ident = value;
            }
        }
    });

    quote! {
        #item

        #[openbrush::trait_definition]
        pub trait #trait_ident : Storage<#struct_ident>{
            #(#get_impls)*
            #(#set_impls)*
        }
    }
}

fn generate_struct(s: &synstructure::Structure, struct_item: DataStruct) -> TokenStream {
    let struct_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let attrs = s.ast().attrs.clone();
    let types = s.ast().generics.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();

    let fields = struct_item
        .clone()
        .fields
        .into_iter()
        .map(|mut field| consume_attrs(&mut field));

    match struct_item.fields {
        Fields::Unnamed(_) => {
            quote! {
                #(#attrs)*
                #vis struct #struct_ident #types #where_closure (
                    #(#fields),*
                );
            }
        }
        _ => {
            quote! {
                #(#attrs)*
                #vis struct #struct_ident #types #where_closure {
                    #(#fields),*
                }
            }
        }
    }
}

fn consume_attrs(field: &mut syn::Field) -> Field {
    let attr = field
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("get") && !a.path.is_ident("set"))
        .cloned()
        .collect();

    field.attrs = attr;

    field.clone()
}

fn extract_fields(s: synstructure::Structure, attr: &str) -> Vec<Field> {
    let struct_item = match s.ast().data.clone() {
        Data::Struct(struct_item) => struct_item,
        _ => panic!("Only structs are supported"),
    };

    struct_item
        .fields
        .iter()
        .filter(|field| field.attrs.iter().any(|a| a.path.is_ident(attr)))
        .cloned()
        .collect::<Vec<_>>()
}
