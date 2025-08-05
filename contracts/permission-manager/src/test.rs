#![cfg(test)]

use super::contract::PermissionManager;
use soroban_sdk::{Address, Env, Symbol};

#[test]
fn test_constructor() {
    let env = Env::default();
    let admin = Address::generate(&env);
    
    let contract = PermissionManager::new(&env);
    contract.__constructor(&admin);
    
    // Test that admin has admin role
    assert!(contract.has_role(&"admin".to_string(), &admin));
}

#[test]
fn test_grant_and_revoke_role() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    let contract = PermissionManager::new(&env);
    contract.__constructor(&admin);
    
    // Grant role
    contract.grant_role(&"MINTER_ROLE".to_string(), &user);
    assert!(contract.has_role(&"MINTER_ROLE".to_string(), &user));
    
    // Revoke role
    contract.revoke_role(&"MINTER_ROLE".to_string(), &user);
    assert!(!contract.has_role(&"MINTER_ROLE".to_string(), &user));
}

#[test]
fn test_renounce_role() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    let contract = PermissionManager::new(&env);
    contract.__constructor(&admin);
    
    // Grant role
    contract.grant_role(&"MINTER_ROLE".to_string(), &user);
    assert!(contract.has_role(&"MINTER_ROLE".to_string(), &user));#![cfg(test)]

    use super::contract::PermissionManager;
    use soroban_sdk::{Address, Env, Symbol};
    
    #[test]
    fn test_constructor() {
        let env = Env::default();
        let admin = Address::generate(&env);
        
        let contract = PermissionManager::new(&env);
        contract.__constructor(&admin);
        
        // Test that admin has admin role
        assert!(contract.has_role(&"admin".to_string(), &admin));
    }
    
    #[test]
    fn test_grant_and_revoke_role() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        
        let contract = PermissionManager::new(&env);
        contract.__constructor(&admin);
        
        // Grant role
        contract.grant_role(&"MINTER_ROLE".to_string(), &user);
        assert!(contract.has_role(&"MINTER_ROLE".to_string(), &user));
        
        // Revoke role
        contract.revoke_role(&"MINTER_ROLE".to_string(), &user);
        assert!(!contract.has_role(&"MINTER_ROLE".to_string(), &user));
    }
    
    #[test]
    fn test_renounce_role() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        
        let contract = PermissionManager::new(&env);
        contract.__constructor(&admin);
        
        // Grant role
        contract.grant_role(&"MINTER_ROLE".to_string(), &user);
        assert!(contract.has_role(&"MINTER_ROLE".to_string(), &user));
        
        // Renounce role
        contract.renounce_role(&"MINTER_ROLE".to_string(), &user);
        assert!(!contract.has_role(&"MINTER_ROLE".to_string(), &user));
    }
    
    #[test]
    #[should_panic(expected = "Cannot renounce whitelisted role")]
    fn test_cannot_renounce_whitelisted_role() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        
        let contract = PermissionManager::new(&env);
        contract.__constructor(&admin);
        
        // Grant whitelisted role
        contract.grant_role(&"WHITELISTED_ROLE".to_string(), &user);
        
        // Try to renounce whitelisted role - should panic
        contract.renounce_role(&"WHITELISTED_ROLE".to_string(), &user);
    } 
    
    // Renounce role
    contract.renounce_role(&"MINTER_ROLE".to_string(), &user);
    assert!(!contract.has_role(&"MINTER_ROLE".to_string(), &user));
}

#[test]
#[should_panic(expected = "Cannot renounce whitelisted role")]
fn test_cannot_renounce_whitelisted_role() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    let contract = PermissionManager::new(&env);
    contract.__constructor(&admin);
    
    // Grant whitelisted role
    contract.grant_role(&"WHITELISTED_ROLE".to_string(), &user);
    
    // Try to renounce whitelisted role - should panic
    contract.renounce_role(&"WHITELISTED_ROLE".to_string(), &user);
} 
