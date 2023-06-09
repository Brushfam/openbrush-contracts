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

use std::collections::HashMap;

use crate::{
    internal,
    internal::*,
};
use proc_macro2::TokenStream;
use quote::{
    quote,
    ToTokens,
};
use syn::{
    Block,
    Item,
    Path,
};

pub fn generate(_: TokenStream, ink_module: TokenStream) -> TokenStream {
    if internal::skip() {
        return (quote! {}).into()
    }
    let input: TokenStream = ink_module.into();
    let mut module = syn::parse2::<syn::ItemMod>(input.clone()).expect("Can't parse contract module");
    let (braces, items) = match module.clone().content {
        Some((brace, items)) => (brace, items),
        None => {
            panic!(
                "{}",
                "out-of-line openbrush modules are not supported, use `#[openbrush::contract] mod name {{ ... }}`",
            )
        }
    };

    // we will look for overriden functions and remove them from the mod
    let (map, mut items) = consume_overriders(items);

    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp22::InternalImpl for Contract {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp22::Internal for Contract {
            fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, amount: Balance) {
                InternalImpl::_emit_transfer_event(self, from, to, amount)
            }

            fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
                InternalImpl::_emit_approval_event(self, owner, spender, amount)
            }

            fn _total_supply(&self) -> Balance {
                InternalImpl::_total_supply(self)
            }

            fn _balance_of(&self, owner: &AccountId) -> Balance {
                InternalImpl::_balance_of(self, owner)
            }

            fn _allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance {
                InternalImpl::_allowance(self, owner, spender)
            }

            fn _transfer_from_to(
                &mut self,
                from: AccountId,
                to: AccountId,
                amount: Balance,
                data: Vec<u8>,
            ) -> Result<(), PSP22Error> {
                InternalImpl::_transfer_from_to(self, from, to, amount, data)
            }

            fn _approve_from_to(
                &mut self,
                owner: AccountId,
                spender: AccountId,
                amount: Balance,
            ) -> Result<(), PSP22Error> {
                InternalImpl::_approve_from_to(self, owner, spender, amount)
            }

            fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                InternalImpl::_mint_to(self, account, amount)
            }

            fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                InternalImpl::_burn_from(self, account, amount)
            }

            fn _before_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                InternalImpl::_before_token_transfer(self, from, to, amount)
            }

            fn _after_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                InternalImpl::_after_token_transfer(self, from, to, amount)
            }
        }
    ))
    .expect("Should parse");

    let psp22_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Impl for Contract {}
    ))
    .expect("Should parse");

    let psp22 = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22 for Contract {
            #[ink(message)]
            fn total_supply(&self) -> Balance {
                PSP22Impl::total_supply(self)
            }

            #[ink(message)]
            fn balance_of(&self, owner: AccountId) -> Balance {
                PSP22Impl::balance_of(self, owner)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
                PSP22Impl::allowance(self, owner, spender)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
                PSP22Impl::transfer(self, to, value, data)
            }

            #[ink(message)]
            fn transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                value: Balance,
                data: Vec<u8>,
            ) -> Result<(), PSP22Error> {
                PSP22Impl::transfer_from(self, from, to, value, data)
            }

            #[ink(message)]
            fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
                PSP22Impl::approve(self, spender, value)
            }

            #[ink(message)]
            fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
                PSP22Impl::increase_allowance(self, spender, delta_value)
            }

            #[ink(message)]
            fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
                PSP22Impl::decrease_allowance(self, spender, delta_value)
            }
        }
    ))
    .expect("Should parse");

    if let Some(overrides) = map.get("psp22::Internal") {
        // we will find which fns we wanna override
        for (fn_name, fn_code) in overrides {
            for item in internal.items.iter_mut() {
                if let syn::ImplItem::Method(method) = item {
                    if &method.sig.ident.to_string() == fn_name {
                        method.block = *fn_code.clone();
                    }
                }
            }
        }
    }

    items.push(syn::Item::Impl(internal_impl));
    items.push(syn::Item::Impl(internal));
    items.push(syn::Item::Impl(psp22_impl));
    items.push(syn::Item::Impl(psp22));

    module.content = Some((braces.clone(), items));

    let result = quote! {
        #module
    };
    result.into()
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
