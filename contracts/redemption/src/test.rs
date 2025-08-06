#![cfg(test)]

extern crate std;

use super::contract::{Redemption, RedemptionArgs, RedemptionClient};
use soroban_sdk::{contract, testutils::Address as _, Address, Env};

#[contract]
struct MockContract;

#[test]
fn test_owner_setup() {
    let e = Env::default();
    e.mock_all_auths();

    let owner: Address = Address::generate(&e);
    let contract_address = e.register(Redemption, RedemptionArgs::__constructor(&owner.clone()));
    let client = RedemptionClient::new(&e, &contract_address);

    let fetched_owner = client.get_owner();

    assert_eq!(fetched_owner, Some(owner));
}

#[test]
fn test_require_owner_auth_for_set_permission_manager() {
    let e = Env::default();
    e.mock_all_auths();

    let owner: Address = Address::generate(&e);
    let contract_address = e.register(Redemption, RedemptionArgs::__constructor(&owner.clone()));
    let client = RedemptionClient::new(&e, &contract_address);
    let permission_manager: Address = Address::generate(&e);

    client.set_permission_manager(&permission_manager);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &owner);
}

#[test]
fn test_require_owner_auth_for_add_token() {
    let e = Env::default();
    e.mock_all_auths();

    let owner: Address = Address::generate(&e);
    let contract_address = e.register(Redemption, RedemptionArgs::__constructor(&owner.clone()));
    let client = RedemptionClient::new(&e, &contract_address);
    let token: Address = Address::generate(&e);

    client.add_token(&token);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &owner);
}

#[test]
fn test_require_owner_auth_for_remove_token() {
    let e = Env::default();
    e.mock_all_auths();

    let owner: Address = Address::generate(&e);
    let contract_address = e.register(Redemption, RedemptionArgs::__constructor(&owner.clone()));
    let client = RedemptionClient::new(&e, &contract_address);
    let token: Address = Address::generate(&e);

    client.remove_token(&token);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &owner);
}

#[test]
fn test_on_redeem_should_fail_if_token_is_not_set() {
    let e = Env::default();
    e.mock_all_auths();

    let owner: Address = Address::generate(&e);
    let contract_address = e.register(Redemption, RedemptionArgs::__constructor(&owner.clone()));
    let client = RedemptionClient::new(&e, &contract_address);
    let non_token_contract: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);

    let result = client.try_on_redeem(&non_token_contract, &user, &100u128, &100u128);

    assert!(result.is_err());
}

#[test]
fn test_on_redeem_should_pass_if_token_is_set() {
    let e = Env::default();
    e.mock_all_auths();

    let owner: Address = Address::generate(&e);
    let contract_address = e.register(Redemption, RedemptionArgs::__constructor(&owner.clone()));
    let client = RedemptionClient::new(&e, &contract_address);
    let token: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    client.add_token(&token);

    let result = client.try_on_redeem(&token, &user, &100u128, &100u128);

    assert!(result.is_ok());
}

#[test]
fn test_on_redeem_should_require_token_auth() {
    let e = Env::default();
    e.mock_all_auths();

    let owner: Address = Address::generate(&e);
    let contract_address = e.register(Redemption, RedemptionArgs::__constructor(&owner.clone()));
    let client = RedemptionClient::new(&e, &contract_address);
    let token: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    client.add_token(&token);

    client.on_redeem(&token, &user, &100u128, &100u128);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &token);
}
