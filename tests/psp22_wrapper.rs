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
#[openbrush::implementation(PSP22, PSP22Wrapper)]
#[openbrush::contract]
mod psp22_wrapper {
    use openbrush::test_utils::accounts;

    #[ink(storage)]
    #[derive(Default)]
    #[openbrush::storage]
    pub struct PSP22WrapperStruct {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        wrapper: wrapper::Data,
        contract_balance: Balance,
    }

    /// We will override cross-contract wrapper calls in tests
    /// The cross-contract interaction will be tested in integration tests
    #[overrider(wrapper::Internal)]
    fn _deposit(&mut self, amount: Balance) -> Result<(), PSP22Error> {
        self.contract_balance += amount;
        Ok(())
    }

    #[overrider(wrapper::Internal)]
    fn _withdraw(&mut self, _account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self.contract_balance -= amount;
        Ok(())
    }

    #[overrider(wrapper::Internal)]
    fn _underlying_balance(&mut self) -> Balance {
        self.contract_balance
    }

    impl PSP22WrapperStruct {
        #[ink(constructor)]
        pub fn new(underlying: AccountId) -> Self {
            let mut instance = Self::default();
            wrapper::Internal::_init(&mut instance, underlying);
            instance
        }

        #[ink(message)]
        pub fn recover(&mut self) -> Result<Balance, PSP22Error> {
            wrapper::Internal::_recover(self, self.env().caller())
        }

        #[ink(message)]
        pub fn burn(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            psp22::Internal::_burn_from(self, self.env().caller(), amount)
        }
    }

    #[ink::test]
    fn deposit_for_works() {
        let accounts = accounts();
        let mut wrapper = PSP22WrapperStruct::new(AccountId::from([0x1; 32]));

        assert_eq!(PSP22::balance_of(&mut wrapper, accounts.alice), 0);
        assert_eq!(PSP22::total_supply(&mut wrapper), 0);

        assert!(PSP22Wrapper::deposit_for(&mut wrapper, accounts.alice, 100).is_ok());

        assert_eq!(PSP22::balance_of(&mut wrapper, accounts.alice), 100);
        assert_eq!(PSP22::total_supply(&mut wrapper), 100);
    }

    #[ink::test]
    fn withdraw_to_works() {
        let accounts = accounts();
        let mut wrapper = PSP22WrapperStruct::new(AccountId::from([0x1; 32]));

        assert!(PSP22Wrapper::deposit_for(&mut wrapper, accounts.alice, 100).is_ok());
        assert_eq!(PSP22::balance_of(&mut wrapper, accounts.alice), 100);
        assert_eq!(PSP22::total_supply(&mut wrapper), 100);
        assert!(PSP22Wrapper::withdraw_to(&mut wrapper, accounts.alice, 100).is_ok());

        assert_eq!(PSP22::balance_of(&mut wrapper, accounts.alice), 0);
        assert_eq!(PSP22::total_supply(&mut wrapper), 0);
    }

    #[ink::test]
    fn recover_works() {
        let accounts = accounts();
        let mut wrapper = PSP22WrapperStruct::new(AccountId::from([0x1; 32]));

        assert!(PSP22Wrapper::deposit_for(&mut wrapper, accounts.alice, 100).is_ok());
        assert!(wrapper.burn(100).is_ok());
        assert_eq!(PSP22::balance_of(&mut wrapper, accounts.alice), 0);
        assert_eq!(PSP22::total_supply(&mut wrapper), 0);

        assert!(wrapper.recover().is_ok());

        assert_eq!(PSP22::balance_of(&mut wrapper, accounts.alice), 100);
        assert_eq!(PSP22::total_supply(&mut wrapper), 100);
    }
}
