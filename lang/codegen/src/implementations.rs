use quote::quote;
use std::collections::HashMap;
use syn::Block;

pub(crate) fn impl_psp22(
    map: &HashMap<String, Vec<(String, Box<Block>)>>,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp22::InternalImpl for Contract {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp22::Internal for Contract {
            fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, amount: Balance) {
                psp22::InternalImpl::_emit_transfer_event(self, from, to, amount)
            }

            fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
                psp22::InternalImpl::_emit_approval_event(self, owner, spender, amount)
            }

            fn _total_supply(&self) -> Balance {
                psp22::InternalImpl::_total_supply(self)
            }

            fn _balance_of(&self, owner: &AccountId) -> Balance {
                psp22::InternalImpl::_balance_of(self, owner)
            }

            fn _allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance {
                psp22::InternalImpl::_allowance(self, owner, spender)
            }

            fn _transfer_from_to(
                &mut self,
                from: AccountId,
                to: AccountId,
                amount: Balance,
                data: Vec<u8>,
            ) -> Result<(), PSP22Error> {
                psp22::InternalImpl::_transfer_from_to(self, from, to, amount, data)
            }

            fn _approve_from_to(
                &mut self,
                owner: AccountId,
                spender: AccountId,
                amount: Balance,
            ) -> Result<(), PSP22Error> {
                psp22::InternalImpl::_approve_from_to(self, owner, spender, amount)
            }

            fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                psp22::InternalImpl::_mint_to(self, account, amount)
            }

            fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                psp22::InternalImpl::_burn_from(self, account, amount)
            }

            fn _before_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                psp22::InternalImpl::_before_token_transfer(self, from, to, amount)
            }

            fn _after_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                psp22::InternalImpl::_after_token_transfer(self, from, to, amount)
            }
        }
    ))
    .expect("Should parse");

    let psp22_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Impl for Contract {}
    ))
    .expect("Should parse");

    let mut psp22 = syn::parse2::<syn::ItemImpl>(quote!(
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

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp22::*;
    ))
    .expect("Should parse");
    imports.insert("PSP22", import);

    override_functions("psp22::Internal", &mut internal, &map);
    override_functions("PSP22", &mut psp22, &map);

    items.push(syn::Item::Impl(internal_impl));
    items.push(syn::Item::Impl(internal));
    items.push(syn::Item::Impl(psp22_impl));
    items.push(syn::Item::Impl(psp22));
}

pub(crate) fn impl_psp22_mintable(
    map: &HashMap<String, Vec<(String, Box<Block>)>>,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let mintable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22MintableImpl for Contract {}
    ))
    .expect("Should parse");

    let mut mintable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Mintable for Contract {
            #[ink(message)]
            fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                PSP22MintableImpl::mint(self, account, amount)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp22::extensions::mintable::*;
    ))
    .expect("Should parse");
    imports.insert("PSP22Mintable", import);

    override_functions("PSP22Mintable", &mut mintable, &map);

    items.push(syn::Item::Impl(mintable_impl));
    items.push(syn::Item::Impl(mintable));
}

pub(crate) fn impl_psp22_burnable(
    map: &HashMap<String, Vec<(String, Box<Block>)>>,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let burnable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22BurnableImpl for Contract {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Burnable for Contract {
            #[ink(message)]
            fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                PSP22BurnableImpl::burn(self, account, amount)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp22::extensions::burnable::*;
    ))
    .expect("Should parse");
    imports.insert("PSP22Burnable", import);

    override_functions("PSP22Burnable", &mut burnable, &map);

    items.push(syn::Item::Impl(burnable_impl));
    items.push(syn::Item::Impl(burnable));
}

pub(crate) fn impl_psp22_metadata(
    map: &HashMap<String, Vec<(String, Box<Block>)>>,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let metadata_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22MetadataImpl for Contract {}
    ))
    .expect("Should parse");

    let mut metadata = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Metadata for Contract {
            #[ink(message)]
            fn token_name(&self) -> Option<String> {
                PSP22MetadataImpl::token_name(self)
            }

            #[ink(message)]
            fn token_symbol(&self) -> Option<String> {
                PSP22MetadataImpl::token_symbol(self)
            }

            #[ink(message)]
            fn token_decimals(&self) -> u8 {
                PSP22MetadataImpl::token_decimals(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp22::extensions::metadata::*;
    ))
    .expect("Should parse");
    imports.insert("PSP22Metadata", import);

    override_functions("PSP22Metadata", &mut metadata, &map);

    items.push(syn::Item::Impl(metadata_impl));
    items.push(syn::Item::Impl(metadata));
}

