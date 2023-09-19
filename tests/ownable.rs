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

#[cfg(feature = "ownable")]
#[openbrush::implementation(Ownable)]
#[openbrush::contract]
mod ownable {
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::{
        test_utils::change_caller,
        traits::Storage,
    };

    #[ink(event)]
    pub struct OwnershipTransferred {
        #[ink(topic)]
        previous: Option<AccountId>,
        #[ink(topic)]
        new: Option<AccountId>,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyOwnable {
        #[storage_field]
        ownable: Data,
    }

    type Event = <MyOwnable as ::ink::reflect::ContractEventBase>::Type;

    impl MyOwnable {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut inst = Self::default();
            ownable::Internal::_init_with_owner(&mut inst, Self::env().caller());
            inst
        }

        #[ink(message)]
        pub fn temp(&self) {}
    }

    #[overrider(ownable::Internal)]
    fn _emit_ownership_transferred_event(&self, previous: Option<AccountId>, new: Option<AccountId>) {
        self.env().emit_event(OwnershipTransferred { previous, new })
    }

    fn assert_ownership_transferred_event(
        event: &ink::env::test::EmittedEvent,
        expected_previous_owner: Option<AccountId>,
        expected_new_owner: Option<AccountId>,
    ) {
        let Event::OwnershipTransferred(OwnershipTransferred { previous, new }) =
            <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");

        assert_eq!(
            previous, expected_previous_owner,
            "Previous owner was not equal to expected previous owner."
        );
        assert_eq!(
            new, expected_new_owner,
            "New owner was not equal to expected new owner."
        );
    }

    #[ink::test]
    fn constructor_works() {
        let instance = MyOwnable::new();

        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(1, emitted_events.len());

        assert_ownership_transferred_event(&emitted_events[0], None, Ownable::owner(&instance))
    }

    #[ink::test]
    fn owner_works() {
        let my_ownable = MyOwnable::new();
        let caller = my_ownable.env().caller();
        assert_eq!(Ownable::owner(&my_ownable), Some(caller))
    }

    #[ink::test]
    fn renounce_ownership_works() {
        let mut my_ownable = MyOwnable::new();
        let caller = my_ownable.env().caller();
        let creator = Ownable::owner(&mut my_ownable);
        assert_eq!(creator, Some(caller));
        assert!(Ownable::renounce_ownership(&mut my_ownable,).is_ok());
        assert_eq!(Ownable::owner(&mut my_ownable), None);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(2, emitted_events.len());
        assert_ownership_transferred_event(&emitted_events[0], None, creator);
        assert_ownership_transferred_event(&emitted_events[1], creator, None);
    }

    #[ink::test]
    fn renounce_ownership_fails() {
        let mut my_ownable = MyOwnable::new();
        // Change the caller of `renounce_ownership` method.
        change_caller(AccountId::from([0x13; 32]));
        let result = Ownable::renounce_ownership(&mut my_ownable);
        assert!(result.is_err());
        assert_eq!(result, Err(OwnableError::CallerIsNotOwner));
    }

    #[ink::test]
    fn transfer_ownership_works() {
        let mut my_ownable = MyOwnable::new();
        let caller = my_ownable.env().caller();
        let creator = Ownable::owner(&mut my_ownable);
        assert_eq!(creator, Some(caller));
        let new_owner = AccountId::from([5u8; 32]);
        assert!(Ownable::transfer_ownership(&mut my_ownable, Some(new_owner)).is_ok());
        assert_eq!(Ownable::owner(&mut my_ownable), Some(new_owner));
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(2, emitted_events.len());
        assert_ownership_transferred_event(&emitted_events[0], None, creator);
        assert_ownership_transferred_event(&emitted_events[1], creator, Some(new_owner));
    }

    #[ink::test]
    fn transfer_ownership_fails() {
        let mut my_ownable = MyOwnable::new();
        // Change the caller of `transfer_ownership` method.
        change_caller(AccountId::from([0x13; 32]));
        let new_owner = AccountId::from([5u8; 32]);
        assert_eq!(
            Ownable::transfer_ownership(&mut my_ownable, Some(new_owner)),
            Err(OwnableError::CallerIsNotOwner)
        );
    }
}
