#[openbrush::trait_definition]
pub trait Trait1 {
    #[ink(message)]
    fn foo(&mut self) -> bool;
}

#[openbrush::wrapper]
type Trait1Ref = dyn Trait1;

fn main() {}