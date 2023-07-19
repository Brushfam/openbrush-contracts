#[openbrush::trait_definition]
pub trait Trait1 {
    #[ink(message)]
    fn foo(&mut self) -> bool;
}

#[openbrush::wrapper]
type Trait1Ref = dyn Trait1;

#[openbrush::trait_definition]
pub trait Trait2 {
    #[ink(message)]
    fn bar(&mut self, callee: openbrush::traits::AccountId) -> bool{
        Trait1Ref::foo(&callee)
    }
}

#[openbrush::wrapper]
type Trait1and2Ref = dyn Trait1 + Trait2;

fn main() {}