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
