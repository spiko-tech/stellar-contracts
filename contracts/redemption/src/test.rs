#![cfg(test)]

extern crate std;

use super::contract::{Redemption, RedemptionArgs, RedemptionClient};
use soroban_sdk::{contract, testutils::Address as _, Address, Env};

#[contract]
struct MockContract;

#[test]
fn test_owner_setup() {
    let e = Env::default();
    e.mock_all_auths();

    let owner: Address = Address::generate(&e);
    let contract_address = e.register(Redemption, RedemptionArgs::__constructor(&owner.clone()));
    let client = RedemptionClient::new(&e, &contract_address);

    let fetched_owner = client.get_owner();

    assert_eq!(fetched_owner, Some(owner));
}
