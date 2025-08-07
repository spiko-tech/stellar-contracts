// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.4.1

//! # Security
//!
//! For security issues, please contact: tech@spiko.tech

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Symbol};
use stellar_access::ownable::{self as ownable, Ownable};
use stellar_contract_utils::pausable::{self as pausable, Pausable};
use stellar_contract_utils::upgradeable::UpgradeableInternal;
use stellar_macros::{default_impl, only_owner, only_role, when_not_paused, Upgradeable};
use stellar_tokens::fungible::{Base, FungibleToken};

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
impl Ownable for Token {}

/*
    fn mint(ref self: TContractState, recipient: ContractAddress, amount: u256);
    fn burn(ref self: TContractState, amount: u256);
    fn pause(ref self: TContractState);
    fn unpause(ref self: TContractState);
    fn redeem(ref self: TContractState, amount: u256, salt: felt252);
*/
