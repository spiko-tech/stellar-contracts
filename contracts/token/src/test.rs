#![cfg(test)]

extern crate std;

use crate::contract::{BurnBatchOperation, MintBatchOperation};

use super::contract::{Token, TokenClient};
use contracts_utils::role::{BURNER_ROLE, MINTER_ROLE, WHITELISTED_ROLE};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events},
    xdr::ToXdr,
    Address, Env, String, Vec,
};

mod permission_manager {
    use soroban_sdk::contractimport;

    contractimport!(file = "../../wasm/permission_manager.wasm");
}

mod redemption {
    use soroban_sdk::contractimport;

    contractimport!(file = "../../wasm/redemption.wasm");
}

fn setup_env() -> Env {
    let e: Env = Env::default();
    e.mock_all_auths();
    e
}

fn deploy_token(e: &Env) -> (Address, Address, TokenClient) {
    let owner: Address = Address::generate(e);
    let name: String = String::from_str(e, "Token");
    let symbol: String = String::from_str(e, "EUTBL");
    let decimals: u32 = 6;
    let token_address = e.register(Token, (owner.clone(), name, symbol, decimals));
    let client = TokenClient::new(e, &token_address);

    (owner, token_address, client)
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

fn deploy_redemption(e: &Env) -> (Address, Address, redemption::Client<'_>) {
    let owner: Address = Address::generate(e);
    let redemption_address = e.register(
        redemption::WASM,
        redemption::Args::__constructor(&owner.clone()),
    );
    let redemption_client = redemption::Client::new(e, &redemption_address);

    (owner, redemption_address, redemption_client)
}

#[test]
fn test_should_set_owner_on_constructor() {
    let e = setup_env();
    let (owner, _, client) = deploy_token(&e);

    let fetched_owner = client.get_owner();

    assert_eq!(fetched_owner, Some(owner));
}

//// set_permission_manager

#[test]
fn test_set_permission_manager_should_require_owner_auth() {
    let e = setup_env();
    let (owner, _, client) = deploy_token(&e);
    let (_, permission_manager_address, _) = deploy_permission_manager(&e);

    client.set_permission_manager(&permission_manager_address);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &owner);
}

//// set_redemption

#[test]
fn test_set_redemption_should_require_owner_auth() {
    let e = setup_env();
    let (owner, _, client) = deploy_token(&e);
    let (_, redemption_address, _) = deploy_redemption(&e);

    client.set_redemption(&redemption_address);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &owner);
}

//// mint

#[test]
fn test_mint_should_emit_a_mint_event() {
    let e = setup_env();
    let minter: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let amount: i128 = 1000000;
    let idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let (_, token_address, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user, &WHITELISTED_ROLE);

    client.mint(&user, &amount, &minter, &idempotency_key);

    let events = e.events().clone().all();
    assert_eq!(Vec::len(&events), 1);
    let event = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(event.0, token_address);
    assert_eq!(Vec::len(&event.1), 2);
    let first_event_topic = Vec::get(&event.1, 0).expect("First event topic should be present");
    let second_event_topic = Vec::get(&event.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_event_topic.to_xdr(&e),
        symbol_short!("mint").to_xdr(&e)
    );
    assert_eq!(second_event_topic.to_xdr(&e), user.to_xdr(&e));
    assert_eq!(event.2.to_xdr(&e), amount.to_xdr(&e));
}

#[test]
fn test_mint_should_require_auth_and_mint_and_emit_a_mint_event() {
    let e = setup_env();
    let minter: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let amount: i128 = 1000000;
    let idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let (_, token_address, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user, &WHITELISTED_ROLE);

    client.mint(&user, &amount, &minter, &idempotency_key);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &minter);

    let events = e.events().clone().all();
    assert_eq!(Vec::len(&events), 1);
    let event = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(event.0, token_address);
    assert_eq!(Vec::len(&event.1), 2);
    let first_event_topic = Vec::get(&event.1, 0).expect("First event topic should be present");
    let second_event_topic = Vec::get(&event.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_event_topic.to_xdr(&e),
        symbol_short!("mint").to_xdr(&e)
    );
    assert_eq!(second_event_topic.to_xdr(&e), user.clone().to_xdr(&e));
    assert_eq!(event.2.to_xdr(&e), amount.to_xdr(&e));

    let balance = client.balance(&user);
    assert_eq!(balance, amount);
}

