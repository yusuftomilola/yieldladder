#![cfg(test)]

use super::{VaultFlex, VaultFlexClient};
use soroban_sdk::{Env, Address, testutils::Address as _};
use shared::fixed_point::SCALING_FACTOR;

fn setup_test_environment(env: &Env) -> (VaultFlexClient, Address, Address, Address) {
    let contract_id = env.register_contract(None, VaultFlex);
    let client = VaultFlexClient::new(env, &contract_id);
    
    let admin = Address::generate(env);
    let strategy = Address::generate(env);
    let user = Address::generate(env);

    (client, admin, strategy, user)
}

#[test]
fn test_initialization_boundaries() {
    let env = Env::default();
    let (client, admin, strategy, _) = setup_test_environment(&env);

    client.initialize(&admin, &strategy);

    // Initializing a second time must trigger an unrecoverable panic
    let result = std::panic::catch_unwind(|| {
        client.initialize(&admin, &strategy);
    });
    assert!(result.is_err());
}

#[test]
fn test_successful_deposit_and_shares_minting() {
    let env = Env::default();
    let (client, admin, _, user) = setup_test_environment(&env);

    client.initialize(&admin, &strategy);

    env.mock_all_auths();

    let deposit_amount = 50 * SCALING_FACTOR; // 50.0000000 USDC
    client.deposit(&user, &deposit_amount);

    // Assert 1.00x constant scaling conditions are preserved
    assert_eq!(client.balance(&user), deposit_amount);
    assert_eq!(client.shares(&user), deposit_amount);
    assert_eq!(client.total_shares(), deposit_amount);
}

#[test]
#[should_panic(expected = "VaultFlex: Deposit amount below minimum requirement of 1 USDC")]
fn test_minimum_deposit_rejection() {
    let env = Env::default();
    let (client, admin, _, user) = setup_test_environment(&env);

    client.initialize(&admin, &strategy);
    env.mock_all_auths();

    // Rejects sub-minimum deposits (0.95 USDC)
    let bad_deposit = 950_000; 
    client.deposit(&user, &bad_deposit);
}

#[test]
fn test_pro_rata_yield_withdrawal() {
    let env = Env::default();
    let (client, admin, _, user) = setup_test_environment(&env);

    client.initialize(&admin, &strategy);
    env.mock_all_auths();

    env.ledger().set_sequence(100);
    client.deposit(&user, &(100 * SCALING_FACTOR));

    // Advance sequence frame to clear M-01 same-ledger frontrun protections
    env.ledger().set_sequence(101);

    // Simulate strategy picking up an aggregate balance of 110 USDC via performance harvesting
    let strategy_simulated_balance = 110 * SCALING_FACTOR;
    let payout = client.withdraw(&user, &strategy_simulated_balance);

    // Expect principal + 10 USDC yield rewards
    assert_eq!(payout, 110 * SCALING_FACTOR);
    
    // Position state must be destroyed upon withdrawal
    assert_eq!(client.balance(&user), 0);
    assert_eq!(client.shares(&user), 0);
    assert_eq!(client.total_shares(), 0);
}

#[test]
#[should_panic(expected = "VaultFlex: Zero share profile detected or double-withdraw requested")]
fn test_double_withdraw_protection() {
    let env = Env::default();
    let (client, admin, _, user) = setup_test_environment(&env);

    client.initialize(&admin, &strategy);
    env.mock_all_auths();

    env.ledger().set_sequence(100);
    client.deposit(&user, &(100 * SCALING_FACTOR));

    env.ledger().set_sequence(101);
    client.withdraw(&user, &(100 * SCALING_FACTOR));
    
    // Second call triggers double-withdraw protection panic
    client.withdraw(&user, &0);
}m