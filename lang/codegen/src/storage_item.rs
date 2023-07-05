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

use crate::internal::is_attr;
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
    quote_spanned,
    ToTokens,
};
use syn::{
    spanned::Spanned,
    Data,
    DataEnum,
    DataStruct,
    DataUnion,
    Field,
    Fields,
};

fn wrap_upgradeable_fields(structure_name: &str, fields: Fields) -> (Vec<Field>, Vec<Option<TokenStream>>) {
    fields
        .iter()
        .map(|field| {
            if is_attr(&field.attrs, "lazy") {
                let mut new_field = field.clone();
                let ty = field.ty.clone().to_token_stream();
                let span = field.ty.span();
                let field_name = field.ident.as_ref().unwrap().to_string();

                let key_name = format_ident!(
                    "STORAGE_KEY_{}_{}",
                    structure_name.to_uppercase(),
                    field_name.to_uppercase()
                );

                new_field.ty = syn::Type::Verbatim(quote_spanned!(span =>
                    ::ink::storage::Lazy<#ty, ::ink::storage::traits::ManualKey<#key_name>>
                ));
                new_field.attrs = field
                    .attrs
                    .iter()
                    .filter(|attr| !attr.path.is_ident("lazy"))
                    .cloned()
                    .collect();

                let storage_key = quote! {
                    pub const #key_name: u32 = ::openbrush::storage_unique_key!(#structure_name, #field_name);
                };

                (new_field, Some(storage_key))
            } else {
                let mut new_field = field.clone();
                let span = field.ty.span();
                let field_name = field.ident.as_ref().unwrap().to_string();

                let key_name = format_ident!(
                    "STORAGE_KEY_{}_{}",
                    structure_name.to_uppercase(),
                    field_name.to_uppercase()
                );

                let is_mapping = if let syn::Type::Path(path) = &field.ty {
                    if let Some(segment) = path.path.segments.last() {
                        segment.ident == "Mapping" || segment.ident == "MultiMapping"
                    } else {
                        false
                    }
                } else {
                    false
                };

                if let syn::Type::Path(path) = &mut new_field.ty {
                    if let Some(segment) = path.path.segments.last_mut() {
                        if segment.ident == "Mapping" || segment.ident == "MultiMapping" {
                            let mut args = segment.arguments.clone();
                            if let syn::PathArguments::AngleBracketed(args) = &mut args {
                                if let Some(syn::GenericArgument::Type(ty)) = args.args.iter_mut().nth(1) {
                                        *ty = syn::Type::Verbatim(quote_spanned!(span =>
                                            #ty, ::ink::storage::traits::ManualKey<#key_name>
                                        ));
                                }
                            }
                            segment.arguments = args;
                        }
                    }
                }

                let storage_key = if is_mapping {
                    Some(quote! {
                        pub const #key_name: u32 = ::openbrush::storage_unique_key!(#structure_name, #field_name);
                    })
                } else {
                    None
                };

                (new_field, storage_key)
            }
        })
        .unzip()
}

fn generate_struct(s: &synstructure::Structure, struct_item: DataStruct) -> TokenStream {
    let struct_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let types = s.ast().generics.clone();
    let attrs = s.ast().attrs.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();

    let (fields, storage_keys) = wrap_upgradeable_fields(struct_ident.to_string().as_str(), struct_item.fields.clone());

    match struct_item.fields {
        Fields::Unnamed(_) => {
            quote! {
                #(#attrs)*
                #vis struct #struct_ident #types #where_closure (
                    #(#fields),*
                );

                #(#storage_keys)*
            }
        }
        _ => {
            quote! {
                #(#attrs)*
                #vis struct #struct_ident #types #where_closure {
                    #(#fields),*
                }

                #(#storage_keys)*
            }
        }
    }
}

fn generate_enum(s: &synstructure::Structure, enum_item: DataEnum) -> TokenStream {
    let enum_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let attrs = s.ast().attrs.clone();
    let types = s.ast().generics.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();
    let mut all_storage_keys: Vec<Option<TokenStream>> = vec![];

    let variants = enum_item.variants.into_iter().map(|variant| {
        let attrs = variant.attrs;
        let variant_ident = &variant.ident;
        let discriminant = if let Some((eq, expr)) = variant.discriminant {
            quote! { #eq #expr}
        } else {
            quote! {}
        };

        let (fields, storage_keys) = wrap_upgradeable_fields(
            format!("{}_{}", enum_ident, variant_ident).as_str(),
            variant.fields.clone(),
        );

        let fields = match variant.fields {
            Fields::Named(_) => quote! { { #(#fields),* } },
            Fields::Unnamed(_) => quote! { ( #(#fields),* ) },
            Fields::Unit => quote! {},
        };

        all_storage_keys.extend(storage_keys);

        quote! {
            #(#attrs)*
            #variant_ident #fields #discriminant,
        }
    });

    quote! {
        #(#attrs)*
        #vis enum #enum_ident #types #where_closure {
            #(#variants),*
        }

        #(#all_storage_keys)*
    }
}

fn generate_union(s: &synstructure::Structure, union_item: DataUnion) -> TokenStream {
    let union_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let attrs = s.ast().attrs.clone();
    let types = s.ast().generics.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();

    let fields = union_item.fields.named.iter().enumerate().map(|(_i, field)| field);

    quote! {
        #(#attrs)*
        #vis union #union_ident #types #where_closure {
            #(#fields),*
        }
    }
}

pub fn storage_item(_attrs: TokenStream, s: synstructure::Structure) -> TokenStream {
    let item = match s.ast().data.clone() {
        Data::Struct(struct_item) => generate_struct(&s, struct_item),
        Data::Enum(enum_item) => generate_enum(&s, enum_item),
        Data::Union(union_item) => generate_union(&s, union_item),
    };

    quote! {
        #[::ink::storage_item]
        #item
    }
}
