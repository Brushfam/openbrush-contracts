---
sidebar_position: 2
title: PSP34 Mintable
---

This example shows how you can reuse the implementation of [PSP34](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp34) token with [PSP34Mintable](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp34/extensions/mintable.rs) extension.

## How to use this extension

First, you should implement basic version of [PSP34](../psp34.md).

After you can just add implementation of PSP34Mintable via `#[openbrush::implementation(PSP34Mintable)]` attribute.

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Mintable)]
#[openbrush::contract]
pub mod my_psp34_mintable {
    use openbrush::traits::Storage;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
```

And that's it! Your `PSP34` is now extended by the `PSP34Mintable` extension and ready to use its functions!
You can check an example of the usage of [PSP34 Mintable](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp34_extensions/mintable).
