// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.4.1

use soroban_sdk::{
    contract, contractclient, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
};
use stellar_access::ownable::{self as ownable, Ownable};
use stellar_contract_utils::upgradeable::{Upgradeable, UpgradeableInternal};
use stellar_macros::{default_impl, only_owner, Upgradeable};

use contracts_utils::role::REDEMPTION_EXECUTOR_ROLE;

#[contractclient(name = "PermissionManagerClient")]
pub trait PermissionManagerInterface {
    fn has_role(account: &Address, role: &Symbol) -> Option<u32>;
}

#[contractclient(name = "TokenClient")]
pub trait TokenInterface {
    fn burn(e: &Env, account: Address, amount: i128);
}

#[derive(Upgradeable)]
#[contract]
pub struct Redemption;

pub const PERMISSION_MANAGER_KEY: Symbol = symbol_short!("PERM");

pub const REDEMPTION_EVENT: Symbol = symbol_short!("REDEEM");
pub const REDEMPTION_INITIATED_EVENT: Symbol = symbol_short!("INIT");
pub const REDEMPTION_EXECUTED_EVENT: Symbol = symbol_short!("EXEC");
pub const REDEMPTION_CANCELLED_EVENT: Symbol = symbol_short!("CANCEL");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedemptionEntry(pub Address, pub Address, pub i128, pub u128);

#[contractimpl]
impl Redemption {
    pub fn __constructor(e: &Env, owner: Address) {
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
        assert!(client.has_role(account, role).is_some(), "Invalid role");
    }

    fn assert_token_registered(e: &Env, token: &Address) {
        let token_set: bool = e
            .storage()
            .persistent()
            .get(&token)
            .expect("Caller should be token contract");
        assert!(token_set, "Caller should be token contract");
    }

    #[only_owner]
    pub fn add_token(e: &Env, token_contract_address: Address) {
        e.storage().persistent().set(&token_contract_address, &true);
    }

    #[only_owner]
    pub fn remove_token(e: &Env, token_contract_address: Address) {
        e.storage().persistent().remove(&token_contract_address);
    }

    #[only_owner]
    pub fn set_permission_manager(e: &Env, permission_manager: Address) {
        e.storage()
            .persistent()
            .set(&PERMISSION_MANAGER_KEY, &permission_manager);
    }

    pub fn on_redeem(e: &Env, token: Address, from: Address, amount: i128, salt: u128) {
        token.require_auth();
        Self::assert_token_registered(e, &token);

        let previous_redemption: Option<RedemptionEntry> = e.storage().persistent().get(&salt);
        assert!(previous_redemption.is_none(), "Redemption already exists");

        let redemption_entry = RedemptionEntry(token, from, amount, salt);
        e.storage().persistent().set(&salt, &redemption_entry);
        e.events().publish(
            (REDEMPTION_EVENT, REDEMPTION_INITIATED_EVENT),
            redemption_entry,
        );
    }

    pub fn execute_redemption(
        e: &Env,
        caller: Address,
        token: Address,
        from: Address,
        amount: i128,
        salt: u128,
    ) {
        caller.require_auth();
        Self::assert_has_role(e, &caller, &REDEMPTION_EXECUTOR_ROLE);
        Self::assert_token_registered(e, &token);

        let client: TokenClient<'_> = TokenClient::new(e, &token);

        let redemption_entry: RedemptionEntry = e
            .storage()
            .persistent()
            .get(&salt)
            .expect("Redemption does not exist");
        client.burn(&from, &amount);
        e.storage().persistent().remove(&salt);
        e.events().publish(
            (REDEMPTION_EVENT, REDEMPTION_EXECUTED_EVENT),
            redemption_entry,
        );
    }

    pub fn cancel_redemption(
        e: &Env,
        caller: Address,
        token: Address,
        _from: Address,
        _amount: i128,
        salt: u128,
    ) {
        caller.require_auth();
        Self::assert_has_role(e, &caller, &REDEMPTION_EXECUTOR_ROLE);
        Self::assert_token_registered(e, &token);

        let redemption_entry: RedemptionEntry = e
            .storage()
            .persistent()
            .get(&salt)
            .expect("Redemption does not exist");
        // TODO: add transfer from token contract
        e.storage().persistent().remove(&salt);
        e.events().publish(
            (REDEMPTION_EVENT, REDEMPTION_CANCELLED_EVENT),
            redemption_entry,
        );
    }
}

#[default_impl]
#[contractimpl]
impl Ownable for Redemption {}

impl UpgradeableInternal for Redemption {
    fn _require_auth(e: &Env, operator: &Address) {
        operator.require_auth();
        let owner = ownable::get_owner(e).expect("Owner not set");
        if *operator != owner {
            panic!("Only owner can call this function");
        }
    }
}
