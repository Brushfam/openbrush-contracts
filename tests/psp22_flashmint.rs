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
#[openbrush::implementation(PSP22, Flashmint)]
#[openbrush::contract]
mod psp22_flashmint {
    use ink::codegen::Env;
    use openbrush::{test_utils::accounts, traits::Storage};

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PSP22FlashMintStruct {
        #[storage_field]
        psp22: psp22::Data,
    }

    // we remove cross contract call in test

    // we will add 1% fee to the amount
    #[overrider(flashmint::Internal)]
    fn _get_fee(&self, amount: Balance) -> Balance {
        amount / 100
    }

    #[overrider(flashmint::Internal)]
    fn _on_flashloan(
        &mut self,
        _receiver_account: AccountId,
        _token: AccountId,
        _fee: Balance,
        _amount: Balance,
        _data: Vec<u8>,
    ) -> Result<(), FlashLenderError> {
        Ok(())
    }

    impl PSP22FlashMintStruct {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            assert!(psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).is_ok());
            instance
        }
    }

    #[ink::test]
    fn new_works() {
        let total_supply = 1000;
        let mut instance = PSP22FlashMintStruct::new(total_supply);
        let account_id = instance.env().account_id();

        // max flashloan is max - total supply
        assert_eq!(
            FlashLender::max_flashloan(&mut instance, account_id),
            Balance::MAX - total_supply
        );
        // flash fee is 1/100 of amount
        assert_eq!(FlashLender::flash_fee(&mut instance, account_id, 100), Ok(1));
        // wrong token
        assert_eq!(
            FlashLender::max_flashloan(&mut instance, AccountId::from([0x10; 32])),
            0
        );
        // flash fee on wrong token throws error
        assert_eq!(
            FlashLender::flash_fee(&mut instance, AccountId::from([0x10; 32]), 100),
            Err(FlashLenderError::WrongTokenAddress)
        );
    }

    #[ink::test]
    fn flashloan_works() {
        let total_supply = 1000;
        let mut instance = PSP22FlashMintStruct::new(total_supply);

        let receiver = AccountId::from([0x1; 32]);
        let token = instance.env().account_id();
        let loan_amount = 100;
        let fee = flashmint::Internal::_get_fee(&instance, loan_amount);

        assert!(PSP22::approve(&mut instance, token, loan_amount + fee).is_ok());
        assert!(FlashLender::flashloan(&mut instance, receiver, token, loan_amount, Vec::<u8>::new()).is_ok());
        assert_eq!(PSP22::total_supply(&instance), total_supply - fee);
        assert_eq!(PSP22::balance_of(&instance, accounts().alice), total_supply - fee);
    }

    #[ink::test]
    fn no_allowance_for_fee() {
        let total_supply = 1000;
        let mut instance = PSP22FlashMintStruct::new(total_supply);

        let receiver = AccountId::from([0x1; 32]);
        let token = instance.env().account_id();
        let loan_amount = 100;

        assert_eq!(
            FlashLender::flashloan(&mut instance, receiver, token, loan_amount, Vec::<u8>::new()),
            Err(FlashLenderError::AllowanceDoesNotAllowRefund)
        );
    }
}
