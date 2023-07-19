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

#![cfg_attr(not(feature = "std"), no_std, no_main)]

use proc_macro::TokenStream;

use openbrush_lang_codegen::{
    accessors,
    contract,
    implementation,
    modifier_definition,
    modifiers,
    storage_derive,
    storage_item,
    trait_definition,
    wrapper,
};

/// Entry point for use OpenBrush's macros in ink! smart contracts.
///
/// # Description
///
/// The macro consumes OpenBrush's macros to simplify the usage of the library.
/// After consumption, it pastes ink! code and then ink!'s macros will be processed.
///
/// This macro consumes impl section for traits defined with [`#[openbrush::trait_definition]`](`macro@crate::trait_definition`).
#[proc_macro_attribute]
pub fn contract(_attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    contract::generate(_attrs.into(), ink_module.into()).into()
}

/// Defines extensible trait in the scope of `openbrush::contract`.
/// It is a common rust trait, so you can use any features of rust inside of this trait.
/// If this trait contains some methods marked with `#[ink(message)]` or `#[ink(constructor)]` attributes,
/// this macro will extract these attributes and will put them into a separate trait
/// (the separate trait only is used to call methods from the original trait), but the macro will not touch methods.
///
/// This macro stores definition of the trait in a temporary file during build process.
/// Based on this definition [`#[openbrush::contract]`](`macro@crate::contract`)
/// will generate implementation of additional traits.
///
///  ** Note ** The name of the trait defined via this macro must be unique for the whole project.
///  ** Note ** You can't use aliases, generics, and other rust's stuff in signatures of ink!'s methods.
///
/// # Example: Definition
///
/// ```
/// mod doc {
/// use ink::storage::Mapping;
/// use openbrush::traits::{ AccountId, Balance, Storage };
///
/// #[derive(Debug)]
/// #[ink::storage_item]
/// pub struct Data {
///     pub balances: Mapping<AccountId, Balance>,
/// }
///
/// #[openbrush::trait_definition]
/// pub trait PSP22: Storage<Data> {
///     /// Returns the account Balance for the specified `owner`.
///     #[ink(message)]
///     fn balance_of(&self, owner: AccountId) -> Balance {
///         self.data().balances.get(&owner).unwrap_or(0)
///     }
///
///     /// Transfers `value` amount of tokens from the caller's account to account `to`.
///     #[ink(message)]
///     fn transfer(&mut self, to: AccountId, value: Balance) {
///         self._transfer_from_to(to, to, value)
///     }
///
///     fn _transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance) {
///         let from_balance = self.balance_of(from);
///         assert!(from_balance >= amount, "InsufficientBalance");
///         self.data().balances.insert(from, &(from_balance - amount));
///         let to_balance = self.balance_of(to);
///         self.data().balances.insert(to, &(to_balance + amount));
///     }
/// }
/// }
/// ```
///
/// # Example: Implementation
///
/// ```
/// #[openbrush::contract]
/// mod base_psp22 {
///     use ink::storage::traits::ManualKey;
///     use ink::storage::Mapping;
///     use ink::storage::Lazy;
///     use openbrush::traits::Storage;
///
///     const STORAGE_KEY_1: u32 = 101;
///     const STORAGE_KEY_2: u32 = 102;
///     const STORAGE_KEY_3: u32 = 103;
///
///     #[derive(Default, Debug)]
///     #[ink::storage_item]
///     pub struct Data {
///         pub supply: Lazy<Balance, ManualKey<STORAGE_KEY_1>>,
///         pub balances: Mapping<AccountId, Balance, ManualKey<STORAGE_KEY_2>>,
///         pub allowances: Mapping<(AccountId, AccountId), Balance, ManualKey<STORAGE_KEY_3>>,
///     }
///
///     #[openbrush::trait_definition]
///     pub trait PSP22Example: Storage<Data> {
///         /// Returns the account Balance for the specified `owner`.
///         #[ink(message)]
///         fn balance_of(&self, owner: AccountId) -> Balance {
///             self.data().balances.get(&owner).unwrap_or(0)
///         }
///
///         /// Transfers `value` amount of tokens from the caller's account to account `to`.
///         #[ink(message)]
///         fn transfer(&mut self, to: AccountId, value: Balance) {
///             let from = Self::env().caller();
///             self._transfer_from_to(from, to, value)
///         }
///
///         fn _transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance) {
///             let from_balance = self.balance_of(from);
///             assert!(from_balance >= amount, "InsufficientBalance");
///             self.data().balances.insert(from, &(from_balance - amount));
///             let to_balance = self.balance_of(to);
///             self.data().balances.insert(to, &(to_balance + amount));
///         }
///     }
///
///     #[ink(storage)]
///     #[derive(Storage, Default)]
///     pub struct PSP22Struct {
///         #[storage_field]
///         example: Data,
///         hated_account: Option<AccountId>,
///     }
///
///     impl PSP22Example for PSP22Struct {}
///
///     impl PSP22Struct {
///         #[ink(constructor)]
///         pub fn new(hated_account: AccountId) -> Self {
///             let mut instance = Self::default();
///             instance.hated_account = Some(hated_account);
///             instance
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn trait_definition(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    trait_definition::generate(_attrs.into(), _input.into()).into()
}