pub(crate) fn impl_psp22_capped(
    map: &HashMap<String, Vec<(String, Box<Block>)>>,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl capped::InternalImpl for Contract {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl capped::Internal for Contract {
            fn _init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error> {
                capped::InternalImpl::_init_cap(self, cap)
            }

            fn _is_cap_exceeded(&self, amount: &Balance) -> bool {
                capped::InternalImpl::_is_cap_exceeded(self, amount)
            }

            fn _cap(&self) -> Balance {
                capped::InternalImpl::_cap(self)
            }
        }
    ))
    .expect("Should parse");

    let capped_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22CappedImpl for Contract {}
    ))
    .expect("Should parse");

    let mut capped = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Capped for Contract {
            #[ink(message)]
            fn cap(&self) -> Balance {
                PSP22CappedImpl::cap(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp22::extensions::capped::*;
    ))
    .expect("Should parse");
    imports.insert("PSP22Capped", import);

    override_functions("capped::Internal", &mut internal, &map);
    override_functions("PSP22Capped", &mut capped, &map);

    items.push(syn::Item::Impl(internal_impl));
    items.push(syn::Item::Impl(internal));
    items.push(syn::Item::Impl(capped_impl));
    items.push(syn::Item::Impl(capped));
}

pub(crate) fn impl_psp22_wrapper(
    map: &HashMap<String, Vec<(String, Box<Block>)>>,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl wrapper::InternalImpl for Contract {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl wrapper::Internal for Contract {
            fn _recover(&mut self, account: AccountId) -> Result<Balance, PSP22Error> {
                wrapper::InternalImpl::_recover(self, account)
            }

            fn _deposit(&mut self, amount: Balance) -> Result<(), PSP22Error> {
                wrapper::InternalImpl::_deposit(self, amount)
            }

            fn _withdraw(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                wrapper::InternalImpl::_withdraw(self, account, amount)
            }

            fn _underlying_balance(&mut self) -> Balance {
                wrapper::InternalImpl::_underlying_balance(self)
            }

            fn _init(&mut self, underlying: AccountId) {
                wrapper::InternalImpl::_init(self, underlying)
            }

            fn _underlying(&mut self) -> &mut PSP22Ref {
                wrapper::InternalImpl::_underlying(self)
            }
        }
    ))
    .expect("Should parse");

    let wrapper_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22WrapperImpl for Contract {}
    ))
    .expect("Should parse");

    let mut wrapper = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Wrapper for Contract {
            #[ink(message)]
            fn deposit_for(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                PSP22WrapperImpl::deposit_for(self, account, amount)
            }

            #[ink(message)]
            fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                PSP22WrapperImpl::withdraw_to(self, account, amount)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp22::extensions::wrapper::*;
    ))
    .expect("Should parse");
    imports.insert("PSP22Wrapper", import);

    override_functions("wrapper::Internal", &mut internal, &map);
    override_functions("PSP22Wrapper", &mut wrapper, &map);

    items.push(syn::Item::Impl(internal_impl));
    items.push(syn::Item::Impl(internal));
    items.push(syn::Item::Impl(wrapper_impl));
    items.push(syn::Item::Impl(wrapper));
}

