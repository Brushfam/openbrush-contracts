#![cfg(feature = "e2e-tests")]

extern crate my_psp22_permit;

use ink::env::hash::{
    Blake2x256,
    HashOutput,
};
#[rustfmt::skip]
use ink_e2e::build_message;
use openbrush::contracts::{
    nonces::nonces_external::Nonces,
    psp22::{
        extensions::permit::psp22permit_external::PSP22Permit,
        psp22_external::PSP22,
    },
};
#[rustfmt::skip]
use crate::my_psp22_permit::*;
// use openbrush::contracts::psp22::extensions::permit::PERMIT_TYPE_HASH;
use openbrush::{
    contracts::psp22::extensions::permit::PermitMessage,
    traits::{
        AccountId,
        Balance,
    },
    utils::crypto::{
        hash_blake2b256,
        Signature,
    },
};
use scale::Encode;
use secp256k1::{
    ecdsa::RecoverableSignature,
    Message,
    PublicKey,
    SecretKey,
    SECP256K1,
};
use test_helpers::{
    address_of,
    balance_of,
    method_call,
    method_call_dry_run,
};

type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn assigns_initial_balance<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
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
async fn nonce_should_be_equal_zero<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
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
async fn check_domain_separator<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
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
async fn permit_accepts_owner_signature<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
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
    let pubkey = PublicKey::from_secret_key(
        &SECP256K1,
        &SecretKey::from_slice(&seckey).expect("seckey creation failed"),
    );

    let owner = AccountId::from(hash_blake2b256(&pubkey.serialize().to_vec()));
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

#[ink_e2e::test]
async fn permit_rejects_reused_signature<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
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
    let pubkey = PublicKey::from_secret_key(
        &SECP256K1,
        &SecretKey::from_slice(&seckey).expect("seckey creation failed"),
    );

    let owner = AccountId::from(hash_blake2b256(&pubkey.serialize().to_vec()));
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

    let first_permit_result = method_call_dry_run!(
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
    let _call_permit = method_call!(
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

    assert!(matches!(first_permit_result, Ok(_)));

    let second_permit_result = method_call_dry_run!(
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

    println!("second_permit_result: {:?}", second_permit_result);

    assert!(matches!(second_permit_result, Err(_)));

    Ok(())
}

#[ink_e2e::test]
async fn permit_rejects_other_signature<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
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
    let _pubkey = PublicKey::from_secret_key(
        &SECP256K1,
        &SecretKey::from_slice(&seckey).expect("seckey creation failed"),
    );

    let owner = address_of!(Alice);
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

    let first_permit_result = method_call_dry_run!(
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

    assert!(matches!(first_permit_result, Err(_)));

    Ok(())
}

#[ink_e2e::test]
async fn permit_rejects_expired_permit<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let address = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    let nonce: u64 = method_call_dry_run!(client, address, nonces(address_of!(Alice)));
    let deadline: u64 = 1;
    let amount: Balance = 1000;

    let domain_separator: [u8; 32] = method_call_dry_run!(client, address, domain_separator());

    let seckey = [
        59, 148, 11, 85, 134, 130, 61, 253, 2, 174, 59, 70, 27, 180, 51, 107, 94, 203, 174, 253, 102, 39, 170, 146, 46,
        252, 4, 143, 236, 12, 136, 28,
    ];
    let pubkey = PublicKey::from_secret_key(
        &SECP256K1,
        &SecretKey::from_slice(&seckey).expect("seckey creation failed"),
    );

    let owner = AccountId::from(hash_blake2b256(&pubkey.serialize().to_vec()));
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

    assert!(matches!(permit_signature, Err(_)));

    Ok(())
}
