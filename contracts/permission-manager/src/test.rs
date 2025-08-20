#![cfg(test)]

extern crate std;

use super::contract::{PermissionManager, PermissionManagerArgs, PermissionManagerClient};
use soroban_sdk::{contract, testutils::Address as _, Address, Env, Vec};

use contracts_utils::role::{MINTER_ROLE, WHITELISTED_ROLE, WHITELISTER_ROLE};

#[contract]
struct MockContract;

fn setup_env() -> Env {
    let e: Env = Env::default();
    e.mock_all_auths();
    e
}

fn deploy_permission_manager(e: &Env) -> (Address, PermissionManagerClient) {
    let admin: Address = Address::generate(e);
    let contract_address = e.register(
        PermissionManager,
        PermissionManagerArgs::__constructor(&admin.clone()),
    );
    let client = PermissionManagerClient::new(e, &contract_address);
    client.initialize();

    (admin, client)
}

#[test]
fn test_admin_setup() {
    let e = setup_env();
    let (admin, client) = deploy_permission_manager(&e);

    let fetched_admin = client.get_admin();

    assert_eq!(fetched_admin, Some(admin));
}

#[test]
fn test_has_role_should_return_some_when_user_has_role() {
    let e = setup_env();
    let (admin, client) = deploy_permission_manager(&e);
    let user: Address = Address::generate(&e);

    client.grant_role(&admin, &user, &WHITELISTED_ROLE);
    let has_role = client.has_role(&user, &WHITELISTED_ROLE);

    assert_eq!(has_role, Some(0));
}

#[test]
fn test_has_role_should_return_none_when_user_does_not_have_role() {
    let e = setup_env();
    let (_admin, client) = deploy_permission_manager(&e);
    let user: Address = Address::generate(&e);

    let has_role = client.has_role(&user, &WHITELISTED_ROLE);

    assert_eq!(has_role, None);
}

#[test]
fn test_grant_role_whitelister_role_should_be_able_to_grant_whitelisted_role() {
    let e = setup_env();
    let (admin, client) = deploy_permission_manager(&e);
    let whitelister: Address = Address::generate(&e);
    let whitelisted: Address = Address::generate(&e);

    client.grant_role(&admin, &whitelister, &WHITELISTER_ROLE);
    client.grant_role(&whitelister, &whitelisted, &WHITELISTED_ROLE);
    let has_role_whitelisted: Option<u32> = client.has_role(&whitelisted, &WHITELISTED_ROLE);

    assert_eq!(has_role_whitelisted, Some(0));
}

#[test]
fn test_grant_role_non_whitelister_role_should_not_be_able_to_grant_whitelisted_role_case_1() {
    let e = setup_env();
    let (_admin, client) = deploy_permission_manager(&e);
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
    let e = setup_env();
    let (admin, client) = deploy_permission_manager(&e);
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

#[test]
fn test_grant_role_batch_whitelister_role_should_be_able_to_grant_whitelisted_roles() {
    let e = setup_env();
    let (admin, client) = deploy_permission_manager(&e);
    let whitelister: Address = Address::generate(&e);
    let whitelisted1: Address = Address::generate(&e);
    let whitelisted2: Address = Address::generate(&e);
    let mut whitelisted = Vec::new(&e);
    whitelisted.push_back(whitelisted1.clone());
    whitelisted.push_back(whitelisted2.clone());

    client.grant_role(&admin, &whitelister, &WHITELISTER_ROLE);
    client.grant_role_batch(&whitelister, &whitelisted, &WHITELISTED_ROLE);

    let has_role_whitelisted1: Option<u32> = client.has_role(&whitelisted1, &WHITELISTED_ROLE);
    let has_role_whitelisted2: Option<u32> = client.has_role(&whitelisted2, &WHITELISTED_ROLE);
    assert_eq!(has_role_whitelisted1, Some(0));
    assert_eq!(has_role_whitelisted2, Some(1));
}

#[test]
fn test_grant_role_batch_non_whitelister_role_should_not_be_able_to_grant_whitelisted_role() {
    let e = setup_env();
    let (_admin, client) = deploy_permission_manager(&e);
    let non_whitelister: Address = Address::generate(&e);
    let whitelisted1: Address = Address::generate(&e);
    let whitelisted2: Address = Address::generate(&e);
    let mut whitelisted = Vec::new(&e);
    whitelisted.push_back(whitelisted1.clone());
    whitelisted.push_back(whitelisted2.clone());

    let result = client.try_grant_role_batch(&non_whitelister, &whitelisted, &WHITELISTED_ROLE);

    let is_failure = match result {
        Ok(Ok(_)) => false,
        Ok(Err(_)) => true,
        Err(_) => true,
    };
    assert!(is_failure, "Expected function to fail, but it succeeded");
}

#[test]
fn test_revoke_role_batch_whitelister_role_should_be_able_to_revoke_whitelisted_roles() {
    let e = setup_env();
    let (admin, client) = deploy_permission_manager(&e);
    let whitelister: Address = Address::generate(&e);
    let whitelisted1: Address = Address::generate(&e);
    let whitelisted2: Address = Address::generate(&e);
    client.grant_role(&admin, &whitelister, &WHITELISTER_ROLE);
    client.grant_role(&admin, &whitelisted1, &WHITELISTED_ROLE);
    client.grant_role(&admin, &whitelisted2, &WHITELISTED_ROLE);
    let mut whitelisted = Vec::new(&e);
    whitelisted.push_back(whitelisted1.clone());
    whitelisted.push_back(whitelisted2.clone());

    client.revoke_role_batch(&whitelister, &whitelisted, &WHITELISTED_ROLE);

    let has_role_whitelisted1: Option<u32> = client.has_role(&whitelisted1, &WHITELISTED_ROLE);
    let has_role_whitelisted2: Option<u32> = client.has_role(&whitelisted2, &WHITELISTED_ROLE);
    assert_eq!(has_role_whitelisted1, None);
    assert_eq!(has_role_whitelisted2, None);
}

#[test]
fn test_revoke_role_batch_non_whitelister_role_should_not_be_able_to_revoke_whitelisted_role() {
    let e = setup_env();
    let (admin, client) = deploy_permission_manager(&e);
    let non_whitelister: Address = Address::generate(&e);
    let whitelisted1: Address = Address::generate(&e);
    let whitelisted2: Address = Address::generate(&e);
    client.grant_role(&admin, &whitelisted1, &WHITELISTED_ROLE);
    client.grant_role(&admin, &whitelisted2, &WHITELISTED_ROLE);
    let mut whitelisted = Vec::new(&e);
    whitelisted.push_back(whitelisted1.clone());
    whitelisted.push_back(whitelisted2.clone());

    let result = client.try_revoke_role_batch(&non_whitelister, &whitelisted, &WHITELISTED_ROLE);

    let is_failure = match result {
        Ok(Ok(_)) => false,
        Ok(Err(_)) => true,
        Err(_) => true,
    };
    assert!(is_failure, "Expected function to fail, but it succeeded");
}