#[test]
fn test_mint_should_fail_if_minter_is_not_minter() {
    let e = setup_env();
    let minter: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let amount: i128 = 1000000;
    let idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let (_, _, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &user, &WHITELISTED_ROLE);

    let result = client.try_mint(&user, &amount, &minter, &idempotency_key);

    assert!(result.is_err());
}

#[test]
fn test_mint_should_fail_if_user_is_not_whitelisted() {
    let e = setup_env();
    let minter: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let amount: i128 = 1000000;
    let idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let (_, _, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);

    let result = client.try_mint(&user, &amount, &minter, &idempotency_key);

    assert!(result.is_err());
}

#[test]
fn test_mint_should_fail_if_idempotency_key_is_already_used() {
    let e = setup_env();
    let minter: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let amount: i128 = 1000000;
    let idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let (_, _, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user, &WHITELISTED_ROLE);

    client.mint(&user, &amount, &minter, &idempotency_key);
    let result = client.try_mint(&user, &amount, &minter, &idempotency_key);

    assert!(result.is_err());
}

//// mint_batch

#[test]
fn test_mint_batch_should_require_auth_and_mint_and_emit_mint_events() {
    let e = setup_env();
    let minter: Address = Address::generate(&e);
    let user1: Address = Address::generate(&e);
    let user2: Address = Address::generate(&e);
    let amount1: i128 = 1000000;
    let amount2: i128 = 2000000;
    let idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let (_, token_address, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user1, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &user2, &WHITELISTED_ROLE);
    let mut operations = Vec::new(&e);
    operations.push_front(MintBatchOperation(user1.clone(), amount1));
    operations.push_front(MintBatchOperation(user2.clone(), amount2));

    client.mint_batch(&operations, &minter, &idempotency_key);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &minter);

    let events = e.events().clone().all();
    assert_eq!(Vec::len(&events), 2);
    let event1 = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(event1.0, token_address);
    assert_eq!(Vec::len(&event1.1), 2);
    let first_event1_topic = Vec::get(&event1.1, 0).expect("First event topic should be present");
    let second_event1_topic = Vec::get(&event1.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_event1_topic.to_xdr(&e),
        symbol_short!("mint").to_xdr(&e)
    );
    assert_eq!(second_event1_topic.to_xdr(&e), user2.clone().to_xdr(&e));
    assert_eq!(event1.2.to_xdr(&e), amount2.to_xdr(&e));

    let event2 = Vec::get(&events, 1).expect("Event should be present");
    assert_eq!(event2.0, token_address);
    assert_eq!(Vec::len(&event2.1), 2);
    let first_event2_topic = Vec::get(&event2.1, 0).expect("First event topic should be present");
    let second_event2_topic = Vec::get(&event2.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_event2_topic.to_xdr(&e),
        symbol_short!("mint").to_xdr(&e)
    );
    assert_eq!(second_event2_topic.to_xdr(&e), user1.clone().to_xdr(&e));
    assert_eq!(event2.2.to_xdr(&e), amount1.to_xdr(&e));

    let balance1 = client.balance(&user1);
    assert_eq!(balance1, amount1);
    let balance2 = client.balance(&user2);
    assert_eq!(balance2, amount2);
}

#[test]
fn test_mint_batch_should_fail_if_minter_is_not_minter() {
    let e = setup_env();
    let minter: Address = Address::generate(&e);
    let user1: Address = Address::generate(&e);
    let user2: Address = Address::generate(&e);
    let amount1: i128 = 1000000;
    let amount2: i128 = 2000000;
    let idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let (_, _, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &user1, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &user2, &WHITELISTED_ROLE);
    let mut operations = Vec::new(&e);
    operations.push_front(MintBatchOperation(user1.clone(), amount1));
    operations.push_front(MintBatchOperation(user2.clone(), amount2));

    let result = client.try_mint_batch(&operations, &minter, &idempotency_key);

    assert!(result.is_err());
}

#[test]
fn test_mint_batch_should_fail_if_one_of_the_users_is_not_whitelisted() {
    let e = setup_env();
    let minter: Address = Address::generate(&e);
    let user1: Address = Address::generate(&e);
    let user2: Address = Address::generate(&e);
    let amount1: i128 = 1000000;
    let amount2: i128 = 2000000;
    let idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let (_, _, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user1, &WHITELISTED_ROLE);
    let mut operations = Vec::new(&e);
    operations.push_front(MintBatchOperation(user1.clone(), amount1));
    operations.push_front(MintBatchOperation(user2.clone(), amount2));

    let result = client.try_mint_batch(&operations, &minter, &idempotency_key);

    assert!(result.is_err());
}

