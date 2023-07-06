pub struct Env;

impl ink_env::Environment for Env {
    const MAX_EVENT_TOPICS: usize = 3;
    type AccountId = [u8; 32];
    type Balance = u64;
    type Hash = [u8; 32];
    type Timestamp = u64;
    type BlockNumber = u64;
    type ChainExtension = ();
}

#[openbrush::contract(env = super::Env)]
mod base_psp22 {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct PSP22Struct {
        pub value: bool
    }

    impl PSP22Struct {
        #[ink(constructor)]
        pub fn new(value: bool) -> Self {
            Self {
                value
            }
        }

        #[ink(message)]
        pub fn set_value(&mut self, value: bool) {
            self.value = value;
        }

        #[ink(message)]
        pub fn get_value(&self) -> bool {
            self.value
        }
    }
}

fn main() {}