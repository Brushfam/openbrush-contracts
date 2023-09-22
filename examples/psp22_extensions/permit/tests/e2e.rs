#![cfg(feature = "e2e-tests")]

extern crate my_psp22_permit;

use ink::{
    env::hash::{
        Blake2x256,
        HashOutput,
    },
    primitives::AccountId,

};
use ink_e2e::ContractsBackend;
use std::str::FromStr;
use std::task::Context;
use openbrush::contracts::{
    nonces::nonces_external::Nonces,
    psp22::{
        extensions::permit::psp22permit_external::PSP22Permit,
        psp22_external::PSP22,
    },
};
#[rustfmt::skip]
use crate::my_psp22_permit::*;
use crate::my_psp22_permit::Contract;
// use openbrush::contracts::psp22::extensions::permit::PERMIT_TYPE_HASH;
use openbrush::{
    contracts::psp22::extensions::permit::PermitMessage,
    traits::Balance,
    utils::crypto::{
        hash_blake2b256,
        Signature,
    },
};
use scale::Encode;
use test_helpers::{
    address_of,
    balance_of,
    method_call,
    method_call_dry_run,
};

use subxt_signer::{
    ecdsa::{
        Keypair as EcdsaKeypair,
        Keypair,
    },
    sr25519::Keypair as Sr25519Keypair,
    SecretUri,
};

type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn assigns_initial_balance<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let contract = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed");
    let call = contract.call::<Contract>();
    assert!(matches!(balance_of!(client, call, Alice), 1000));

    Ok(())
}

#[ink_e2e::test]
async fn nonce_should_be_equal_zero<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let contract = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed");
    let mut call = contract.call::<Contract>();

    let nonce = method_call_dry_run!(client, call, nonces(address_of!(Alice)));

    assert!(matches!(nonce, 0));

    Ok(())
}

#[ink_e2e::test]
async fn check_domain_separator<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let contract = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed");
    let mut  call = contract.call::<Contract>();

    let mut output = <Blake2x256 as HashOutput>::Type::default();
    ink::env::hash_bytes::<Blake2x256>(&contract.account_id.encode(), &mut output);

    let domain_separator: [u8; 32] = method_call_dry_run!(client, call, domain_separator());
    let real_domain_separator: [u8; 32] = output;
    assert_eq!(domain_separator, real_domain_separator);

    Ok(())
}

#[ink_e2e::test]
async fn permit_accepts_owner_ecdsa_signature<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let contract = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed");
    let mut call = contract.call::<Contract>();

    let nonce: u64 = method_call_dry_run!(client, call, nonces(address_of!(Alice)));
    let deadline: u64 = 30_000_000_000_000;
    let amount: Balance = 1000;

    let domain_separator: [u8; 32] = method_call_dry_run!(client, call, domain_separator());

    let signer = EcdsaKeypair::from_uri(&SecretUri::from_str("//Alice").unwrap()).unwrap();

    let owner = AccountId::from(signer.public_key().to_account_id().0);
    let spender = address_of!(Bob);

    let signature = get_ecdsa_signature(signer, domain_separator, spender, amount, deadline, nonce);

    let permit_signature = method_call!(client, call, permit(owner, spender, amount, deadline, signature));

    let allowance = method_call_dry_run!(client, call, allowance(owner, spender));
    println!("{}", &allowance);
    assert!(matches!(allowance, amount));

    Ok(())
}