#[test]
fn test_mint_batch_should_fail_if_idempotency_key_is_already_used() {
    let e = setup_env();
    let minter: Address = Address::generate(&e);
    let user1: Address = Address::generate(&e);
    let user2: Address = Address::generate(&e);
    let amount1: i128 = 1000000;
    let amount2: i128 = 2000000;
    let idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let (_, _, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user1, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &user2, &WHITELISTED_ROLE);
    let mut operations = Vec::new(&e);
    operations.push_front(MintBatchOperation(user1.clone(), amount1));
    operations.push_front(MintBatchOperation(user2.clone(), amount2));

    client.mint_batch(&operations, &minter, &idempotency_key);
    let result = client.try_mint_batch(&operations, &minter, &idempotency_key);

    assert!(result.is_err());
}

//// redeem

#[test]
fn test_redeem_should_require_auth_and_redeem_and_emit_a_redeem_event_and_call_redemption_on_redeem(
) {
    let e = setup_env();
    let amount: i128 = 1000000;
    let redeem_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let user: Address = Address::generate(&e);
    let minter: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, token_address, client) = deploy_token(&e);
    let (_, redemption_address, redemption_client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    client.set_redemption(&redemption_address);
    redemption_client.add_token(&token_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &redemption_address, &WHITELISTED_ROLE);
    client.mint(&user, &amount, &minter, &mint_idempotency_key);

    client.redeem(&amount, &user, &redeem_idempotency_key);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &user);

    let events = e.events().clone().all();
    assert_eq!(Vec::len(&events), 2);

    let transfer_event = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(transfer_event.0, token_address);
    assert_eq!(Vec::len(&transfer_event.1), 3);
    let first_transfer_event_topic =
        Vec::get(&transfer_event.1, 0).expect("First event topic should be present");
    let second_transfer_event_topic =
        Vec::get(&transfer_event.1, 1).expect("Second event topic should be present");
    let third_transfer_event_topic =
        Vec::get(&transfer_event.1, 2).expect("Third event topic should be present");
    assert_eq!(
        first_transfer_event_topic.to_xdr(&e),
        symbol_short!("transfer").to_xdr(&e)
    );
    assert_eq!(
        second_transfer_event_topic.to_xdr(&e),
        user.clone().to_xdr(&e)
    );
    assert_eq!(
        third_transfer_event_topic.to_xdr(&e),
        redemption_address.clone().to_xdr(&e)
    );
    assert_eq!(transfer_event.2.to_xdr(&e), (amount as i128).to_xdr(&e));

    let redemption_event: (Address, Vec<soroban_sdk::Val>, soroban_sdk::Val) =
        Vec::get(&events, 1).expect("Event should be present");
    assert_eq!(redemption_event.0, redemption_address);
    assert_eq!(Vec::len(&redemption_event.1), 2);
    let first_redemption_event_topic =
        Vec::get(&redemption_event.1, 0).expect("First event topic should be present");
    let second_redemption_event_topic =
        Vec::get(&redemption_event.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_redemption_event_topic.to_xdr(&e),
        symbol_short!("REDEEM").to_xdr(&e)
    );
    assert_eq!(
        second_redemption_event_topic.to_xdr(&e),
        symbol_short!("INIT").to_xdr(&e)
    );
    assert_eq!(
        redemption_event.2.to_xdr(&e),
        (token_address, user, amount, redeem_idempotency_key).to_xdr(&e)
    );

    let balance = client.balance(&redemption_address);
    assert_eq!(balance, amount);
}

#[test]
fn test_redeem_should_fail_if_amount_is_not_positive() {
    let e = setup_env();
    let amount: i128 = 0;
    let redeem_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let user: Address = Address::generate(&e);
    let minter: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, token_address, client) = deploy_token(&e);
    let (_, redemption_address, redemption_client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    client.set_redemption(&redemption_address);
    redemption_client.add_token(&token_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &redemption_address, &WHITELISTED_ROLE);
    client.mint(&user, &amount, &minter, &mint_idempotency_key);

    let result = client.try_redeem(&amount, &user, &redeem_idempotency_key);

    assert!(result.is_err());
}

