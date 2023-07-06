// Copyright (c) 2012-2022 Supercolony
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

#[cfg(feature = "psp22")]
#[openbrush::implementation(PSP22, PSP22Metadata)]
#[openbrush::contract]
mod psp22_metadata {
    /// Imports all the definitions from the outer scope so we can use them here.
    use openbrush::contracts::psp22::extensions::metadata::*;
    use openbrush::traits::{
        Storage,
        String,
    };

    /// A simple PSP-22 contract.
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PSP22Struct {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP22Struct {
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut instance = Self::default();
            instance.metadata.name.set(&name);
            instance.metadata.symbol.set(&symbol);
            instance.metadata.decimals.set(&decimal);
            instance
        }
    }

    #[ink::test]
    fn init_with_name_and_symbol_works() {
        let token = PSP22Struct::new(Some(String::from("TOKEN")), Some(String::from("TKN")), 18);

        assert_eq!(PSP22Metadata::token_name(&token), Some(String::from("TOKEN")));
        assert_eq!(PSP22Metadata::token_symbol(&token), Some(String::from("TKN")));
        assert_eq!(PSP22Metadata::token_decimals(&token), 18);
    }
}
