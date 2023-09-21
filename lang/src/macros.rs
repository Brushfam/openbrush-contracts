// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Return the hash of the format!("{}::{}", ::core::module_path!(), struct_name).
/// Also, if field naming is provided, it will return the hash of the format!("{}::{}::{}", ::core::module_path!(), struct_name, field_name).
/// It cam be used to generate unique storage key of the struct.
#[macro_export]
macro_rules! storage_unique_key {
    ($struct:ident) => {{
        $crate::traits::ConstHasher::hash($crate::traits::const_format::concatcp!(
            ::core::module_path!(),
            "::",
            ::core::stringify!($struct)
        ))
    }};
    ($struct:literal, $field:literal) => {{
        $crate::traits::ConstHasher::hash($crate::traits::const_format::concatcp!(
            ::core::module_path!(),
            "::",
            $struct,
            "::",
            $field
        ))
    }};
}

#[test]
fn correct_storage_key() {
    use crate::traits::ConstHasher;
    use ink::storage::traits::StorageKey;

    mod contracts {
        pub mod psp22 {
            use ink::storage::traits::StorageKey;

            pub struct Data;

            impl StorageKey for Data {
                const KEY: u32 = storage_unique_key!(Data);
            }
        }

        pub mod psp34 {
            use ink::storage::traits::StorageKey;

            pub struct Data;

            impl StorageKey for Data {
                const KEY: u32 = storage_unique_key!(Data);
            }
        }
    }

    let expected_hash_psp22 = ConstHasher::hash("openbrush_lang::macros::contracts::psp22::Data");
    assert_eq!(expected_hash_psp22, <contracts::psp22::Data as StorageKey>::KEY);

    let expected_hash_psp34 = ConstHasher::hash("openbrush_lang::macros::contracts::psp34::Data");
    assert_eq!(expected_hash_psp34, <contracts::psp34::Data as StorageKey>::KEY);
}
