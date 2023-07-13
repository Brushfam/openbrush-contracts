---
sidebar_position: 4
title: Shares contract
---

Similarly, we will implement another [PSP22](../PSP22/psp22.md) token 
which will represent the ownership of assets available by the smart contract 
to be lent. In this token, we will need [PSP22Metadata](../PSP22/Extensions/metadata.md) 
and we will also need to mint and burn this token. We only want our contract(lending contract) to 
perform these actions, so we will also add the [Ownable](../ownable.md) extension.

## Definition of the `Shares` trait

In the `traits/shares.rs`, we will define a `Shares` trait.
That trait contains the next super traits: `PSP22`, `PSP22Mintable`, `PSP22Burnable`, `PSP22Metadata`, and `Ownable`, without any other method.
That shows that `Shares` is `PSP22` with mint and burn methods that can be called only by the owner.
In the implementation of the contract, we will implement that trait to be sure that all super traits are also implemented.
`SharesRef` can be used by other developers to do a cross contract call to `SharesContract`.

```rust
use openbrush::contracts::traits::{
    ownable::*,
    psp22::{
        extensions::{
            burnable::*,
            metadata::*,
            mintable::*,
        },
        *,
    },
};

#[openbrush::wrapper]
pub type SharesRef = dyn PSP22 + PSP22Mintable + PSP22Burnable + PSP22Metadata + Ownable;

#[openbrush::trait_definition]
pub trait Shares: PSP22 + PSP22Mintable + PSP22Burnable + PSP22Metadata + Ownable {}
```

## Add dependencies

In addition to the dependencies imported in the [PSP22](../PSP22/psp22.md)
documentation, we will also add the `ownable` dependency the same way as in the
[ownable](../ownable.md) documentation. We will be using `SharesContract`
as a dependency in our lending contract to instantiate it. So we need to also add
the `"rlib"` crate type to have the ability to import the `SharesContract` as a dependency.

## Implement the contract

Implementing our shares contract will follow the same steps as implementing 
the basic `PSP22` contract in the previous step, but we will do some small 
changes for the token to be mintable, burnable, and for these functions to 
be restricted. Therefore, on top of the imports in the previous contract, 
we also need these imports:

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// This contract will be used to represent the shares of a user
/// and other instance of this contract will be used to represent
/// the amount of borrowed tokens
#[openbrush::implementation(PSP22, PSP22Mintable, PSP22Burnable, PSP22Metadata, Ownable)]
#[openbrush::contract]
pub mod shares {
    use openbrush::traits::String;
    use lending_project::traits::shares::*;
    use openbrush::{
        modifiers,
        traits::Storage,
    };
```

## Define the storage

In this storage, we will also derive the storage trait related to `Ownable` 
and declare the field related to this trait.

```rust
/// Define the storage for PSP22 data, Metadata data and Ownable data
#[ink(storage)]
#[derive(Default, Storage)]
pub struct SharesContract {
    #[storage_field]
    psp22: psp22::Data,
    #[storage_field]
    ownable: ownable::Data,
    #[storage_field]
    metadata: metadata::Data,
}
```

## Implement the extension traits

We will be using these extensions in our token, so we will implement them for 
our storage.

```rust
// It forces the compiler to check that you implemented all super traits
impl Shares for SharesContract {}
```

## Implement the Burnable and Mintable traits

Now we will implement the `PSP22Burnable` and `PSP22Mintable` traits. 
These are a little different so we are doing it in a separate section. 
We don't want anybody to mint or burn the tokens, we only want the owner, 
in this case, our lending contract, to do it. So we will add the `PSP22Burnable` 
and `PSP22Mintable` and mark the functions of these traits with the `only_owner` 
restriction. Here we are using the `#[default_impl]` macro to mark, that we want to use default implementation of the trait's 
method but to override the modifiers or other attributes of the method.

```rust
/// override the `mint` function to add the `only_owner` modifier
#[default_impl(PSP22Mintable)]
#[modifiers(only_owner)]
fn mint() {}

/// override the `burn` function to add the `only_owner` modifier
#[default_impl(PSP22Burnable)]
#[modifiers(only_owner)]
fn burn() {}
```

This will restrict accounts other than the owner of the token (which will be the lending contract) 
from calling these functions.

## Define the constructor

Finally, we will define the constructor where we will set the name and the symbol 
of the token and then initialize the owner of the token 
(which then will be able to mint and burn the tokens).

```rust
impl SharesContract {
    /// constructor with name and symbol
    #[ink(constructor)]
    pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
        let mut instance = Self::default();

        let caller = Self::env().caller();
        instance.metadata.name = name;
        instance.metadata.symbol = symbol;
        instance.metadata.decimals = 18;
        instance._init_with_owner(caller);
        
        instance
    }
}
```