use quote::quote;
use std::collections::HashMap;
use syn::Block;

pub type IsDefault = bool;
pub type OverridenFnMap = HashMap<String, Vec<(String, (Box<Block>, Vec<syn::Attribute>, IsDefault))>>;

pub(crate) fn impl_psp22(map: &OverridenFnMap, items: &mut Vec<syn::Item>, imports: &mut HashMap<&str, syn::ItemUse>) {
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
    map: &OverridenFnMap,
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
    map: &OverridenFnMap,
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
    map: &OverridenFnMap,
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
    map: &OverridenFnMap,
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
    map: &OverridenFnMap,
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
    map: &OverridenFnMap,
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
    map: &OverridenFnMap,
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
    map: &OverridenFnMap,
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
    map: &OverridenFnMap,
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
    map: &OverridenFnMap,
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
    map: &OverridenFnMap,
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

pub(crate) fn impl_psp34(
    map: &OverridenFnMap,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
    overriden_traits: &mut HashMap<&str, syn::Item>,
) {
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp34::InternalImpl for Contract {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp34::Internal for Contract {
            fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
                psp34::InternalImpl::_emit_transfer_event(self, from, to, id)
            }

            fn _emit_approval_event(&self, from: AccountId, to: AccountId, id: Option<Id>, approved: bool) {
                psp34::InternalImpl::_emit_approval_event(self, from, to, id, approved)
            }

            fn _approve_for(&mut self, to: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
                psp34::InternalImpl::_approve_for(self, to, id, approved)
            }

            fn _owner_of(&self, id: &Id) -> Option<AccountId> {
                psp34::InternalImpl::_owner_of(self, id)
            }

            fn _transfer_token(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
                psp34::InternalImpl::_transfer_token(self, to, id, data)
            }

            fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error> {
                psp34::InternalImpl::_mint_to(self, to, id)
            }

            fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error> {
                psp34::InternalImpl::_burn_from(self, from, id)
            }

            fn _allowance(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> bool {
                psp34::InternalImpl::_allowance(self, owner, operator, id)
            }

            fn _check_token_exists(&self, id: &Id) -> Result<AccountId, PSP34Error> {
                psp34::InternalImpl::_check_token_exists(self, id)
            }

            fn _before_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                id: &Id,
            ) -> Result<(), PSP34Error> {
                psp34::InternalImpl::_before_token_transfer(self, from, to, id)
            }

            fn _after_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                id: &Id,
            ) -> Result<(), PSP34Error> {
                psp34::InternalImpl::_after_token_transfer(self, from, to, id)
            }
        }
    ))
    .expect("Should parse");

    let psp34_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34Impl for Contract {}
    ))
    .expect("Should parse");

    let mut psp34 = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34 for Contract {
            #[ink(message)]
            fn collection_id(&self) -> Id {
                PSP34Impl::collection_id(self)
            }

            #[ink(message)]
            fn balance_of(&self, owner: AccountId) -> u32 {
                PSP34Impl::balance_of(self, owner)
            }

            #[ink(message)]
            fn owner_of(&self, id: Id) -> Option<AccountId> {
                PSP34Impl::owner_of(self, id)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
                PSP34Impl::allowance(self, owner, operator, id)
            }

            #[ink(message)]
            fn approve(&mut self, operator: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
                PSP34Impl::approve(self, operator, id, approved)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
                PSP34Impl::transfer(self, to, id, data)
            }

            #[ink(message)]
            fn total_supply(&self) -> Balance {
                PSP34Impl::total_supply(self)
            }
        }
    ))
    .expect("Should parse");

    let psp34_balances_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp34::BalancesManagerImpl for Contract {}
    ))
    .expect("Should parse");

    let mut psp34_balances = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp34::BalancesManager for Contract {
            fn _balance_of(&self, owner: &Owner) -> u32 {
                psp34::BalancesManagerImpl::_balance_of(self, owner)
            }

            fn _increase_balance(&mut self, owner: &Owner, id: &Id, increase_supply: bool) {
                psp34::BalancesManagerImpl::_increase_balance(self, owner, id, increase_supply)
            }

            fn _decrease_balance(&mut self, owner: &Owner, id: &Id, decrease_supply: bool) {
                psp34::BalancesManagerImpl::_decrease_balance(self, owner, id, decrease_supply)
            }

            fn _total_supply(&self) -> u128 {
                psp34::BalancesManagerImpl::_total_supply(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp34::*;
    ))
    .expect("Should parse");
    imports.insert("PSP34", import);

    override_functions("psp34::BalancesManager", &mut psp34_balances, &map);
    override_functions("psp34::Internal", &mut internal, &map);
    override_functions("PSP34", &mut psp34, &map);

    // only insert this if it is not present
    overriden_traits
        .entry("psp34::BalancesManager")
        .or_insert(syn::Item::Impl(psp34_balances));

    items.push(syn::Item::Impl(psp34_balances_impl));
    items.push(syn::Item::Impl(internal_impl));
    items.push(syn::Item::Impl(internal));
    items.push(syn::Item::Impl(psp34_impl));
    items.push(syn::Item::Impl(psp34));
}

