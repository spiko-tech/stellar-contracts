#![cfg(test)]

extern crate std;

use super::contract::{PermissionManager, PermissionManagerArgs, PermissionManagerClient};
use soroban_sdk::{
    contract, symbol_short, testutils::Address as _, Address, Env, InvokeError, Symbol,
};

#[contract]
struct MockContract;

// Role constants from the contract
const MINTER_ROLE: Symbol = symbol_short!("0");
const PAUSER_ROLE: Symbol = symbol_short!("1");
const BURNER_ROLE: Symbol = symbol_short!("2");
const WHITELISTER_ROLE: Symbol = symbol_short!("3");
const WHITELISTED_ROLE: Symbol = symbol_short!("4");
const REDEMPTION_EXECUTOR_ROLE: Symbol = symbol_short!("5");

#[test]
fn test_admin_setup() {
    let e = Env::default();
    e.mock_all_auths();

    let admin: Address = Address::generate(&e);
    let contract_address = e.register(
        PermissionManager,
        PermissionManagerArgs::__constructor(&admin.clone()),
    );
    let client = PermissionManagerClient::new(&e, &contract_address);

    let fetched_admin = client.get_admin();

    assert_eq!(fetched_admin, Some(admin));
}

#[test]
fn test_has_role_should_return_some_when_user_has_role() {
    let e = Env::default();
    e.mock_all_auths();

    let admin: Address = Address::generate(&e);
    let contract_address = e.register(
        PermissionManager,
        PermissionManagerArgs::__constructor(&admin.clone()),
    );
    let client = PermissionManagerClient::new(&e, &contract_address);
    let user: Address = Address::generate(&e);

    client.grant_role(&admin, &user, &WHITELISTED_ROLE);
    let has_role = client.has_role(&user, &WHITELISTED_ROLE);

    assert_eq!(has_role, Some(0));
}

#[test]
fn test_has_role_should_return_none_when_user_does_not_have_role() {
    let e = Env::default();
    e.mock_all_auths();

    let admin: Address = Address::generate(&e);
    let contract_address = e.register(
        PermissionManager,
        PermissionManagerArgs::__constructor(&admin.clone()),
    );
    let client = PermissionManagerClient::new(&e, &contract_address);
    let user: Address = Address::generate(&e);

    let has_role = client.has_role(&user, &WHITELISTED_ROLE);

    assert_eq!(has_role, None);
}

#[test]
fn test_grant_role_whitelister_role_should_be_able_to_grant_whitelisted_role() {
    let e = Env::default();
    e.mock_all_auths();

    let admin: Address = Address::generate(&e);
    let contract_address = e.register(
        PermissionManager,
        PermissionManagerArgs::__constructor(&admin.clone()),
    );
    let client = PermissionManagerClient::new(&e, &contract_address);
    let whitelister: Address = Address::generate(&e);
    let whitelisted: Address = Address::generate(&e);

    client.grant_role(&admin, &whitelister, &WHITELISTER_ROLE);
    client.grant_role(&whitelister, &whitelisted, &WHITELISTED_ROLE);
    let has_role_whitelisted: Option<u32> = client.has_role(&whitelisted, &WHITELISTED_ROLE);

    assert_eq!(has_role_whitelisted, Some(0));
}

#[test]
fn test_grant_role_non_whitelister_role_should_not_be_able_to_grant_whitelisted_role_case_1() {
    let e = Env::default();
    e.mock_all_auths();

    let admin: Address = Address::generate(&e);
    let contract_address = e.register(
        PermissionManager,
        PermissionManagerArgs::__constructor(&admin.clone()),
    );
    let client = PermissionManagerClient::new(&e, &contract_address);
    let user: Address = Address::generate(&e);
    let non_whitelisted: Address = Address::generate(&e);

    let result = client.try_grant_role(&user, &non_whitelisted, &WHITELISTED_ROLE);

    let is_failure = match result {
        Ok(Ok(_)) => false,
        Ok(Err(_)) => true,
        Err(_) => true,
    };
    assert!(is_failure, "Expected function to fail, but it succeeded");
}

#[test]
fn test_grant_role_non_whitelister_role_should_not_be_able_to_grant_whitelisted_role_case_2() {
    let e = Env::default();
    e.mock_all_auths();

    let admin: Address = Address::generate(&e);
    let contract_address = e.register(
        PermissionManager,
        PermissionManagerArgs::__constructor(&admin.clone()),
    );
    let client = PermissionManagerClient::new(&e, &contract_address);
    let user: Address = Address::generate(&e);
    let non_whitelisted: Address = Address::generate(&e);

    client.grant_role(&admin, &user, &MINTER_ROLE);
    let result = client.try_grant_role(&user, &non_whitelisted, &WHITELISTED_ROLE);

    let is_failure = match result {
        Ok(Ok(_)) => false,
        Ok(Err(_)) => true,
        Err(_) => true,
    };
    assert!(is_failure, "Expected function to fail, but it succeeded");
}