/// This macro only checks that some free-standing function satisfies a set of rules.
///
/// Rules:
/// - First argument should not be `self`.
/// - First argument must be a reference to a type `instance: &T`. In most cases it's the instance of contract.
/// - Second argument is function's body(this function contains the main code of method attached to the modifier).
/// The type must be `Fn(&T)`, `FnMut(&T)` or `FnOnce(&T)`.
/// - Every next argument should not be references to object.
/// Because modifier allows only to pass arguments by value(Modifier will pass the clone of argument).
/// - The return type of body function(second argument) must be the same as the return type of modifier.
///
/// # Example: Definition
///
/// ```
/// #[derive(Default)]
/// struct Contract {
///     initialized: bool,
/// }
///
/// #[openbrush::modifier_definition]
/// fn once<BodyFn: FnOnce(&mut Contract)>(instance: &mut Contract, body: BodyFn, _example_data1: u8, _example_data2: u8) {
///     assert!(!instance.initialized, "Contract is already initialized");
///     body(instance);
///     instance.initialized = true;
/// }
/// ```
#[proc_macro_attribute]
pub fn modifier_definition(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    modifier_definition::generate(_attrs.into(), _input.into()).into()
}

/// Macro calls every modifier function by passing self and the code of function's body.
/// It means that modifiers must be available in the scope of the marked method.
///
/// Modifiers are designed to be used for methods in impl sections.
/// The method can have several modifiers. They will be expanded from left to right.
/// The modifier can accept arguments from the scope of the method definition
/// (you can pass an argument from the signature of marked method or from the outside scope of function).
/// The modifier accepts arguments only by value and the type of argument must support `Clone` trait,
/// because macro will clone the argument and will pass it to the modifier.
///
/// # Explanation:
///
/// Let's define next modifiers.
/// ```
/// #[openbrush::modifier_definition]
/// fn A<T>(instance: &T, body: impl FnOnce(&T) -> &'static str) -> &'static str {
///     println!("A before");
///     let result = body(instance);
///     println!("A after");
///     result
/// }
///
/// #[openbrush::modifier_definition]
/// fn B<T, F: FnOnce(&T) -> &'static str>(instance: &T, body: F, data1: u8, data2: u8) -> &'static str {
///     println!("B before {} {}", data1, data2);
///     let result = body(instance);
///     println!("B after {} {}", data1, data2);
///     result
/// }
///
/// #[openbrush::modifier_definition]
/// fn C<T, F>(instance: &T, body: F) -> &'static str
///     where F: FnOnce(&T) -> &'static str
/// {
///     println!("C before");
///     let result = body(instance);
///     println!("C after");
///     result
/// }
///
/// struct Contract {}
///
/// impl Contract {
///     #[openbrush::modifiers(A, B(_data, 13), C)]
///     fn main_logic(&self, _data: u8) -> &'static str {
///         return "Return value";
///     }
/// }
/// ```
/// The code above will be expanded into:
/// ```
/// #[openbrush::modifier_definition]
/// fn A<T>(instance: &T, body: impl FnOnce(&T) -> &'static str) -> &'static str {
///     println!("A before");
///     let result = body(instance);
///     println!("A after");
///     result
/// }
///
/// #[openbrush::modifier_definition]
/// fn B<T, F: FnOnce(&T) -> &'static str>(instance: &T, body: F, data1: u8, data2: u8) -> &'static str {
///     println!("B before {} {}", data1, data2);
///     let result = body(instance);
///     println!("B after {} {}", data1, data2);
///     result
/// }
///
/// #[openbrush::modifier_definition]
/// fn C<T, F>(instance: &T, body: F) -> &'static str
///     where F: FnOnce(&T) -> &'static str
/// {
///     println!("C before");
///     let result = body(instance);
///     println!("C after");
///     result
/// }
///
/// struct Contract {}
///
/// impl Contract {
///     fn main_logic(&self, _data: u8) -> &'static str {
///         let mut __openbrush_body_2 = |__openbrush_instance_modifier: &Self| {
///             let __openbrush_cloned_0 = _data.clone();
///             let __openbrush_cloned_1 = 13.clone();
///             let mut __openbrush_body_1 = |__openbrush_instance_modifier: &Self| {
///                 let mut __openbrush_body_0 = |__openbrush_instance_modifier: &Self| return "Return value";;
///                 C(__openbrush_instance_modifier, __openbrush_body_0)
///             };
///             B(__openbrush_instance_modifier, __openbrush_body_1, __openbrush_cloned_0, __openbrush_cloned_1)
///         };
///         A(self, __openbrush_body_2)
///     }
/// }
/// ```
///
/// # Example: Usage
///
/// ```
/// #[openbrush::contract]
/// mod example {
///     #[ink(storage)]
///     pub struct Contract {
///         initialized: bool,
///         owner: AccountId,
///     }
///
///     impl Default for Contract {
///         fn default() -> Self {
///             Self {
///                 initialized: false,
///                 owner: [0u8; 32].into(),
///             }
///         }
///     }
///
///     #[openbrush::modifier_definition]
///     fn once(instance: &mut Contract, body: impl FnOnce(&mut Contract)) {
///         assert!(!instance.initialized, "Contract is already initialized");
///         body(instance);
///         instance.initialized = true;
///     }
///
///     impl Contract {
///         #[ink(constructor)]
///         pub fn new() -> Self {
///             Self::default()
///         }
///
///         #[ink(message)]
///         #[openbrush::modifiers(once)]
///         pub fn init(&mut self, owner: AccountId) {
///             self.owner = owner;
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn modifiers(_attrs: TokenStream, method: TokenStream) -> TokenStream {
    modifiers::generate(_attrs.into(), method.into()).into()
}

