use soroban_sdk::{
    contract, contractclient, contractimpl, contracttype, crypto::Hash, symbol_short, xdr::ToXdr,
    Address, Bytes, Env, String, Symbol, Vec,
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
    fn burn(account: Address, amount: i128);
    fn transfer(from: Address, to: Address, amount: i128);
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
pub struct RedemptionEntry(pub Address, pub Address, pub i128, pub String);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RedemptionStatus {
    Null,
    Pending,
    Executed,
    Canceled,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecuteRedemptionOperation(pub Address, pub Address, pub i128, pub String);

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

    fn compute_redemption_hash(
        e: &Env,
        token: &Address,
        from: &Address,
        amount: i128,
        salt: &String,
    ) -> Hash<32> {
        let mut redemption_entry_serialized: Bytes = token.clone().to_xdr(&e);
        redemption_entry_serialized.append(&from.clone().to_xdr(&e));
        redemption_entry_serialized.append(&amount.to_xdr(&e));
        redemption_entry_serialized.append(&salt.clone().to_xdr(&e));
        e.crypto().sha256(&redemption_entry_serialized)
    }

    pub fn on_redeem(e: &Env, token: Address, from: Address, amount: i128, salt: String) {
        token.require_auth();
        Self::assert_token_registered(e, &token);

        let redemption_hash = Self::compute_redemption_hash(e, &token, &from, amount, &salt);

        let redemption_status: RedemptionStatus = e
            .storage()
            .persistent()
            .get(&redemption_hash)
            .unwrap_or(RedemptionStatus::Null);
        assert!(
            redemption_status == RedemptionStatus::Null,
            "Redemption already exists"
        );

        e.storage()
            .persistent()
            .set(&redemption_hash, &RedemptionStatus::Pending);
        e.events().publish(
            (REDEMPTION_EVENT, REDEMPTION_INITIATED_EVENT),
            RedemptionEntry(token, from, amount, salt),
        );
    }

    pub fn execute_redemptions(
        e: &Env,
        caller: Address,
        operations: Vec<ExecuteRedemptionOperation>,
    ) {
        caller.require_auth();
        Self::assert_has_role(e, &caller, &REDEMPTION_EXECUTOR_ROLE);

        // check all operations are valid
        for operation in &operations {
            let token = operation.0;
            let from = operation.1;
            let amount = operation.2;
            let salt = operation.3;

            Self::assert_token_registered(e, &token);

            let redemption_hash = Self::compute_redemption_hash(e, &token, &from, amount, &salt);

            let redemption_status: RedemptionStatus = e
                .storage()
                .persistent()
                .get(&redemption_hash)
                .unwrap_or(RedemptionStatus::Null);
            assert!(
                redemption_status == RedemptionStatus::Pending,
                "Redemption not pending"
            );
        }

        for operation in &operations {
            let token = operation.0;
            let from = operation.1;
            let amount = operation.2;
            let salt = operation.3;

            let client: TokenClient<'_> = TokenClient::new(e, &token);

            let redemption_hash = Self::compute_redemption_hash(e, &token, &from, amount, &salt);

            client.burn(&from, &amount);
            e.storage()
                .persistent()
                .set(&redemption_hash, &RedemptionStatus::Executed);
            e.events().publish(
                (REDEMPTION_EVENT, REDEMPTION_EXECUTED_EVENT),
                RedemptionEntry(token, from, amount, salt),
            );
        }
    }

    pub fn cancel_redemption(
        e: &Env,
        caller: Address,
        token: Address,
        from: Address,
        amount: i128,
        salt: String,
    ) {
        caller.require_auth();
        Self::assert_has_role(e, &caller, &REDEMPTION_EXECUTOR_ROLE);
        Self::assert_token_registered(e, &token);

        let client: TokenClient<'_> = TokenClient::new(e, &token);

        let redemption_hash = Self::compute_redemption_hash(e, &token, &from, amount, &salt);

        let redemption_status: RedemptionStatus = e
            .storage()
            .persistent()
            .get(&redemption_hash)
            .unwrap_or(RedemptionStatus::Null);
        assert!(
            redemption_status == RedemptionStatus::Pending,
            "Redemption not pending"
        );

        client.transfer(&e.current_contract_address(), &from, &amount);
        e.storage()
            .persistent()
            .set(&redemption_hash, &RedemptionStatus::Canceled);
        e.events().publish(
            (REDEMPTION_EVENT, REDEMPTION_CANCELLED_EVENT),
            RedemptionEntry(token, from, amount, salt),
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
