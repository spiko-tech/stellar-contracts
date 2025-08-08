#![cfg(test)]

extern crate std;

use super::contract::{Token, TokenClient};
use contracts_utils::role::{MINTER_ROLE, WHITELISTED_ROLE};
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

#[test]
fn test_mint_should_emit_a_mint_event() {
    let e = setup_env();
    let minter: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let amount: i128 = 1000000;
    let (_, token_address, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user, &WHITELISTED_ROLE);

    client.mint(&user, &amount, &minter);

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
    let (_, token_address, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);
    permission_manager_client.grant_role(&admin, &user, &WHITELISTED_ROLE);

    client.mint(&user, &amount, &minter);

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
    let (_, _, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &user, &WHITELISTED_ROLE);

    let result = client.try_mint(&user, &amount, &minter);

    assert!(result.is_err());
}

#[test]
fn test_mint_should_fail_if_user_is_not_whitelisted() {
    let e = setup_env();
    let minter: Address = Address::generate(&e);
    let user: Address = Address::generate(&e);
    let amount: i128 = 1000000;
    let (_, _, client) = deploy_token(&e);
    let (admin, permission_manager_address, permission_manager_client) =
        deploy_permission_manager(&e);
    client.set_permission_manager(&permission_manager_address);
    permission_manager_client.grant_role(&admin, &minter, &MINTER_ROLE);

    let result = client.try_mint(&user, &amount, &minter);

    assert!(result.is_err());
}
