use openbrush::traits::Storage;

#[openbrush::accessors(AccessDataAccessors)]
#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct AccessData {
    #[get]
    #[set]
    read_write: u32,
    #[get]
    read_only: u32,
    #[set]
    write_only: u32,
}

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(AccessData);

fn main() {}