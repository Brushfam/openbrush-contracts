// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod access;
mod finance;
pub mod governance;
mod security;
mod token;
mod upgradeability;
pub mod utils;

pub mod traits;

#[cfg(feature = "diamond")]
pub mod diamond;

// Modules with implementation of traits above
#[cfg(feature = "access_control")]
pub use access::access_control;
#[cfg(feature = "ownable")]
pub use access::ownable;
#[cfg(feature = "payment_splitter")]
pub use finance::payment_splitter;
#[cfg(feature = "timelock_controller")]
pub use governance::timelock_controller;
#[cfg(feature = "governance")]
pub use governance::*;
#[cfg(feature = "pausable")]
pub use security::pausable;
#[cfg(feature = "reentrancy_guard")]
pub use security::reentrancy_guard;
#[cfg(feature = "psp22")]
pub use token::psp22;
#[cfg(feature = "psp22_pallet")]
pub use token::psp22_pallet;
#[cfg(feature = "psp34")]
pub use token::psp34;
#[cfg(feature = "psp37")]
pub use token::psp37;
#[cfg(feature = "proxy")]
pub use upgradeability::proxy;
#[cfg(feature = "upgradeable")]
pub use upgradeability::upgradeable;
#[cfg(feature = "nonces")]
pub use utils::nonces;
#[cfg(feature = "psp61")]
pub use utils::psp61;
