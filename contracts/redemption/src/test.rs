#![cfg(test)]

extern crate std;

use crate::contract::ExecuteRedemptionOperation;

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

    contractimport!(file = "../../wasm/permission_manager.wasm");
}

mod token {
    use soroban_sdk::{contract, contractimpl, Address};

    use crate::contract::TokenInterface;

    #[contract]
    pub struct Mock;
    #[contractimpl]
    impl TokenInterface for Mock {
        fn burn(_account: Address, _amount: i128) {}
        fn transfer(_from: Address, _to: Address, _amount: i128) {}
    }
}

fn setup_env() -> Env {
    let e: Env = Env::default();
    e.mock_all_auths();

    e
}

fn deploy_redemption(e: &Env) -> (Address, Address, RedemptionClient) {
    let owner: Address = Address::generate(e);
    let redemption_address = e.register(Redemption, RedemptionArgs::__constructor(&owner.clone()));
    let client = RedemptionClient::new(e, &redemption_address);

    (owner, redemption_address, client)
}

fn deploy_permission_manager(e: &Env) -> (Address, Address, permission_manager::Client<'_>) {
    let admin: Address = Address::generate(e);
    let permission_manager_address = e.register(
        permission_manager::WASM,
        permission_manager::Args::__constructor(&admin.clone()),
    );
    let permission_manager_client = permission_manager::Client::new(e, &permission_manager_address);
    permission_manager_client.initialize();

    (admin, permission_manager_address, permission_manager_client)
}

fn deploy_token(e: &Env, token_address: &Address) {
    e.register_at(&token_address, token::Mock, ());
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
    deploy_token(&e, &token);

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

    let result = client.try_on_redeem(&non_token_contract, &user, &100, &100);

    assert!(result.is_err());
}

#[test]
fn test_on_redeem_should_pass_if_token_is_set() {
    let e = setup_env();
    let (_, _, client) = deploy_redemption(&e);
    let token: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    deploy_token(&e, &token);
    client.add_token(&token);

    let result = client.try_on_redeem(&token, &user, &100, &100);

    assert!(result.is_ok());
}

#[test]
fn test_on_redeem_should_require_token_auth() {
    let e = setup_env();
    let (_, _, client) = deploy_redemption(&e);
    let token: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    deploy_token(&e, &token);
    client.add_token(&token);

    client.on_redeem(&token, &user, &100, &100);

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
    deploy_token(&e, &token);
    client.add_token(&token);

    client.on_redeem(&token, &user, &100, &salt);
    let result = client.try_on_redeem(&token, &user, &100, &salt);

    assert!(result.is_err());
}

