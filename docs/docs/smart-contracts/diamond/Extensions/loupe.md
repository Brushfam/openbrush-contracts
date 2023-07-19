---
sidebar_position: 1
title: Diamond Loupe
---

This example shows how you can reuse the implementation of [Diamond Standard](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/upgradeability/diamond) with [Diamond Loupe](https://github.com/Brushfam/openbrush-contracts/blob/main/contracts/src/upgradeability/diamond/extensions/diamond_loupe.rs) extension, which allows you to iterate over diamond contract's facets and available functions.

## How to use this extension

First, you should implement basic version of [Diamond standard](../diamond.md).

After you can just add implementation of DiamondLoupe via `#[openbrush::implementation(Diamond, DiamondLoupe)]` attribute.

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Diamond, DiamondLoupe)]
#[openbrush::contract]
pub mod my_diamond_loupe {
    ...
```

## Find result

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Ownable, Diamond, DiamondLoupe)]
#[openbrush::contract]
pub mod diamond {
    use openbrush::{
        modifiers,
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

    #[default_impl(Diamond)]
    #[modifiers(only_owner)]
    fn diamond_cut() {}

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
```

And that's it! Your `Diamond` is now extended by the `DiamondLoupe` extension and ready to use its functions!
You can check an example of the usage of [Diamond Loupe](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/diamond).
