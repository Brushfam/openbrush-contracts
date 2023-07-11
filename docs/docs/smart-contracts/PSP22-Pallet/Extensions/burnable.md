---
sidebar_position: 3
title: PSP22 Pallet Burnable
---

This example shows how you can reuse the implementation of
[PSP22](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet) token with [PSP22Burnable](https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet/extensions/burnable.rs) extension via `pallet-assets` chain extension.

## How to use this extension

First, you should implement basic version of [PSP22 Pallet](../psp22-pallet.md).

After you can just add implementation of PSP22PalletBurnable via `#[openbrush::implementation(PSP22PalletBurnable)]` attribute.

```rust
#[openbrush::implementation(PSP22Pallet, PSP22PalletBurnable)]
#[openbrush::contract]
pub mod my_psp22_pallet {
    ...
```

And that's it! Your `PSP22 Pallet` is now extended by the `PSP22Burnable` extension and ready to use its functions!