pub(crate) fn impl_psp34_burnable(
    map: &OverridenFnMap,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let burnable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34BurnableImpl for Contract {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34Burnable for Contract {
            #[ink(message)]
            fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
                PSP34BurnableImpl::burn(self, account, id)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp34::extensions::burnable::*;
    ))
    .expect("Should parse");
    imports.insert("PSP34Burnable", import);

    override_functions("PSP34Burnable", &mut burnable, &map);

    items.push(syn::Item::Impl(burnable_impl));
    items.push(syn::Item::Impl(burnable));
}

pub(crate) fn impl_psp34_mintable(
    map: &OverridenFnMap,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let mintable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34MintableImpl for Contract {}
    ))
    .expect("Should parse");

    let mut mintable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34Mintable for Contract {
            #[ink(message)]
            fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
                PSP34MintableImpl::mint(self, account, id)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp34::extensions::mintable::*;
    ))
    .expect("Should parse");
    imports.insert("PSP34Mintable", import);

    override_functions("PSP34Mintable", &mut mintable, &map);

    items.push(syn::Item::Impl(mintable_impl));
    items.push(syn::Item::Impl(mintable));
}

pub(crate) fn impl_psp34_metadata(
    map: &OverridenFnMap,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl metadata::InternalImpl for Contract {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl metadata::Internal for Contract {
            fn _emit_attribute_set_event(&self, id: Id, key: String, data: String) {
                metadata::InternalImpl::_emit_attribute_set_event(self, id, key, data)
            }

            fn _set_attribute(&mut self, id: Id, key: String, value: String) {
                metadata::InternalImpl::_set_attribute(self, id, key, value)
            }
        }
    ))
    .expect("Should parse");

    let metadata_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34MetadataImpl for Contract {}
    ))
    .expect("Should parse");

    let mut metadata = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34Metadata for Contract {
            #[ink(message)]
            fn get_attribute(&self, id: Id, key: String) -> Option<String> {
                PSP34MetadataImpl::get_attribute(self, id, key)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp34::extensions::metadata::*;
    ))
    .expect("Should parse");
    imports.insert("PSP34Metadata", import);

    override_functions("metadata::Internal", &mut internal, &map);
    override_functions("PSP34Mintable", &mut metadata, &map);

    items.push(syn::Item::Impl(internal_impl));
    items.push(syn::Item::Impl(internal));
    items.push(syn::Item::Impl(metadata_impl));
    items.push(syn::Item::Impl(metadata));
}

