// Copyright (c) 2012-2023 727-ventures
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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
