#![cfg(test)]

use acbu_minting::{MintingContract, MintingContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env};
use shared::CurrencyCode;

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    let reserve_tracker = Address::generate(&env);
    let acbu_token = Address::generate(&env);
    let usdc_token = Address::generate(&env);
    let fee_rate = 300; // 0.3%

    let contract_id = env.register_contract(None, MintingContract);
    let client = MintingContractClient::new(&env, &contract_id);

    client.initialize(
        &admin,
        &oracle,
        &reserve_tracker,
        &acbu_token,
        &usdc_token,
        &fee_rate,
    );

    assert_eq!(client.get_fee_rate(), fee_rate);
    assert_eq!(client.is_paused(), false);
}

#[test]
#[should_panic(expected = "Contract already initialized")]
fn test_initialize_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    let reserve_tracker = Address::generate(&env);
    let acbu_token = Address::generate(&env);
    let usdc_token = Address::generate(&env);
    let fee_rate = 300;

    let contract_id = env.register_contract(None, MintingContract);
    let client = MintingContractClient::new(&env, &contract_id);

    client.initialize(
        &admin,
        &oracle,
        &reserve_tracker,
        &acbu_token,
        &usdc_token,
        &fee_rate,
    );

    client.initialize(
        &admin,
        &oracle,
        &reserve_tracker,
        &acbu_token,
        &usdc_token,
        &fee_rate,
    );
}

#[test]
fn test_pause_unpause() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    let reserve_tracker = Address::generate(&env);
    let acbu_token = Address::generate(&env);
    let usdc_token = Address::generate(&env);
    let fee_rate = 300;

    let contract_id = env.register_contract(None, MintingContract);
    let client = MintingContractClient::new(&env, &contract_id);

    client.initialize(
        &admin,
        &oracle,
        &reserve_tracker,
        &acbu_token,
        &usdc_token,
        &fee_rate,
    );

    assert_eq!(client.is_paused(), false);
    client.pause();
    assert_eq!(client.is_paused(), true);
    client.unpause();
    assert_eq!(client.is_paused(), false);
}

#[test]
fn test_set_fee_rate() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    let reserve_tracker = Address::generate(&env);
    let acbu_token = Address::generate(&env);
    let usdc_token = Address::generate(&env);
    let fee_rate = 300;

    let contract_id = env.register_contract(None, MintingContract);
    let client = MintingContractClient::new(&env, &contract_id);

    client.initialize(
        &admin,
        &oracle,
        &reserve_tracker,
        &acbu_token,
        &usdc_token,
        &fee_rate,
    );

    let new_fee_rate = 500; // 0.5%
    client.set_fee_rate(&new_fee_rate);
    assert_eq!(client.get_fee_rate(), new_fee_rate);
}
