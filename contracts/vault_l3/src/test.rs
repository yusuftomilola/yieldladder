#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env};

fn setup_with_cap(cap: i128) -> (Env, VaultL3Client<'static>, Address, Address) {
    let env = Env::default();
    let contract_id = env.register_contract(None, VaultL3);
    let client = VaultL3Client::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let governance = Address::generate(&env);
    let strategy = Address::generate(&env);
    let usdc = Address::generate(&env);

    env.mock_all_auths();
    client.initialize(&admin, &governance, &strategy, &usdc, &cap);

    (env, client, admin, governance)
}

fn setup() -> (Env, VaultL3Client<'static>, Address, Address) {
    // 10,000 USDC default cap for most tests
    setup_with_cap(100_000_000_000)
}

#[test]
fn test_deposit_and_shares() {
    let (env, client, _admin, _gov) = setup();
    let user = Address::generate(&env);
    env.mock_all_auths();

    let amount = 500_000_000;
    client.deposit(&user, &amount);

    assert_eq!(client.shares(&user), 525_000_000);
    assert_eq!(client.balance(&user), amount);
    assert_eq!(client.total_balance(), amount);
}

#[test]
#[should_panic(expected = "BelowMinDeposit")]
fn test_deposit_below_min() {
    let (env, client, _admin, _gov) = setup();
    let user = Address::generate(&env);
    env.mock_all_auths();
    client.deposit(&user, &499_999_999_i128);
}

#[test]
#[should_panic(expected = "DepositCapExceeded")]
fn test_deposit_above_cap_rejected() {
    // Cap: exactly 1,000 USDC
    let (env, client, _admin, _gov) = setup_with_cap(10_000_000_000);
    let user = Address::generate(&env);
    env.mock_all_auths();

    // First deposit fills cap
    client.deposit(&user, &10_000_000_000_i128);

    // Second deposit would exceed it
    let user2 = Address::generate(&env);
    client.deposit(&user2, &500_000_000_i128);
}

#[test]
fn test_deposit_exactly_at_cap_succeeds() {
    let cap: i128 = 500_000_000; // 50 USDC == min deposit == cap
    let (env, client, _admin, _gov) = setup_with_cap(cap);
    let user = Address::generate(&env);
    env.mock_all_auths();
    client.deposit(&user, &cap);
    assert_eq!(client.total_balance(), cap);
    assert_eq!(client.remaining_capacity(), 0);
}

#[test]
fn test_remaining_capacity_decreases_on_deposit() {
    let cap: i128 = 2_000_000_000; // 200 USDC
    let (env, client, _admin, _gov) = setup_with_cap(cap);
    let user = Address::generate(&env);
    env.mock_all_auths();

    client.deposit(&user, &500_000_000_i128);
    assert_eq!(client.remaining_capacity(), cap - 500_000_000);
}

#[test]
fn test_set_max_tvl_by_governance() {
    let (env, client, _admin, gov) = setup();
    env.mock_all_auths();
    let new_cap: i128 = 500_000_000_000;
    client.set_max_tvl(&new_cap);
    assert_eq!(client.max_tvl(), new_cap);
}

#[test]
#[should_panic]
fn test_set_max_tvl_by_non_governance_rejected() {
    let (env, client, _admin, _gov) = setup();
    // Do NOT mock auths — call should fail without governance signature
    let new_cap: i128 = 500_000_000_000;
    client.set_max_tvl(&new_cap);
}

#[test]
fn test_lower_cap_does_not_affect_existing_depositors() {
    let (env, client, _admin, _gov) = setup();
    let user = Address::generate(&env);
    env.mock_all_auths();

    client.deposit(&user, &500_000_000_i128);

    // Lower cap below current TVL
    client.set_max_tvl(&100_000_000_i128);

    // Existing balance unchanged
    assert_eq!(client.balance(&user), 500_000_000);
    // remaining_capacity is 0 (clamped)
    assert_eq!(client.remaining_capacity(), 0);
}

#[test]
#[should_panic(expected = "LockNotExpired")]
fn test_withdraw_early_fails() {
    let (env, client, _admin, _gov) = setup();
    let user = Address::generate(&env);
    env.mock_all_auths();
    client.deposit(&user, &500_000_000_i128);
    client.withdraw(&user);
}

#[test]
fn test_withdraw_at_maturity() {
    let (env, client, _admin, _gov) = setup();
    let user = Address::generate(&env);
    env.mock_all_auths();

    client.deposit(&user, &500_000_000_i128);
    let seq = env.ledger().sequence();
    env.ledger().set_sequence(seq + 777_600);

    let returned = client.withdraw(&user);
    assert_eq!(returned, 500_000_000);
    assert_eq!(client.total_balance(), 0);
}