#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env};

fn setup() -> (Env, VaultL12Client<'static>, Address, Address, Address) {
    let env = Env::default();
    let contract_id = env.register_contract(None, VaultL12);
    let client = VaultL12Client::new(&env, &contract_id);

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

    // 250 USDC
    let amount = 2_500_000_000;
    client.deposit(&user, &amount);

    // 1.40x multiplier -> 3,500,000,000 shares
    assert_eq!(client.shares(&user), 3_500_000_000);
    assert_eq!(client.lock_until(&user), env.ledger().sequence() + 3_110_400);
}

#[test]
#[should_panic(expected = "BelowMinDeposit")]
fn test_deposit_below_min() {
    let (env, client, _admin, _strategy, _usdc) = setup();
    let user = Address::generate(&env);
    env.mock_all_auths();

    let amount = 2_499_999_999;
    client.deposit(&user, &amount);
}

#[test]
#[should_panic(expected = "LockNotExpired")]
fn test_withdraw_early_fails() {
    let (env, client, _admin, _strategy, _usdc) = setup();
    let user = Address::generate(&env);
    env.mock_all_auths();

    let amount = 2_500_000_000;
    client.deposit(&user, &amount);
    client.withdraw(&user);
}

#[test]
fn test_withdraw_maturity() {
    let (env, client, _admin, _strategy, _usdc) = setup();
    let user = Address::generate(&env);
    env.mock_all_auths();

    let amount = 2_500_000_000;
    client.deposit(&user, &amount);

    let current_seq = env.ledger().sequence();
    env.ledger().set_sequence(current_seq + 3_110_400);

    assert_eq!(client.withdraw(&user), amount);
}

#[test]
fn test_early_exit_fee() {
    let (env, client, _admin, _strategy, _usdc) = setup();
    let user = Address::generate(&env);
    env.mock_all_auths();

    let amount = 2_500_000_000;
    client.deposit(&user, &amount);

    // Fee: 3.00% of 2,500,000,000 = 75,000,000
    // Net: 2,425,000,000
    assert_eq!(client.early_exit(&user), 2_425_000_000);
}
