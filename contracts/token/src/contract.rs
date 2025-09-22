use soroban_sdk::{
    contract, contractclient, contractimpl, contracttype, symbol_short, Address, Env, String,
    Symbol, Vec,
};
use stellar_access::ownable::{self as ownable, Ownable};
use stellar_contract_utils::pausable::{self as pausable, Pausable};
use stellar_contract_utils::upgradeable::UpgradeableInternal;
use stellar_macros::{default_impl, only_owner, when_not_paused, Upgradeable};
use stellar_tokens::fungible::burnable::emit_burn;
use stellar_tokens::fungible::Base;

use contracts_utils::role::{BURNER_ROLE, MINTER_ROLE, PAUSER_ROLE, WHITELISTED_ROLE};

#[contractclient(name = "PermissionManagerClient")]
pub trait PermissionManagerInterface {
    fn has_role(account: &Address, role: &Symbol) -> Option<u32>;
}

#[contractclient(name = "RedemptionClient")]
pub trait RedemptionInterface {
    fn on_redeem(e: &Env, token: Address, from: Address, amount: i128, salt: String);
}

#[derive(Upgradeable)]
#[contract]
pub struct Token;

pub const PERMISSION_MANAGER_KEY: Symbol = symbol_short!("PERM");
pub const REDEMPTION_KEY: Symbol = symbol_short!("REDEMP");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintBatchOperation(pub Address, pub i128);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BurnBatchOperation(pub Address, pub i128);

const ONE_DAY_LEDGERS: u32 = 17_280;

#[contractimpl]
impl Token {
    pub fn __constructor(e: &Env, owner: Address, name: String, symbol: String, decimals: u32) {
        Base::set_metadata(e, decimals, name, symbol);
        ownable::set_owner(e, &owner);
    }

    fn assert_has_role(e: &Env, account: &Address, role: &Symbol) {
        let permission_manager: Address = e
            .storage()
            .instance()
            .get(&PERMISSION_MANAGER_KEY)
            .expect("Permission manager not set");
        let client: PermissionManagerClient<'_> =
            PermissionManagerClient::new(e, &permission_manager);
        assert!(client.has_role(account, &role).is_some(), "Invalid role");
    }

    /// Set the permission manager (central role management authority).
    ///
    /// # Arguments
    ///
    /// * `permission_manager` - The address of the permission manager.
    ///
    #[only_owner]
    pub fn set_permission_manager(e: &Env, permission_manager: Address) {
        e.storage()
            .instance()
            .set(&PERMISSION_MANAGER_KEY, &permission_manager);
    }

    /// Set the redemption (redemption contract).
    ///
    /// # Arguments
    ///
    /// * `redemption` - The address of the redemption contract.
    ///
    #[only_owner]
    pub fn set_redemption(e: &Env, redemption: Address) {
        e.storage().instance().set(&REDEMPTION_KEY, &redemption);
    }

    /// Assert that the idempotency key is not used.
    ///
    /// # Arguments
    ///
    /// * `idempotency_key` - The idempotency key. It is used to prevent duplicate calls to the same function. It is locked for 7 days.
    ///
    /// # Errors
    ///
    /// The idempotency key must not be used.
    ///
    fn assert_idempotency_key_not_used(e: &Env, idempotency_key: &String) {
        let idempotency_key_already_used: bool = e
            .storage()
            .temporary()
            .get(idempotency_key)
            .unwrap_or(false);
        assert!(
            !idempotency_key_already_used,
            "Idempotency key already used"
        );
    }

    fn consume_idempotency_key(e: &Env, idempotency_key: &String) {
        e.storage().temporary().set(idempotency_key, &true);
        e.storage()
            .temporary()
            .extend_ttl(idempotency_key, ONE_DAY_LEDGERS, ONE_DAY_LEDGERS * 7);
    }

    fn auth_mint(e: &Env, caller: Address) {
        caller.require_auth();
        Self::assert_has_role(e, &caller, &MINTER_ROLE);
    }

