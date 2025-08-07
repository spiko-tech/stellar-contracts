#![cfg(test)]

extern crate std;

use super::contract::{Token, TokenArgs, TokenClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

mod permission_manager {
    use soroban_sdk::contractimport;

    contractimport!(file = "../../wasm/permission_manager.wasm");
}

mod redemption {
    use soroban_sdk::contractimport;

    contractimport!(file = "../../wasm/redemption.wasm");
}

fn setup_env() -> Env {
    let e: Env = Env::default();
    e.mock_all_auths();
    e
}

fn deploy_token(
    e: &Env,
    name: String,
    symbol: String,
    decimals: u32,
) -> (Address, Address, TokenClient) {
    let owner: Address = Address::generate(e);
    let token_address = e.register(
        Token,
        TokenArgs::__constructor(&owner.clone(), &name.clone(), &symbol.clone(), &decimals),
    );
    let client = TokenClient::new(e, &token_address);

    (owner, token_address, client)
}

fn deploy_permission_manager(e: &Env) -> (Address, Address, permission_manager::Client<'_>) {
    let admin: Address = Address::generate(e);
    let permission_manager_address = e.register(
        permission_manager::WASM,
        permission_manager::Args::__constructor(&admin.clone()),
    );
    let permission_manager_client = permission_manager::Client::new(e, &permission_manager_address);
    permission_manager_client.initialize();

    (admin, permission_manager_address, permission_manager_client)
}

fn deploy_redemption(e: &Env) -> (Address, Address, redemption::Client<'_>) {
    let owner: Address = Address::generate(e);
    let redemption_address = e.register(
        redemption::WASM,
        redemption::Args::__constructor(&owner.clone()),
    );
    let redemption_client = redemption::Client::new(e, &redemption_address);

    (owner, redemption_address, redemption_client)
}

fn initial_state() {
    let env = Env::default();

    let contract_addr = env.register(
        Token,
        (
            Address::generate(&env),
            Address::generate(&env),
            Address::generate(&env),
            Address::generate(&env),
        ),
    );
    let client = TokenClient::new(&env, &contract_addr);

    assert_eq!(client.name(), String::from_str(&env, "Token"));
}

// Add more tests bellow
