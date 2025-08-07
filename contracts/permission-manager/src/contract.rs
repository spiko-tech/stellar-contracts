// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.4.1

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};
use stellar_access::access_control::{self as access_control, AccessControl};
use stellar_contract_utils::upgradeable::UpgradeableInternal;
use stellar_macros::{default_impl, Upgradeable};

use contracts_utils::role::{WHITELISTED_ROLE, WHITELISTER_ROLE};

#[derive(Upgradeable)]
#[contract]
pub struct PermissionManager;

#[contractimpl]
impl PermissionManager {
    pub fn __constructor(e: &Env, admin: Address) {
        access_control::set_admin(e, &admin);
    }

    pub fn initialize(e: &Env) {
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
        operator.require_auth();
        let admin = access_control::get_admin(e).expect("Admin not set");
        if *operator != admin {
            panic!("Only admin can call this function");
        }
    }
}
