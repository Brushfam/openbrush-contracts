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
#[openbrush::implementation(PSP37, PSP37Enumerable, PSP37Batch, PSP37Burnable)]
#[openbrush::contract]
mod psp37_enumerable {
    use openbrush::{
        contracts::psp37::Id,
        test_utils::{
            accounts,
            change_caller,
        },
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct PSP37Struct {
        #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        enumerable: enumerable::Data,
    }

    impl PSP37Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint(&mut self, acc: AccountId, id: Id, amount: Balance) -> Result<(), PSP37Error> {
            psp37::Internal::_mint_to(self, acc, vec![(id, amount)])
        }
    }

    #[ink::test]
    fn enumerable_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let nft = PSP37Struct::new();
        // check that alice does not have token by index
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&nft, accounts.alice, 0u128),
            None
        );
        // token by index 1 does not exists
        assert_eq!(PSP37Enumerable::token_by_index(&nft, 0u128), None)
    }

    #[ink::test]
    fn enumerable_mint_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();

        let token_id = Id::U128(1);

        // Create token Id 1 for Alice
        assert!(psp37::Internal::_mint_to(&mut nft, accounts.alice, vec![(token_id.clone(), 20)]).is_ok());
        // check Alice token by index
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Some(token_id.clone())
        );
        // check token by index
        assert_eq!(PSP37Enumerable::token_by_index(&mut nft, 0u128), Some(token_id));
    }

    #[ink::test]
    fn enumerable_transfer_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        // Create token Id 1 and Id 2 for Alice

        let token_id1 = Id::U128(1);
        let token_id2 = Id::U128(2);
        let token_amount1 = 1;
        let token_amount2 = 20;

        assert!(psp37::Internal::_mint_to(
            &mut nft,
            accounts.alice,
            vec![(token_id1.clone(), token_amount1), (token_id2.clone(), token_amount2)]
        )
        .is_ok());
        // check Alice token by index
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Some(token_id1.clone())
        );
        // act. transfer token from alice to bob
        assert!(PSP37::transfer(&mut nft, accounts.bob, token_id1.clone(), token_amount1, vec![]).is_ok());
        // bob owns token
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.bob, 0u128),
            Some(token_id1)
        );
        // alice does not own token Id 1
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Some(token_id2.clone())
        );
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 1u128),
            None
        );
        // act. transfer token from alice to alice
        assert!(PSP37::transfer(&mut nft, accounts.bob, token_id2.clone(), 10, vec![]).is_ok());
        // check Alice token by index
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Some(token_id2.clone())
        );
        // check Bob token by index
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.bob, 1u128),
            Some(token_id2.clone())
        );
    }

    #[ink::test]
    fn enumerable_batch_transfer_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        // Create token Id 1 and Id 2 for Alice

        let token_id1 = Id::U128(1);
        let token_id2 = Id::U128(2);
        let token_amount1 = 1;
        let token_amount2 = 20;

        assert!(psp37::Internal::_mint_to(
            &mut nft,
            accounts.alice,
            vec![(token_id1.clone(), token_amount1), (token_id2.clone(), token_amount2)]
        )
        .is_ok());
        // check Alice token by index
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Some(token_id1.clone())
        );
        // act. transfer token from alice to bob
        assert!(PSP37::transfer(&mut nft, accounts.bob, token_id1.clone(), token_amount1, vec![]).is_ok());
        // bob owns token
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.bob, 0u128),
            Some(token_id1)
        );
        // alice does not own token Id 1
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Some(token_id2.clone())
        );
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 1u128),
            None
        );
        // act. transfer token from alice to alice
        assert!(PSP37::transfer(&mut nft, accounts.alice, token_id2.clone(), token_amount2, vec![]).is_ok());
        // check Alice token by index
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Some(token_id2)
        );
    }

    #[ink::test]
    fn enumerable_self_transfer_works() {
        let accounts = accounts();

        let mut nft = PSP37Struct::new();

        let token_id1 = Id::U128(1);
        let token_id2 = Id::U128(2);
        let token_id3 = Id::U128(3);
        let token_id4 = Id::U128(4);

        assert!(psp37::Internal::_mint_to(
            &mut nft,
            accounts.alice,
            vec![
                (token_id1.clone(), 1),
                (token_id2.clone(), 2),
                (token_id3.clone(), 3),
                (token_id4.clone(), 4)
            ]
        )
        .is_ok());

        assert!(PSP37::transfer(&mut nft, accounts.alice, token_id2.clone(), 1, vec![]).is_ok());

        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Some(token_id1.clone())
        );
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 1u128),
            Some(token_id2.clone())
        );
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 2u128),
            Some(token_id3.clone())
        );
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 3u128),
            Some(token_id4.clone())
        );

        assert!(PSP37::transfer(&mut nft, accounts.alice, token_id2.clone(), 2, vec![]).is_ok());

        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Some(token_id1)
        );
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 1u128),
            Some(token_id4)
        );
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 2u128),
            Some(token_id3)
        );
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 3u128),
            Some(token_id2)
        );
    }

    #[ink::test]
    fn token_by_index_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();

        let token_id1 = Id::U128(1);
        let token_id2 = Id::U128(2);
        let token_id3 = Id::U128(3);
        let token_amount1 = 1u128;
        let token_amount2 = 1u128;
        let token_amount3 = 1u128;

        // Create token Id 1 for Alice
        assert!(psp37::Internal::_mint_to(
            &mut nft,
            accounts.alice,
            vec![
                (token_id1.clone(), token_amount1),
                (token_id2.clone(), token_amount2),
                (token_id3.clone(), token_amount3)
            ]
        )
        .is_ok());

        assert!(PSP37::transfer(&mut nft, accounts.bob, token_id1.clone(), token_amount1, vec![]).is_ok());
        assert!(PSP37::transfer(&mut nft, accounts.bob, token_id3.clone(), token_amount3, vec![]).is_ok());
        change_caller(accounts.bob);
        assert!(PSP37::transfer(&mut nft, accounts.alice, token_id1.clone(), token_amount1, vec![]).is_ok());
        assert!(PSP37Burnable::burn(&mut nft, accounts.alice, vec![(token_id2, token_amount2)]).is_ok());
        assert!(PSP37::transfer(&mut nft, accounts.alice, token_id3.clone(), token_amount3, vec![]).is_ok());
        change_caller(accounts.alice);
        assert!(PSP37::transfer(&mut nft, accounts.bob, token_id3.clone(), token_amount3, vec![]).is_ok());
        // alice does not own token
        assert_eq!(PSP37Enumerable::token_by_index(&mut nft, 0u128), Some(token_id1));
        assert_eq!(PSP37Enumerable::token_by_index(&mut nft, 1u128), Some(token_id3));
        assert_eq!(PSP37Enumerable::token_by_index(&mut nft, 2u128), None);
    }

    #[ink::test]
    fn enumerable_burn_works() {
        let accounts = accounts();
        let token_id = Id::U128(1);
        let token_amount = 1u128;
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert!(psp37::Internal::_mint_to(&mut nft, accounts.alice, vec![(token_id.clone(), token_amount)]).is_ok());
        // alice still owns token id 1
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Some(token_id.clone())
        );
        // index 0 points to token with id 1
        assert_eq!(PSP37Enumerable::token_by_index(&mut nft, 0u128), Some(token_id.clone()));
        // Destroy token Id 1.
        assert!(PSP37Burnable::burn(&mut nft, accounts.alice, vec![(token_id, token_amount)]).is_ok());
        // alice does not owns any tokens
        assert_eq!(
            PSP37Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            None
        );
        // token by index 1 does not exists
        assert_eq!(PSP37Enumerable::token_by_index(&mut nft, 0u128), None);
    }

    #[ink::test]
    fn total_supply_works() {
        let token_id1 = Id::U128(1);
        let token_id2 = Id::U128(2);
        let token_id3 = Id::U128(3);

        let token_amount1 = 1;
        let token_amount2 = 20;
        let token_amount3 = 1;

        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert_eq!(PSP37::total_supply(&mut nft, None), 0);
        // mint some token 1
        assert!(nft.mint(accounts.alice, token_id1.clone(), token_amount1).is_ok());
        assert!(nft.mint(accounts.alice, token_id2.clone(), token_amount2).is_ok());

        assert_eq!(PSP37::total_supply(&mut nft, None), 2);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id1.clone())), token_amount1);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id2.clone())), token_amount2);

        assert!(nft.mint(accounts.bob, token_id3.clone(), token_amount3).is_ok());

        assert_eq!(PSP37::total_supply(&mut nft, None), 3);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id3.clone())), token_amount3);
    }

    #[ink::test]
    fn balance_of() {
        let token_id1 = Id::U128(1);
        let token_id2 = Id::U128(2);
        let token_amount1 = 1;
        let token_amount2 = 20;

        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        // Token 1 does not exists.
        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, Some(token_id1.clone())), 0);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 0);
        // mint some token 1
        assert!(nft.mint(accounts.alice, token_id1.clone(), token_amount1).is_ok());
        assert!(nft.mint(accounts.alice, token_id2.clone(), token_amount2).is_ok());

        assert_eq!(
            PSP37::balance_of(&mut nft, accounts.alice, Some(token_id1.clone())),
            token_amount1
        );
        assert_eq!(
            PSP37::balance_of(&mut nft, accounts.alice, Some(token_id2.clone())),
            token_amount2
        );

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 2);
    }

    #[ink::test]
    fn transfer() {
        let token_id1 = Id::U128(1);
        let token_id2 = Id::U128(2);
        let token_id3 = Id::U128(3);
        let token_amount1 = 1;
        let token_amount2 = 20;
        let token_amount3 = 30;

        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.alice, token_id1.clone(), token_amount1).is_ok());
        assert!(nft.mint(accounts.alice, token_id2.clone(), token_amount2).is_ok());
        assert!(nft.mint(accounts.alice, token_id3.clone(), token_amount3).is_ok());

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 3);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id1.clone())), token_amount1);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id2.clone())), token_amount2);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id3.clone())), token_amount3);
        assert_eq!(PSP37::total_supply(&mut nft, None), 3);

        assert!(PSP37::transfer(&mut nft, accounts.bob, token_id2.clone(), 10, vec![]).is_ok());
        assert!(PSP37::transfer(&mut nft, accounts.bob, token_id3.clone(), 10, vec![]).is_ok());

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, Some(token_id2.clone())), 10);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, Some(token_id3.clone())), 20);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, Some(token_id2.clone())), 10);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, Some(token_id3.clone())), 10);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 3);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, None), 2);

        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id2.clone())), token_amount2);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id3.clone())), token_amount3);
        assert_eq!(PSP37::total_supply(&mut nft, None), 3);

        assert!(PSP37::transfer(&mut nft, accounts.charlie, token_id3.clone(), 10, vec![]).is_ok());

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, Some(token_id3.clone())), 10);
        assert_eq!(
            PSP37::balance_of(&mut nft, accounts.charlie, Some(token_id3.clone())),
            10
        );
        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 3);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, None), 2);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.charlie, None), 1);

        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id3.clone())), token_amount3);

        assert!(PSP37::transfer(&mut nft, accounts.charlie, token_id3.clone(), 10, vec![]).is_ok());

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, Some(token_id3.clone())), 0);
        assert_eq!(
            PSP37::balance_of(&mut nft, accounts.charlie, Some(token_id3.clone())),
            20
        );
        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 2);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, None), 2);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.charlie, None), 1);

        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id1.clone())), token_amount1);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id2.clone())), token_amount2);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id3.clone())), token_amount3);
        assert_eq!(PSP37::total_supply(&mut nft, None), 3);
    }

    #[ink::test]
    fn burn_works() {
        let token_id1 = Id::U128(1);
        let token_id2 = Id::U128(2);
        let token_amount1 = 1;
        let token_amount2 = 10;
        let accounts = accounts();

        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.alice, token_id1.clone(), token_amount1).is_ok());
        assert!(nft.mint(accounts.alice, token_id2.clone(), token_amount2).is_ok());

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 2);
        assert_eq!(PSP37::total_supply(&mut nft, None), 2);

        assert!(nft.mint(accounts.bob, token_id2.clone(), token_amount2).is_ok());

        assert_eq!(PSP37::total_supply(&mut nft, None), 2);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id1.clone())), 1);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id2.clone())), 20);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 2);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, None), 1);

        assert!(PSP37Burnable::burn(
            &mut nft,
            accounts.bob,
            vec![(token_id2.clone(), token_amount2), (token_id1.clone(), 0)]
        )
        .is_ok());

        assert_eq!(PSP37::total_supply(&mut nft, None), 2);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id2.clone())), 10);

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 2);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, None), 0);

        assert!(PSP37Burnable::burn(&mut nft, accounts.alice, vec![(token_id2.clone(), token_amount2)]).is_ok());

        assert_eq!(PSP37::total_supply(&mut nft, None), 1);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id2.clone())), 0);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 1);
    }
}
