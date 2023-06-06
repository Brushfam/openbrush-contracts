#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod diamond {
    use openbrush::{
        contracts::diamond::extensions::diamond_loupe::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        diamond: diamond::Data,
        #[storage_field]
        loupe: diamond_loupe::Data,
    }

    impl ownable::InternalImpl for Contract {}

    impl ownable::Internal for Contract {
        fn _emit_ownership_transferred_event(&self, previous: Option<AccountId>, new: Option<AccountId>) {
            ownable::InternalImpl::_emit_ownership_transferred_event(self, previous, new)
        }

        fn _init_with_owner(&mut self, owner: AccountId) {
            ownable::InternalImpl::_init_with_owner(self, owner)
        }
    }

    impl OwnableImpl for Contract {}

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

    impl diamond::InternalImpl for Contract {}

    impl diamond::Internal for Contract {
        fn _emit_diamond_cut_event(&self, diamond_cut: &Vec<FacetCut>, init: &Option<InitCall>) {
            diamond::InternalImpl::_emit_diamond_cut_event(self, diamond_cut, init)
        }

        fn _diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError> {
            diamond::InternalImpl::_diamond_cut(self, diamond_cut, init)
        }

        fn _diamond_cut_facet(&mut self, facet_cut: &FacetCut) -> Result<(), DiamondError> {
            diamond::InternalImpl::_diamond_cut_facet(self, facet_cut)
        }

        fn _fallback(&self) -> ! {
            diamond::InternalImpl::_fallback(self)
        }

        fn _init_call(&self, call: InitCall) -> ! {
            diamond::InternalImpl::_init_call(self, call)
        }

        fn _remove_facet(&mut self, code_hash: Hash) {
            diamond::InternalImpl::_remove_facet(self, code_hash)
        }

        fn _remove_selectors(&mut self, facet_cut: &FacetCut) {
            diamond::InternalImpl::_remove_selectors(self, facet_cut)
        }
    }

    impl diamond_loupe::DiamondCutLoupeImpl for Contract {}

    impl diamond::DiamondCut for Contract {
        fn _on_add_facet(&mut self, code_hash: Hash) {
            diamond_loupe::DiamondCutLoupeImpl::_on_add_facet(self, code_hash)
        }

        fn _on_remove_facet(&mut self, code_hash: Hash) {
            diamond_loupe::DiamondCutLoupeImpl::_on_remove_facet(self, code_hash)
        }
    }

    impl DiamondImpl for Contract {}

    impl Diamond for Contract {
        #[ink(message)]
        fn diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError> {
            DiamondImpl::diamond_cut(self, diamond_cut, init)
        }
    }

    impl DiamondLoupeImpl for Contract {}

    impl DiamondLoupe for Contract {
        #[ink(message)]
        fn facets(&self) -> Vec<FacetCut> {
            diamond_loupe::DiamondLoupeImpl::facets(self)
        }

        #[ink(message)]
        fn facet_function_selectors(&self, facet: Hash) -> Vec<Selector> {
            diamond_loupe::DiamondLoupeImpl::facet_function_selectors(self, facet)
        }

        #[ink(message)]
        fn facet_code_hashes(&self) -> Vec<Hash> {
            diamond_loupe::DiamondLoupeImpl::facet_code_hashes(self)
        }

        #[ink(message)]
        fn facet_code_hash(&self, selector: Selector) -> Option<Hash> {
            diamond_loupe::DiamondLoupeImpl::facet_code_hash(self, selector)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            let mut instance = Self::default();
            ownable::Internal::_init_with_owner(&mut instance, owner);

            instance
        }

        #[ink(message, payable, selector = _)]
        pub fn forward(&self) {
            diamond::Internal::_fallback(self)
        }
    }
}
