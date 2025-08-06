#![cfg(test)]

extern crate std;

use super::contract::{Redemption, RedemptionArgs, RedemptionClient, RedemptionEntry};
use soroban_sdk::{
    contract, symbol_short,
    testutils::{Address as _, Events},
    vec,
    xdr::ToXdr,
    Address, Env, Vec,
};

#[contract]
struct MockContract;

#[test]
fn test_should_set_owner_on_constructor() {
    let e = Env::default();
    e.mock_all_auths();

    let owner: Address = Address::generate(&e);
    let contract_address = e.register(Redemption, RedemptionArgs::__constructor(&owner.clone()));
    let client = RedemptionClient::new(&e, &contract_address);

    let fetched_owner = client.get_owner();

    assert_eq!(fetched_owner, Some(owner));
}

#[test]
fn test_set_permission_manager_should_require_owner_auth() {
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
fn test_add_token_should_require_owner_auth() {
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
fn test_remove_token_should_require_owner_auth() {
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

#[test]
fn test_on_redeem_should_fail_if_redemption_already_exists() {
    let e = Env::default();
    e.mock_all_auths();

    let owner: Address = Address::generate(&e);
    let contract_address = e.register(Redemption, RedemptionArgs::__constructor(&owner.clone()));
    let client = RedemptionClient::new(&e, &contract_address);
    let token: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let salt: u128 = 100;
    client.add_token(&token);

    client.on_redeem(&token, &user, &100u128, &salt);
    let result = client.try_on_redeem(&token, &user, &20u128, &salt);

    assert!(result.is_err());
}

#[test]
fn test_on_redeem_should_emit_a_redemption_initiated_event() {
    let e = Env::default();
    e.mock_all_auths();

    let owner: Address = Address::generate(&e);
    let contract_address = e.register(Redemption, RedemptionArgs::__constructor(&owner.clone()));
    let client = RedemptionClient::new(&e, &contract_address);
    let token: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let salt: u128 = 100;
    client.add_token(&token);

    client.on_redeem(&token, &user, &100u128, &salt);

    let events = e.events().all();
    assert_eq!(Vec::len(&events), 1);
    let event = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(event.0, contract_address);
    assert_eq!(Vec::len(&event.1), 2);
    let first_event_topic = Vec::get(&event.1, 0).expect("First event topic should be present");
    let second_event_topic = Vec::get(&event.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_event_topic.to_xdr(&e),
        symbol_short!("redeem").to_xdr(&e)
    );
    assert_eq!(
        second_event_topic.to_xdr(&e),
        symbol_short!("init").to_xdr(&e)
    );
    assert_eq!(
        event.2.to_xdr(&e),
        RedemptionEntry(token, user, 100, salt).to_xdr(&e)
    );
}
