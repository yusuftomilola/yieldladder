#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env};

fn setup() -> (Env, VaultL3Client<'static>, Address, Address, Address) {
    let env = Env::default();
    let contract_id = env.register_contract(None, VaultL3);
    let client = VaultL3Client::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let strategy = Address::generate(&env);
    let usdc = Address::generate(&env);

    client.initialize(&admin, &strategy, &usdc);

    (env, client, admin, strategy, usdc)
}

#[test]
fn test_deposit_and_shares() {
    let (env, client, _admin, _strategy, _usdc) = setup();
    let user = Address::generate(&env);

    env.mock_all_auths();

    // Min deposit: 50 USDC (500_000_000 stroops)
    let amount = 500_000_000;
    client.deposit(&user, &amount);

    // Check multiplier 1.05x -> 525,000,000 shares
    assert_eq!(client.shares(&user), 525_000_000);
    assert_eq!(client.balance(&user), amount);
    assert_eq!(client.total_shares(), 525_000_000);

    // Check lock_until
    let expected_lock = env.ledger().sequence() + 777_600;
    assert_eq!(client.lock_until(&user), expected_lock);
}

#[test]
#[should_panic(expected = "BelowMinDeposit")]
fn test_deposit_below_min() {
    let (env, client, _admin, _strategy, _usdc) = setup();
    let user = Address::generate(&env);

    env.mock_all_auths();

    // Less than 50 USDC
    let amount = 499_999_999;
    client.deposit(&user, &amount);
}

#[test]
#[should_panic(expected = "LockNotExpired")]
fn test_withdraw_early_fails() {
    let (env, client, _admin, _strategy, _usdc) = setup();
    let user = Address::generate(&env);

    env.mock_all_auths();

    let amount = 500_000_000;
    client.deposit(&user, &amount);

    // Try withdrawing immediately
    client.withdraw(&user);
}

#[test]
fn test_withdraw_maturity() {
    let (env, client, _admin, _strategy, _usdc) = setup();
    let user = Address::generate(&env);

    env.mock_all_auths();

    let amount = 500_000_000;
    client.deposit(&user, &amount);

    // Fast forward ledger past lock_duration
    let current_seq = env.ledger().sequence();
    env.ledger().set_sequence(current_seq + 777_600);

    let returned = client.withdraw(&user);
    assert_eq!(returned, amount);
    assert_eq!(client.shares(&user), 0);
    assert_eq!(client.balance(&user), 0);
    assert_eq!(client.total_shares(), 0);
}

#[test]
fn test_early_exit_fee() {
    let (env, client, _admin, _strategy, _usdc) = setup();
    let user = Address::generate(&env);

    env.mock_all_auths();

    let amount = 500_000_000;
    client.deposit(&user, &amount);

    // Early exit before maturity
    // Fee is 0.50% of 500,000,000 = 2,500,000
    // Net = 497,500,000
    let returned = client.early_exit(&user);
    assert_eq!(returned, 497_500_000);
    assert_eq!(client.shares(&user), 0);
    assert_eq!(client.balance(&user), 0);
}
