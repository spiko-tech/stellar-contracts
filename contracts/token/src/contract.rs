// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.4.1

//! # Security
//!
//! For security issues, please contact: tech@spiko.tech

use soroban_sdk::{
    contract, contractclient, contractimpl, symbol_short, Address, Env, String, Symbol,
};
use stellar_access::ownable::{self as ownable, Ownable};
use stellar_contract_utils::pausable::{self as pausable, Pausable};
use stellar_contract_utils::upgradeable::UpgradeableInternal;
use stellar_macros::{default_impl, only_owner, when_not_paused, Upgradeable};
use stellar_tokens::fungible::burnable::FungibleBurnable;
use stellar_tokens::fungible::{Base, FungibleToken};

use contracts_utils::role::{BURNER_ROLE, MINTER_ROLE, PAUSER_ROLE, WHITELISTED_ROLE};

#[contractclient(name = "PermissionManagerClient")]
pub trait PermissionManagerInterface {
    fn has_role(account: &Address, role: &Symbol) -> Option<u32>;
}

#[contractclient(name = "RedemptionClient")]
pub trait RedemptionInterface {
    fn on_redeem(e: &Env, token: Address, from: Address, amount: u128, salt: u128);
}

#[derive(Upgradeable)]
#[contract]
pub struct Token;

pub const PERMISSION_MANAGER_KEY: Symbol = symbol_short!("PERM");
pub const REDEMPTION_KEY: Symbol = symbol_short!("REDEMP");

#[contractimpl]
impl Token {
    pub fn __constructor(e: &Env, owner: Address, name: String, symbol: String, decimals: u32) {
        Base::set_metadata(e, decimals, name, symbol);
        ownable::set_owner(e, &owner);
    }

    #[only_owner]
    pub fn set_permission_manager(e: &Env, permission_manager: Address) {
        e.storage()
            .persistent()
            .set(&PERMISSION_MANAGER_KEY, &permission_manager);
    }

    #[only_owner]
    pub fn set_redemption(e: &Env, redemption: Address) {
        e.storage().persistent().set(&REDEMPTION_KEY, &redemption);
    }

    #[when_not_paused]
    pub fn mint(e: &Env, account: Address, amount: i128, caller: Address) {
        caller.require_auth();

        let permission_manager: Address = e
            .storage()
            .persistent()
            .get(&PERMISSION_MANAGER_KEY)
            .expect("Permission manager not set");

        let client: PermissionManagerClient<'_> =
            PermissionManagerClient::new(e, &permission_manager);
        assert!(
            client.has_role(&caller, &MINTER_ROLE).is_some(),
            "Caller should have minter role"
        );

        Base::mint(e, &account, amount);
    }

    #[when_not_paused]
    pub fn redeem(e: &Env, amount: u128, salt: u128, caller: Address) {
        caller.require_auth();

        assert!(amount > 0, "Redemption amount should be more than zero");

        let permission_manager: Address = e
            .storage()
            .persistent()
            .get(&PERMISSION_MANAGER_KEY)
            .expect("Permission manager not set");
        let client: PermissionManagerClient<'_> =
            PermissionManagerClient::new(e, &permission_manager);
        assert!(
            client.has_role(&caller, &WHITELISTED_ROLE).is_some(),
            "Caller should have whitelisted role"
        );

        let redemption: Address = e
            .storage()
            .persistent()
            .get(&REDEMPTION_KEY)
            .expect("Redemption not set");
        let client: RedemptionClient<'_> = RedemptionClient::new(e, &redemption);
        Self::transfer(e, caller.clone(), redemption, amount as i128);
        client.on_redeem(&e.current_contract_address(), &caller, &amount, &salt);
    }
}

#[default_impl]
#[contractimpl]
impl FungibleToken for Token {
    type ContractType = Base;

    #[when_not_paused]
    fn transfer(e: &Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let permission_manager: Address = e
            .storage()
            .persistent()
            .get(&PERMISSION_MANAGER_KEY)
            .expect("Permission manager not set");
        let client: PermissionManagerClient<'_> =
            PermissionManagerClient::new(e, &permission_manager);
        assert!(
            client.has_role(&from, &WHITELISTED_ROLE).is_some(),
            "From address should have whitelisted role"
        );
        assert!(
            client.has_role(&to, &WHITELISTED_ROLE).is_some(),
            "To address should have whitelisted role"
        );

        Base::transfer(e, &from, &to, amount);
    }

    #[when_not_paused]
    fn transfer_from(e: &Env, spender: Address, from: Address, to: Address, amount: i128) {
        let permission_manager: Address = e
            .storage()
            .persistent()
            .get(&PERMISSION_MANAGER_KEY)
            .expect("Permission manager not set");
        let client: PermissionManagerClient<'_> =
            PermissionManagerClient::new(e, &permission_manager);
        assert!(
            client.has_role(&from, &WHITELISTED_ROLE).is_some(),
            "From address should have whitelisted role"
        );
        assert!(
            client.has_role(&to, &WHITELISTED_ROLE).is_some(),
            "To address should have whitelisted role"
        );

        Base::transfer_from(e, &spender, &from, &to, amount);
    }
}

#[default_impl]
#[contractimpl]
impl FungibleBurnable for Token {
    #[when_not_paused]
    fn burn(e: &Env, account: Address, amount: i128) {
        account.require_auth();

        let permission_manager: Address = e
            .storage()
            .persistent()
            .get(&PERMISSION_MANAGER_KEY)
            .expect("Permission manager not set");
        let client: PermissionManagerClient<'_> =
            PermissionManagerClient::new(e, &permission_manager);
        assert!(
            client.has_role(&account, &BURNER_ROLE).is_some(),
            "Account should have burner role"
        );

        let redemption: Address = e
            .storage()
            .persistent()
            .get(&REDEMPTION_KEY)
            .expect("Redemption not set");
        assert!(
            redemption == account,
            "Only tokens on redemption contract can be burned"
        );

        Base::burn(e, &account, amount);
    }
}

impl UpgradeableInternal for Token {
    fn _require_auth(e: &Env, operator: &Address) {
        operator.require_auth();
        let owner = ownable::get_owner(e).expect("Owner not set");
        if *operator != owner {
            panic!("Only owner can call this function");
        }
    }
}

#[contractimpl]
impl Pausable for Token {
    fn paused(e: &Env) -> bool {
        pausable::paused(e)
    }

    fn pause(e: &Env, caller: Address) {
        caller.require_auth();

        let permission_manager: Address = e
            .storage()
            .persistent()
            .get(&PERMISSION_MANAGER_KEY)
            .expect("Permission manager not set");

        let client: PermissionManagerClient<'_> =
            PermissionManagerClient::new(e, &permission_manager);
        assert!(
            client.has_role(&caller, &PAUSER_ROLE).is_some(),
            "Caller should have pauser role"
        );

        pausable::pause(e);
    }

    fn unpause(e: &Env, caller: Address) {
        caller.require_auth();

        let permission_manager: Address = e
            .storage()
            .persistent()
            .get(&PERMISSION_MANAGER_KEY)
            .expect("Permission manager not set");

        let client: PermissionManagerClient<'_> =
            PermissionManagerClient::new(e, &permission_manager);
        assert!(
            client.has_role(&caller, &PAUSER_ROLE).is_some(),
            "Caller should have pauser role"
        );

        pausable::unpause(e);
    }
}

#[default_impl]
#[contractimpl]
impl Ownable for Token {}

/*
    fn redeem(ref self: TContractState, amount: u256, salt: felt252);
*/