pub(crate) fn impl_flashmint(
    map: &HashMap<String, Vec<(String, Box<Block>)>>,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl flashmint::InternalImpl for Contract {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl flashmint::Internal for Contract {
            fn _get_fee(&self, amount: Balance) -> Balance {
                flashmint::InternalImpl::_get_fee(self, amount)
            }

            fn _on_flashloan(
                &mut self,
                receiver_account: AccountId,
                token: AccountId,
                fee: Balance,
                amount: Balance,
                data: Vec<u8>,
            ) -> Result<(), FlashLenderError> {
                flashmint::InternalImpl::_on_flashloan(self, receiver_account, token, fee, amount, data)
            }
        }
    ))
    .expect("Should parse");

    let flashlender_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl FlashLenderImpl for Contract {}
    ))
    .expect("Should parse");

    let mut flashlender = syn::parse2::<syn::ItemImpl>(quote!(
        impl FlashLender for Contract {
            #[ink(message)]
            fn max_flashloan(&mut self, token: AccountId) -> Balance {
                FlashLenderImpl::max_flashloan(self, token)
            }

            #[ink(message)]
            fn flash_fee(&self, token: AccountId, amount: Balance) -> Result<Balance, FlashLenderError> {
                FlashLenderImpl::flash_fee(self, token, amount)
            }

            #[ink(message)]
            fn flashloan(
                &mut self,
                receiver_account: AccountId,
                token: AccountId,
                amount: Balance,
                data: Vec<u8>,
            ) -> Result<(), FlashLenderError> {
                FlashLenderImpl::flashloan(self, receiver_account, token, amount, data)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp22::extensions::flashmint::*;
    ))
    .expect("Should parse");
    imports.insert("Flashmint", import);

    override_functions("flashmint::Internal", &mut internal, &map);
    override_functions("FlashLender", &mut flashlender, &map);

    items.push(syn::Item::Impl(internal_impl));
    items.push(syn::Item::Impl(internal));
    items.push(syn::Item::Impl(flashlender_impl));
    items.push(syn::Item::Impl(flashlender));
}

pub(crate) fn impl_token_timelock(
    map: &HashMap<String, Vec<(String, Box<Block>)>>,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl token_timelock::InternalImpl for Contract {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl token_timelock::Internal for Contract {
            fn _withdraw(&mut self, amount: Balance) -> Result<(), PSP22TokenTimelockError> {
                token_timelock::InternalImpl::_withdraw(self, amount)
            }

            fn _contract_balance(&mut self) -> Balance {
                token_timelock::InternalImpl::_contract_balance(self)
            }

            fn _init(
                &mut self,
                token: AccountId,
                beneficiary: AccountId,
                release_time: Timestamp,
            ) -> Result<(), PSP22TokenTimelockError> {
                token_timelock::InternalImpl::_init(self, token, beneficiary, release_time)
            }

            fn _token(&mut self) -> &mut PSP22Ref {
                token_timelock::InternalImpl::_token(self)
            }

            fn _beneficiary(&self) -> AccountId {
                token_timelock::InternalImpl::_beneficiary(self)
            }
        }
    ))
    .expect("Should parse");

    let timelock_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22TokenTimelockImpl for Contract {}
    ))
    .expect("Should parse");

    let mut timelock = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22TokenTimelock for Contract {
            #[ink(message)]
            fn token(&self) -> AccountId {
                PSP22TokenTimelockImpl::token(self)
            }

            #[ink(message)]
            fn beneficiary(&self) -> AccountId {
                PSP22TokenTimelockImpl::beneficiary(self)
            }

            #[ink(message)]
            fn release_time(&self) -> Timestamp {
                PSP22TokenTimelockImpl::release_time(self)
            }

            #[ink(message)]
            fn release(&mut self) -> Result<(), PSP22TokenTimelockError> {
                PSP22TokenTimelockImpl::release(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp22::utils::token_timelock::*;
    ))
    .expect("Should parse");
    imports.insert("PSP22TokenTimelock", import);

    override_functions("token_timelock::Internal", &mut internal, &map);
    override_functions("PSP22TokenTimelock", &mut timelock, &map);

    items.push(syn::Item::Impl(internal_impl));
    items.push(syn::Item::Impl(internal));
    items.push(syn::Item::Impl(timelock_impl));
    items.push(syn::Item::Impl(timelock));
}

