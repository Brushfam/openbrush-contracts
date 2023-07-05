---
sidebar_position: 2
title: PSP34 Mintable
---

This example shows how you can reuse the implementation of [PSP34](https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/src/token/psp34) token with [PSP34Mintable](https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/src/token/psp34/extensions/mintable.rs) extension.

## How to use this extension

First, you should implement basic version of [PSP34](/smart-contracts/PSP34).

After you can just add implementation of PSP34Mintable via `#[openbrush::implementation(PSP34Mintable)]` attribute.

```rust
use openbrush::contracts::psp34::extensions::mintable::*;

impl PSP34Mintable for Contract {}
```

And that's it! Your `PSP34` is now extended by the `PSP34Mintable` extension and ready to use its functions!
You can check an example of the usage of [PSP34 Mintable](https://github.com/727-Ventures/openbrush-contracts/tree/main/examples/psp34_extensions/mintable).

You can also check the documentation for the basic implementation of [PSP34](/smart-contracts/PSP34).
