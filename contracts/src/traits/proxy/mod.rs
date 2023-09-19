// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::traits::{
    errors::OwnableError,
    ownable::*,
};
use openbrush::traits::Hash;

#[openbrush::wrapper]
pub type ProxyRef = dyn Proxy + Ownable;

// Delegate calls were marked as a possible attack vector in ink!
// Therefore the proxy and diamond contracts will be disabled within OpenBrush until this is reimplemented in ink! 4.

#[openbrush::trait_definition]
pub trait Proxy: Ownable {
    #[ink(message)]
    fn get_delegate_code(&self) -> Hash;

    #[ink(message)]
    fn change_delegate_code(&mut self, new_code_hash: Hash) -> Result<(), OwnableError>;
}
