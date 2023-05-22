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

use proc_macro2::TokenStream;
use quote::{
    quote,
    quote_spanned,
    ToTokens,
};
use syn::{
    parse2,
    spanned::Spanned,
    Data,
    DeriveInput,
};

pub fn storage_derive(item: TokenStream) -> TokenStream {
    let derive: syn::DeriveInput = parse2(item).expect("Expected DeriveInput");

    let struct_ident = derive.ident.clone();
    let (impls, types, where_clause) = derive.generics.split_for_impl();

    let key_salt = salt(&derive.clone());

    let fields: Vec<_> = match &derive.data {
        Data::Struct(st) => {
            st.fields
                .iter()
                .enumerate()
                .map(|(i, field)| convert_into_storage_field(&struct_ident, None, &key_salt, i, field))
                .collect()
        }
        Data::Enum(en) => {
            en.variants
                .iter()
                .flat_map(|v| {
                    v.fields.iter().enumerate().map(|(i, field)| {
                        convert_into_storage_field(&struct_ident, Some(&v.ident), &key_salt, i, field)
                    })
                })
                .collect()
        }
        Data::Union(un) => {
            un.fields
                .named
                .iter()
                .enumerate()
                .map(|(i, field)| convert_into_storage_field(&struct_ident, None, &key_salt, i, field))
                .collect()
        }
    };

    let impls = fields
        .iter()
        .filter(|field| field.attrs.iter().find(|a| a.path.is_ident("storage_field")).is_some())
        .map(|field| {
            let field_ident = field.ident.clone();
            let ty = field.ty.clone();
            let span = field.span();

            quote_spanned!(span=>
                impl #impls ::openbrush::traits::Storage<#ty> for #struct_ident #types #where_clause {
                    fn get(&self) -> &#ty {
                        &self.#field_ident
                    }

                    fn get_mut(&mut self) -> &mut #ty {
                        &mut self.#field_ident
                    }
                }

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

    (quote! {
        #(#impls)*
    })
    .into()
}

fn convert_into_storage_field(
    struct_ident: &syn::Ident,
    variant_ident: Option<&syn::Ident>,
    storage_key: &TokenStream,
    index: usize,
    field: &syn::Field,
) -> syn::Field {
    let field_name = if let Some(field_ident) = &field.ident {
        field_ident.to_string()
    } else {
        index.to_string()
    };

    let variant_name = if let Some(variant_ident) = variant_ident {
        variant_ident.to_string()
    } else {
        "".to_string()
    };

    let key = ::ink_primitives::KeyComposer::compute_key(
        struct_ident.to_string().as_str(),
        variant_name.as_str(),
        field_name.as_str(),
    )
    .expect("unable to compute the storage key for the field");

    let mut new_field = field.clone();
    let ty = field.ty.clone().to_token_stream();
    let span = field.ty.span();
    let new_ty = syn::Type::Verbatim(quote_spanned!(span =>
        <#ty as ::ink::storage::traits::AutoStorableHint<
            ::ink::storage::traits::ManualKey<#key, #storage_key>,
        >>::Type
    ));
    new_field.ty = new_ty;
    new_field
}

fn salt(s: &DeriveInput) -> TokenStream {
    if let Some(param) = find_storage_key_salt(&s) {
        param.ident.to_token_stream()
    } else {
        quote! { () }
    }
}
fn find_storage_key_salt(input: &DeriveInput) -> Option<syn::TypeParam> {
    input.generics.params.iter().find_map(|param| {
        if let syn::GenericParam::Type(type_param) = param {
            if let Some(syn::TypeParamBound::Trait(trait_bound)) = type_param.bounds.first() {
                let segments = &trait_bound.path.segments;
                if let Some(last) = segments.last() {
                    if last.ident == "StorageKey" {
                        return Some(type_param.clone())
                    }
                }
            }
        }
        None
    })
}