/// This macro allows you to define a wrapper type for traits defined via
/// [`#[openbrush::trait_definition]`](`macro@crate::trait_definition`).
/// It is a wrapper for `AccountId` that knows how to do cross-contract calls to another contract.
///
/// To define a wrapper you need to use the follow construction:
/// `type TraitName = dyn Trait_1 + Trait_2 ... + Trait_n`, where `Trait_i` contains ink! messages
/// and defined via [`#[openbrush::trait_definition]`](`macro@crate::trait_definition`).
/// If `Trait_i` doesn't contain ink! messages, then you don't need to create a wrapper for that trait,
/// because the wrapped methods are created only for ink! messages. Otherwise, you will get an error like
///
/// `use of undeclared crate or module `trait_i_external``
///
///  ** Note ** The first argument of method should be a reference on `AccountId` of callee
/// contract(even if the signature of the method requires a mutable reference).
///  ** Note ** Crated wrapper is only a type, so you can't create an instance of this object.
///  ** Note ** The wrapper contains only ink's methods of the trait, it doesn't include a method of super traits.
/// If you want to wrap them too, you need to explicitly specify them.
///
/// # Example: Definition
///
/// ```should_panic
/// {
/// use openbrush::traits::AccountId;
///
/// #[openbrush::trait_definition]
/// pub trait Trait1 {
///     #[ink(message)]
///     fn foo(&mut self) -> bool;
/// }
///
/// #[openbrush::wrapper]
/// type Trait1Ref = dyn Trait1;
///
/// #[openbrush::trait_definition]
/// pub trait Trait2 {
///     #[ink(message)]
///     fn bar(&mut self, callee: openbrush::traits::AccountId) {
///         let foo_bool = Trait1Ref::foo(&callee);
///     }
/// }
///
/// #[openbrush::wrapper]
/// type Trait1and2Ref = dyn Trait1 + Trait2;
///
/// // Example of explicit call
/// let to: AccountId = [0; 32].into();
/// let callee: AccountId = [0; 32].into();
/// Trait1and2Ref::bar(&to, callee);
///
/// // Example of implicit call
/// let to: &Trait1and2Ref = &to;
/// to.bar(callee);
///
/// // Example how to get ink! call builder
/// let to: AccountId = [0; 32].into();
/// let builder_for_foo: ::ink::env::call::CallBuilder<_, _, _, _> = Trait1and2Ref::foo_builder(&to);
/// let ink_result: Result<bool, ink::LangError> = builder_for_foo.try_invoke().unwrap();
/// }
/// ```
#[proc_macro_attribute]
pub fn wrapper(attrs: TokenStream, input: TokenStream) -> TokenStream {
    wrapper::generate(attrs.into(), input.into()).into()
}

