#![cfg(test)]

extern crate std;

use super::contract::{Redemption, RedemptionArgs, RedemptionClient, RedemptionEntry};
use contracts_utils::role::{REDEMPTION_EXECUTOR_ROLE, WHITELISTED_ROLE};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events},
    xdr::ToXdr,
    Address, Env, Vec,
};

mod permission_manager {
    use soroban_sdk::contractimport;

    contractimport!(file = "./permission_manager.wasm");
}

fn setup_env() -> Env {
    let e: Env = Env::default();
    e.mock_all_auths();
    e
}

fn deploy_redemption(e: &Env) -> (Address, Address, RedemptionClient) {
    let owner: Address = Address::generate(&e);
    let redemption_address = e.register(Redemption, RedemptionArgs::__constructor(&owner.clone()));
    let client = RedemptionClient::new(&e, &redemption_address);

    (owner, redemption_address, client)
}

fn deploy_permission_manager(e: &Env) -> (Address, Address, permission_manager::Client<'_>) {
    let admin: Address = Address::generate(&e);
    let permission_manager_address = e.register(
        permission_manager::WASM,
        permission_manager::Args::__constructor(&admin.clone()),
    );
    let permission_manager_client =
        permission_manager::Client::new(&e, &permission_manager_address);
    permission_manager_client.initialize();

    (admin, permission_manager_address, permission_manager_client)
}

#[test]
fn test_should_set_owner_on_constructor() {
    let e = setup_env();
    let (owner, _, client) = deploy_redemption(&e);

    let fetched_owner = client.get_owner();

    assert_eq!(fetched_owner, Some(owner));
}

#[test]
fn test_set_permission_manager_should_require_owner_auth() {
    let e = setup_env();
    let (owner, _, client) = deploy_redemption(&e);
    let (_, permission_manager_address, _) = deploy_permission_manager(&e);

    client.set_permission_manager(&permission_manager_address);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &owner);
}

#[test]
fn test_add_token_should_require_owner_auth() {
    let e = setup_env();
    let (owner, _, client) = deploy_redemption(&e);
    let token: Address = Address::generate(&e);

    client.add_token(&token);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &owner);
}

#[test]
fn test_remove_token_should_require_owner_auth() {
    let e = setup_env();
    let (owner, _, client) = deploy_redemption(&e);
    let token: Address = Address::generate(&e);

    client.remove_token(&token);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &owner);
}

#[test]
fn test_on_redeem_should_fail_if_token_is_not_set() {
    let e = setup_env();
    let (_, _, client) = deploy_redemption(&e);
    let non_token_contract: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);

    let result = client.try_on_redeem(&non_token_contract, &user, &100u128, &100u128);

    assert!(result.is_err());
}

#[test]
fn test_on_redeem_should_pass_if_token_is_set() {
    let e = setup_env();
    let (_, _, client) = deploy_redemption(&e);
    let token: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    client.add_token(&token);

    let result = client.try_on_redeem(&token, &user, &100u128, &100u128);

    assert!(result.is_ok());
}

#[test]
fn test_on_redeem_should_require_token_auth() {
    let e = setup_env();
    let (_, _, client) = deploy_redemption(&e);
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
    let e = setup_env();
    let (_, _, client) = deploy_redemption(&e);
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
    let e = setup_env();
    let (_, redemption_address, client) = deploy_redemption(&e);
    let token: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let salt: u128 = 100;
    client.add_token(&token);

    client.on_redeem(&token, &user, &100u128, &salt);

    let events = e.events().all();
    assert_eq!(Vec::len(&events), 1);
    let event = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(event.0, redemption_address);
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

#[test]
fn test_execute_redemption_should_emit_a_redemption_executed_event() {
    let e = setup_env();
    let (_, redemption_address, client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    let token: Address = Address::generate(&e);
    let relayer: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let salt: u128 = 100;
    permission_manager_client.grant_role(&admin, &relayer, &REDEMPTION_EXECUTOR_ROLE);
    client.add_token(&token);

    client.on_redeem(&token, &user, &100u128, &salt);
    client.execute_redemption(&relayer, &token, &user, &100u128, &salt);

    let events = e.events().all();
    assert_eq!(Vec::len(&events), 1);
    let event = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(event.0, redemption_address);
    assert_eq!(Vec::len(&event.1), 2);
    let first_event_topic = Vec::get(&event.1, 0).expect("First event topic should be present");
    let second_event_topic = Vec::get(&event.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_event_topic.to_xdr(&e),
        symbol_short!("redeem").to_xdr(&e)
    );
    assert_eq!(
        second_event_topic.to_xdr(&e),
        symbol_short!("exec").to_xdr(&e)
    );
    assert_eq!(
        event.2.to_xdr(&e),
        RedemptionEntry(token, user, 100, salt).to_xdr(&e)
    );
}

#[test]
fn test_execute_redemption_fail_if_not_redemption_executor() {
    let e = setup_env();
    let (_, _, client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    let token: Address = Address::generate(&e);
    let relayer: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let salt: u128 = 100;
    permission_manager_client.grant_role(&admin, &relayer, &WHITELISTED_ROLE);
    client.add_token(&token);

    client.on_redeem(&token, &user, &100u128, &salt);
    let result = client.try_execute_redemption(&relayer, &token, &user, &100u128, &salt);

    assert!(result.is_err());
}

#[test]
fn test_cancel_redemption_should_emit_a_redemption_cancelled_event() {
    let e = setup_env();
    let (_, redemption_address, client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    let token: Address = Address::generate(&e);
    let relayer: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let salt: u128 = 100;
    permission_manager_client.grant_role(&admin, &relayer, &REDEMPTION_EXECUTOR_ROLE);
    client.add_token(&token);

    client.on_redeem(&token, &user, &100u128, &salt);
    client.cancel_redemption(&relayer, &token, &user, &100u128, &salt);

    let events = e.events().all();
    assert_eq!(Vec::len(&events), 1);
    let event = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(event.0, redemption_address);
    assert_eq!(Vec::len(&event.1), 2);
    let first_event_topic = Vec::get(&event.1, 0).expect("First event topic should be present");
    let second_event_topic = Vec::get(&event.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_event_topic.to_xdr(&e),
        symbol_short!("redeem").to_xdr(&e)
    );
    assert_eq!(
        second_event_topic.to_xdr(&e),
        symbol_short!("cancel").to_xdr(&e)
    );
    assert_eq!(
        event.2.to_xdr(&e),
        RedemptionEntry(token, user, 100, salt).to_xdr(&e)
    );
}

#[test]
fn test_cancel_redemption_fail_if_not_redemption_executor() {
    let e = setup_env();
    let (_, _, client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    let token: Address = Address::generate(&e);
    let relayer: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let salt: u128 = 100;
    permission_manager_client.grant_role(&admin, &relayer, &WHITELISTED_ROLE);
    client.add_token(&token);

    client.on_redeem(&token, &user, &100u128, &salt);
    let result = client.try_cancel_redemption(&relayer, &token, &user, &100u128, &salt);

    assert!(result.is_err());
}
