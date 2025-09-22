use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec};
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
        access_control::set_role_admin_no_auth(e, &WHITELISTED_ROLE, &WHITELISTER_ROLE);
    }

    pub fn grant_role_batch(e: &Env, caller: Address, users: Vec<Address>, role: Symbol) {
        for (index, user) in users.iter().enumerate() {
            if index == 0 {
                access_control::grant_role(e, &caller, &user, &role);
            } else {
                access_control::grant_role_no_auth(e, &caller, &user, &role);
            }
        }
    }

    pub fn revoke_role_batch(e: &Env, caller: Address, users: Vec<Address>, role: Symbol) {
        for (index, user) in users.iter().enumerate() {
            if index == 0 {
                access_control::revoke_role(e, &caller, &user, &role);
            } else {
                access_control::revoke_role_no_auth(e, &caller, &user, &role);
            }
        }
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
