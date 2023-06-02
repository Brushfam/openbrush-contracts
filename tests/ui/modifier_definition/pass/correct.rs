#[openbrush::contract]
mod base_psp22 {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct Contract {
        pub value: bool,
        pub initialized: bool
    }

    #[openbrush::modifier_definition]
    fn once<BodyFn: FnOnce(&mut Contract)>(instance: &mut Contract, body: BodyFn) {
        assert!(!instance.initialized, "Contract is already initialized");
        body(instance);
        instance.initialized = true;
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(value: bool) -> Self {
            Self {
                value,
                initialized: false
            }
        }

        #[ink(message)]
        #[openbrush::modifiers(once)]
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