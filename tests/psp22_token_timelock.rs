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

#[cfg(feature = "psp22")]
#[openbrush::implementation(PSP22TokenTimelock)]
#[openbrush::contract]
mod psp22_timelock {
    use openbrush::{test_utils::accounts, traits::Storage};

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PSP22TokenTimelockStruct {
        #[storage_field]
        timelock: Data,
        locked_tokens: Balance,
    }

    /// We will override timelock functions so they are not using cross-contract call in tests
    /// We will just remove calls to the locked token
    /// The cross-contract interaction will be tested in integration tests
    #[overrider(token_timelock::Internal)]
    fn _withdraw(&mut self, amount: Balance) -> Result<(), PSP22TokenTimelockError> {
        self.locked_tokens -= amount;
        Ok(())
    }

    #[overrider(token_timelock::Internal)]
    fn _contract_balance(&mut self) -> Balance {
        self.locked_tokens
    }

    impl PSP22TokenTimelockStruct {
        #[ink(constructor)]
        pub fn new(token_address: AccountId, beneficiary: AccountId, release_time: Timestamp) -> Self {
            let mut instance = Self::default();
            assert!(token_timelock::Internal::_init(&mut instance, token_address, beneficiary, release_time).is_ok());
            instance
        }

        /// Helper function for mocked deposit
        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance) {
            self.locked_tokens += amount;
        }

        /// Helper function to get mocked balance
        #[ink(message)]
        pub fn balance(&mut self) -> Balance {
            self.locked_tokens
        }
    }

    /// Test the constructor
    #[ink::test]
    fn new_works() {
        let accounts = accounts();
        let timelock = PSP22TokenTimelockStruct::new(AccountId::from([0x1; 32]), accounts.alice, day());

        assert_eq!(PSP22TokenTimelock::token(&timelock), Some(AccountId::from([0x1; 32])));
        assert_eq!(PSP22TokenTimelock::beneficiary(&timelock), Some(accounts.alice));
        assert_eq!(PSP22TokenTimelock::release_time(&timelock), day());
    }

    #[ink::test]
    fn release_works() {
        let deposited_tokens = 1000;
        let accounts = accounts();
        let mut timelock = PSP22TokenTimelockStruct::new(AccountId::from([0x1; 32]), accounts.alice, day());
        // deposit tokens to the contract
        timelock.deposit(deposited_tokens);
        // pass one day
        for _ in 0..day_blocks() {
            advance_block();
        }
        assert_eq!(timelock.balance(), deposited_tokens);

        // release the tokens
        assert!(PSP22TokenTimelock::release(&mut timelock).is_ok());

        // timelock should be empty
        assert_eq!(timelock.balance(), 0);
    }

    #[ink::test]
    fn release_soon_should_panic() {
        let deposited_tokens = 1000;
        let accounts = accounts();
        let mut timelock = PSP22TokenTimelockStruct::new(AccountId::from([0x1; 32]), accounts.alice, day());
        // deposit tokens to the contract
        timelock.deposit(deposited_tokens);

        // release the tokens should panic
        assert_eq!(
            PSP22TokenTimelock::release(&mut timelock),
            Err(PSP22TokenTimelockError::CurrentTimeIsBeforeReleaseTime)
        );
    }

    #[ink::test]
    fn release_without_deposit_should_panic() {
        let accounts = accounts();
        let mut timelock = PSP22TokenTimelockStruct::new(AccountId::from([0x1; 32]), accounts.alice, day());
        // pass one day
        for _ in 0..day_blocks() {
            advance_block();
        }
        assert_eq!(timelock.balance(), 0);

        // release the tokens
        assert_eq!(
            PSP22TokenTimelock::release(&mut timelock),
            Err(PSP22TokenTimelockError::NoTokensToRelease)
        );
    }

    type DefEnv = ink::env::DefaultEnvironment;

    fn day_blocks() -> u32 {
        (60 * 60 * 24) / 5
    }

    fn day() -> Timestamp {
        get_time() + (60 * 60 * 24)
    }

    fn advance_block() {
        let _ = ink::env::test::advance_block::<DefEnv>();
    }

    fn get_time() -> Timestamp {
        ink::env::block_timestamp::<DefEnv>()
    }
}