pub(crate) fn impl_psp34_enumerable(
    map: &OverridenFnMap,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
    overriden_traits: &mut HashMap<&str, syn::Item>,
) {
    let enumerable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34EnumerableImpl for Contract {}
    ))
    .expect("Should parse");

    let mut psp34_enumerable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34Enumerable for Contract {
            #[ink(message)]
            fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error> {
                PSP34EnumerableImpl::owners_token_by_index(self, owner, index)
            }

            #[ink(message)]
            fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error> {
                PSP34EnumerableImpl::token_by_index(self, index)
            }
        }

    ))
    .expect("Should parse");

    let psp34_balances_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl enumerable::BalancesManagerImpl for Contract {}
    ))
    .expect("Should parse");

    let mut psp34_balances = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp34::BalancesManager for Contract {
            fn _balance_of(&self, owner: &Owner) -> u32 {
                enumerable::BalancesManagerImpl::_balance_of(self, owner)
            }

            fn _increase_balance(&mut self, owner: &Owner, id: &Id, increase_supply: bool) {
                enumerable::BalancesManagerImpl::_increase_balance(self, owner, id, increase_supply)
            }

            fn _decrease_balance(&mut self, owner: &Owner, id: &Id, decrease_supply: bool) {
                enumerable::BalancesManagerImpl::_decrease_balance(self, owner, id, decrease_supply)
            }

            fn _total_supply(&self) -> u128 {
                enumerable::BalancesManagerImpl::_total_supply(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp34::extensions::enumerable::*;
    ))
    .expect("Should parse");
    imports.insert("PSP34Enumerable", import);

    override_functions("enumerable::BalancesManager", &mut psp34_balances, &map);
    override_functions("PSP34Enumerable", &mut psp34_enumerable, &map);

    overriden_traits.insert("psp34::BalancesManager", syn::Item::Impl(psp34_balances));

    items.push(syn::Item::Impl(psp34_balances_impl));
    items.push(syn::Item::Impl(enumerable_impl));
    items.push(syn::Item::Impl(psp34_enumerable));
}

