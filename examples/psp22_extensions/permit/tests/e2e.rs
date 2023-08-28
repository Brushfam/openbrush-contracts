#![cfg(feature = "e2e-tests")]

extern crate my_psp22_permit;

use openbrush::contracts::psp22::extensions::permit::psp22permit_external::PSP22Permit;
use openbrush::contracts::psp22::psp22_external::PSP22;
#[rustfmt::skip]
use crate::my_psp22_permit::*;
#[rustfmt::skip]
use ink_e2e::{build_message};
use ink::env::hash::{Blake2x256, HashOutput};
// use openbrush::contracts::psp22::extensions::permit::PERMIT_TYPE_HASH;
use openbrush::contracts::psp22::extensions::permit::PermitMessage;
use openbrush::traits::Balance;
use openbrush::utils::hash_blake2b256;
use scale::Encode;

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

    assert!(matches!(balance_of!(client, address, Alice), 1000));

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

    let nonce = method_call_dry_run!(client, address, nonces(address_of!(Alice)));

    assert!(matches!(nonce, 0));

    Ok(())
}

#[ink_e2e::test]
async fn check_domain_separator(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let address = &client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    let mut output = <Blake2x256 as HashOutput>::Type::default();
    ink::env::hash_bytes::<Blake2x256>(&address.encode(), &mut output);
    let domain_separator: [u8; 32] = method_call_dry_run!(client, address, domain_separator());
    let real_domain_separator: [u8; 32] = output;
    assert_eq!(domain_separator, real_domain_separator);

    Ok(())
}

#[ink_e2e::test]
async fn check_permit_signature(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    use secp256k1::{
        ecdsa::{RecoverableSignature, RecoveryId},
        Message, SECP256K1,
    };

    let constructor = ContractRef::new(1000);
    let address = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    let nonce: u64 = method_call_dry_run!(client, address, nonces(address_of!(Alice)));
    let deadline: u64 = 30_000_000_000_000;
    let amount: Balance = 1000;

    let domain_separator: [u8; 32] = method_call_dry_run!(client, address, domain_separator());

    let permit_message = PermitMessage {
        domain_separator: domain_separator.clone(),
        owner: address_of!(Alice),
        spender: address_of!(Bob),
        amount,
        deadline,
        nonce,
    };

    let message = &scale::Encode::encode(&permit_message);

    let message_hash = hash_blake2b256(message);

    let signature = SECP256K1::sign_recoverable(
        &SECP256K1,
        &Message::parse_slice(&message_hash).unwrap(),
        &ink_e2e::alice().secret_key,
    );

    let permit_signature = method_call_dry_run!(
        client,
        address,
        permit(address_of!(Alice), address_of!(Bob), amount, deadline, signature)
    );

    println!("permit_signature: {:?}", permit_signature);
    assert!(matches!(permit_signature, Ok(_)));

    Ok(())
}