#[ink_e2e::test]
async fn permit_rejects_reused_ecdsa_signature<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let contract = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed");
    let mut call = contract.call::<Contract>();

    let nonce: u64 = method_call_dry_run!(client, call, nonces(address_of!(Alice)));
    let deadline: u64 = 30_000_000_000_000;
    let amount: Balance = 1_000;

    let domain_separator: [u8; 32] = method_call_dry_run!(client, call, domain_separator());

    let signer = EcdsaKeypair::from_uri(&SecretUri::from_str("//Alice").unwrap()).unwrap();

    let owner = AccountId::from(signer.public_key().to_account_id().0);
    let spender = address_of!(Bob);

    let signature = get_ecdsa_signature(signer, domain_separator, spender, amount, deadline, nonce);

    let permit_signature = method_call!(client, call, permit(owner, spender, amount, deadline, signature.clone()));

    let allowance = method_call_dry_run!(client, call, allowance(owner, spender));
    println!("{}", &allowance);
    assert!(matches!(allowance, amount));

    let second_permit_result = method_call_dry_run!(
        client,
        call,
        permit(owner, spender, amount, deadline, signature)
    );
    assert!(matches!(second_permit_result, Err(_)));
    assert!(matches!(allowance, amount));

    Ok(())
}

#[ink_e2e::test]
async fn permit_rejects_other_ecdsa_signature<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let contract = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed");
    let mut call = contract.call::<Contract>();

    let nonce: u64 = method_call_dry_run!(client, call, nonces(address_of!(Alice)));
    let deadline: u64 = 30_000_000_000_000;
    let amount: Balance = 1000;

    let domain_separator: [u8; 32] = method_call_dry_run!(client, call, domain_separator());

    let signer = EcdsaKeypair::from_uri(&SecretUri::from_str("//Alice").unwrap()).unwrap();
    let fake_signer = EcdsaKeypair::from_uri(&SecretUri::from_str("//Charlie").unwrap()).unwrap();

    let owner = AccountId::from(signer.public_key().to_account_id().0);
    let spender = address_of!(Bob);

    let signature = get_ecdsa_signature(fake_signer, domain_separator, spender, amount, deadline, nonce);

    let permit_signature = method_call_dry_run!(client, call, permit(owner, spender, amount, deadline, signature));
    assert!(matches!(permit_signature, Err(_)));

    Ok(())
}

#[ink_e2e::test]
async fn permit_rejects_expired_ecdsa_signature<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let contract = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed");
    let mut call = contract.call::<Contract>();

    let nonce: u64 = method_call_dry_run!(client, call, nonces(address_of!(Alice)));
    let deadline: u64 = 1;
    let amount: Balance = 1000;

    let domain_separator: [u8; 32] = method_call_dry_run!(client, call, domain_separator());

    let signer = EcdsaKeypair::from_uri(&SecretUri::from_str("//Alice").unwrap()).unwrap();

    let owner = AccountId::from(signer.public_key().to_account_id().0);
    let spender = address_of!(Bob);

    let signature = get_ecdsa_signature(signer, domain_separator, spender, amount, deadline, nonce);

    let permit_signature = method_call_dry_run!(client, call, permit(owner, spender, amount, deadline, signature));
    assert!(matches!(permit_signature, Err(_)));
    Ok(())
}

#[ink_e2e::test]
async fn permit_accepts_owner_sr25519_signature<Client: E2EBackend>(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let contract = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed");
    let mut call = contract.call::<Contract>();

    let nonce: u64 = method_call_dry_run!(client, call, nonces(address_of!(Alice)));
    let deadline: u64 = 30_000_000_000_000;
    let amount: Balance = 1000;

    let domain_separator: [u8; 32] = method_call_dry_run!(client, call, domain_separator());

    let signer = ink_e2e::alice();
    let owner = address_of!(Alice);

    let spender = address_of!(Bob);

    let signature = get_sr25519_signature(signer, domain_separator, spender, amount, deadline, nonce);


    let permit_signature = method_call!(client, call, permit(owner, spender, amount, deadline, signature));

    let allowance = method_call_dry_run!(client, call, allowance(owner, spender));
    println!("{}", &allowance);
    assert!(matches!(allowance, amount));

    Ok(())
}