#[test]
fn test_redeem_should_fail_if_user_is_not_whitelisted() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let redeem_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let user: Address = Address::generate(&e);
    let minter: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, token_address, client) = deploy_token(&e);
    let (_, redemption_address, redemption_client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    client.set_redemption(&redemption_address);
    redemption_client.add_token(&token_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &redemption_address, &WHITELISTED_ROLE);
    client.mint(&user, &amount, &minter, &mint_idempotency_key);
    permission_manager_client.revoke_role(&admin, &user, &WHITELISTED_ROLE);

    let result = client.try_redeem(&amount, &user, &redeem_idempotency_key);

    assert!(result.is_err());
}

#[test]
fn test_redeem_should_fail_if_redemption_contract_is_not_whitelisted() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let redeem_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let user: Address = Address::generate(&e);
    let minter: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, token_address, client) = deploy_token(&e);
    let (_, redemption_address, redemption_client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    client.set_redemption(&redemption_address);
    redemption_client.add_token(&token_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &redemption_address, &WHITELISTED_ROLE);
    client.mint(&user, &amount, &minter, &mint_idempotency_key);
    permission_manager_client.revoke_role(&admin, &redemption_address, &WHITELISTED_ROLE);

    let result = client.try_redeem(&amount, &user, &redeem_idempotency_key);

    assert!(result.is_err());
}

#[test]
fn test_redeem_should_fail_if_not_enough_balance() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let redeem_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let user: Address = Address::generate(&e);
    let minter: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, token_address, client) = deploy_token(&e);
    let (_, redemption_address, redemption_client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    client.set_redemption(&redemption_address);
    redemption_client.add_token(&token_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &redemption_address, &WHITELISTED_ROLE);
    client.mint(&user, &(amount / 2), &minter, &mint_idempotency_key);

    let result = client.try_redeem(&amount, &user, &redeem_idempotency_key);

    assert!(result.is_err());
}

#[test]
fn test_redeem_should_fail_if_idempotency_key_is_already_used() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let redeem_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let user: Address = Address::generate(&e);
    let minter: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, token_address, client) = deploy_token(&e);
    let (_, redemption_address, redemption_client) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    client.set_redemption(&redemption_address);
    redemption_client.add_token(&token_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &redemption_address, &WHITELISTED_ROLE);
    client.mint(&user, &(amount * 3), &minter, &mint_idempotency_key);

    client.redeem(&amount, &user, &redeem_idempotency_key);
    let result = client.try_redeem(&amount, &user, &redeem_idempotency_key);

    assert!(result.is_err());
}

//// transfer

#[test]
fn test_safe_transfer_should_transfer_tokens_and_emit_a_transfer_event() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let user1: Address = Address::generate(&e);
    let user2: Address = Address::generate(&e);
    let minter: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let transfer_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, token_address, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user1, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &user2, &WHITELISTED_ROLE);
    client.mint(&user1, &amount, &minter, &mint_idempotency_key);

    client.safe_transfer(&user1, &user2, &amount, &transfer_idempotency_key);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &user1);

    let events = e.events().clone().all();
    assert_eq!(Vec::len(&events), 1);

    let transfer_event = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(transfer_event.0, token_address);
    assert_eq!(Vec::len(&transfer_event.1), 3);
    let first_transfer_event_topic =
        Vec::get(&transfer_event.1, 0).expect("First event topic should be present");
    let second_transfer_event_topic =
        Vec::get(&transfer_event.1, 1).expect("Second event topic should be present");
    let third_transfer_event_topic =
        Vec::get(&transfer_event.1, 2).expect("Third event topic should be present");
    assert_eq!(
        first_transfer_event_topic.to_xdr(&e),
        symbol_short!("transfer").to_xdr(&e)
    );
    assert_eq!(
        second_transfer_event_topic.to_xdr(&e),
        user1.clone().to_xdr(&e)
    );
    assert_eq!(
        third_transfer_event_topic.to_xdr(&e),
        user2.clone().to_xdr(&e)
    );
    assert_eq!(transfer_event.2.to_xdr(&e), (amount as i128).to_xdr(&e));

    let balance1 = client.balance(&user1);
    assert_eq!(balance1, 0);
    let balance2 = client.balance(&user2);
    assert_eq!(balance2, amount);
}

