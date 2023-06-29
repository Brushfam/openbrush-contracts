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

use crate::internal::{
    is_attr,
    remove_attr,
};
use proc_macro2::TokenStream;
use quote::{
    quote,
    quote_spanned,
    ToTokens,
};
use syn::{
    spanned::Spanned,
    Data,
    DeriveInput,
    Field,
    Fields,
    Type,
};

pub fn storage(_attrs: TokenStream, s: synstructure::Structure) -> TokenStream {
    let fields = generate_fields(s.clone());
    let impls = generate_storage_impls(s.clone(), fields);

    let item = match s.ast().data.clone() {
        Data::Struct(struct_item) => generate_struct(&s, struct_item),
        Data::Enum(enum_item) => generate_enum(&s, enum_item),
        Data::Union(union_item) => generate_union(&s, union_item),
    };

    (quote! {
        #item

        #impls
    })
    .into()
}

fn generate_fields(s: synstructure::Structure) -> Vec<Field> {
    match &s.ast().data {
        Data::Struct(st) => {
            st.clone()
                .fields
                .iter()
                .enumerate()
                .map(|(_, field)| field.clone())
                .collect()
        }
        Data::Enum(en) => {
            en.clone()
                .variants
                .iter()
                .flat_map(|v| v.fields.iter().enumerate().map(|(_, field)| field.clone()))
                .collect()
        }
        Data::Union(un) => {
            un.clone()
                .fields
                .named
                .iter()
                .enumerate()
                .map(|(_, field)| field.clone())
                .collect()
        }
    }
}

fn generate_storage_impls(s: synstructure::Structure, fields: Vec<Field>) -> TokenStream {
    let struct_ident = s.ast().ident.clone();
    let (impls, types, where_clause) = s.ast().generics.split_for_impl();

    let impls = fields
        .iter()
        .filter(|field| {
            field
                .attrs
                .iter()
                .find(|a| a.path.is_ident("storage_field") || a.path.is_ident("upgradeable_storage_field"))
                .is_some()
        })
        .map(|field| {
            let field_ident = field.ident.clone();
            let ty = field.ty.clone();

            let span = field.span();

            quote_spanned!(span=>
                impl #impls ::openbrush::traits::StorageAccess<#ty> for #struct_ident #types #where_clause {
                    fn get(&self) -> Option<#ty> {
                        ::openbrush::traits::StorageAccess::<#ty>::get(&self.#field_ident)
                    }

                    fn set(&mut self, value: &#ty) {
                        ::openbrush::traits::StorageAccess::<#ty>::set(&mut self.#field_ident, value);
                    }

                    fn get_or_default(&self) -> #ty {
                        ::openbrush::traits::StorageAccess::<#ty>::get_or_default(&self.#field_ident)
                    }
                }
            )
        });

    quote! {
        #(#impls)*
    }
}

pub fn generate_struct(s: &synstructure::Structure, struct_item: syn::DataStruct) -> TokenStream {
    let struct_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let types = s.ast().generics.clone();
    let attrs = s.ast().attrs.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();

    let fields = wrap_upgradeable_fields(struct_item.fields.clone())
        .iter()
        .map(|field| {
            let mut new_field = field.clone();
            new_field.attrs = remove_attr(&new_field.attrs, "upgradeable_storage_field");
            new_field.attrs = remove_attr(&new_field.attrs, "storage_field");
            new_field
        })
        .collect::<Vec<_>>();

    match struct_item.fields {
        syn::Fields::Unnamed(_) => {
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

pub fn generate_enum(s: &synstructure::Structure, enum_item: syn::DataEnum) -> TokenStream {
    let enum_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let attrs = s.ast().attrs.clone();
    let types = s.ast().generics.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();

    let variants = enum_item.variants.into_iter().map(|variant| {
        let attrs = variant.attrs;
        let variant_ident = &variant.ident;
        let discriminant = if let Some((eq, expr)) = variant.discriminant {
            quote! { #eq #expr}
        } else {
            quote! {}
        };

        let fields: Vec<_> = wrap_upgradeable_fields(variant.fields.clone())
            .iter()
            .map(|field| {
                let mut new_field = field.clone();
                new_field.attrs = remove_attr(&new_field.attrs, "upgradeable_storage_field");
                new_field.attrs = remove_attr(&new_field.attrs, "storage_field");
                new_field
            })
            .collect::<Vec<_>>();

        let fields = match variant.fields {
            syn::Fields::Named(_) => quote! { { #(#fields),* } },
            syn::Fields::Unnamed(_) => quote! { ( #(#fields),* ) },
            syn::Fields::Unit => quote! {},
        };

        quote! {
            #(#attrs)*
            #variant_ident #fields #discriminant
        }
    });

    quote! {
        #(#attrs)*
        #vis enum #enum_ident #types #where_closure {
            #(#variants),*
        }
    }
}

pub fn generate_union(s: &synstructure::Structure, union_item: syn::DataUnion) -> TokenStream {
    let union_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let attrs = s.ast().attrs.clone();
    let types = s.ast().generics.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();

    let upgradeable_fields = wrap_upgradeable_fields(syn::Fields::Named(union_item.fields.clone()));

    let fields = upgradeable_fields.iter().map(|field| {
        let ty = &field.ty.clone();
        let new_ty = quote! { ::openbrush::traits::Lazy<#ty> };
        let mut new_field = field.clone();
        new_field.ty = Type::Verbatim(new_ty);
        new_field
    });

    quote! {
        #(#attrs)*
        #vis union #union_ident #types #where_closure {
            #(#fields),*
        }
    }
}

fn wrap_upgradeable_fields(fields: Fields) -> Vec<syn::Field> {
    fields
        .iter()
        .map(|field| {
            if is_attr(&field.attrs, "upgradeable_storage_field") {
                let mut new_field = field.clone();
                let ty = field.ty.clone().to_token_stream();
                let span = field.ty.span();
                let new_ty = syn::Type::Verbatim(quote_spanned!(span =>
                    ::openbrush::storage::Lazy<#ty>
                ));
                new_field.ty = new_ty;
                new_field
            } else {
                field.clone()
            }
        })
        .collect::<Vec<_>>()
}
