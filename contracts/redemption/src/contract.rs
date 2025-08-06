// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.4.1

use soroban_sdk::{contract, contractclient, contractimpl, symbol_short, Address, Env, Symbol};
use stellar_access::ownable::{self as ownable, Ownable};
use stellar_contract_utils::upgradeable::{Upgradeable, UpgradeableInternal};
use stellar_macros::{default_impl, only_owner, Upgradeable};

#[contractclient(name = "PermissionManagerClient")]
pub trait PermissionManagerInterface {
    fn has_role(account: &Address, role: &Symbol) -> Option<u32>;
}

#[derive(Upgradeable)]
#[contract]
pub struct Redemption;

pub const PERMISSION_MANAGER_KEY: Symbol = symbol_short!("PERM");

#[contractimpl]
impl Redemption {
    pub fn __constructor(e: &Env, owner: Address) {
        ownable::set_owner(e, &owner);
    }

    #[only_owner]
    pub fn add_token(e: &Env, token_contract_address: Address) {
        e.storage().persistent().set(&token_contract_address, &true);
    }

    #[only_owner]
    pub fn remove_token(e: &Env, token_contract_address: Address) {
        e.storage()
            .persistent()
            .set(&token_contract_address, &false);
    }

    #[only_owner]
    pub fn set_permission_manager(e: &Env, permission_manager: Address) {
        e.storage()
            .persistent()
            .set(&PERMISSION_MANAGER_KEY, &permission_manager);
    }

    fn assert_has_role(e: &Env, role: Symbol, address: Address) {
        let permission_manager: Address = e
            .storage()
            .persistent()
            .get(&PERMISSION_MANAGER_KEY)
            .expect("Permission manager not set");

        let client = PermissionManagerClient::new(&e, &permission_manager);

        match client.has_role(&address, &role) {
            Some(0) => {}
            _ => {
                panic!("Wrong role: role should be {}", role)
            }
        }
    }

    pub fn on_redeem(e: &Env, token: Address, from: Address, amount: u128, salt: u128) {
        let permission_manager: Address = e
            .storage()
            .persistent()
            .get(&PERMISSION_MANAGER_KEY)
            .expect("Permission manager not set");
    }
}

#[default_impl]
#[contractimpl]
impl Ownable for Redemption {}

impl UpgradeableInternal for Redemption {
    fn _require_auth(e: &Env, operator: &Address) {
        operator.require_auth();

        match ownable::get_owner(e) {
            Some(owner) => {
                if *operator != owner {
                    panic!("Only owner can call this function");
                }
            }
            None => {
                panic!("Owner not set");
            }
        }
    }
}

/*

fn on_redeem(
        ref self: TContractState,
        token: ContractAddress,
        from: ContractAddress,
        amount: u256,
        salt: felt252
    );
    fn execute_redemption(
        ref self: TContractState,
        token: ContractAddress,
        from: ContractAddress,
        amount: u256,
        salt: felt252
    );
    fn cancel_redemption(
        ref self: TContractState,
        token: ContractAddress,
        from: ContractAddress,
        amount: u256,
        salt: felt252
    );

    */