pub(crate) fn impl_psp37(
    map: &OverridenFnMap,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
    overriden_traits: &mut HashMap<&str, syn::Item>,
) {
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp37::InternalImpl for Contract {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp37::Internal for Contract {
            fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id, amount: Balance) {
                psp37::InternalImpl::_emit_transfer_event(self, from, to, id, amount)
            }

            fn _emit_transfer_batch_event(
                &self,
                from: Option<AccountId>,
                to: Option<AccountId>,
                ids_amounts: Vec<(Id, Balance)>,
            ) {
                psp37::InternalImpl::_emit_transfer_batch_event(self, from, to, ids_amounts)
            }

            fn _emit_approval_event(&self, owner: AccountId, operator: AccountId, id: Option<Id>, value: Balance) {
                psp37::InternalImpl::_emit_approval_event(self, owner, operator, id, value)
            }

            fn _mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_mint_to(self, to, ids_amounts)
            }

            fn _burn_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_burn_from(self, from, ids_amounts)
            }

            fn _transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                id: Id,
                amount: Balance,
                data: Vec<u8>,
            ) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_transfer_from(self, from, to, id, amount, data)
            }

            fn _get_allowance(&self, account: &AccountId, operator: &AccountId, id: &Option<&Id>) -> Balance {
                psp37::InternalImpl::_get_allowance(self, account, operator, id)
            }

            fn _approve_for(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_approve_for(self, operator, id, value)
            }

            fn _decrease_allowance(
                &mut self,
                owner: &AccountId,
                operator: &AccountId,
                id: &Id,
                value: Balance,
            ) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_decrease_allowance(self, owner, operator, id, value)
            }

            fn _transfer_token(
                &mut self,
                from: &AccountId,
                to: &AccountId,
                id: Id,
                amount: Balance,
                data: &Vec<u8>,
            ) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_transfer_token(self, from, to, id, amount, data)
            }

            fn _before_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                ids: &Vec<(Id, Balance)>,
            ) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_before_token_transfer(self, from, to, ids)
            }

            fn _after_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                ids: &Vec<(Id, Balance)>,
            ) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_after_token_transfer(self, from, to, ids)
            }
        }

    ))
    .expect("Should parse");

    let psp37_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37Impl for Contract {}
    ))
    .expect("Should parse");

    let mut psp37 = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37 for Contract {
            #[ink(message)]
            fn balance_of(&self, owner: AccountId, id: Option<Id>) -> Balance {
                PSP37Impl::balance_of(self, owner, id)
            }

            #[ink(message)]
            fn total_supply(&self, id: Option<Id>) -> Balance {
                PSP37Impl::total_supply(self, id)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> Balance {
                PSP37Impl::allowance(self, owner, operator, id)
            }

            #[ink(message)]
            fn approve(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), PSP37Error> {
                PSP37Impl::approve(self, operator, id, value)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, id: Id, value: Balance, data: Vec<u8>) -> Result<(), PSP37Error> {
                PSP37Impl::transfer(self, to, id, value, data)
            }

            #[ink(message)]
            fn transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                id: Id,
                value: Balance,
                data: Vec<u8>,
            ) -> Result<(), PSP37Error> {
                PSP37Impl::transfer_from(self, from, to, id, value, data)
            }
        }
    ))
    .expect("Should parse");

    let psp37_balances_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp37::BalancesManagerImpl for Contract {}
    ))
    .expect("Should parse");

    let mut psp37_balances = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp37::BalancesManager for Contract {
            fn _balance_of(&self, owner: &AccountId, id: &Option<&Id>) -> Balance {
                psp37::BalancesManagerImpl::_balance_of(self, owner, id)
            }

            fn _total_supply(&self, id: &Option<&Id>) -> Balance {
                psp37::BalancesManagerImpl::_total_supply(self, id)
            }

            fn _increase_balance(
                &mut self,
                owner: &AccountId,
                id: &Id,
                amount: &Balance,
                mint: bool,
            ) -> Result<(), PSP37Error> {
                psp37::BalancesManagerImpl::_increase_balance(self, owner, id, amount, mint)
            }

            fn _decrease_balance(
                &mut self,
                owner: &AccountId,
                id: &Id,
                amount: &Balance,
                burn: bool,
            ) -> Result<(), PSP37Error> {
                psp37::BalancesManagerImpl::_decrease_balance(self, owner, id, amount, burn)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp37::*;
    ))
    .expect("Should parse");
    imports.insert("PSP37", import);

    override_functions("psp37::BalancesManager", &mut psp37_balances, &map);
    override_functions("psp37::Internal", &mut internal, &map);
    override_functions("PSP37", &mut psp37, &map);

    // only insert this if it is not present
    overriden_traits
        .entry("psp37::BalancesManager")
        .or_insert(syn::Item::Impl(psp37_balances));

    items.push(syn::Item::Impl(psp37_balances_impl));
    items.push(syn::Item::Impl(internal_impl));
    items.push(syn::Item::Impl(internal));
    items.push(syn::Item::Impl(psp37_impl));
    items.push(syn::Item::Impl(psp37));
}

pub(crate) fn impl_psp37_batch(
    map: &OverridenFnMap,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl batch::InternalImpl for Contract {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl batch::Internal for Contract {
            fn _batch_transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                ids_amounts: Vec<(Id, Balance)>,
                data: Vec<u8>,
            ) -> Result<(), PSP37Error> {
                batch::InternalImpl::_batch_transfer_from(self, from, to, ids_amounts, data)
            }
        }
    ))
    .expect("Should parse");

    let batch_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37BatchImpl for Contract {}
    ))
    .expect("Should parse");

    let mut batch = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37Batch for Contract {
            #[ink(message)]
            fn batch_transfer(
                &mut self,
                to: AccountId,
                ids_amounts: Vec<(Id, Balance)>,
                data: Vec<u8>,
            ) -> Result<(), PSP37Error> {
                PSP37BatchImpl::batch_transfer(self, to, ids_amounts, data)
            }

            #[ink(message)]
            fn batch_transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                ids_amounts: Vec<(Id, Balance)>,
                data: Vec<u8>,
            ) -> Result<(), PSP37Error> {
                PSP37BatchImpl::batch_transfer_from(self, from, to, ids_amounts, data)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp37::extensions::batch::*;
    ))
    .expect("Should parse");
    imports.insert("PSP37Batch", import);

    override_functions("batch::Internal", &mut internal, &map);
    override_functions("PSP37Batch", &mut batch, &map);

    items.push(syn::Item::Impl(internal_impl));
    items.push(syn::Item::Impl(internal));
    items.push(syn::Item::Impl(batch_impl));
    items.push(syn::Item::Impl(batch));
}

