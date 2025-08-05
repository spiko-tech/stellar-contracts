// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.4.1

//! # Security
//!
//! For security issues, please contact: tech@spiko.tech

use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol};
use stellar_access::access_control::{self as access_control, AccessControl};
use stellar_contract_utils::pausable::{self as pausable, Pausable};
use stellar_contract_utils::upgradeable::UpgradeableInternal;
use stellar_macros::{default_impl, only_role, when_not_paused, Upgradeable};
use stellar_tokens::fungible::{Base, FungibleToken};

#[derive(Upgradeable)]
#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    pub fn __constructor(
        e: &Env,
        admin: Address,
        pauser: Address,
        upgrader: Address,
        minter: Address,
    ) {
        Base::set_metadata(
            e,
            18,
            String::from_str(e, "Token"),
            String::from_str(e, "EUTBL"),
        );
        access_control::set_admin(e, &admin);
        access_control::grant_role_no_auth(e, &admin, &pauser, &Symbol::new(e, "pauser"));
        access_control::grant_role_no_auth(e, &admin, &upgrader, &Symbol::new(e, "upgrader"));
        access_control::grant_role_no_auth(e, &admin, &minter, &Symbol::new(e, "minter"));
    }

    #[only_role(caller, "minter")]
    #[when_not_paused]
    pub fn mint(e: &Env, account: Address, amount: i128, caller: Address) {
        Base::mint(e, &account, amount);
    }
}

#[default_impl]
#[contractimpl]
impl FungibleToken for Token {
    type ContractType = Base;

    #[when_not_paused]
    fn transfer(e: &Env, from: Address, to: Address, amount: i128) {
        Self::ContractType::transfer(e, &from, &to, amount);
    }

    #[when_not_paused]
    fn transfer_from(e: &Env, spender: Address, from: Address, to: Address, amount: i128) {
        Self::ContractType::transfer_from(e, &spender, &from, &to, amount);
    }
}

//
// Utils
//

impl UpgradeableInternal for Token {
    fn _require_auth(e: &Env, operator: &Address) {
        access_control::ensure_role(e, operator, &Symbol::new(e, "upgrader"));
        operator.require_auth();
    }
}

#[contractimpl]
impl Pausable for Token {
    fn paused(e: &Env) -> bool {
        pausable::paused(e)
    }

    #[only_role(caller, "pauser")]
    fn pause(e: &Env, caller: Address) {
        pausable::pause(e);
    }

    #[only_role(caller, "pauser")]
    fn unpause(e: &Env, caller: Address) {
        pausable::unpause(e);
    }
}

#[default_impl]
#[contractimpl]
impl AccessControl for Token {}