#[test]
fn test_safe_transfer_should_fail_if_user1_is_not_whitelisted() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let user1: Address = Address::generate(&e);
    let user2: Address = Address::generate(&e);
    let minter: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let transfer_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, _, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user1, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &user2, &WHITELISTED_ROLE);
    client.mint(&user1, &amount, &minter, &mint_idempotency_key);
    permission_manager_client.revoke_role(&admin, &user1, &WHITELISTED_ROLE);

    let result = client.try_safe_transfer(&user1, &user2, &amount, &transfer_idempotency_key);

    assert!(result.is_err());
}

#[test]
fn test_safe_transfer_should_fail_if_user2_is_not_whitelisted() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let user1: Address = Address::generate(&e);
    let user2: Address = Address::generate(&e);
    let minter: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let transfer_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, _, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user1, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &user2, &WHITELISTED_ROLE);
    client.mint(&user1, &amount, &minter, &mint_idempotency_key);
    permission_manager_client.revoke_role(&admin, &user2, &WHITELISTED_ROLE);

    let result = client.try_safe_transfer(&user1, &user2, &amount, &transfer_idempotency_key);

    assert!(result.is_err());
}

#[test]
fn test_transfer_should_fail_if_not_enough_balance() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let user1: Address = Address::generate(&e);
    let user2: Address = Address::generate(&e);
    let minter: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let transfer_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, _, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user1, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &user2, &WHITELISTED_ROLE);
    client.mint(&user1, &(amount / 2), &minter, &mint_idempotency_key);

    let result = client.try_safe_transfer(&user1, &user2, &amount, &transfer_idempotency_key);

    assert!(result.is_err());
}

#[test]
fn test_transfer_should_fail_if_idempotency_key_is_already_used() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let user1: Address = Address::generate(&e);
    let user2: Address = Address::generate(&e);
    let minter: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let transfer_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, _, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user1, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &user2, &WHITELISTED_ROLE);
    client.mint(&user1, &(amount * 3), &minter, &mint_idempotency_key);

    client.safe_transfer(&user1, &user2, &amount, &transfer_idempotency_key);
    let result = client.try_safe_transfer(&user1, &user2, &amount, &transfer_idempotency_key);

    assert!(result.is_err());
}

//// burn

#[test]
fn test_burn_should_require_auth_and_burn_and_emit_a_burn_event() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let minter: Address = Address::generate(&e);
    let burner: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let burn_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, token_address, client) = deploy_token(&e);
    let (_, redemption_address, _) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &redemption_address, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &burner, &BURNER_ROLE);
    client.mint(&redemption_address, &amount, &minter, &mint_idempotency_key);

    client.burn(&redemption_address, &amount, &burner, &burn_idempotency_key);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &burner);

    let events = e.events().clone().all();
    assert_eq!(Vec::len(&events), 1);

    let transfer_event = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(transfer_event.0, token_address);
    assert_eq!(Vec::len(&transfer_event.1), 2);
    let first_transfer_event_topic =
        Vec::get(&transfer_event.1, 0).expect("First event topic should be present");
    let second_transfer_event_topic =
        Vec::get(&transfer_event.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_transfer_event_topic.to_xdr(&e),
        symbol_short!("burn").to_xdr(&e)
    );
    assert_eq!(
        second_transfer_event_topic.to_xdr(&e),
        redemption_address.clone().to_xdr(&e)
    );
    assert_eq!(transfer_event.2.to_xdr(&e), (amount as i128).to_xdr(&e));

    let balance = client.balance(&redemption_address);
    assert_eq!(balance, 0);
}

#[test]
fn test_burn_should_fail_if_not_burner() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let minter: Address = Address::generate(&e);
    let burner: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let burn_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, _, client) = deploy_token(&e);
    let (_, redemption_address, _) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    client.set_redemption(&redemption_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &redemption_address, &WHITELISTED_ROLE);
    client.mint(&redemption_address, &amount, &minter, &mint_idempotency_key);

    let result = client.try_burn(&redemption_address, &amount, &burner, &burn_idempotency_key);

    assert!(result.is_err());
}

#[test]
fn test_burn_should_fail_if_idempotency_key_is_already_used() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let minter: Address = Address::generate(&e);
    let burner: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let burn_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, _, client) = deploy_token(&e);
    let (_, redemption_address, _) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &redemption_address, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &burner, &BURNER_ROLE);
    client.mint(
        &redemption_address,
        &(amount * 3),
        &minter,
        &mint_idempotency_key,
    );

    client.burn(&redemption_address, &amount, &burner, &burn_idempotency_key);
    let result = client.try_burn(&redemption_address, &amount, &burner, &burn_idempotency_key);

    assert!(result.is_err());
}