/// The macro implements `openbrush::traits::Storage`
/// trait for each field marked by `#[storage_field]` attribute,
/// so it will be possible to access them via `self.data::<Type>()` method. It is mostly used for OpenBrush
/// to understand which fields should be accessed by traits.
///
/// # Example
/// ```skip
///     #[ink(storage)]
///     #[derive(Storage)]
///     pub struct Contract {
///         #[storage_field]
///         field: u32,
///     }
/// ```
#[proc_macro_derive(Storage, attributes(storage_field))]
pub fn storage_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    storage_derive::storage_derive(item.into()).into()
}

synstructure::decl_attribute!(
    [accessors] =>
    /// Macro that automatically implements accessors like get/set for struct fields, that implements `scale::Encode`
    /// and `scale::Decode` traits. You should specify the getters trait naming in the macro's attribute.
    /// Also, fields that you want getters to be generated, should be marked by `#[get]` attribute.
    /// Fields, that you want setters to be generated, should be marked by `#[set]` attribute.
    /// The name of the accessor message will be concatenation of `get/set` + `_` + field's name.
    ///
    /// # Example:
    /// ```skip
    ///
    /// use openbrush::traits::Storage;
    ///
    /// #[openbrush::accessors(SomeStructGetters)]
    /// #[derive(Default)]
    /// #[ink::storage_item]
    /// pub struct SomeStruct {
    ///     #[get]
    ///     a: u32,
    ///     b: u32,
    ///     #[set]
    ///     c: u32,
    /// }
    ///
    /// #[openbrush::contract]
    /// pub mod contract {
    ///     use crate::*;
    ///     use openbrush::traits::Storage;
    ///
    ///     #[ink(storage)]
    ///     #[derive(Storage, Default)]
    ///     pub struct Contract {
    ///         #[storage_field]
    ///         some_struct: SomeStruct,
    ///     }
    ///
    ///     impl SomeStructGetters for Contract {}
    ///
    ///     impl Contract {
    ///         #[ink(constructor)]
    ///         pub fn new() -> Self {
    ///             Self::default()
    ///         }
    ///     }
    /// }
    /// ```
    accessors::accessors
);

/// This macro implements the default traits defined in OpenBrush, while also allowing users
/// to override them with `#[overrider]` or `#[default_impl]` attributes. `#[overrider]` is used when
/// you want to change the behavior of the method by your implementation. `#[default_impl]` is used when
/// you want to keep the default implementation from OpenBrush, but you want to attach some modifiers to
/// that function.
///
/// # Example
///
/// ```skip
/// #[openbrush::implementation(PSP22)]
/// #[openbrush::contract]
/// pub mod MyInkToken {
///     use openbrush::traits::Storage;
///     
///     #[ink(storage)]
///     #[derive(Storage)]
///     pub struct MyInkToken {
///         #[storage_field]
///         psp22: psp22::Data
///     }
///
///     // this will override a function from psp22::Internal
///     #[overrider(psp22::Internal)]
///     fn _before_token_transfer(
///         &mut self,
///         from: Option<&AccountId>,
///         to: Option<&AccountId>,
///         amount: &Balance,
///     ) -> Result<(), PSP22Error> {
///         // here we can change the behavior before token transfer
///     }
///
///     // this will override a function from PSP22
///     #[overrider(PSP22)]
///     fn balance_of(&self, owner: AccountId) -> Balance {
///          // here we can change the behavior of balance_of
///     }
///
///     // this will keep the default implementation of this method,
///     // however, it will add the modifier (and possibly other attributes defined by user)
///     // to the function. In this case, we don't even have to worry about the attributes and
///     // return type of the function
///     #[default_impl(PSP22)]
///     #[modifiers(...)]
///     fn transfer_from() {}
///
///     impl Contract {
///         // we can add constructor and other messages
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn implementation(attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    implementation::generate(attrs.into(), ink_module.into()).into()
}

synstructure::decl_attribute!(
    [storage_item] =>
    /// The macro implements `ink::storage_item` macro for the struct, which means that it prepares your struct
    /// to be a part of contract's storage. Also, inside of struct marked by this macro you can use
    /// `#[lazy]` attribute to mark fields, that should be lazily loaded and wrapped in `::ink::storage::Lazy`.
    /// The macro also generates constant storage keys for every mapping or lazy field and inserts them into
    /// type definition.
    ///
    /// # Example
    /// ```skip
    /// #[openbrush::storage_item]
    /// pub struct MyStruct {
    ///    a: u32,
    ///    b: u32,
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```skip
    /// #[openbrush::storage_item]
    /// pub struct MyStruct {
    ///     #[lazy]
    ///     a: u32,
    ///     #[lazy]
    ///     b: u32,
    /// }
    ///
    storage_item::storage_item
);