    /// Mint tokens to an account.
    ///
    /// # Arguments
    ///
    /// * `account` - The address of the account to mint tokens to.
    /// * `amount` - The amount of tokens to mint.
    /// * `caller` - The address of the caller.
    ///
    /// # Errors
    ///
    /// The caller must have the MINTER_ROLE.
    /// The account must have the WHITELISTED_ROLE.
    /// The amount must be greater than zero.
    ///
    #[when_not_paused]
    pub fn mint(e: &Env, account: Address, amount: i128, caller: Address) {
        Self::auth_mint(e, caller);
        Self::assert_has_role(e, &account, &WHITELISTED_ROLE);
        assert!(amount > 0, "Invalid zero-amount mint");
        Base::mint(e, &account, amount);
    }

    /// Mint tokens to a batch of accounts.
    ///
    /// # Arguments
    ///
    /// * `operations` - The operations to mint tokens to.
    /// * `caller` - The address of the caller.
    /// * `idempotency_key` - The idempotency key. It is used to prevent duplicate calls to the same function. It is locked for 7 days.
    ///
    /// # Errors
    ///
    /// The caller must have the MINTER_ROLE.
    /// The idempotency key must not be used.
    /// The batch must not be empty.
    /// All accounts must have the WHITELISTED_ROLE.
    /// All amounts must be greater than zero.
    ///
    #[when_not_paused]
    pub fn mint_batch(
        e: &Env,
        operations: Vec<MintBatchOperation>,
        caller: Address,
        idempotency_key: String,
    ) {
        Self::auth_mint(e, caller);
        Self::assert_idempotency_key_not_used(e, &idempotency_key);

        assert!(operations.len() > 0, "Empty batch");

        for operation in &operations {
            let account = operation.0;
            Self::assert_has_role(e, &account, &WHITELISTED_ROLE);
            let amount = operation.1;
            assert!(amount > 0, "Invalid zero-amount mint");
            Base::mint(e, &account, amount);
        }
        Self::consume_idempotency_key(e, &idempotency_key);
    }

    fn auth_burn(e: &Env, caller: Address) {
        caller.require_auth();
        Self::assert_has_role(e, &caller, &BURNER_ROLE);
    }

    fn burn_no_auth(e: &Env, account: Address, amount: i128) {
        Base::update(e, Some(&account), None, amount);
        emit_burn(e, &account, amount);
    }

    /// Burn tokens from an account.
    ///
    /// # Arguments
    ///
    /// * `account` - The address of the account to burn tokens from.
    /// * `amount` - The amount of tokens to burn.
    /// * `caller` - The address of the caller.
    ///
    /// # Errors
    ///
    /// The caller must have the BURNER_ROLE.
    /// The amount must be greater than zero.
    ///
    #[when_not_paused]
    pub fn burn(e: &Env, account: Address, amount: i128, caller: Address) {
        Self::auth_burn(e, caller);
        assert!(amount > 0, "Invalid zero-amount burn");
        Self::burn_no_auth(e, account, amount);
    }

    /// Burn tokens from a batch of accounts.
    ///
    /// # Arguments
    ///
    /// * `operations` - The operations to burn tokens from.
    /// * `caller` - The address of the caller.
    /// * `idempotency_key` - The idempotency key. It is used to prevent duplicate calls to the same function. It is locked for 7 days.
    ///
    /// # Errors
    ///
    /// The caller must have the BURNER_ROLE.
    /// The idempotency key must not be used.
    /// The batch must not be empty.
    /// All amounts must be greater than zero.
    ///
    #[when_not_paused]
    pub fn burn_batch(
        e: &Env,
        operations: Vec<BurnBatchOperation>,
        caller: Address,
        idempotency_key: String,
    ) {
        Self::auth_burn(e, caller);
        Self::assert_idempotency_key_not_used(e, &idempotency_key);
        assert!(operations.len() > 0, "Empty batch");

        for operation in &operations {
            let account = operation.0;
            let amount = operation.1;
            assert!(amount > 0, "Invalid zero-amount burn");
            Self::burn_no_auth(e, account, amount);
        }
        Self::consume_idempotency_key(e, &idempotency_key);
    }

