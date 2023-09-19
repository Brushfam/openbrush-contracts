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

#[cfg(feature = "psp37")]
#[openbrush::implementation(PSP37, PSP37Batch)]
#[openbrush::contract]
mod psp37_batch {
    use ink::codegen::Env;
    use openbrush::{
        test_utils::{
            accounts,
            change_caller,
        },
        traits::Storage,
    };

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        id: Id,
        value: Balance,
    }

    #[ink(event)]
    pub struct TransferBatch {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        ids_amounts: Vec<(Id, Balance)>,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        id: Option<Id>,
        value: Balance,
    }

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct PSP37Struct {
        #[storage_field]
        psp37: psp37::Data,
    }

    #[openbrush::overrider(psp37::Internal)]
    fn _emit_approval_event(&self, owner: AccountId, operator: AccountId, id: Option<Id>, value: Balance) {
        self.env().emit_event(Approval {
            owner,
            operator,
            id,
            value,
        });
    }

    #[openbrush::overrider(psp37::Internal)]
    fn _emit_transfer_batch_event(
        &self,
        from: Option<AccountId>,
        to: Option<AccountId>,
        ids_amounts: Vec<(Id, Balance)>,
    ) {
        self.env().emit_event(TransferBatch { from, to, ids_amounts });
    }

    impl PSP37Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint(&mut self, acc: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            psp37::Internal::_mint_to(self, acc, ids_amounts)
        }
    }

    #[ink::test]
    fn batch_transfer() {
        let token_id1 = Id::U128(1);
        let token_id2 = Id::U128(2);
        let id_1_amount = 1;
        let id_2_amount = 20;
        let ids_amounts = vec![(token_id1.clone(), id_1_amount), (token_id2.clone(), id_2_amount)];
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.alice, ids_amounts.clone()).is_ok());

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 2);

        assert!(PSP37Batch::batch_transfer(&mut nft, accounts.bob, ids_amounts.clone(), vec![]).is_ok());

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, Some(token_id1.clone())), 0);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, Some(token_id2.clone())), 0);

        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, Some(token_id1)), id_1_amount);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, Some(token_id2)), id_2_amount);

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 0);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, None), 2);

        assert_eq!(ink::env::test::recorded_events().count(), 2);
    }

    #[ink::test]
    fn transfer_batch_from() {
        let token_id_1 = Id::U128(1);
        let token_id_2 = Id::U128(2);
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids_amounts = vec![
            (token_id_1.clone(), token_1_amount),
            (token_id_2.clone(), token_2_amount),
        ];
        let amounts = vec![token_1_amount, token_2_amount];
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.alice, ids_amounts.clone()).is_ok());

        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, Some(token_id_1.clone())), 0);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, Some(token_id_2.clone())), 0);

        assert_eq!(
            PSP37::balance_of(&mut nft, accounts.alice, Some(token_id_1.clone())),
            amounts[0]
        );
        assert_eq!(
            PSP37::balance_of(&mut nft, accounts.alice, Some(token_id_2.clone())),
            amounts[1]
        );

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 2);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, None), 0);

        assert!(
            PSP37Batch::batch_transfer_from(&mut nft, accounts.alice, accounts.bob, ids_amounts.clone(), vec![])
                .is_ok()
        );

        assert_eq!(
            PSP37::balance_of(&mut nft, accounts.bob, Some(token_id_1.clone())),
            amounts[0]
        );
        assert_eq!(
            PSP37::balance_of(&mut nft, accounts.bob, Some(token_id_2.clone())),
            amounts[1]
        );

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, Some(token_id_1.clone())), 0);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, Some(token_id_2.clone())), 0);

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 0);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, None), 2);
    }

    #[ink::test]
    fn batch_transfer_from_insufficient_balance() {
        let token_id_1 = Id::U128(1);
        let token_id_2 = Id::U128(2);
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids_amounts = vec![(token_id_1, token_1_amount), (token_id_2, token_2_amount)];
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.alice, ids_amounts.clone()).is_ok());
        assert_eq!(
            PSP37Batch::batch_transfer_from(
                &mut nft,
                accounts.alice,
                accounts.bob,
                ids_amounts
                    .iter()
                    .map(|(_, amount)| { (Id::U128(123), *amount) })
                    .collect(),
                vec![]
            ),
            Err(PSP37Error::InsufficientBalance),
        );
    }

    #[ink::test]
    fn batch_transfer_from_no_approve() {
        let token_id_1 = Id::U128(1);
        let token_id_2 = Id::U128(2);
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids_amounts = vec![(token_id_1, token_1_amount), (token_id_2, token_2_amount)];
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.bob, ids_amounts.clone()).is_ok());

        assert_eq!(
            PSP37Batch::batch_transfer_from(&mut nft, accounts.bob, accounts.alice, ids_amounts, vec![]),
            Err(PSP37Error::NotAllowed)
        );
    }

    #[ink::test]
    fn batch_transfer_with_approve() {
        let token_id_1 = Id::U128(1);
        let token_id_2 = Id::U128(2);
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids_amounts = vec![
            (token_id_1.clone(), token_1_amount),
            (token_id_2.clone(), token_2_amount),
        ];
        let amounts = vec![token_1_amount, token_2_amount];
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.alice, ids_amounts.clone()).is_ok());
        assert!(PSP37::approve(&mut nft, accounts.bob, None, Balance::MAX).is_ok());

        change_caller(accounts.bob);
        assert!(
            PSP37Batch::batch_transfer_from(&mut nft, accounts.alice, accounts.bob, ids_amounts.clone(), vec![])
                .is_ok()
        );

        assert_eq!(
            PSP37::balance_of(&mut nft, accounts.bob, Some(token_id_1.clone())),
            amounts[0]
        );
        assert_eq!(
            PSP37::balance_of(&mut nft, accounts.bob, Some(token_id_2.clone())),
            amounts[1]
        );
        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, Some(token_id_1)), 0);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, Some(token_id_2)), 0);

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 0);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, None), 2);

        assert_eq!(ink::env::test::recorded_events().count(), 3);
    }
}