#[test]
fn test_on_redeem_should_emit_a_redemption_initiated_event() {
    let e = setup_env();
    let (_, redemption_address, client) = deploy_redemption(&e);
    let token: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let salt: u128 = 100;
    deploy_token(&e, &token);
    client.add_token(&token);

    client.on_redeem(&token, &user, &100, &salt);

    let events = e.events().all();
    assert_eq!(Vec::len(&events), 1);
    let event = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(event.0, redemption_address);
    assert_eq!(Vec::len(&event.1), 2);
    let first_event_topic = Vec::get(&event.1, 0).expect("First event topic should be present");
    let second_event_topic = Vec::get(&event.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_event_topic.to_xdr(&e),
        symbol_short!("REDEEM").to_xdr(&e)
    );
    assert_eq!(
        second_event_topic.to_xdr(&e),
        symbol_short!("INIT").to_xdr(&e)
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
    deploy_token(&e, &token);
    client.add_token(&token);

    client.on_redeem(&token, &user, &100, &salt);
    client.execute_redemption(&relayer, &token, &user, &100, &salt);

    let events = e.events().all();
    assert_eq!(Vec::len(&events), 1);
    let event = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(event.0, redemption_address);
    assert_eq!(Vec::len(&event.1), 2);
    let first_event_topic = Vec::get(&event.1, 0).expect("First event topic should be present");
    let second_event_topic = Vec::get(&event.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_event_topic.to_xdr(&e),
        symbol_short!("REDEEM").to_xdr(&e)
    );
    assert_eq!(
        second_event_topic.to_xdr(&e),
        symbol_short!("EXEC").to_xdr(&e)
    );
    assert_eq!(
        event.2.to_xdr(&e),
        RedemptionEntry(token, user, 100, salt).to_xdr(&e)
    );
}

#[test]
fn test_execute_redemption_fail_if_redemption_not_initiated() {
    let e = setup_env();
    let (_, _, client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    let token: Address = Address::generate(&e);
    let relayer: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let salt: u128 = 100;
    permission_manager_client.grant_role(&admin, &relayer, &REDEMPTION_EXECUTOR_ROLE);
    deploy_token(&e, &token);
    client.add_token(&token);

    let result = client.try_execute_redemption(&relayer, &token, &user, &100, &salt);

    assert!(result.is_err());
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
    deploy_token(&e, &token);
    client.add_token(&token);

    client.on_redeem(&token, &user, &100, &salt);
    let result = client.try_execute_redemption(&relayer, &token, &user, &100, &salt);

    assert!(result.is_err());
}

#[test]
fn test_execute_redemption_batch_should_emit_a_redemption_executed_events() {
    let e = setup_env();
    let (_, redemption_address, client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    let token: Address = Address::generate(&e);
    let relayer: Address = Address::generate(&e);
    let user1: Address = Address::generate(&e);
    let user2: Address = Address::generate(&e);
    let salt1: u128 = 100;
    let salt2: u128 = 200;
    let amount1: i128 = 1000000;
    let amount2: i128 = 2000000;
    let mut operations = Vec::new(&e);
    operations.push_front(ExecuteRedemptionOperation(
        token.clone(),
        user1.clone(),
        amount1,
        salt1,
    ));
    operations.push_front(ExecuteRedemptionOperation(
        token.clone(),
        user2.clone(),
        amount2,
        salt2,
    ));
    permission_manager_client.grant_role(&admin, &relayer, &REDEMPTION_EXECUTOR_ROLE);
    deploy_token(&e, &token);
    client.add_token(&token);

    client.on_redeem(&token, &user1, &amount1, &salt1);
    client.on_redeem(&token, &user2, &amount2, &salt2);
    client.execute_redemption_batch(&relayer, &operations);

    let events = e.events().all();
    assert_eq!(Vec::len(&events), 2);

    let event1 = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(event1.0, redemption_address);
    assert_eq!(Vec::len(&event1.1), 2);
    let first_event1_topic = Vec::get(&event1.1, 0).expect("First event topic should be present");
    let second_event1_topic = Vec::get(&event1.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_event1_topic.to_xdr(&e),
        symbol_short!("REDEEM").to_xdr(&e)
    );
    assert_eq!(
        second_event1_topic.to_xdr(&e),
        symbol_short!("EXEC").to_xdr(&e)
    );
    assert_eq!(
        event1.2.to_xdr(&e),
        RedemptionEntry(token.clone(), user2, amount2, salt2).to_xdr(&e)
    );

    let event2 = Vec::get(&events, 1).expect("Event should be present");
    assert_eq!(event2.0, redemption_address);
    assert_eq!(Vec::len(&event2.1), 2);
    let first_event2_topic = Vec::get(&event2.1, 0).expect("First event topic should be present");
    let second_event2_topic = Vec::get(&event2.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_event2_topic.to_xdr(&e),
        symbol_short!("REDEEM").to_xdr(&e)
    );
    assert_eq!(
        second_event2_topic.to_xdr(&e),
        symbol_short!("EXEC").to_xdr(&e)
    );
    assert_eq!(
        event2.2.to_xdr(&e),
        RedemptionEntry(token, user1, amount1, salt1).to_xdr(&e)
    );
}

#[test]
fn test_execute_redemption_batch_fail_if_redemption_not_initiated() {
    let e = setup_env();
    let (_, _, client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    let token: Address = Address::generate(&e);
    let relayer: Address = Address::generate(&e);
    let user1: Address = Address::generate(&e);
    let user2: Address = Address::generate(&e);
    let salt1: u128 = 100;
    let salt2: u128 = 200;
    let amount1: i128 = 1000000;
    let amount2: i128 = 2000000;
    let mut operations = Vec::new(&e);
    operations.push_front(ExecuteRedemptionOperation(
        token.clone(),
        user1.clone(),
        amount1,
        salt1,
    ));
    operations.push_front(ExecuteRedemptionOperation(
        token.clone(),
        user2.clone(),
        amount2,
        salt2,
    ));
    permission_manager_client.grant_role(&admin, &relayer, &REDEMPTION_EXECUTOR_ROLE);
    deploy_token(&e, &token);
    client.add_token(&token);

    client.on_redeem(&token, &user1, &amount1, &salt1);
    let result = client.try_execute_redemption_batch(&relayer, &operations);

    assert!(result.is_err());
}

#[test]
fn test_execute_redemption_batch_fail_if_not_redemption_executor() {
    let e = setup_env();
    let (_, _, client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    let token: Address = Address::generate(&e);
    let relayer: Address = Address::generate(&e);
    let user1: Address = Address::generate(&e);
    let user2: Address = Address::generate(&e);
    let salt1: u128 = 100;
    let salt2: u128 = 200;
    let amount1: i128 = 1000000;
    let amount2: i128 = 2000000;
    let mut operations = Vec::new(&e);
    operations.push_front(ExecuteRedemptionOperation(
        token.clone(),
        user1.clone(),
        amount1,
        salt1,
    ));
    operations.push_front(ExecuteRedemptionOperation(
        token.clone(),
        user2.clone(),
        amount2,
        salt2,
    ));
    deploy_token(&e, &token);
    client.add_token(&token);

    client.on_redeem(&token, &user1, &amount1, &salt1);
    client.on_redeem(&token, &user2, &amount2, &salt2);
    let result = client.try_execute_redemption_batch(&relayer, &operations);

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
    deploy_token(&e, &token);
    client.add_token(&token);

    client.on_redeem(&token, &user, &100, &salt);
    client.cancel_redemption(&relayer, &token, &user, &100, &salt);

    let events = e.events().all();
    assert_eq!(Vec::len(&events), 1);
    let event = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(event.0, redemption_address);
    assert_eq!(Vec::len(&event.1), 2);
    let first_event_topic = Vec::get(&event.1, 0).expect("First event topic should be present");
    let second_event_topic = Vec::get(&event.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_event_topic.to_xdr(&e),
        symbol_short!("REDEEM").to_xdr(&e)
    );
    assert_eq!(
        second_event_topic.to_xdr(&e),
        symbol_short!("CANCEL").to_xdr(&e)
    );
    assert_eq!(
        event.2.to_xdr(&e),
        RedemptionEntry(token, user, 100, salt).to_xdr(&e)
    );
}

#[test]
fn test_cancel_redemption_fail_if_redemption_not_initiated() {
    let e = setup_env();
    let (_, _, client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    let token: Address = Address::generate(&e);
    let relayer: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let salt: u128 = 100;
    permission_manager_client.grant_role(&admin, &relayer, &REDEMPTION_EXECUTOR_ROLE);
    deploy_token(&e, &token);
    client.add_token(&token);

    let result = client.try_cancel_redemption(&relayer, &token, &user, &100, &salt);

    assert!(result.is_err());
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
    deploy_token(&e, &token);
    client.add_token(&token);

    client.on_redeem(&token, &user, &100, &salt);
    let result = client.try_cancel_redemption(&relayer, &token, &user, &100, &salt);

    assert!(result.is_err());
}
