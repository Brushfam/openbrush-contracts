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

#[cfg(feature = "psp34")]
#[openbrush::implementation(PSP34, PSP34Metadata)]
#[openbrush::contract]
mod psp34_metadata {
    use openbrush::traits::{
        Storage,
        String,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct PSP34Struct {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: Data,
    }

    impl PSP34Struct {
        #[ink(constructor)]
        pub fn new(id: Id, key: String, val: String) -> Self {
            let mut instance = Self::default();
            metadata::Internal::_set_attribute(&mut instance, id, key, val);
            instance
        }
    }

    #[ink::test]
    fn init_with_name_and_symbol_works() {
        let id = Id::U8(1u8);
        let nft = PSP34Struct::new(id.clone(), String::from("KEY"), String::from("VAL"));

        assert_eq!(
            PSP34Metadata::get_attribute(&nft, id.clone(), String::from("KEY")),
            Some(String::from("VAL"))
        );
    }
}
