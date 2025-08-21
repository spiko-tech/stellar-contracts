#![cfg(test)]

extern crate std;

use super::contract::{NonceManager, NonceManagerArgs, NonceManagerClient};
use soroban_sdk::{contract, testutils::Address as _, Address, Env};

#[contract]
struct MockContract;

fn setup_env() -> Env {
    let e: Env = Env::default();
    e.mock_all_auths();
    e
}

fn deploy_nonce_manager(e: &Env) -> (Address, NonceManagerClient) {
    let owner: Address = Address::generate(e);
    let contract_address = e.register(
        NonceManager,
        NonceManagerArgs::__constructor(&owner.clone()),
    );
    let client = NonceManagerClient::new(e, &contract_address);

    (owner, client)
}

#[test]
fn test_get_nonce_should_return_0_when_user_has_no_nonce() {
    let e = setup_env();
    let (_, client) = deploy_nonce_manager(&e);
    let user: Address = Address::generate(&e);

    let nonce = client.get_nonce(&user);

    assert_eq!(nonce, 0);
}

#[test]
fn test_consume_nonce_should_increment_nonce_if_nonce_is_correct() {
    let e = setup_env();
    let (_, client) = deploy_nonce_manager(&e);
    let user: Address = Address::generate(&e);
    let nonce = client.get_nonce(&user);

    client.consume_nonce(&user, &nonce);

    let next_nonce = client.get_nonce(&user);
    assert_eq!(next_nonce, 1);
}

#[test]
fn test_consume_nonce_should_fail_if_nonce_is_incorrect() {
    let e = setup_env();
    let (_, client) = deploy_nonce_manager(&e);
    let user: Address = Address::generate(&e);
    let nonce: u128 = client.get_nonce(&user);
    let incorrect_nonce: u128 = nonce + 1;

    let result = client.try_consume_nonce(&user, &incorrect_nonce);

    assert!(result.is_err());
}
