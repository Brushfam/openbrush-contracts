#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// This will be a simple lending contract where users can:
///
/// 1. Lend tokens accepted by the smart contract.
/// The allowance and disallowance of tokens is done by the accounts which have a manager role
/// Upon lending, the user gets a PSP22 token representing their share of the current liquidity pool
///
/// 2. Borrow tokens from the smart contract by depositing collateral tokens.
/// The tokens which can be deposited as collateral are allowed and disallowed by the accounts with manager role
/// Upon borrowing user gets a PSP34 token representing info about their loan (how much assets were borrowed,
/// when did they borrow, what asset was borrowed, what asset was used as collateral, how much collateral assets
/// were deposited, the liquidation price of the loan and if it was liquidated or not)
///
/// 3. Repay their loan by depositing the borrowed amount of borrowed assets along with interest.
/// The contract determines how much a user needs to deposit and how much collateral they get back by an NFT token
/// which the user gets upon borrowing the assets. The user is also able to repay a portion of the loan, but will only get
/// a portion of their collateral assets back, while the liquidation price will stay the same
///
/// 4. Withdraw tokens deposited to the smart contract
/// User deposits their share tokens to the smart contract and the smart contract determines how much of the underlying
/// asset they get back
///
/// 5. Liquidate a loan
/// User can call a liquidation of a loan. If the price of collateral token of the loan is below or equal to the liquidation price,
/// the loan is then liquidated and the user performing the liquidation will get 1% of the liquidated assets
///
/// 6. Allow and disallow assets for lending
/// This can only be done by the accounts with the manager role
///
/// 7. Allow and disallow assets to be used as a collateral
/// This can only be done by the accounts with the manager role
///
/// 8. Pause the contract
/// Users with the manager role can pause the contract. If the contract is paused, no borrowing or lending can be performed
/// Users can still repay their loans, liquidate loans or withdraw their deposits
#[openbrush::implementation(AccessControl, Pausable)]
#[openbrush::contract]
pub mod my_lending {
    use ink::ToAccountId;
    use lending_project::impls::lending::*;
    use loan_contract::loan::LoanContractRef;
    use openbrush::traits::{
        xxh32,
        DefaultEnv,
        Storage,
        String,
    };
    use scale::Encode;
    use shares_contract::shares::SharesContractRef;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct LendingContract {
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        pause: pausable::Data,
        #[storage_field]
        lending: lending::data::Data,
    }

    impl lending_internal::Internal for LendingContract {}

    impl LendingImpl for LendingContract {}

    impl Lending for LendingContract {
        #[ink(message)]
        fn total_asset(&self, asset_address: AccountId) -> Result<Balance, LendingError> {
            LendingImpl::total_asset(self, asset_address)
        }

        #[ink(message)]
        fn total_shares(&self, asset_address: AccountId) -> Result<Balance, LendingError> {
            LendingImpl::total_shares(self, asset_address)
        }

        #[ink(message)]
        fn get_asset_shares(&self, asset_address: AccountId) -> Result<AccountId, LendingError> {
            LendingImpl::get_asset_shares(self, asset_address)
        }

        #[ink(message)]
        fn is_accepted_lending(&self, asset_address: AccountId) -> bool {
            LendingImpl::is_accepted_lending(self, asset_address)
        }

        #[ink(message)]
        fn is_accepted_collateral(&self, asset_address: AccountId) -> bool {
            LendingImpl::is_accepted_collateral(self, asset_address)
        }

        #[ink(message)]
        fn lend_assets(&mut self, asset_address: AccountId, amount: Balance) -> Result<(), LendingError> {
            LendingImpl::lend_assets(self, asset_address, amount)
        }

        #[ink(message)]
        fn borrow_assets(
            &mut self,
            asset_address: AccountId,
            collateral_address: AccountId,
            amount: Balance,
        ) -> Result<(), LendingError> {
            LendingImpl::borrow_assets(self, asset_address, collateral_address, amount)
        }

        #[ink(message)]
        fn repay(&mut self, loan_id: Id, repay_amount: Balance) -> Result<bool, LendingError> {
            LendingImpl::repay(self, loan_id, repay_amount)
        }

        #[ink(message)]
        fn withdraw_asset(&mut self, shares_address: AccountId, shares_amount: Balance) -> Result<(), LendingError> {
            LendingImpl::withdraw_asset(self, shares_address, shares_amount)
        }

        #[ink(message)]
        fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), LendingError> {
            LendingImpl::liquidate_loan(self, loan_id)
        }
    }

    impl LendingPermissionedImpl for LendingContract {}

    impl LendingPermissioned for LendingContract {
        #[ink(message, payable)]
        fn allow_asset(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
            LendingPermissionedImpl::allow_asset(self, asset_address)
        }

        #[ink(message)]
        fn disallow_lending(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
            LendingPermissionedImpl::disallow_lending(self, asset_address)
        }

        #[ink(message)]
        fn allow_collateral(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
            LendingPermissionedImpl::allow_collateral(self, asset_address)
        }

        #[ink(message)]
        fn disallow_collateral(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
            LendingPermissionedImpl::disallow_collateral(self, asset_address)
        }

        #[ink(message)]
        fn set_asset_price(
            &mut self,
            asset_in: AccountId,
            asset_out: AccountId,
            price: Balance,
        ) -> Result<(), LendingError> {
            LendingPermissionedImpl::set_asset_price(self, asset_in, asset_out, price)
        }
    }

    impl lending::Instantiator for LendingContract {
        fn _instantiate_shares_contract(&self, contract_name: &str, contract_symbol: &str) -> AccountId {
            let code_hash = self.lending.shares_contract_code_hash;

            let salt = (<Self as DefaultEnv>::env().block_timestamp(), contract_name).encode();

            let hash = xxh32(&salt, 0).to_le_bytes();

            let contract =
                SharesContractRef::new(Some(String::from(contract_name)), Some(String::from(contract_symbol)))
                    .endowment(0)
                    .code_hash(code_hash)
                    .salt_bytes(&hash[..4])
                    .instantiate();
            contract.to_account_id()
        }
    }

    impl LendingContract {
        /// constructor with name and symbol
        #[ink(constructor, payable)]
        pub fn new(shares_hash: Hash, loan_hash: Hash) -> Self {
            let mut instance = Self::default();
            let caller = <Self as DefaultEnv>::env().caller();
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
            AccessControl::grant_role(&mut instance, MANAGER, Some(caller)).expect("Can not set manager role");
            instance.lending.shares_contract_code_hash = shares_hash;
            // instantiate NFT contract and store its account id
            let nft = LoanContractRef::new()
                .endowment(0)
                .code_hash(loan_hash)
                .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();
            instance.lending.loan_account = nft.to_account_id();

            instance
        }
    }
}
