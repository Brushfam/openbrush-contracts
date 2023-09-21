// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use ink::prelude::vec::Vec;

#[openbrush::trait_definition]
pub trait PSP61 {
    #[ink(message)]
    fn supports_interface(&self, interface_id: u32) -> bool;

    #[ink(message)]
    fn supported_interfaces(&self) -> Vec<u32>;
}
