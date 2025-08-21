// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.4.1

//! # Security
//!
//! For security issues, please contact: tech@spiko.tech

use soroban_sdk::{
    contract, contractclient, contractimpl, contracttype, symbol_short, Address, Env, String,
    Symbol, Vec,
};
use stellar_access::ownable::{self as ownable, Ownable};
use stellar_contract_utils::pausable::{self as pausable, Pausable};
use stellar_contract_utils::upgradeable::UpgradeableInternal;
use stellar_macros::{default_impl, only_owner, when_not_paused, Upgradeable};
use stellar_tokens::fungible::{Base, FungibleToken};

use contracts_utils::role::{BURNER_ROLE, MINTER_ROLE, PAUSER_ROLE, WHITELISTED_ROLE};

#[contractclient(name = "PermissionManagerClient")]
pub trait PermissionManagerInterface {
    fn has_role(account: &Address, role: &Symbol) -> Option<u32>;
}

#[contractclient(name = "RedemptionClient")]
pub trait RedemptionInterface {
    fn on_redeem(e: &Env, token: Address, from: Address, amount: i128, salt: u128);
}

#[derive(Upgradeable)]
#[contract]
pub struct Token;

pub const PERMISSION_MANAGER_KEY: Symbol = symbol_short!("PERM");
pub const REDEMPTION_KEY: Symbol = symbol_short!("REDEMP");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintBatchOperation(pub Address, pub i128);

#[contractimpl]
impl Token {
    pub fn __constructor(e: &Env, owner: Address, name: String, symbol: String, decimals: u32) {
        Base::set_metadata(e, decimals, name, symbol);
        ownable::set_owner(e, &owner);
    }

    fn assert_has_role(e: &Env, account: &Address, role: &Symbol) {
        let permission_manager: Address = e
            .storage()
            .persistent()
            .get(&PERMISSION_MANAGER_KEY)
            .expect("Permission manager not set");
        let client: PermissionManagerClient<'_> =
            PermissionManagerClient::new(e, &permission_manager);
        assert!(client.has_role(account, &role).is_some(), "Invalid role");
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
        Self::assert_has_role(e, &caller, &MINTER_ROLE);
        Self::assert_has_role(e, &account, &WHITELISTED_ROLE);
        Base::mint(e, &account, amount);
    }

    #[when_not_paused]
    pub fn mint_batch(e: &Env, operations: Vec<MintBatchOperation>, caller: Address) {
        caller.require_auth();
        Self::assert_has_role(e, &caller, &MINTER_ROLE);

        for operation in &operations {
            let account = operation.0;
            Self::assert_has_role(e, &account, &WHITELISTED_ROLE);
        }

        for operation in &operations {
            let account = operation.0;
            let amount = operation.1;
            Base::mint(e, &account, amount);
        }
    }

    #[when_not_paused]
    pub fn burn(e: &Env, account: Address, amount: i128) {
        Self::assert_has_role(e, &account, &BURNER_ROLE);

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

    #[when_not_paused]
    pub fn redeem(e: &Env, amount: i128, salt: u128, caller: Address) {
        assert!(amount > 0, "Redemption amount should be more than zero");
        Self::assert_has_role(e, &caller, &WHITELISTED_ROLE);

        let redemption: Address = e
            .storage()
            .persistent()
            .get(&REDEMPTION_KEY)
            .expect("Redemption not set");
        let client: RedemptionClient<'_> = RedemptionClient::new(e, &redemption);
        Self::assert_has_role(e, &redemption, &WHITELISTED_ROLE);

        Base::transfer(e, &caller, &redemption, amount);
        client.on_redeem(&e.current_contract_address(), &caller, &amount, &salt);
    }
}

#[default_impl]
#[contractimpl]
impl FungibleToken for Token {
    type ContractType = Base;

    #[when_not_paused]
    fn transfer(e: &Env, from: Address, to: Address, amount: i128) {
        Self::assert_has_role(e, &from, &WHITELISTED_ROLE);
        Self::assert_has_role(e, &to, &WHITELISTED_ROLE);
        Base::transfer(e, &from, &to, amount);
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
        Self::assert_has_role(e, &caller, &PAUSER_ROLE);
        pausable::pause(e);
    }

    fn unpause(e: &Env, caller: Address) {
        caller.require_auth();
        Self::assert_has_role(e, &caller, &PAUSER_ROLE);
        pausable::unpause(e);
    }
}

#[default_impl]
#[contractimpl]
impl Ownable for Token {}
