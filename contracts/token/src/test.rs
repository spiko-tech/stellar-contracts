#![cfg(test)]

extern crate std;

use soroban_sdk::{ testutils::Address as _, Address, Env, String };

use crate::contract::{ Token, TokenClient };

#[test]
fn initial_state() {
    let env = Env::default();

    let contract_addr = env.register(Token, (Address::generate(&env),Address::generate(&env),Address::generate(&env),Address::generate(&env)));
    let client = TokenClient::new(&env, &contract_addr);

    assert_eq!(client.name(), String::from_str(&env, "Token"));
}

// Add more tests bellow
