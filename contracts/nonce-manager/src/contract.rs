use soroban_sdk::{contract, contractimpl, Address, Env};
use stellar_access::ownable::{self as ownable, Ownable};
use stellar_contract_utils::upgradeable::UpgradeableInternal;
use stellar_macros::{default_impl, Upgradeable};

#[derive(Upgradeable)]
#[contract]
pub struct NonceManager;

#[contractimpl]
impl NonceManager {
    pub fn __constructor(e: &Env, owner: Address) {
        ownable::set_owner(e, &owner);
    }

    pub fn get_nonce(e: &Env, user: Address) -> u128 {
        e.storage().instance().get(&user).unwrap_or(0u128)
    }

    pub fn consume_nonce(e: &Env, user: Address, nonce: u128) {
        let current_nonce = e.storage().instance().get(&user).unwrap_or(0u128);
        assert!(current_nonce == nonce);

        let new_nonce = current_nonce + 1;
        e.storage().instance().set(&user, &new_nonce);
    }
}

#[default_impl]
#[contractimpl]
impl Ownable for NonceManager {}

impl UpgradeableInternal for NonceManager {
    fn _require_auth(e: &Env, operator: &Address) {
        operator.require_auth();
        let owner = ownable::get_owner(e).expect("Owner not set");
        if *operator != owner {
            panic!("Only owner can call this function");
        }
    }
}