pub(crate) fn impl_psp37_burnable(
    map: &OverridenFnMap,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let burnable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37BurnableImpl for Contract {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37Burnable for Contract {
            #[ink(message)]
            fn burn(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
                PSP37BurnableImpl::burn(self, from, ids_amounts)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp37::extensions::burnable::*;
    ))
    .expect("Should parse");
    imports.insert("PSP37Burnable", import);

    override_functions("PSP37Burnable", &mut burnable, &map);

    items.push(syn::Item::Impl(burnable_impl));
    items.push(syn::Item::Impl(burnable));
}

pub(crate) fn impl_psp37_metadata(
    map: &OverridenFnMap,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl metadata::InternalImpl for Contract {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl metadata::Internal for Contract {
            fn _emit_attribute_set_event(&self, id: &Id, key: &String, data: &String) {
                metadata::InternalImpl::_emit_attribute_set_event(self, id, key, data);
            }

            fn _set_attribute(&mut self, id: &Id, key: &String, data: &String) -> Result<(), PSP37Error> {
                metadata::InternalImpl::_set_attribute(self, id, key, data)
            }

            fn _get_attribute(&self, id: &Id, key: &String) -> Option<String> {
                metadata::InternalImpl::_get_attribute(self, id, key)
            }
        }
    ))
    .expect("Should parse");

    let metadata_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37MetadataImpl for Contract {}
    ))
    .expect("Should parse");

    let mut metadata = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37Metadata for Contract {
            #[ink(message)]
            fn get_attribute(&self, id: Id, key: String) -> Option<String> {
                PSP37MetadataImpl::get_attribute(self, id, key)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp37::extensions::metadata::*;
    ))
    .expect("Should parse");
    imports.insert("PSP37Metadata", import);

    override_functions("metadata::Internal", &mut internal, &map);
    override_functions("PSP37Metadata", &mut metadata, &map);

    items.push(syn::Item::Impl(internal_impl));
    items.push(syn::Item::Impl(internal));
    items.push(syn::Item::Impl(metadata_impl));
    items.push(syn::Item::Impl(metadata));
}

pub(crate) fn impl_psp37_mintable(
    map: &OverridenFnMap,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let mintable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37MintableImpl for Contract {}
    ))
    .expect("Should parse");

    let mut mintable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37Mintable for Contract {
            #[ink(message)]
            fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
                PSP37MintableImpl::mint(self, to, ids_amounts)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp37::extensions::mintable::*;
    ))
    .expect("Should parse");
    imports.insert("PSP37Mintable", import);

    override_functions("PSP37Mintable", &mut mintable, &map);

    items.push(syn::Item::Impl(mintable_impl));
    items.push(syn::Item::Impl(mintable));
}

