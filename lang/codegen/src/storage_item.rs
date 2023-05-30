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

use crate::internal;
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};
use syn::{
    parse2,
    Data,
};

pub fn storage_item(attrs: TokenStream, s: synstructure::Structure) -> TokenStream {
    let storage_key = attrs.clone();

    let storage = storage_derive(s.clone());
    let storage_key_derived = storage_key_derive(&storage_key, s.clone());
    let storable_hint = storable_hint_derive(&storage_key, s.clone());

    let storage_key_ts = quote! {
        ::ink::storage::traits::ManualKey<#storage_key>
    };

    let item = match s.ast().data.clone() {
        Data::Struct(struct_item) => internal::generate_struct(&s, struct_item, &storage_key_ts),
        Data::Enum(enum_item) => internal::generate_enum(&s, enum_item, &storage_key_ts),
        Data::Union(union_item) => internal::generate_union(&s, union_item, &storage_key_ts),
    };

    (quote! {
        #[derive(::ink::storage::traits::Storable, Clone)]
        #[cfg_attr(feature = "std", derive(
            ::scale_info::TypeInfo,
            ::ink::storage::traits::StorageLayout
        ))]
        #item

        #storage_key_derived
        #storable_hint

        #storage
    })
    .into()
}

pub fn storage_derive(mut s: synstructure::Structure) -> TokenStream {
    s.add_bounds(synstructure::AddBounds::None).underscore_const(true);
    let storage = s.gen_impl(quote! {
        #[cfg(not(feature = "upgradeable"))]
        gen impl ::openbrush::traits::Storage<Self> for @Self {
            fn get(&self) -> &Self {
                self
            }

            fn get_mut(&mut self) -> &mut Self {
                self
            }
        }
    });

    let storage_access = s.gen_impl(quote! {
        #[cfg(not(feature = "upgradeable"))]
        gen impl ::openbrush::traits::StorageAccess<Self> for @Self {
            fn get(&self) -> Option<Self> {
                Some(self.clone())
            }

            fn set(&mut self, value: &Self) {
                *self = value.clone();
            }

            fn get_or_default(&self) -> Self {
                self.clone()
            }
        }
    });

    (quote! {
        #storage
        #storage_access
    })
    .into()
}

pub fn storage_key_derive(storage_key: &TokenStream, mut s: synstructure::Structure) -> TokenStream {
    s.add_bounds(synstructure::AddBounds::None).underscore_const(true);

    s.gen_impl(quote! {
        gen impl ::ink::storage::traits::StorageKey for @Self {
            const KEY: ::ink::primitives::Key = #storage_key;
        }
    })
}

fn storable_hint_inner(storage_key: &TokenStream, s: synstructure::Structure) -> TokenStream {
    let ident = s.ast().ident.clone();
    let salt_ident = format_ident!("__ink_generic_salt");

    let mut generics = s.ast().generics.clone();
    generics
        .params
        .push(parse2(quote! { #salt_ident : ::ink::storage::traits::StorageKey }).unwrap());

    let (impl_generics, _, where_clause) = generics.split_for_impl();
    let (_, ty_generics_original, _) = s.ast().generics.split_for_impl();

    quote! {
        impl #impl_generics ::ink::storage::traits::StorableHint<#salt_ident> for #ident #ty_generics_original #where_clause {
            type Type = #ident #ty_generics_original;
            type PreferredKey = ::ink::storage::traits::ManualKey<#storage_key>;
        }
    }
}

pub fn storable_hint_derive(storage_key: &TokenStream, s: synstructure::Structure) -> TokenStream {
    let derive = storable_hint_inner(storage_key, s);

    quote! {
        const _ : () = {
            #derive
        };
    }
}
