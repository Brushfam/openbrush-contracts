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

#[cfg(feature = "diamond")]
#[openbrush::implementation(Ownable, Diamond, DiamondLoupe)]
#[openbrush::contract]
mod diamond {
    use ink::env::{
        test::DefaultAccounts,
        DefaultEnvironment,
    };
    use openbrush::test_utils::accounts;

    #[ink(storage)]
    #[derive(Default)]
    #[openbrush::storage]
    pub struct DiamondContract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        diamond: diamond::Data,
        #[storage_field]
        diamond_loupe: diamond_loupe::Data,
    }

    impl DiamondContract {
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
    fn setup() -> DefaultAccounts<DefaultEnvironment> {
        let accounts = accounts();
        accounts
    }

    #[ink::test]
    fn constructor_works() {
        let accounts = setup();
        let diamond = DiamondContract::new(accounts.alice);
        // assert
        assert_eq!(Ownable::owner(&diamond), Some(accounts.alice));
    }

    #[ink::test]
    fn facets_empty_works() {
        let accounts = setup();
        let diamond = DiamondContract::new(accounts.alice);
        // assert
        assert_eq!(DiamondLoupe::facets(&diamond), vec![]);
    }

    #[ink::test]
    fn facets_not_empty_works() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[0u8; 4]],
        };
        // act
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(DiamondLoupe::facets(&diamond), vec![facet_cut]);
    }

    #[ink::test]
    fn hash_is_clear_should_fails() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [0u8; 32].into(),
            selectors: vec![[0u8; 4]],
        };
        // assert
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, vec![facet_cut.clone()], Option::None),
            Result::Err(DiamondError::EmptyCodeHash)
        );
    }

    #[ink::test]
    fn facet_function_selectors_works() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[0u8; 4]],
        };
        let selectors: Vec<Selector> = vec![];
        assert_eq!(
            DiamondLoupe::facet_function_selectors(&diamond, facet_cut.hash),
            selectors
        );
        // act
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(
            DiamondLoupe::facet_function_selectors(&diamond, facet_cut.hash),
            facet_cut.selectors
        );
    }

    #[ink::test]
    fn facet_code_hashes_works() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);
        let facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[0u8; 4]],
        };
        assert_eq!(DiamondLoupe::facet_code_hashes(&diamond), vec![]);
        // act
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(DiamondLoupe::facet_code_hashes(&diamond), vec![facet_cut.hash]);
    }

    #[ink::test]
    fn facet_code_hash_works() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[0u8; 4]],
        };
        assert_eq!(
            DiamondLoupe::facet_code_hash(&diamond, facet_cut.selectors[0]),
            Option::None
        );
        // act
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(
            DiamondLoupe::facet_code_hash(&diamond, facet_cut.selectors[0]),
            Option::Some(facet_cut.hash)
        );
    }

    #[ink::test]
    fn facets_add_selectors_works() {
        // arrange
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let mut facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[1u8; 4]],
        };
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        facet_cut.selectors.push([2u8; 4]);
        // act
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(DiamondLoupe::facets(&diamond)[0].selectors.len(), 2);
        assert_eq!(DiamondLoupe::facets(&diamond), vec![facet_cut.clone()]);
    }

    #[ink::test]
    fn facets_remove_selectors_works() {
        // arrange
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let mut facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[1u8; 4], [2u8; 4], [3u8; 4]],
        };
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        facet_cut.selectors.pop();
        assert_eq!(facet_cut.selectors.len(), 2);
        // act
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(DiamondLoupe::facets(&diamond)[0].selectors.len(), 2);
        assert_eq!(DiamondLoupe::facets(&diamond), vec![facet_cut.clone()]);
    }

    #[ink::test]
    fn facets_edit_selectors_works() {
        // arrange
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let mut facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[1u8; 4], [2u8; 4], [3u8; 4]],
        };

        assert_eq!(
            Diamond::diamond_cut(&mut diamond, vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        assert_eq!(DiamondLoupe::facets(&diamond)[0].selectors.len(), 3);
        // act
        facet_cut.selectors[2] = [4u8; 4];
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(DiamondLoupe::facets(&diamond)[0].selectors.len(), 3);
        assert_eq!(DiamondLoupe::facets(&diamond), vec![facet_cut.clone()]);
        assert_eq!(DiamondLoupe::facets(&diamond)[0].selectors[2], [4u8; 4]);
    }

    #[ink::test]
    fn facets_add_facetcut_works() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[1u8; 4]],
        };
        assert_eq!(DiamondLoupe::facets(&diamond).len(), 0);
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        assert_eq!(DiamondLoupe::facets(&diamond).len(), 1);
        // act
        let facet_cut_new = FacetCut {
            hash: [2u8; 32].into(),
            selectors: vec![[2u8; 4]],
        };
        let v = vec![facet_cut, facet_cut_new];
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, v.clone(), Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(DiamondLoupe::facets(&diamond).len(), 2);
        assert_eq!(DiamondLoupe::facets(&diamond), v);
    }

    #[ink::test]
    fn facets_add_facetcut_should_fail_replace_existing() {
        // arrange
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[1u8; 4]],
        };
        assert_eq!(DiamondLoupe::facets(&diamond).len(), 0);
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        assert_eq!(DiamondLoupe::facets(&diamond).len(), 1);
        let facet_cut_new = FacetCut {
            hash: [2u8; 32].into(),
            selectors: vec![[1u8; 4]],
        };
        let v = vec![facet_cut.clone(), facet_cut_new];
        // act
        let result = Diamond::diamond_cut(&mut diamond, v.clone(), Option::None);
        // assert
        assert_eq!(result, Err(DiamondError::ReplaceExisting([1u8; 32].into())));
        assert_eq!(DiamondLoupe::facets(&diamond).len(), 1);
        assert_eq!(DiamondLoupe::facets(&diamond), vec![facet_cut]);
    }

    #[ignore]
    #[ink::test]
    fn facets_edit_facetcut_works() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[1u8; 4]],
        };
        let mut facet_cut_new = FacetCut {
            hash: [2u8; 32].into(),
            selectors: vec![[2u8; 4]],
        };
        let mut v = vec![facet_cut.clone(), facet_cut_new.clone()];
        assert_eq!(DiamondLoupe::facets(&diamond).len(), 0);
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, v.clone(), Option::None),
            Result::Ok(())
        );
        assert_eq!(DiamondLoupe::facets(&diamond).len(), 2);
        facet_cut_new.selectors = vec![[5u8; 4], [6u8; 4]];
        v[1] = facet_cut_new;
        // act
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, v.clone(), Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(DiamondLoupe::facets(&diamond).len(), 2);
        assert_eq!(DiamondLoupe::facets(&diamond), v);
    }

    #[ignore]
    #[ink::test]
    fn facets_remove_facetcut_works() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [3u8; 32].into(),
            selectors: vec![[1u8; 4]],
        };
        let facet_cut_new = FacetCut {
            hash: [2u8; 32].into(),
            selectors: vec![[2u8; 4]],
        };
        let mut v = vec![facet_cut.clone(), facet_cut_new];
        assert_eq!(DiamondLoupe::facets(&diamond).len(), 0);
        assert_eq!(
            Diamond::diamond_cut(&mut diamond, v.clone(), Option::None),
            Result::Ok(())
        );
        assert_eq!(DiamondLoupe::facets(&diamond).len(), 2);
        // act
        v[1].selectors = vec![];
        assert_eq!(Diamond::diamond_cut(&mut diamond, v, Option::None), Result::Ok(()));
        // assert
        assert_eq!(DiamondLoupe::facets(&diamond).len(), 1);
        assert_eq!(DiamondLoupe::facets(&diamond), vec![facet_cut]);
    }
}