    /// Redeem tokens from an account. The tokens are transferred to the redemption contract. The redemption contract will then burn the tokens.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount of tokens to redeem.
    /// * `caller` - The address of the caller.
    /// * `idempotency_key` - The idempotency key. It is used to prevent duplicate calls to the same function. It is locked for 7 days.
    ///
    /// # Errors
    ///
    /// The caller must have the WHITELISTED_ROLE.
    /// The idempotency key must not be used.
    /// The amount must be greater than zero.
    /// The redemption contract must be set.
    /// The redemption contract must have the WHITELISTED_ROLE.
    ///
    #[when_not_paused]
    pub fn redeem(e: &Env, amount: i128, caller: Address, idempotency_key: String) {
        assert!(amount > 0, "Redemption amount should be more than zero");
        Self::assert_has_role(e, &caller, &WHITELISTED_ROLE);
        Self::assert_idempotency_key_not_used(e, &idempotency_key);

        let redemption: Address = e
            .storage()
            .instance()
            .get(&REDEMPTION_KEY)
            .expect("Redemption not set");
        let client: RedemptionClient<'_> = RedemptionClient::new(e, &redemption);
        Self::assert_has_role(e, &redemption, &WHITELISTED_ROLE);

        Base::transfer(e, &caller, &redemption, amount);
        client.on_redeem(
            &e.current_contract_address(),
            &caller,
            &amount,
            &idempotency_key,
        );
        Self::consume_idempotency_key(e, &idempotency_key);
    }

    /// Transfer tokens from one account to another.
    ///
    /// # Arguments
    ///
    /// * `from` - The address of the account to transfer tokens from.
    /// * `to` - The address of the account to transfer tokens to.
    /// * `amount` - The amount of tokens to transfer.
    ///
    /// # Errors
    ///
    /// The from account must have the WHITELISTED_ROLE.
    /// The to account must have the WHITELISTED_ROLE.
    /// The amount must be greater than zero.
    ///
    #[when_not_paused]
    pub fn transfer(e: &Env, from: Address, to: Address, amount: i128) {
        Self::assert_has_role(e, &from, &WHITELISTED_ROLE);
        Self::assert_has_role(e, &to, &WHITELISTED_ROLE);
        assert!(amount > 0, "Invalid zero-amount transfer");
        Base::transfer(e, &from, &to, amount);
    }

    /// Transfer tokens from one account to another. The transfer is idempotent.
    ///
    /// # Arguments
    ///
    /// * `from` - The address of the account to transfer tokens from.
    /// * `to` - The address of the account to transfer tokens to.
    /// * `amount` - The amount of tokens to transfer.
    /// * `idempotency_key` - The idempotency key. It is used to prevent duplicate calls to the same function. It is locked for 7 days.
    ///
    /// # Errors
    ///
    /// The from account must have the WHITELISTED_ROLE.
    /// The to account must have the WHITELISTED_ROLE.
    /// The idempotency key must not be used.
    /// The amount must be greater than zero.
    ///
    #[when_not_paused]
    pub fn safe_transfer(
        e: &Env,
        from: Address,
        to: Address,
        amount: i128,
        idempotency_key: String,
    ) {
        Self::assert_has_role(e, &from, &WHITELISTED_ROLE);
        Self::assert_has_role(e, &to, &WHITELISTED_ROLE);
        Self::assert_idempotency_key_not_used(e, &idempotency_key);
        assert!(amount > 0, "Invalid zero-amount transfer");
        Base::transfer(e, &from, &to, amount);
        Self::consume_idempotency_key(e, &idempotency_key);
    }

    /// Get the total supply of tokens.
    pub fn total_supply(e: &Env) -> i128 {
        Base::total_supply(e)
    }

    /// Get the balance of an account.
    ///
    /// # Arguments
    ///
    /// * `account` - The address of the account to get the balance of.
    pub fn balance(e: &Env, account: Address) -> i128 {
        Base::balance(e, &account)
    }

    pub fn decimals(e: &Env) -> u32 {
        Base::decimals(e)
    }

    pub fn name(e: &Env) -> String {
        Base::name(e)
    }

    pub fn symbol(e: &Env) -> String {
        Base::symbol(e)
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
