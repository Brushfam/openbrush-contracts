#![cfg(feature = "e2e-tests")]

extern crate my_psp22_permit;

use ink::env::hash::{Blake2x256, HashOutput};
#[rustfmt::skip]
use ink_e2e::build_message;
use openbrush::contracts::psp22::extensions::permit::psp22permit_external::PSP22Permit;
use openbrush::contracts::psp22::psp22_external::PSP22;
#[rustfmt::skip]
use crate::my_psp22_permit::*;
// use openbrush::contracts::psp22::extensions::permit::PERMIT_TYPE_HASH;
use openbrush::contracts::psp22::extensions::permit::PermitMessage;
use openbrush::traits::{AccountId, Balance, Signature};
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
    use ink_e2e::Keypair;
    use secp256k1::{ecdsa::RecoverableSignature, Message, PublicKey, SecretKey, SECP256K1};

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

    let seckey = [
        59, 148, 11, 85, 134, 130, 61, 253, 2, 174, 59, 70, 27, 180, 51, 107, 94, 203, 174, 253, 102, 39, 170, 146, 46,
        252, 4, 143, 236, 12, 136, 28,
    ];
    let pubkey = PublicKey::from_slice(&[
        2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91, 141, 134, 245, 114, 45,
        63, 82, 19, 251, 210, 57, 79, 54,
    ])
    .expect("pubkey creation failed");

    let owner = AccountId::from([
        2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91, 141, 134, 245, 114, 45,
        63, 82, 19, 251, 210, 57, 79,
    ]);
    let spender = address_of!(Bob);

    let permit_message = PermitMessage {
        domain_separator,
        owner,
        spender,
        amount,
        deadline,
        nonce,
    };

    let message = &scale::Encode::encode(&permit_message);

    let msg_hash = hash_blake2b256(message);

    let msg = Message::from_slice(&msg_hash).expect("message creation failed");
    let seckey = SecretKey::from_slice(&seckey).expect("secret key creation failed");
    let recoverable_signature: RecoverableSignature = SECP256K1.sign_ecdsa_recoverable(&msg, &seckey);

    let recovery_id = recoverable_signature.serialize_compact().0.to_i32() as u8;
    let mut signature = recoverable_signature.serialize_compact().1.to_vec();
    signature.push(recovery_id);
    let signature_with_recovery_id: [u8; 65] = signature
        .try_into()
        .expect("unable to create signature with recovery id");

    let permit_signature = method_call_dry_run!(
        client,
        address,
        permit(
            owner,
            spender,
            amount,
            deadline,
            Signature::ECDSA(signature_with_recovery_id)
        )
    );

    println!("permit_signature: {:?}", permit_signature);
    assert!(matches!(permit_signature, Ok(_)));

    Ok(())
}
