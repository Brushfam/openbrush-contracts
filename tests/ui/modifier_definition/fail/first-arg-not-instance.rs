#[openbrush::contract]
mod base_psp22 {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct Contract {
        pub value: bool,
        pub initialized: bool
    }


    impl Contract {
        #[openbrush::modifier_definition]
        fn once<BodyFn: FnOnce(&mut Contract)>(body: BodyFn) {
            assert!(!instance.initialized, "Contract is already initialized");
            body(instance);
            instance.initialized = true;
        }

        #[ink(constructor)]
        pub fn new(value: bool) -> Self {
            Self {
                value,
                initialized: false
            }
        }

        #[ink(message)]
        pub fn init(&mut self, value: bool) {
            self.value = value;
        }

        #[ink(message)]
        pub fn get_value(&self) -> bool {
            self.value
        }
    }
}

fn main() {}