//// burn_batch

#[test]
fn test_burn_batch_should_require_auth_and_burn_and_emit_a_burn_events() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let minter: Address = Address::generate(&e);
    let burner: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let burn_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, token_address, client) = deploy_token(&e);
    let (_, redemption_address, _) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &redemption_address, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &burner, &BURNER_ROLE);
    client.mint(&redemption_address, &amount, &minter, &mint_idempotency_key);
    let mut operations = Vec::new(&e);
    operations.push_front(BurnBatchOperation(redemption_address.clone(), amount / 2));
    operations.push_front(BurnBatchOperation(redemption_address.clone(), amount / 2));

    client.burn_batch(&operations, &burner, &burn_idempotency_key);

    let auths = e.auths();
    assert_eq!(auths.len(), 1);
    let (addr, _invocation) = &auths[0];
    assert_eq!(addr, &burner);

    let events = e.events().clone().all();
    assert_eq!(Vec::len(&events), 2);

    let event1 = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(event1.0, token_address);
    assert_eq!(Vec::len(&event1.1), 2);
    let first_event1_topic = Vec::get(&event1.1, 0).expect("First event topic should be present");
    let second_event1_topic = Vec::get(&event1.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_event1_topic.to_xdr(&e),
        symbol_short!("burn").to_xdr(&e)
    );
    assert_eq!(
        second_event1_topic.to_xdr(&e),
        redemption_address.clone().to_xdr(&e)
    );
    assert_eq!(event1.2.to_xdr(&e), (amount / 2 as i128).to_xdr(&e));

    let event2 = Vec::get(&events, 0).expect("Event should be present");
    assert_eq!(event2.0, token_address);
    assert_eq!(Vec::len(&event2.1), 2);
    let first_event2_topic = Vec::get(&event2.1, 0).expect("First event topic should be present");
    let second_event2_topic = Vec::get(&event2.1, 1).expect("Second event topic should be present");
    assert_eq!(
        first_event2_topic.to_xdr(&e),
        symbol_short!("burn").to_xdr(&e)
    );
    assert_eq!(
        second_event2_topic.to_xdr(&e),
        redemption_address.clone().to_xdr(&e)
    );
    assert_eq!(event2.2.to_xdr(&e), (amount / 2 as i128).to_xdr(&e));

    let balance = client.balance(&redemption_address);
    assert_eq!(balance, 0);
}

#[test]
fn test_burn_batch_should_fail_if_not_burner() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let minter: Address = Address::generate(&e);
    let burner: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let burn_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, _, client) = deploy_token(&e);
    let (_, redemption_address, _) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    client.set_redemption(&redemption_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &redemption_address, &WHITELISTED_ROLE);
    client.mint(&redemption_address, &amount, &minter, &mint_idempotency_key);
    let mut operations = Vec::new(&e);
    operations.push_front(BurnBatchOperation(redemption_address.clone(), amount / 2));
    operations.push_front(BurnBatchOperation(redemption_address.clone(), amount / 2));

    let result = client.try_burn_batch(&operations, &burner, &burn_idempotency_key);

    assert!(result.is_err());
}

#[test]
fn test_burn_batch_should_fail_if_idempotency_key_is_already_used() {
    let e = setup_env();
    let amount: i128 = 1000000;
    let minter: Address = Address::generate(&e);
    let burner: Address = Address::generate(&e);
    let mint_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY");
    let burn_idempotency_key: String = String::from_str(&e, "IDEMPOTENCY_KEY2");
    let (_, _, client) = deploy_token(&e);
    let (_, redemption_address, _) = deploy_redemption(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &redemption_address, &WHITELISTED_ROLE);
    permission_manager_client.grant_role(&admin, &burner, &BURNER_ROLE);
    client.mint(
        &redemption_address,
        &(amount * 3),
        &minter,
        &mint_idempotency_key,
    );
    let mut operations = Vec::new(&e);
    operations.push_front(BurnBatchOperation(redemption_address.clone(), amount / 2));
    operations.push_front(BurnBatchOperation(redemption_address.clone(), amount / 2));

    client.burn_batch(&operations, &burner, &burn_idempotency_key);
    let result = client.try_burn_batch(&operations, &burner, &burn_idempotency_key);

    assert!(result.is_err());
}
