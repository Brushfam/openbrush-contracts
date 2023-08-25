#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod my_governor {
    impl ::ink::env::ContractEnv for Contract {
        type Env = ::ink::env::DefaultEnvironment;
    }
    type Environment = <Contract as ::ink::env::ContractEnv>::Env;
    type AccountId = <<Contract as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::AccountId;
    type Balance = <<Contract as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Balance;
    type Hash = <<Contract as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Hash;
    type Timestamp = <<Contract as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Timestamp;
    type BlockNumber = <<Contract as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::BlockNumber;
    type ChainExtension = <<Contract as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::ChainExtension;
    const MAX_EVENT_TOPICS: usize = <<Contract as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::MAX_EVENT_TOPICS;
    const _: () = {
        struct Check {
            salt: (),
            field_0: Data,
        }
    };
    #[cfg(not(feature = "__ink_dylint_Storage"))]
    pub struct Contract {
        #[storage_field]
        data: <Data as ::ink::storage::traits::AutoStorableHint<
            ::ink::storage::traits::ManualKey<3689466876u32, ()>,
        >>::Type,
    }
    const _: () = {
        impl<
            __ink_generic_salt: ::ink::storage::traits::StorageKey,
        > ::ink::storage::traits::StorableHint<__ink_generic_salt> for Contract {
            type Type = Contract;
            type PreferredKey = ::ink::storage::traits::AutoKey;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageKey for Contract {
            const KEY: ::ink::primitives::Key = <() as ::ink::storage::traits::StorageKey>::KEY;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::Storable for Contract {
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn decode<__ink_I: ::scale::Input>(
                __input: &mut __ink_I,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(Contract {
                    data: <<Data as ::ink::storage::traits::AutoStorableHint<
                        ::ink::storage::traits::ManualKey<3689466876u32, ()>,
                    >>::Type as ::ink::storage::traits::Storable>::decode(__input)?,
                })
            }
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn encode<__ink_O: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __dest: &mut __ink_O,
            ) {
                match self {
                    Contract { data: __binding_0 } => {
                        ::ink::storage::traits::Storable::encode(__binding_0, __dest);
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for Contract {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new("Contract", "my_governor::my_governor"),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f
                                    .ty::<
                                        <Data as ::ink::storage::traits::AutoStorableHint<
                                            ::ink::storage::traits::ManualKey<3689466876u32, ()>,
                                        >>::Type,
                                    >()
                                    .name("data")
                                    .type_name(
                                        "<Data as::ink::storage::traits::AutoStorableHint<::ink::storage\n::traits::ManualKey<3689466876u32, ()>,>>::Type",
                                    )
                            }),
                    )
            }
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageLayout for Contract {
            fn layout(
                __key: &::ink::primitives::Key,
            ) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(
                    ::ink::metadata::layout::StructLayout::new(
                        "Contract",
                        [
                            ::ink::metadata::layout::FieldLayout::new(
                                "data",
                                <<Data as ::ink::storage::traits::AutoStorableHint<
                                    ::ink::storage::traits::ManualKey<3689466876u32, ()>,
                                >>::Type as ::ink::storage::traits::StorageLayout>::layout(
                                    __key,
                                ),
                            ),
                        ],
                    ),
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::default::Default for Contract {
        #[inline]
        fn default() -> Contract {
            Contract {
                data: ::core::default::Default::default(),
            }
        }
    }
    impl ::openbrush::traits::Storage<Data> for Contract {
        fn get(&self) -> &Data {
            &self.data
        }
        fn get_mut(&mut self) -> &mut Data {
            &mut self.data
        }
    }
    const _: () = {
        impl ::ink::reflect::ContractName for Contract {
            const NAME: &'static str = "Contract";
        }
    };
    const _: () = {
        impl<'a> ::ink::codegen::Env for &'a Contract {
            type EnvAccess = ::ink::EnvAccess<
                'a,
                <Contract as ::ink::env::ContractEnv>::Env,
            >;
            fn env(self) -> Self::EnvAccess {
                <<Self as ::ink::codegen::Env>::EnvAccess as ::core::default::Default>::default()
            }
        }
        impl<'a> ::ink::codegen::StaticEnv for Contract {
            type EnvAccess = ::ink::EnvAccess<
                'static,
                <Contract as ::ink::env::ContractEnv>::Env,
            >;
            fn env() -> Self::EnvAccess {
                <<Self as ::ink::codegen::StaticEnv>::EnvAccess as ::core::default::Default>::default()
            }
        }
    };
    const _: () = {
        #[allow(unused_imports)]
        use ::ink::codegen::{Env as _, StaticEnv as _};
    };
    impl ::ink::reflect::DispatchableConstructorInfo<0x9BAE9D5E_u32> for Contract {
        type Input = (u128, u128);
        type Output = Self;
        type Storage = Contract;
        type Error = <::ink::reflect::ConstructorOutputValue<
            Self,
        > as ::ink::reflect::ConstructorOutput<Contract>>::Error;
        const IS_RESULT: ::core::primitive::bool = <::ink::reflect::ConstructorOutputValue<
            Self,
        > as ::ink::reflect::ConstructorOutput<Contract>>::IS_RESULT;
        const CALLABLE: fn(Self::Input) -> Self::Output = |
            (__ink_binding_0, __ink_binding_1)|
        { Contract::new(__ink_binding_0, __ink_binding_1) };
        const PAYABLE: ::core::primitive::bool = false;
        const SELECTOR: [::core::primitive::u8; 4usize] = [
            0x9B_u8,
            0xAE_u8,
            0x9D_u8,
            0x5E_u8,
        ];
        const LABEL: &'static ::core::primitive::str = "new";
    }
    impl ::ink::reflect::DispatchableMessageInfo<0x14D577CC_u32> for Contract {
        type Input = ();
        type Output = u128;
        type Storage = Contract;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output = |
            storage,
            _|
        { Contract::get_value1(storage) };
        const SELECTOR: [::core::primitive::u8; 4usize] = [
            0x14_u8,
            0xD5_u8,
            0x77_u8,
            0xCC_u8,
        ];
        const PAYABLE: ::core::primitive::bool = false;
        const MUTATES: ::core::primitive::bool = false;
        const LABEL: &'static ::core::primitive::str = "get_value1";
    }
    impl ::ink::reflect::DispatchableMessageInfo<0x8667F63E_u32> for Contract {
        type Input = ();
        type Output = u128;
        type Storage = Contract;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output = |
            storage,
            _|
        { Contract::get_value2(storage) };
        const SELECTOR: [::core::primitive::u8; 4usize] = [
            0x86_u8,
            0x67_u8,
            0xF6_u8,
            0x3E_u8,
        ];
        const PAYABLE: ::core::primitive::bool = false;
        const MUTATES: ::core::primitive::bool = false;
        const LABEL: &'static ::core::primitive::str = "get_value2";
    }
    impl ::ink::reflect::DispatchableMessageInfo<0x82DADD5E_u32> for Contract {
        type Input = u128;
        type Output = ();
        type Storage = Contract;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output = |
            storage,
            __ink_binding_0|
        { Contract::set_value1(storage, __ink_binding_0) };
        const SELECTOR: [::core::primitive::u8; 4usize] = [
            0x82_u8,
            0xDA_u8,
            0xDD_u8,
            0x5E_u8,
        ];
        const PAYABLE: ::core::primitive::bool = false;
        const MUTATES: ::core::primitive::bool = true;
        const LABEL: &'static ::core::primitive::str = "set_value1";
    }
    impl ::ink::reflect::DispatchableMessageInfo<0x205C085C_u32> for Contract {
        type Input = u128;
        type Output = ();
        type Storage = Contract;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output = |
            storage,
            __ink_binding_0|
        { Contract::set_value2(storage, __ink_binding_0) };
        const SELECTOR: [::core::primitive::u8; 4usize] = [
            0x20_u8,
            0x5C_u8,
            0x08_u8,
            0x5C_u8,
        ];
        const PAYABLE: ::core::primitive::bool = false;
        const MUTATES: ::core::primitive::bool = true;
        const LABEL: &'static ::core::primitive::str = "set_value2";
    }
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_ConstructorDecoder {
            Constructor0(
                <Contract as ::ink::reflect::DispatchableConstructorInfo<
                    0x9BAE9D5E_u32,
                >>::Input,
            ),
        }
        impl ::ink::reflect::DecodeDispatch for __ink_ConstructorDecoder {
            fn decode_dispatch<I>(
                input: &mut I,
            ) -> ::core::result::Result<Self, ::ink::reflect::DispatchError>
            where
                I: ::scale::Input,
            {
                const CONSTRUCTOR_0: [::core::primitive::u8; 4usize] = <Contract as ::ink::reflect::DispatchableConstructorInfo<
                    0x9BAE9D5E_u32,
                >>::SELECTOR;
                match <[::core::primitive::u8; 4usize] as ::scale::Decode>::decode(input)
                    .map_err(|_| ::ink::reflect::DispatchError::InvalidSelector)?
                {
                    CONSTRUCTOR_0 => {
                        ::core::result::Result::Ok(
                            Self::Constructor0(
                                <<Contract as ::ink::reflect::DispatchableConstructorInfo<
                                    0x9BAE9D5E_u32,
                                >>::Input as ::scale::Decode>::decode(input)
                                    .map_err(|_| {
                                        ::ink::reflect::DispatchError::InvalidParameters
                                    })?,
                            ),
                        )
                    }
                    _invalid => {
                        ::core::result::Result::Err(
                            ::ink::reflect::DispatchError::UnknownSelector,
                        )
                    }
                }
            }
        }
        impl ::scale::Decode for __ink_ConstructorDecoder {
            fn decode<I>(input: &mut I) -> ::core::result::Result<Self, ::scale::Error>
            where
                I: ::scale::Input,
            {
                <Self as ::ink::reflect::DecodeDispatch>::decode_dispatch(input)
                    .map_err(::core::convert::Into::into)
            }
        }
        impl ::ink::reflect::ExecuteDispatchable for __ink_ConstructorDecoder {
            #[allow(clippy::nonminimal_bool)]
            fn execute_dispatchable(
                self,
            ) -> ::core::result::Result<(), ::ink::reflect::DispatchError> {
                match self {
                    Self::Constructor0(input) => {
                        if {
                            false
                                || {
                                    let constructor_0 = false;
                                    let constructor_0 = <Contract as ::ink::reflect::DispatchableConstructorInfo<
                                        0x9BAE9D5E_u32,
                                    >>::PAYABLE;
                                    constructor_0
                                }
                        }
                            && !<Contract as ::ink::reflect::DispatchableConstructorInfo<
                                0x9BAE9D5E_u32,
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Contract as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Contract as ::ink::reflect::DispatchableConstructorInfo<
                            0x9BAE9D5E_u32,
                        >>::Output = <Contract as ::ink::reflect::DispatchableConstructorInfo<
                            0x9BAE9D5E_u32,
                        >>::CALLABLE(input);
                        let output_value = ::ink::reflect::ConstructorOutputValue::new(
                            result,
                        );
                        let output_result = <::ink::reflect::ConstructorOutputValue<
                            <Contract as ::ink::reflect::DispatchableConstructorInfo<
                                0x9BAE9D5E_u32,
                            >>::Output,
                        > as ::ink::reflect::ConstructorOutput<
                            Contract,
                        >>::as_result(&output_value);
                        if let ::core::result::Result::Ok(contract)
                            = output_result.as_ref()
                        {
                            ::ink::env::set_contract_storage::<
                                ::ink::primitives::Key,
                                Contract,
                            >(
                                &<Contract as ::ink::storage::traits::StorageKey>::KEY,
                                contract,
                            );
                        }
                        ::ink::env::return_value::<
                            ::ink::ConstructorResult<
                                ::core::result::Result<
                                    (),
                                    &<::ink::reflect::ConstructorOutputValue<
                                        <Contract as ::ink::reflect::DispatchableConstructorInfo<
                                            0x9BAE9D5E_u32,
                                        >>::Output,
                                    > as ::ink::reflect::ConstructorOutput<Contract>>::Error,
                                >,
                            >,
                        >(
                            ::ink::env::ReturnFlags::new_with_reverted(
                                output_result.is_err(),
                            ),
                            &::ink::ConstructorResult::Ok(output_result.map(|_| ())),
                        );
                    }
                }
            }
        }
        impl ::ink::reflect::ContractConstructorDecoder for Contract {
            type Type = __ink_ConstructorDecoder;
        }
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_MessageDecoder {
            Message0(
                <Contract as ::ink::reflect::DispatchableMessageInfo<
                    0x14D577CC_u32,
                >>::Input,
            ),
            Message1(
                <Contract as ::ink::reflect::DispatchableMessageInfo<
                    0x8667F63E_u32,
                >>::Input,
            ),
            Message2(
                <Contract as ::ink::reflect::DispatchableMessageInfo<
                    0x82DADD5E_u32,
                >>::Input,
            ),
            Message3(
                <Contract as ::ink::reflect::DispatchableMessageInfo<
                    0x205C085C_u32,
                >>::Input,
            ),
        }
        impl ::ink::reflect::DecodeDispatch for __ink_MessageDecoder {
            fn decode_dispatch<I>(
                input: &mut I,
            ) -> ::core::result::Result<Self, ::ink::reflect::DispatchError>
            where
                I: ::scale::Input,
            {
                const MESSAGE_0: [::core::primitive::u8; 4usize] = <Contract as ::ink::reflect::DispatchableMessageInfo<
                    0x14D577CC_u32,
                >>::SELECTOR;
                const MESSAGE_1: [::core::primitive::u8; 4usize] = <Contract as ::ink::reflect::DispatchableMessageInfo<
                    0x8667F63E_u32,
                >>::SELECTOR;
                const MESSAGE_2: [::core::primitive::u8; 4usize] = <Contract as ::ink::reflect::DispatchableMessageInfo<
                    0x82DADD5E_u32,
                >>::SELECTOR;
                const MESSAGE_3: [::core::primitive::u8; 4usize] = <Contract as ::ink::reflect::DispatchableMessageInfo<
                    0x205C085C_u32,
                >>::SELECTOR;
                match <[::core::primitive::u8; 4usize] as ::scale::Decode>::decode(input)
                    .map_err(|_| ::ink::reflect::DispatchError::InvalidSelector)?
                {
                    MESSAGE_0 => {
                        ::core::result::Result::Ok(
                            Self::Message0(
                                <<Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x14D577CC_u32,
                                >>::Input as ::scale::Decode>::decode(input)
                                    .map_err(|_| {
                                        ::ink::reflect::DispatchError::InvalidParameters
                                    })?,
                            ),
                        )
                    }
                    MESSAGE_1 => {
                        ::core::result::Result::Ok(
                            Self::Message1(
                                <<Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x8667F63E_u32,
                                >>::Input as ::scale::Decode>::decode(input)
                                    .map_err(|_| {
                                        ::ink::reflect::DispatchError::InvalidParameters
                                    })?,
                            ),
                        )
                    }
                    MESSAGE_2 => {
                        ::core::result::Result::Ok(
                            Self::Message2(
                                <<Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x82DADD5E_u32,
                                >>::Input as ::scale::Decode>::decode(input)
                                    .map_err(|_| {
                                        ::ink::reflect::DispatchError::InvalidParameters
                                    })?,
                            ),
                        )
                    }
                    MESSAGE_3 => {
                        ::core::result::Result::Ok(
                            Self::Message3(
                                <<Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x205C085C_u32,
                                >>::Input as ::scale::Decode>::decode(input)
                                    .map_err(|_| {
                                        ::ink::reflect::DispatchError::InvalidParameters
                                    })?,
                            ),
                        )
                    }
                    _invalid => {
                        ::core::result::Result::Err(
                            ::ink::reflect::DispatchError::UnknownSelector,
                        )
                    }
                }
            }
        }
        impl ::scale::Decode for __ink_MessageDecoder {
            fn decode<I>(input: &mut I) -> ::core::result::Result<Self, ::scale::Error>
            where
                I: ::scale::Input,
            {
                <Self as ::ink::reflect::DecodeDispatch>::decode_dispatch(input)
                    .map_err(::core::convert::Into::into)
            }
        }
        fn push_contract(contract: ::core::mem::ManuallyDrop<Contract>, mutates: bool) {
            if mutates {
                ::ink::env::set_contract_storage::<
                    ::ink::primitives::Key,
                    Contract,
                >(&<Contract as ::ink::storage::traits::StorageKey>::KEY, &contract);
            }
        }
        impl ::ink::reflect::ExecuteDispatchable for __ink_MessageDecoder {
            #[allow(clippy::nonminimal_bool, clippy::let_unit_value)]
            fn execute_dispatchable(
                self,
            ) -> ::core::result::Result<(), ::ink::reflect::DispatchError> {
                let key = <Contract as ::ink::storage::traits::StorageKey>::KEY;
                let mut contract: ::core::mem::ManuallyDrop<Contract> = ::core::mem::ManuallyDrop::new(
                    match ::ink::env::get_contract_storage(&key) {
                        ::core::result::Result::Ok(
                            ::core::option::Option::Some(value),
                        ) => value,
                        ::core::result::Result::Ok(::core::option::Option::None) => {
                            ::core::panicking::panic_fmt(
                                format_args!("storage entry was empty"),
                            )
                        }
                        ::core::result::Result::Err(_) => {
                            ::core::panicking::panic_fmt(
                                format_args!("could not properly decode storage entry"),
                            )
                        }
                    },
                );
                match self {
                    Self::Message0(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x14D577CC_u32,
                                    >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x8667F63E_u32,
                                    >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x82DADD5E_u32,
                                    >>::PAYABLE;
                                    message_2
                                }
                                || {
                                    let message_3 = false;
                                    let message_3 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x205C085C_u32,
                                    >>::PAYABLE;
                                    message_3
                                }
                        }
                            && !<Contract as ::ink::reflect::DispatchableMessageInfo<
                                0x14D577CC_u32,
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Contract as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Contract as ::ink::reflect::DispatchableMessageInfo<
                            0x14D577CC_u32,
                        >>::Output = <Contract as ::ink::reflect::DispatchableMessageInfo<
                            0x14D577CC_u32,
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x14D577CC_u32,
                                >>::Output,
                            >::VALUE
                        }
                            && {
                                #[allow(unused_imports)]
                                use ::ink::result_info::IsResultErrFallback as _;
                                ::ink::result_info::IsResultErr(&result).value()
                            };
                        if !is_reverted {
                            push_contract(
                                contract,
                                <Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x14D577CC_u32,
                                >>::MUTATES,
                            );
                        }
                        ::ink::env::return_value::<
                            ::ink::MessageResult<
                                <Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x14D577CC_u32,
                                >>::Output,
                            >,
                        >(
                            ::ink::env::ReturnFlags::new_with_reverted(is_reverted),
                            &::ink::MessageResult::Ok(result),
                        )
                    }
                    Self::Message1(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x14D577CC_u32,
                                    >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x8667F63E_u32,
                                    >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x82DADD5E_u32,
                                    >>::PAYABLE;
                                    message_2
                                }
                                || {
                                    let message_3 = false;
                                    let message_3 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x205C085C_u32,
                                    >>::PAYABLE;
                                    message_3
                                }
                        }
                            && !<Contract as ::ink::reflect::DispatchableMessageInfo<
                                0x8667F63E_u32,
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Contract as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Contract as ::ink::reflect::DispatchableMessageInfo<
                            0x8667F63E_u32,
                        >>::Output = <Contract as ::ink::reflect::DispatchableMessageInfo<
                            0x8667F63E_u32,
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x8667F63E_u32,
                                >>::Output,
                            >::VALUE
                        }
                            && {
                                #[allow(unused_imports)]
                                use ::ink::result_info::IsResultErrFallback as _;
                                ::ink::result_info::IsResultErr(&result).value()
                            };
                        if !is_reverted {
                            push_contract(
                                contract,
                                <Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x8667F63E_u32,
                                >>::MUTATES,
                            );
                        }
                        ::ink::env::return_value::<
                            ::ink::MessageResult<
                                <Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x8667F63E_u32,
                                >>::Output,
                            >,
                        >(
                            ::ink::env::ReturnFlags::new_with_reverted(is_reverted),
                            &::ink::MessageResult::Ok(result),
                        )
                    }
                    Self::Message2(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x14D577CC_u32,
                                    >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x8667F63E_u32,
                                    >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x82DADD5E_u32,
                                    >>::PAYABLE;
                                    message_2
                                }
                                || {
                                    let message_3 = false;
                                    let message_3 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x205C085C_u32,
                                    >>::PAYABLE;
                                    message_3
                                }
                        }
                            && !<Contract as ::ink::reflect::DispatchableMessageInfo<
                                0x82DADD5E_u32,
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Contract as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Contract as ::ink::reflect::DispatchableMessageInfo<
                            0x82DADD5E_u32,
                        >>::Output = <Contract as ::ink::reflect::DispatchableMessageInfo<
                            0x82DADD5E_u32,
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x82DADD5E_u32,
                                >>::Output,
                            >::VALUE
                        }
                            && {
                                #[allow(unused_imports)]
                                use ::ink::result_info::IsResultErrFallback as _;
                                ::ink::result_info::IsResultErr(&result).value()
                            };
                        if !is_reverted {
                            push_contract(
                                contract,
                                <Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x82DADD5E_u32,
                                >>::MUTATES,
                            );
                        }
                        ::ink::env::return_value::<
                            ::ink::MessageResult<
                                <Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x82DADD5E_u32,
                                >>::Output,
                            >,
                        >(
                            ::ink::env::ReturnFlags::new_with_reverted(is_reverted),
                            &::ink::MessageResult::Ok(result),
                        )
                    }
                    Self::Message3(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x14D577CC_u32,
                                    >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x8667F63E_u32,
                                    >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x82DADD5E_u32,
                                    >>::PAYABLE;
                                    message_2
                                }
                                || {
                                    let message_3 = false;
                                    let message_3 = <Contract as ::ink::reflect::DispatchableMessageInfo<
                                        0x205C085C_u32,
                                    >>::PAYABLE;
                                    message_3
                                }
                        }
                            && !<Contract as ::ink::reflect::DispatchableMessageInfo<
                                0x205C085C_u32,
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Contract as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Contract as ::ink::reflect::DispatchableMessageInfo<
                            0x205C085C_u32,
                        >>::Output = <Contract as ::ink::reflect::DispatchableMessageInfo<
                            0x205C085C_u32,
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x205C085C_u32,
                                >>::Output,
                            >::VALUE
                        }
                            && {
                                #[allow(unused_imports)]
                                use ::ink::result_info::IsResultErrFallback as _;
                                ::ink::result_info::IsResultErr(&result).value()
                            };
                        if !is_reverted {
                            push_contract(
                                contract,
                                <Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x205C085C_u32,
                                >>::MUTATES,
                            );
                        }
                        ::ink::env::return_value::<
                            ::ink::MessageResult<
                                <Contract as ::ink::reflect::DispatchableMessageInfo<
                                    0x205C085C_u32,
                                >>::Output,
                            >,
                        >(
                            ::ink::env::ReturnFlags::new_with_reverted(is_reverted),
                            &::ink::MessageResult::Ok(result),
                        )
                    }
                };
            }
        }
        impl ::ink::reflect::ContractMessageDecoder for Contract {
            type Type = __ink_MessageDecoder;
        }
    };
    const _: () = {
        use ::ink::codegen::{Env as _, StaticEnv as _};
        const _: ::ink::codegen::utils::IsSameType<Contract> = ::ink::codegen::utils::IsSameType::<
            Contract,
        >::new();
        impl Contract {
            #[cfg(not(feature = "__ink_dylint_Constructor"))]
            pub fn new(value1: u128, value2: u128) -> Self {
                let mut instance = Self::default();
                instance.data.value1.set(&value1);
                instance.data.value2 = value2;
                instance
            }
            pub fn get_value1(&self) -> u128 {
                self.data.value1.get_or_default()
            }
            pub fn get_value2(&self) -> u128 {
                self.data.value2
            }
            pub fn set_value1(&mut self, value: u128) {
                self.data.value1.set(&value);
            }
            pub fn set_value2(&mut self, value: u128) {
                self.data.value2 = value;
            }
        }
        const _: () = {
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<u128>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<u128>>();
            ::ink::codegen::utils::consume_type::<
                ::ink::codegen::DispatchOutput<u128>,
            >();
            ::ink::codegen::utils::consume_type::<
                ::ink::codegen::DispatchOutput<u128>,
            >();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<u128>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<u128>>();
        };
    };
    const _: () = {
        /// The ink! smart contract's call builder.
        ///
        /// Implements the underlying on-chain calling of the ink! smart contract
        /// messages and trait implementations in a type safe way.
        #[repr(transparent)]
        pub struct CallBuilder {
            account_id: AccountId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CallBuilder {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "CallBuilder",
                    "account_id",
                    &&self.account_id,
                )
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::scale::Encode for CallBuilder {
                fn size_hint(&self) -> usize {
                    ::scale::Encode::size_hint(&&self.account_id)
                }
                fn encode_to<
                    __CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    ::scale::Encode::encode_to(&&self.account_id, __codec_dest_edqy)
                }
                fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                    ::scale::Encode::encode(&&self.account_id)
                }
                fn using_encoded<
                    R,
                    F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R,
                >(&self, f: F) -> R {
                    ::scale::Encode::using_encoded(&&self.account_id, f)
                }
            }
            #[automatically_derived]
            impl ::scale::EncodeLike for CallBuilder {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::scale::Decode for CallBuilder {
                fn decode<__CodecInputEdqy: ::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::scale::Error> {
                    ::core::result::Result::Ok(CallBuilder {
                        account_id: {
                            let __codec_res_edqy = <AccountId as ::scale::Decode>::decode(
                                __codec_input_edqy,
                            );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `CallBuilder::account_id`"),
                                    );
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                    __codec_res_edqy
                                }
                            }
                        },
                    })
                }
                fn decode_into<__CodecInputEdqy: ::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                    dst_: &mut ::core::mem::MaybeUninit<Self>,
                ) -> ::core::result::Result<::scale::DecodeFinished, ::scale::Error> {
                    match (
                        &::core::mem::size_of::<AccountId>(),
                        &::core::mem::size_of::<Self>(),
                    ) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    if !(if ::core::mem::size_of::<AccountId>() > 0 { 1 } else { 0 }
                        <= 1)
                    {
                        ::core::panicking::panic(
                            "assertion failed: if ::core::mem::size_of::<AccountId>() > 0 { 1 } else { 0 } <= 1",
                        )
                    }
                    {
                        let dst_: &mut ::core::mem::MaybeUninit<Self> = dst_;
                        let dst_: &mut ::core::mem::MaybeUninit<AccountId> = unsafe {
                            &mut *dst_
                                .as_mut_ptr()
                                .cast::<::core::mem::MaybeUninit<AccountId>>()
                        };
                        <AccountId as ::scale::Decode>::decode_into(
                            __codec_input_edqy,
                            dst_,
                        )?;
                    }
                    unsafe {
                        ::core::result::Result::Ok(
                            ::scale::DecodeFinished::assert_decoding_finished(),
                        )
                    }
                }
            }
        };
        #[automatically_derived]
        impl ::core::hash::Hash for CallBuilder {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.account_id, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CallBuilder {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CallBuilder {
            #[inline]
            fn eq(&self, other: &CallBuilder) -> bool {
                self.account_id == other.account_id
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for CallBuilder {}
        #[automatically_derived]
        impl ::core::cmp::Eq for CallBuilder {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CallBuilder {
            #[inline]
            fn clone(&self) -> CallBuilder {
                CallBuilder {
                    account_id: ::core::clone::Clone::clone(&self.account_id),
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for CallBuilder {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new(
                                "CallBuilder",
                                "my_governor::my_governor",
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .docs(
                            &[
                                "The ink! smart contract's call builder.",
                                "",
                                "Implements the underlying on-chain calling of the ink! smart contract",
                                "messages and trait implementations in a type safe way.",
                            ],
                        )
                        .composite(
                            ::scale_info::build::Fields::named()
                                .field(|f| {
                                    f
                                        .ty::<AccountId>()
                                        .name("account_id")
                                        .type_name("AccountId")
                                }),
                        )
                }
            }
        };
        const _: () = {
            impl ::ink::storage::traits::StorageLayout for CallBuilder {
                fn layout(
                    __key: &::ink::primitives::Key,
                ) -> ::ink::metadata::layout::Layout {
                    ::ink::metadata::layout::Layout::Struct(
                        ::ink::metadata::layout::StructLayout::new(
                            "CallBuilder",
                            [
                                ::ink::metadata::layout::FieldLayout::new(
                                    "account_id",
                                    <AccountId as ::ink::storage::traits::StorageLayout>::layout(
                                        __key,
                                    ),
                                ),
                            ],
                        ),
                    )
                }
            }
        };
        const _: () = {
            impl ::ink::codegen::ContractCallBuilder for Contract {
                type Type = CallBuilder;
            }
            impl ::ink::env::ContractEnv for CallBuilder {
                type Env = <Contract as ::ink::env::ContractEnv>::Env;
            }
        };
        impl ::ink::env::call::FromAccountId<Environment> for CallBuilder {
            #[inline]
            fn from_account_id(account_id: AccountId) -> Self {
                Self { account_id }
            }
        }
        impl ::ink::ToAccountId<Environment> for CallBuilder {
            #[inline]
            fn to_account_id(&self) -> AccountId {
                <AccountId as ::core::clone::Clone>::clone(&self.account_id)
            }
        }
        impl ::core::convert::AsRef<AccountId> for CallBuilder {
            fn as_ref(&self) -> &AccountId {
                &self.account_id
            }
        }
        impl ::core::convert::AsMut<AccountId> for CallBuilder {
            fn as_mut(&mut self) -> &mut AccountId {
                &mut self.account_id
            }
        }
        impl CallBuilder {
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn get_value1(
                &self,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Environment>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::EmptyArgumentList,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<u128>>,
            > {
                ::ink::env::call::build_call::<Environment>()
                    .call(::ink::ToAccountId::to_account_id(self))
                    .exec_input(
                        ::ink::env::call::ExecutionInput::new(
                            ::ink::env::call::Selector::new([
                                0x14_u8,
                                0xD5_u8,
                                0x77_u8,
                                0xCC_u8,
                            ]),
                        ),
                    )
                    .returns::<u128>()
            }
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn get_value2(
                &self,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Environment>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::EmptyArgumentList,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<u128>>,
            > {
                ::ink::env::call::build_call::<Environment>()
                    .call(::ink::ToAccountId::to_account_id(self))
                    .exec_input(
                        ::ink::env::call::ExecutionInput::new(
                            ::ink::env::call::Selector::new([
                                0x86_u8,
                                0x67_u8,
                                0xF6_u8,
                                0x3E_u8,
                            ]),
                        ),
                    )
                    .returns::<u128>()
            }
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn set_value1(
                &mut self,
                __ink_binding_0: u128,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Environment>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::ArgumentList<
                            ::ink::env::call::utils::Argument<u128>,
                            ::ink::env::call::utils::EmptyArgumentList,
                        >,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<()>>,
            > {
                ::ink::env::call::build_call::<Environment>()
                    .call(::ink::ToAccountId::to_account_id(self))
                    .exec_input(
                        ::ink::env::call::ExecutionInput::new(
                                ::ink::env::call::Selector::new([
                                    0x82_u8,
                                    0xDA_u8,
                                    0xDD_u8,
                                    0x5E_u8,
                                ]),
                            )
                            .push_arg(__ink_binding_0),
                    )
                    .returns::<()>()
            }
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn set_value2(
                &mut self,
                __ink_binding_0: u128,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Environment>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::ArgumentList<
                            ::ink::env::call::utils::Argument<u128>,
                            ::ink::env::call::utils::EmptyArgumentList,
                        >,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<()>>,
            > {
                ::ink::env::call::build_call::<Environment>()
                    .call(::ink::ToAccountId::to_account_id(self))
                    .exec_input(
                        ::ink::env::call::ExecutionInput::new(
                                ::ink::env::call::Selector::new([
                                    0x20_u8,
                                    0x5C_u8,
                                    0x08_u8,
                                    0x5C_u8,
                                ]),
                            )
                            .push_arg(__ink_binding_0),
                    )
                    .returns::<()>()
            }
        }
    };
    pub struct ContractRef {
        inner: <Contract as ::ink::codegen::ContractCallBuilder>::Type,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ContractRef {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ContractRef",
                "inner",
                &&self.inner,
            )
        }
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Encode for ContractRef {
            fn size_hint(&self) -> usize {
                ::scale::Encode::size_hint(&&self.inner)
            }
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::scale::Encode::encode_to(&&self.inner, __codec_dest_edqy)
            }
            fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                ::scale::Encode::encode(&&self.inner)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::scale::Encode::using_encoded(&&self.inner, f)
            }
        }
        #[automatically_derived]
        impl ::scale::EncodeLike for ContractRef {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Decode for ContractRef {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(ContractRef {
                    inner: {
                        let __codec_res_edqy = <<Contract as ::ink::codegen::ContractCallBuilder>::Type as ::scale::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `ContractRef::inner`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                __codec_res_edqy
                            }
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    impl ::core::hash::Hash for ContractRef {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ContractRef {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ContractRef {
        #[inline]
        fn eq(&self, other: &ContractRef) -> bool {
            self.inner == other.inner
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for ContractRef {}
    #[automatically_derived]
    impl ::core::cmp::Eq for ContractRef {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <Contract as ::ink::codegen::ContractCallBuilder>::Type,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ContractRef {
        #[inline]
        fn clone(&self) -> ContractRef {
            ContractRef {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for ContractRef {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new(
                            "ContractRef",
                            "my_governor::my_governor",
                        ),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f
                                    .ty::<
                                        <Contract as ::ink::codegen::ContractCallBuilder>::Type,
                                    >()
                                    .name("inner")
                                    .type_name(
                                        "<Contract as::ink::codegen::ContractCallBuilder>::Type",
                                    )
                            }),
                    )
            }
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageLayout for ContractRef {
            fn layout(
                __key: &::ink::primitives::Key,
            ) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(
                    ::ink::metadata::layout::StructLayout::new(
                        "ContractRef",
                        [
                            ::ink::metadata::layout::FieldLayout::new(
                                "inner",
                                <<Contract as ::ink::codegen::ContractCallBuilder>::Type as ::ink::storage::traits::StorageLayout>::layout(
                                    __key,
                                ),
                            ),
                        ],
                    ),
                )
            }
        }
    };
    const _: () = {
        impl ::ink::env::ContractReference for Contract {
            type Type = ContractRef;
        }
        impl ::ink::env::call::ConstructorReturnType<ContractRef> for Contract {
            type Output = ContractRef;
            type Error = ();
            fn ok(value: ContractRef) -> Self::Output {
                value
            }
        }
        impl<E> ::ink::env::call::ConstructorReturnType<ContractRef>
        for ::core::result::Result<Contract, E>
        where
            E: ::scale::Decode,
        {
            const IS_RESULT: bool = true;
            type Output = ::core::result::Result<ContractRef, E>;
            type Error = E;
            fn ok(value: ContractRef) -> Self::Output {
                ::core::result::Result::Ok(value)
            }
            fn err(err: Self::Error) -> ::core::option::Option<Self::Output> {
                ::core::option::Option::Some(::core::result::Result::Err(err))
            }
        }
        impl ::ink::env::ContractEnv for ContractRef {
            type Env = <Contract as ::ink::env::ContractEnv>::Env;
        }
    };
    impl ContractRef {
        #[inline]
        #[allow(clippy::type_complexity)]
        pub fn new(
            __ink_binding_0: u128,
            __ink_binding_1: u128,
        ) -> ::ink::env::call::CreateBuilder<
            Environment,
            Self,
            ::ink::env::call::utils::Unset<Hash>,
            ::ink::env::call::utils::Unset<u64>,
            ::ink::env::call::utils::Unset<Balance>,
            ::ink::env::call::utils::Set<
                ::ink::env::call::ExecutionInput<
                    ::ink::env::call::utils::ArgumentList<
                        ::ink::env::call::utils::Argument<u128>,
                        ::ink::env::call::utils::ArgumentList<
                            ::ink::env::call::utils::Argument<u128>,
                            ::ink::env::call::utils::EmptyArgumentList,
                        >,
                    >,
                >,
            >,
            ::ink::env::call::utils::Unset<::ink::env::call::state::Salt>,
            ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<Self>>,
        > {
            ::ink::env::call::build_create::<Self>()
                .exec_input(
                    ::ink::env::call::ExecutionInput::new(
                            ::ink::env::call::Selector::new([
                                0x9B_u8,
                                0xAE_u8,
                                0x9D_u8,
                                0x5E_u8,
                            ]),
                        )
                        .push_arg(__ink_binding_0)
                        .push_arg(__ink_binding_1),
                )
                .returns::<Self>()
        }
        #[inline]
        pub fn get_value1(&self) -> u128 {
            self.try_get_value1()
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "Contract",
                        "get_value1",
                        error,
                    ),
                ))
        }
        #[inline]
        pub fn try_get_value1(&self) -> ::ink::MessageResult<u128> {
            <Self as ::ink::codegen::TraitCallBuilder>::call(self)
                .get_value1()
                .try_invoke()
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "Contract",
                        "get_value1",
                        error,
                    ),
                ))
        }
        #[inline]
        pub fn get_value2(&self) -> u128 {
            self.try_get_value2()
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "Contract",
                        "get_value2",
                        error,
                    ),
                ))
        }
        #[inline]
        pub fn try_get_value2(&self) -> ::ink::MessageResult<u128> {
            <Self as ::ink::codegen::TraitCallBuilder>::call(self)
                .get_value2()
                .try_invoke()
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "Contract",
                        "get_value2",
                        error,
                    ),
                ))
        }
        #[inline]
        pub fn set_value1(&mut self, value: u128) {
            self.try_set_value1(value)
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "Contract",
                        "set_value1",
                        error,
                    ),
                ))
        }
        #[inline]
        pub fn try_set_value1(&mut self, value: u128) -> ::ink::MessageResult<()> {
            <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self)
                .set_value1(value)
                .try_invoke()
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "Contract",
                        "set_value1",
                        error,
                    ),
                ))
        }
        #[inline]
        pub fn set_value2(&mut self, value: u128) {
            self.try_set_value2(value)
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "Contract",
                        "set_value2",
                        error,
                    ),
                ))
        }
        #[inline]
        pub fn try_set_value2(&mut self, value: u128) -> ::ink::MessageResult<()> {
            <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self)
                .set_value2(value)
                .try_invoke()
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "Contract",
                        "set_value2",
                        error,
                    ),
                ))
        }
    }
    const _: () = {
        impl ::ink::codegen::TraitCallBuilder for ContractRef {
            type Builder = <Contract as ::ink::codegen::ContractCallBuilder>::Type;
            #[inline]
            fn call(&self) -> &Self::Builder {
                &self.inner
            }
            #[inline]
            fn call_mut(&mut self) -> &mut Self::Builder {
                &mut self.inner
            }
        }
    };
    impl ::ink::env::call::FromAccountId<Environment> for ContractRef {
        #[inline]
        fn from_account_id(account_id: AccountId) -> Self {
            Self {
                inner: <<Contract as ::ink::codegen::ContractCallBuilder>::Type as ::ink::env::call::FromAccountId<
                    Environment,
                >>::from_account_id(account_id),
            }
        }
    }
    impl ::ink::ToAccountId<Environment> for ContractRef {
        #[inline]
        fn to_account_id(&self) -> AccountId {
            <<Contract as ::ink::codegen::ContractCallBuilder>::Type as ::ink::ToAccountId<
                Environment,
            >>::to_account_id(&self.inner)
        }
    }
    impl ::core::convert::AsRef<AccountId> for ContractRef {
        fn as_ref(&self) -> &AccountId {
            <_ as ::core::convert::AsRef<AccountId>>::as_ref(&self.inner)
        }
    }
    impl ::core::convert::AsMut<AccountId> for ContractRef {
        fn as_mut(&mut self) -> &mut AccountId {
            <_ as ::core::convert::AsMut<AccountId>>::as_mut(&mut self.inner)
        }
    }
    #[cfg(feature = "std")]
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        #[no_mangle]
        pub fn __ink_generate_metadata() -> ::ink::metadata::InkProject {
            let layout = ::ink::metadata::layout::Layout::Root(
                ::ink::metadata::layout::RootLayout::new(
                    <::ink::metadata::layout::LayoutKey as ::core::convert::From<
                        ::ink::primitives::Key,
                    >>::from(<Contract as ::ink::storage::traits::StorageKey>::KEY),
                    <Contract as ::ink::storage::traits::StorageLayout>::layout(
                        &<Contract as ::ink::storage::traits::StorageKey>::KEY,
                    ),
                ),
            );
            ::ink::metadata::layout::ValidateLayout::validate(&layout)
                .unwrap_or_else(|error| {
                    ::core::panicking::panic_fmt(
                        format_args!("metadata ink! generation failed: {0}", error),
                    )
                });
            ::ink::metadata::InkProject::new(
                layout,
                ::ink::metadata::ContractSpec::new()
                    .constructors([
                        ::ink::metadata::ConstructorSpec::from_label("new")
                            .selector([0x9B_u8, 0xAE_u8, 0x9D_u8, 0x5E_u8])
                            .args([
                                ::ink::metadata::MessageParamSpec::new("value1")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            u128,
                                            _,
                                        >(
                                            ::core::iter::Iterator::map(
                                                ::core::iter::IntoIterator::into_iter(["u128"]),
                                                ::core::convert::AsRef::as_ref,
                                            ),
                                        ),
                                    )
                                    .done(),
                                ::ink::metadata::MessageParamSpec::new("value2")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            u128,
                                            _,
                                        >(
                                            ::core::iter::Iterator::map(
                                                ::core::iter::IntoIterator::into_iter(["u128"]),
                                                ::core::convert::AsRef::as_ref,
                                            ),
                                        ),
                                    )
                                    .done(),
                            ])
                            .payable(false)
                            .default(false)
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    if <Contract as ::ink::reflect::DispatchableConstructorInfo<
                                        2611912030u32,
                                    >>::IS_RESULT {
                                        ::core::option::Option::Some(
                                            ::ink::metadata::TypeSpec::with_name_str::<
                                                ::ink::ConstructorResult<
                                                    ::core::result::Result<
                                                        (),
                                                        <Contract as ::ink::reflect::DispatchableConstructorInfo<
                                                            2611912030u32,
                                                        >>::Error,
                                                    >,
                                                >,
                                            >("ink_primitives::ConstructorResult"),
                                        )
                                    } else {
                                        ::core::option::Option::Some(
                                            ::ink::metadata::TypeSpec::with_name_str::<
                                                ::ink::ConstructorResult<()>,
                                            >("ink_primitives::ConstructorResult"),
                                        )
                                    },
                                ),
                            )
                            .docs([])
                            .done(),
                    ])
                    .messages([
                        ::ink::metadata::MessageSpec::from_label("get_value1")
                            .selector([0x14_u8, 0xD5_u8, 0x77_u8, 0xCC_u8])
                            .args([])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::ink::metadata::TypeSpec::with_name_segs::<
                                        ::ink::MessageResult<u128>,
                                        _,
                                    >(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([
                                                "ink",
                                                "MessageResult",
                                            ]),
                                            ::core::convert::AsRef::as_ref,
                                        ),
                                    ),
                                ),
                            )
                            .mutates(false)
                            .payable(false)
                            .default(false)
                            .docs([])
                            .done(),
                        ::ink::metadata::MessageSpec::from_label("get_value2")
                            .selector([0x86_u8, 0x67_u8, 0xF6_u8, 0x3E_u8])
                            .args([])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::ink::metadata::TypeSpec::with_name_segs::<
                                        ::ink::MessageResult<u128>,
                                        _,
                                    >(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([
                                                "ink",
                                                "MessageResult",
                                            ]),
                                            ::core::convert::AsRef::as_ref,
                                        ),
                                    ),
                                ),
                            )
                            .mutates(false)
                            .payable(false)
                            .default(false)
                            .docs([])
                            .done(),
                        ::ink::metadata::MessageSpec::from_label("set_value1")
                            .selector([0x82_u8, 0xDA_u8, 0xDD_u8, 0x5E_u8])
                            .args([
                                ::ink::metadata::MessageParamSpec::new("value")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            u128,
                                            _,
                                        >(
                                            ::core::iter::Iterator::map(
                                                ::core::iter::IntoIterator::into_iter(["u128"]),
                                                ::core::convert::AsRef::as_ref,
                                            ),
                                        ),
                                    )
                                    .done(),
                            ])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::ink::metadata::TypeSpec::with_name_segs::<
                                        ::ink::MessageResult<()>,
                                        _,
                                    >(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([
                                                "ink",
                                                "MessageResult",
                                            ]),
                                            ::core::convert::AsRef::as_ref,
                                        ),
                                    ),
                                ),
                            )
                            .mutates(true)
                            .payable(false)
                            .default(false)
                            .docs([])
                            .done(),
                        ::ink::metadata::MessageSpec::from_label("set_value2")
                            .selector([0x20_u8, 0x5C_u8, 0x08_u8, 0x5C_u8])
                            .args([
                                ::ink::metadata::MessageParamSpec::new("value")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            u128,
                                            _,
                                        >(
                                            ::core::iter::Iterator::map(
                                                ::core::iter::IntoIterator::into_iter(["u128"]),
                                                ::core::convert::AsRef::as_ref,
                                            ),
                                        ),
                                    )
                                    .done(),
                            ])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::ink::metadata::TypeSpec::with_name_segs::<
                                        ::ink::MessageResult<()>,
                                        _,
                                    >(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([
                                                "ink",
                                                "MessageResult",
                                            ]),
                                            ::core::convert::AsRef::as_ref,
                                        ),
                                    ),
                                ),
                            )
                            .mutates(true)
                            .payable(false)
                            .default(false)
                            .docs([])
                            .done(),
                    ])
                    .events([])
                    .docs([])
                    .lang_error(
                        ::ink::metadata::TypeSpec::with_name_segs::<
                            ::ink::LangError,
                            _,
                        >(
                            ::core::iter::Iterator::map(
                                ::core::iter::IntoIterator::into_iter(["ink", "LangError"]),
                                ::core::convert::AsRef::as_ref,
                            ),
                        ),
                    )
                    .environment(
                        ::ink::metadata::EnvironmentSpec::new()
                            .account_id(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    AccountId,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["AccountId"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .balance(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    Balance,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["Balance"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .hash(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    Hash,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["Hash"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .timestamp(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    Timestamp,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["Timestamp"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .block_number(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    BlockNumber,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["BlockNumber"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .chain_extension(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    ChainExtension,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["ChainExtension"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .max_event_topics(MAX_EVENT_TOPICS)
                            .done(),
                    )
                    .done(),
            )
        }
    };
    use ink::prelude::vec::Vec;
    use openbrush::traits::{Storage, String};
    const _: () = {
        struct Check {
            salt: (),
            field_0: ::ink::storage::Lazy<
                u128,
                ::ink::storage::traits::ManualKey<STORAGE_KEY_DATA_VALUE1>,
            >,
            field_1: u128,
        }
    };
    pub struct Data {
        pub value1: <::ink::storage::Lazy<
            u128,
            ::ink::storage::traits::ManualKey<STORAGE_KEY_DATA_VALUE1>,
        > as ::ink::storage::traits::AutoStorableHint<
            ::ink::storage::traits::ManualKey<958258045u32, ()>,
        >>::Type,
        pub value2: <u128 as ::ink::storage::traits::AutoStorableHint<
            ::ink::storage::traits::ManualKey<4259833659u32, ()>,
        >>::Type,
    }
    const _: () = {
        impl<
            __ink_generic_salt: ::ink::storage::traits::StorageKey,
        > ::ink::storage::traits::StorableHint<__ink_generic_salt> for Data {
            type Type = Data;
            type PreferredKey = ::ink::storage::traits::AutoKey;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageKey for Data {
            const KEY: ::ink::primitives::Key = <() as ::ink::storage::traits::StorageKey>::KEY;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::Storable for Data {
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn decode<__ink_I: ::scale::Input>(
                __input: &mut __ink_I,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(Data {
                    value1: <<::ink::storage::Lazy<
                        u128,
                        ::ink::storage::traits::ManualKey<STORAGE_KEY_DATA_VALUE1>,
                    > as ::ink::storage::traits::AutoStorableHint<
                        ::ink::storage::traits::ManualKey<958258045u32, ()>,
                    >>::Type as ::ink::storage::traits::Storable>::decode(__input)?,
                    value2: <<u128 as ::ink::storage::traits::AutoStorableHint<
                        ::ink::storage::traits::ManualKey<4259833659u32, ()>,
                    >>::Type as ::ink::storage::traits::Storable>::decode(__input)?,
                })
            }
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn encode<__ink_O: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __dest: &mut __ink_O,
            ) {
                match self {
                    Data { value1: __binding_0, value2: __binding_1 } => {
                        {
                            ::ink::storage::traits::Storable::encode(
                                __binding_0,
                                __dest,
                            );
                        }
                        {
                            ::ink::storage::traits::Storable::encode(
                                __binding_1,
                                __dest,
                            );
                        }
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for Data {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Data", "my_governor::my_governor"))
                    .type_params(::alloc::vec::Vec::new())
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f
                                    .ty::<
                                        <::ink::storage::Lazy<
                                            u128,
                                            ::ink::storage::traits::ManualKey<STORAGE_KEY_DATA_VALUE1>,
                                        > as ::ink::storage::traits::AutoStorableHint<
                                            ::ink::storage::traits::ManualKey<958258045u32, ()>,
                                        >>::Type,
                                    >()
                                    .name("value1")
                                    .type_name(
                                        "<::ink::storage::Lazy<u128,::ink::storage::traits::ManualKey<\nSTORAGE_KEY_DATA_VALUE1>> as::ink::storage::traits::AutoStorableHint\n<::ink::storage::traits::ManualKey<958258045u32, ()>,>>::Type",
                                    )
                            })
                            .field(|f| {
                                f
                                    .ty::<
                                        <u128 as ::ink::storage::traits::AutoStorableHint<
                                            ::ink::storage::traits::ManualKey<4259833659u32, ()>,
                                        >>::Type,
                                    >()
                                    .name("value2")
                                    .type_name(
                                        "<u128 as::ink::storage::traits::AutoStorableHint<::ink::storage\n::traits::ManualKey<4259833659u32, ()>,>>::Type",
                                    )
                            }),
                    )
            }
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageLayout for Data {
            fn layout(
                __key: &::ink::primitives::Key,
            ) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(
                    ::ink::metadata::layout::StructLayout::new(
                        "Data",
                        [
                            ::ink::metadata::layout::FieldLayout::new(
                                "value1",
                                <<::ink::storage::Lazy<
                                    u128,
                                    ::ink::storage::traits::ManualKey<STORAGE_KEY_DATA_VALUE1>,
                                > as ::ink::storage::traits::AutoStorableHint<
                                    ::ink::storage::traits::ManualKey<958258045u32, ()>,
                                >>::Type as ::ink::storage::traits::StorageLayout>::layout(
                                    __key,
                                ),
                            ),
                            ::ink::metadata::layout::FieldLayout::new(
                                "value2",
                                <<u128 as ::ink::storage::traits::AutoStorableHint<
                                    ::ink::storage::traits::ManualKey<4259833659u32, ()>,
                                >>::Type as ::ink::storage::traits::StorageLayout>::layout(
                                    __key,
                                ),
                            ),
                        ],
                    ),
                )
            }
        }
    };
    pub const STORAGE_KEY_DATA_VALUE1: u32 = {
        ::openbrush_lang::utils::ConstHasher::hash(
            ::const_format::pmr::__AssertStr {
                x: {
                    use ::const_format::__cf_osRcTFl4A;
                    ({
                        #[doc(hidden)]
                        #[allow(unused_mut, non_snake_case)]
                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                            &[
                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                        "my_governor::my_governor",
                                    )
                                    .to_pargument_display(fmt),
                                __cf_osRcTFl4A::pmr::PConvWrapper("::")
                                    .to_pargument_display(fmt),
                                __cf_osRcTFl4A::pmr::PConvWrapper("Data")
                                    .to_pargument_display(fmt),
                                __cf_osRcTFl4A::pmr::PConvWrapper("::")
                                    .to_pargument_display(fmt),
                                __cf_osRcTFl4A::pmr::PConvWrapper("value1")
                                    .to_pargument_display(fmt),
                            ]
                        };
                        {
                            #[doc(hidden)]
                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                CONCATP_NHPMWYD3NJA,
                            );
                            #[doc(hidden)]
                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                [u8; ARR_LEN],
                            > = {
                                use ::const_format::{__write_pvariant, pmr::PVariant};
                                let mut out = ::const_format::pmr::LenAndArray {
                                    len: 0,
                                    array: [0u8; ARR_LEN],
                                };
                                let input = CONCATP_NHPMWYD3NJA;
                                {
                                    let ::const_format::pmr::Range {
                                        start: mut outer_i,
                                        end,
                                    } = 0..input.len();
                                    while outer_i < end {
                                        {
                                            let current = &input[outer_i];
                                            match current.elem {
                                                PVariant::Str(s) => {
                                                    let str = s.as_bytes();
                                                    let is_display = current.fmt.is_display();
                                                    let mut i = 0;
                                                    if is_display {
                                                        while i < str.len() {
                                                            out.array[out.len] = str[i];
                                                            out.len += 1;
                                                            i += 1;
                                                        }
                                                    } else {
                                                        out.array[out.len] = b'"';
                                                        out.len += 1;
                                                        while i < str.len() {
                                                            use ::const_format::pmr::{
                                                                hex_as_ascii, ForEscaping, FOR_ESCAPING,
                                                            };
                                                            let c = str[i];
                                                            let mut written_c = c;
                                                            if c < 128 {
                                                                let shifted = 1 << c;
                                                                if (FOR_ESCAPING.is_escaped & shifted) != 0 {
                                                                    out.array[out.len] = b'\\';
                                                                    out.len += 1;
                                                                    if (FOR_ESCAPING.is_backslash_escaped & shifted) == 0 {
                                                                        out.array[out.len] = b'x';
                                                                        out
                                                                            .array[out.len
                                                                            + 1] = hex_as_ascii(
                                                                            c >> 4,
                                                                            ::const_format::pmr::HexFormatting::Upper,
                                                                        );
                                                                        out.len += 2;
                                                                        written_c = hex_as_ascii(
                                                                            c & 0b1111,
                                                                            ::const_format::pmr::HexFormatting::Upper,
                                                                        );
                                                                    } else {
                                                                        written_c = ForEscaping::get_backslash_escape(c);
                                                                    };
                                                                }
                                                            }
                                                            out.array[out.len] = written_c;
                                                            out.len += 1;
                                                            i += 1;
                                                        }
                                                        out.array[out.len] = b'"';
                                                        out.len += 1;
                                                    }
                                                }
                                                PVariant::Int(int) => {
                                                    let wrapper = ::const_format::pmr::PWrapper(int);
                                                    let debug_display;
                                                    let bin;
                                                    let hex;
                                                    let sa: &::const_format::pmr::StartAndArray<[_]> = match current
                                                        .fmt
                                                    {
                                                        ::const_format::pmr::Formatting::Display => {
                                                            debug_display = wrapper.to_start_array_display();
                                                            &debug_display
                                                        }
                                                        ::const_format::pmr::Formatting::Debug => {
                                                            match current.fmt_flags.num_fmt() {
                                                                ::const_format::pmr::NumberFormatting::Decimal => {
                                                                    debug_display = wrapper.to_start_array_debug();
                                                                    &debug_display
                                                                }
                                                                ::const_format::pmr::NumberFormatting::Binary => {
                                                                    bin = wrapper.to_start_array_binary(current.fmt_flags);
                                                                    &bin
                                                                }
                                                                ::const_format::pmr::NumberFormatting::Hexadecimal => {
                                                                    hex = wrapper.to_start_array_hexadecimal(current.fmt_flags);
                                                                    &hex
                                                                }
                                                            }
                                                        }
                                                    };
                                                    let mut start = sa.start;
                                                    while start < sa.array.len() {
                                                        out.array[out.len] = sa.array[start];
                                                        out.len += 1;
                                                        start += 1;
                                                    }
                                                }
                                                PVariant::Char(c) => {
                                                    let encoded = c.encoded();
                                                    let len = c.len();
                                                    let mut start = 0;
                                                    while start < len {
                                                        out.array[out.len] = encoded[start];
                                                        out.len += 1;
                                                        start += 1;
                                                    }
                                                }
                                            }
                                        }
                                        outer_i += 1;
                                    }
                                }
                                &{ out }
                            };
                            #[doc(hidden)]
                            #[allow(clippy::transmute_ptr_to_ptr)]
                            const CONCAT_STR: &str = unsafe {
                                let slice = ::const_format::pmr::transmute::<
                                    &[u8; ARR_LEN],
                                    &[u8; CONCAT_ARR.len],
                                >(&CONCAT_ARR.array);
                                {
                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                    let string: &'static ::const_format::pmr::str = {
                                        ::const_format::__hidden_utils::PtrToRef {
                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                        }
                                            .reff
                                    };
                                    string
                                }
                            };
                            CONCAT_STR
                        }
                    })
                },
            }
                .x,
        )
    };
    #[automatically_derived]
    impl ::core::default::Default for Data {
        #[inline]
        fn default() -> Data {
            Data {
                value1: ::core::default::Default::default(),
                value2: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Data {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Data",
                "value1",
                &self.value1,
                "value2",
                &&self.value2,
            )
        }
    }
}
