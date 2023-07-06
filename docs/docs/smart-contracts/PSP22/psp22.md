---
sidebar_position: 1
title: PSP22
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22) token. Also, this example shows how you can customize the logic, for example, to reject transferring tokens to `hated_account`.

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to enable `psp22` feature, embed modules data structures and implement them via `#[openbrush::implementation]` macro
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main trait is `PSP22`.

## Step 2: Define constructor

Define constructor where you mint tokens to caller.

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new(total_supply: Balance) -> Self {
        let mut instance = Self {
            psp22: Default::default(),
            hated_storage: HatedStorage {
                hated_account: [255; 32].into(),
            },
        };

        Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

        instance
    }
}
```

## Step 3: Customize your contract

Customize it by adding hated account logic. It will contain two public methods `set_hated_account` and `get_hated_account`. 
Also we will override `_before_token_transfer` method in the `PSP22` implementation(that methods defined in `Transfer` trait), 
and we will add the `hated_account: AccountId` field to the structure.

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

// pub use my_psp22::*;
pub use openbrush::traits::{
    AccountId,
    Storage,
};

// we need to expand this struct before the contract macro is expanded
// that is why we declare it here for this example
#[ink::storage_item]
#[openbrush::accessors(HatedStorageAccessors)]
#[derive(Debug)]
pub struct HatedStorage {
    #[get]
    #[set]
    pub hated_account: AccountId,
}

#[openbrush::implementation(PSP22)]
#[openbrush::contract]
pub mod my_psp22 {
    use crate::*;
    use openbrush::traits::String;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        hated_storage: HatedStorage,
    }

    #[overrider(psp22::Internal)]
    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error> {
        if to == Some(&self.hated_storage.hated_account) {
            return Err(PSP22Error::Custom(String::from("I hate this account!")))
        }
        Ok(())
    }

    impl HatedStorageAccessors for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self {
                psp22: Default::default(),
                hated_storage: HatedStorage {
                    hated_account: [255; 32].into(),
                },
            };

            Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");

            instance
        }
    }
}

```

You can check an example of the usage of [PSP22](https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp22).

Also you can use extensions for PSP22 token:

[PSP22Metadata](/smart-contracts/psp22/extensions/metadata): metadata for PSP22.

[PSP22Mintable](/smart-contracts/psp22/extensions/mintable): creation of new tokens.

[PSP22Burnable](/smart-contracts/psp22/extensions/burnable): destruction of own tokens.

[PSP22Wrapper](/smart-contracts/psp22/extensions/wrapper): token wrapper for PSP22.

[PSP22FlashMint](/smart-contracts/psp22/extensions/flashmint): extension which allows the user to perform flashloans on the token by minting and burning the token.

Check out the utilities for PSP22 token:

[PSP22TokenTimelock](/smart-contracts/psp22/utils/token-timelock): utility for locking PSP22 tokens for a specified time.
