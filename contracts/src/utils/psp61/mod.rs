// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use crate::{
    psp61,
    traits::psp61::*,
};
use ink::prelude::{
    vec,
    vec::Vec,
};

pub trait PSP61Internal {
    fn _interfaces(&self) -> Vec<u32> {
        vec![]
    }
}

pub trait PSP61InternalOB {
    fn _interfaces_ob(&self) -> Vec<u32> {
        vec![]
    }
}

pub trait PSP61Impl: PSP61Internal + PSP61InternalOB {
    fn supports_interface(&self, interface_id: u32) -> bool {
        self._interfaces().contains(&interface_id) || self._interfaces_ob().contains(&interface_id)
    }

    fn supported_interfaces(&self) -> Vec<u32> {
        let mut interfaces = self._interfaces();
        interfaces.append(&mut self._interfaces_ob());
        interfaces
    }
}

#[macro_export]
macro_rules! supported_interfaces {
    ($contract:ident => $($interface_id:expr),*) => {
        impl ::openbrush::contracts::psp61::PSP61Internal for $contract {
            fn _interfaces(&self) -> ::ink::prelude::vec::Vec<u32> {
                ::ink::prelude::vec![$($interface_id),*]
            }
        }
    };
    ($contract:ident) => {
        impl ::openbrush::contracts::psp61::PSP61Internal for $contract {}
    };
}