pub(crate) fn impl_psp37_enumerable(
    map: &OverridenFnMap,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
    overriden_traits: &mut HashMap<&str, syn::Item>,
) {
    let enumerable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37EnumerableImpl for Contract {}
    ))
    .expect("Should parse");

    let mut psp37_enumerable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37Enumerable for Contract {
            #[ink(message)]
            fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Option<Id> {
                PSP37EnumerableImpl::owners_token_by_index(self, owner, index)
            }

            #[ink(message)]
            fn token_by_index(&self, index: u128) -> Option<Id> {
                PSP37EnumerableImpl::token_by_index(self, index)
            }
        }
    ))
    .expect("Should parse");

    let psp37_balances_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl enumerable::BalancesManagerImpl for Contract {}
    ))
    .expect("Should parse");

    let mut psp37_balances = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp37::BalancesManager for Contract {
            fn _balance_of(&self, owner: &AccountId, id: &Option<&Id>) -> Balance {
                enumerable::BalancesManagerImpl::_balance_of(self, owner, id)
            }

            fn _total_supply(&self, id: &Option<&Id>) -> Balance {
                enumerable::BalancesManagerImpl::_total_supply(self, id)
            }

            fn _increase_balance(
                &mut self,
                owner: &AccountId,
                id: &Id,
                amount: &Balance,
                mint: bool,
            ) -> Result<(), PSP37Error> {
                enumerable::BalancesManagerImpl::_increase_balance(self, owner, id, amount, mint)
            }

            fn _decrease_balance(
                &mut self,
                owner: &AccountId,
                id: &Id,
                amount: &Balance,
                burn: bool,
            ) -> Result<(), PSP37Error> {
                enumerable::BalancesManagerImpl::_decrease_balance(self, owner, id, amount, burn)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::psp37::extensions::enumerable::*;
    ))
    .expect("Should parse");
    imports.insert("PSP37Enumerable", import);

    override_functions("enumerable::BalancesManager", &mut psp37_balances, &map);
    override_functions("PSP37Enumerable", &mut psp37_enumerable, &map);

    overriden_traits.insert("psp37::BalancesManager", syn::Item::Impl(psp37_balances));

    items.push(syn::Item::Impl(psp37_balances_impl));
    items.push(syn::Item::Impl(enumerable_impl));
    items.push(syn::Item::Impl(psp37_enumerable));
}

pub(crate) fn impl_ownable(
    map: &OverridenFnMap,
    items: &mut Vec<syn::Item>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl ownable::InternalImpl for Contract {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl ownable::Internal for Contract {
            fn _emit_ownership_transferred_event(&self, previous: Option<AccountId>, new: Option<AccountId>) {
                ownable::InternalImpl::_emit_ownership_transferred_event(self, previous, new)
            }

            fn _init_with_owner(&mut self, owner: AccountId) {
                ownable::InternalImpl::_init_with_owner(self, owner)
            }
        }
    ))
    .expect("Should parse");

    let ownable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl OwnableImpl for Contract {}
    ))
    .expect("Should parse");

    let mut ownable = syn::parse2::<syn::ItemImpl>(quote!(
        impl Ownable for Contract {
            #[ink(message)]
            fn owner(&self) -> AccountId {
                OwnableImpl::owner(self)
            }

            #[ink(message)]
            fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
                OwnableImpl::renounce_ownership(self)
            }

            #[ink(message)]
            fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError> {
                OwnableImpl::transfer_ownership(self, new_owner)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use openbrush::contracts::ownable::*;
    ))
    .expect("Should parse");
    imports.insert("Ownable", import);

    override_functions("ownable::Internal", &mut internal, &map);
    override_functions("Ownable", &mut ownable, &map);

    items.push(syn::Item::Impl(internal_impl));
    items.push(syn::Item::Impl(internal));
    items.push(syn::Item::Impl(ownable_impl));
    items.push(syn::Item::Impl(ownable));
}

fn override_functions(trait_name: &str, implementation: &mut syn::ItemImpl, map: &OverridenFnMap) {
    if let Some(overrides) = map.get(trait_name) {
        // we will find which fns we wanna override
        for (fn_name, (fn_code, attributes, is_default)) in overrides {
            for item in implementation.items.iter_mut() {
                if let syn::ImplItem::Method(method) = item {
                    if &method.sig.ident.to_string() == fn_name {
                        if !is_default {
                            method.block = *fn_code.clone();
                        }
                        method.attrs.append(&mut attributes.to_vec());
                    }
                }
            }
        }
    }
}