pub(crate) fn impl_psp22_pallet(
    map: &HashMap<String, Vec<(String, Box<Block>)>>,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp22_pallet::InternalImpl for Contract {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp22_pallet::Internal for Contract {
            fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {
                psp22_pallet::InternalImpl::_emit_transfer_event(self, _from, _to, _amount)
            }

            fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {
                psp22_pallet::InternalImpl::_emit_approval_event(self, _owner, _spender, _amount)
            }

            fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                psp22_pallet::InternalImpl::_mint_to(self, account, amount)
            }

            fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                psp22_pallet::InternalImpl::_burn_from(self, account, amount)
            }

            fn _create(
                &mut self,
                asset_id: u32,
                admin: AccountId,
                min_balance: Balance,
            ) -> Result<(), Error<DefaultEnvironment>> {
                psp22_pallet::InternalImpl::_create(self, asset_id, admin, min_balance)
            }

            fn _sender(&self) -> AccountId {
                psp22_pallet::InternalImpl::_sender(self)
            }
        }
    ))
    .expect("Should parse");

    let psp22_pallet_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22PalletImpl for Contract {}
    ))
    .expect("Should parse");

    let mut psp22 = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22 for Contract {
            #[ink(message)]
            fn total_supply(&self) -> Balance {
                PSP22PalletImpl::total_supply(self)
            }

            #[ink(message)]
            fn balance_of(&self, owner: AccountId) -> Balance {
                PSP22PalletImpl::balance_of(self, owner)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
                PSP22PalletImpl::allowance(self, owner, spender)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
                PSP22PalletImpl::transfer(self, to, value, data)
            }

            #[ink(message)]
            fn transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                value: Balance,
                data: Vec<u8>,
            ) -> Result<(), PSP22Error> {
                PSP22PalletImpl::transfer_from(self, from, to, value, data)
            }

            #[ink(message)]
            fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
                PSP22PalletImpl::approve(self, spender, value)
            }

            #[ink(message)]
            fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
                PSP22PalletImpl::increase_allowance(self, spender, delta_value)
            }

            #[ink(message)]
            fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
                PSP22PalletImpl::decrease_allowance(self, spender, delta_value)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp22_pallet::*;
    ))
    .expect("Should parse");
    imports.insert("PSP22Pallet", import);

    override_functions("psp22_pallet::Internal", &mut internal, &map);
    override_functions("PSP22", &mut psp22, &map);

    items.push(syn::Item::Impl(internal_impl));
    items.push(syn::Item::Impl(internal));
    items.push(syn::Item::Impl(psp22_pallet_impl));
    items.push(syn::Item::Impl(psp22));
}

pub(crate) fn impl_psp22_pallet_burnable(
    map: &HashMap<String, Vec<(String, Box<Block>)>>,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let burnable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22PalletBurnableImpl for Contract {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Burnable for Contract {
            #[ink(message)]
            fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                PSP22PalletBurnableImpl::burn(self, account, amount)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp22_pallet::extensions::burnable::*;
    ))
    .expect("Should parse");
    imports.insert("PSP22PalletBurnable", import);

    override_functions("PSP22Burnable", &mut burnable, &map);

    items.push(syn::Item::Impl(burnable_impl));
    items.push(syn::Item::Impl(burnable));
}

pub(crate) fn impl_psp22_pallet_metadata(
    map: &HashMap<String, Vec<(String, Box<Block>)>>,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let metadata_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22PalletMetadataImpl for Contract {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Metadata for Contract {
            #[ink(message)]
            fn token_name(&self) -> Option<String> {
                PSP22PalletMetadataImpl::token_name(self)
            }

            #[ink(message)]
            fn token_symbol(&self) -> Option<String> {
                PSP22PalletMetadataImpl::token_symbol(self)
            }

            #[ink(message)]
            fn token_decimals(&self) -> u8 {
                PSP22PalletMetadataImpl::token_decimals(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp22_pallet::extensions::metadata::*;
    ))
    .expect("Should parse");
    imports.insert("PSP22PalletMetadata", import);

    override_functions("PSP22Metadata", &mut burnable, &map);

    items.push(syn::Item::Impl(metadata_impl));
    items.push(syn::Item::Impl(burnable));
}

pub(crate) fn impl_psp22_pallet_mintable(
    map: &HashMap<String, Vec<(String, Box<Block>)>>,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let mintable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22PalletMintableImpl for Contract {}
    ))
    .expect("Should parse");

    let mut mintable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Mintable for Contract {
            #[ink(message)]
            fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                PSP22PalletMintableImpl::mint(self, account, amount)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp22_pallet::extensions::mintable::*;
    ))
    .expect("Should parse");
    imports.insert("PSP22PalletMintable", import);

    override_functions("PSP22Mintable", &mut mintable, &map);

    items.push(syn::Item::Impl(mintable_impl));
    items.push(syn::Item::Impl(mintable));
}

fn override_functions(
    trait_name: &str,
    implementation: &mut syn::ItemImpl,
    map: &HashMap<String, Vec<(String, Box<Block>)>>,
) {
    if let Some(overrides) = map.get(trait_name) {
        // we will find which fns we wanna override
        for (fn_name, fn_code) in overrides {
            for item in implementation.items.iter_mut() {
                if let syn::ImplItem::Method(method) = item {
                    if &method.sig.ident.to_string() == fn_name {
                        method.block = *fn_code.clone();
                    }
                }
            }
        }
    }
}