#[ink_e2e::test]
async fn permit_rejects_reused_sr25519_signature<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let contract = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed");
    let mut call = contract.call::<Contract>();

    let nonce: u64 = method_call_dry_run!(client, call, nonces(address_of!(Alice)));
    let deadline: u64 = 30_000_000_000_000;
    let amount: Balance = 1_000;

    let domain_separator: [u8; 32] = method_call_dry_run!(client, call, domain_separator());

    let signer = ink_e2e::alice();

    let owner = AccountId::from(signer.public_key().to_account_id().0);
    let spender = address_of!(Bob);

    let signature = get_sr25519_signature(signer, domain_separator, spender, amount, deadline, nonce);

    let permit_signature = method_call!(client, call, permit(owner, spender, amount, deadline, signature.clone()));

    let allowance = method_call_dry_run!(client, call, allowance(owner, spender));
    println!("{}", &allowance);
    assert!(matches!(allowance, amount));

    let second_permit_result = method_call_dry_run!(
        client,
        call,
        permit(owner, spender, amount, deadline, signature)
    );
    assert!(matches!(second_permit_result, Err(_)));
    assert!(matches!(allowance, amount));

    Ok(())
}

#[ink_e2e::test]
async fn permit_rejects_other_sr25519_signature<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let contract = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed");
    let mut call = contract.call::<Contract>();

    let nonce: u64 = method_call_dry_run!(client, call, nonces(address_of!(Alice)));
    let deadline: u64 = 30_000_000_000_000;
    let amount: Balance = 1000;

    let domain_separator: [u8; 32] = method_call_dry_run!(client, call, domain_separator());

    let signer = ink_e2e::alice();
    let fake_signer = ink_e2e::charlie();

    let owner = AccountId::from(signer.public_key().to_account_id().0);
    let spender = address_of!(Bob);

    let signature = get_sr25519_signature(fake_signer, domain_separator, spender, amount, deadline, nonce);

    let permit_signature = method_call_dry_run!(client, call, permit(owner, spender, amount, deadline, signature));
    assert!(matches!(permit_signature, Err(_)));

    Ok(())
}

#[ink_e2e::test]
async fn permit_rejects_expired_sr25519_signature<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    let constructor = ContractRef::new(1000);
    let contract = client
        .instantiate("my_psp22_permit", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed");
    let mut call = contract.call::<Contract>();

    let nonce: u64 = method_call_dry_run!(client, call, nonces(address_of!(Alice)));
    let deadline: u64 = 1;
    let amount: Balance = 1000;

    let domain_separator: [u8; 32] = method_call_dry_run!(client, call, domain_separator());

    let signer = ink_e2e::alice();

    let owner = AccountId::from(signer.public_key().to_account_id().0);
    let spender = address_of!(Bob);

    let signature = get_sr25519_signature(signer, domain_separator, spender, amount, deadline, nonce);

    let permit_signature = method_call_dry_run!(client, call, permit(owner, spender, amount, deadline, signature));
    assert!(matches!(permit_signature, Err(_)));
    Ok(())
}

fn get_ecdsa_signature(
    signer: EcdsaKeypair,
    domain_separator: [u8; 32],
    spender: AccountId,
    amount: Balance,
    deadline: u64,
    nonce: u64,
) -> Signature {
    let permit_message = PermitMessage {
        domain_separator,
        owner: AccountId::from(signer.public_key().to_account_id().0),
        spender,
        amount,
        deadline,
        nonce,
    };

    let message = &scale::Encode::encode(&permit_message);

    let signature = signer.sign(message).0;

    Signature::ECDSA(signature)
}

fn get_sr25519_signature(
    signer: Sr25519Keypair,
    domain_separator: [u8; 32],
    spender: AccountId,
    amount: Balance,
    deadline: u64,
    nonce: u64,
) -> Signature {
    let permit_message = PermitMessage {
        domain_separator,
        owner: AccountId::from(signer.public_key().to_account_id().0),
        spender,
        amount,
        deadline,
        nonce,
    };

    let message = &scale::Encode::encode(&permit_message);

    let signature = signer.sign(message).0;

    Signature::SR25519(signature)
}
