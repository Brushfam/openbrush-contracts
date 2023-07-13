#![cfg(feature = "e2e-tests")]

extern crate my_psp22_permit;

use openbrush::contracts::psp22::extensions::permit::psp22permit_external::PSP22Permit;
use openbrush::contracts::psp22::psp22_external::PSP22;
#[rustfmt::skip]
use crate::my_psp22_permit::*;
#[rustfmt::skip]
use ink_e2e::{build_message, PolkadotConfig};
use blake2::digest::Update;
use blake2::Blake2s;
use blake2::Digest;
use ink_e2e::subxt::ext::sp_core;
use ink_e2e::subxt::ext::sp_core::Pair;
use openbrush::contracts::psp22::extensions::permit::PERMIT_TYPE_HASH;
use openbrush::traits::Balance;

use test_helpers::{address_of, balance_of, method_call_dry_run};

type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn assigns_initial_balance(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let address = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    assert!(matches!(balance_of!(client, address, alice), 1000));

    Ok(())
}

#[ink_e2e::test]
async fn nonce_should_be_equal_zero(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let address = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    let nonce = method_call_dry_run!(client, address, nonces(address_of!(alice)));

    assert!(matches!(nonce, 0));

    Ok(())
}

#[ink_e2e::test]
async fn check_domain_separator(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let address = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    let domain_separator: [u8; 32] = method_call_dry_run!(client, address, domain_separator());
    let real_domain_separator: [u8; 32] = Blake2s::new().chain(address).finalize().into();

    assert_eq!(domain_separator, real_domain_separator);

    Ok(())
}

#[ink_e2e::test]
async fn check_permit_signature(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let address = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    let nonce: u64 = method_call_dry_run!(client, address, nonces(address_of!(alice)));
    let deadline: u64 = 30_000_000_000_000;
    let amount: Balance = 1000;

    let domain_separator: [u8; 32] = method_call_dry_run!(client, address, domain_separator);

    let permit_hash: [u8; 32] = Blake2s::new()
        .chain(PERMIT_TYPE_HASH)
        .chain(domain_separator)
        .chain(address_of!(alice))
        .chain(address_of!(bob))
        .chain(amount.to_le_bytes())
        .chain(nonce.to_le_bytes())
        .chain(deadline.to_le_bytes())
        .finalize()
        .into();

    let signature: [u8; 64] = sp_core::sr25519::Pair::from_string("//Alice", None)
        .expect("Should generate pair")
        .sign(&permit_hash)
        .0;

    let permit_signature = method_call_dry_run!(
        client,
        address,
        permit(address_of!(alice), address_of!(bob), amount, deadline, signature)
    );

    println!("permit_signature: {:?}", permit_signature);
    assert!(matches!(permit_signature, Ok(_)));

    Ok(())
}
