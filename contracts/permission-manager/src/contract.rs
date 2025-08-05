// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.4.1

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};
use stellar_access::access_control::{self as access_control, AccessControl};
use stellar_contract_utils::upgradeable::UpgradeableInternal;
use stellar_macros::{default_impl, Upgradeable};

pub const MINTER_ROLE: Symbol = symbol_short!("0");
pub const PAUSER_ROLE: Symbol = symbol_short!("1");
pub const BURNER_ROLE: Symbol = symbol_short!("2");
pub const WHITELISTER_ROLE: Symbol = symbol_short!("3");
pub const WHITELISTED_ROLE: Symbol = symbol_short!("4");
pub const REDEMPTION_EXECUTOR_ROLE: Symbol = symbol_short!("5");

#[derive(Upgradeable)]
#[contract]
pub struct PermissionManager;

#[contractimpl]
impl PermissionManager {
    pub fn __constructor(e: &Env, admin: Address) {
        access_control::set_admin(e, &admin);
        access_control::set_role_admin(e, &WHITELISTED_ROLE, &WHITELISTER_ROLE);
    }
}

#[default_impl]
#[contractimpl]
impl AccessControl for PermissionManager {
    fn renounce_role(e: &Env, caller: Address, role: Symbol) {
        if role == WHITELISTED_ROLE {
            panic!("Cannot renounce whitelisted role");
        }
        access_control::renounce_role(e, &caller, &role);
    }
}

impl UpgradeableInternal for PermissionManager {
    fn _require_auth(e: &Env, operator: &Address) {
        access_control::ensure_role(e, operator, &Symbol::new(e, "admin"));
        operator.require_auth();
    }
}
