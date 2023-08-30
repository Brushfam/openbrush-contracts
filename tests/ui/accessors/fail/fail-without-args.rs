#[cfg_attr(feature = "non-upgradeable-lazy", openbrush::storage_item(lazy = false))]
#[cfg_attr(not(feature = "non-upgradeable-lazy"), openbrush::storage_item)]
#[openbrush::accessors]
#[derive(Default, Debug)]
pub struct AccessData {
    #[get]
    #[set]
    read_write: u32,
    #[get]
    read_only: u32,
    #[set]
    write_only: u32,
}

fn main() {}
