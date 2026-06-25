#![cfg(test)]
use soroban_sdk::{Env, Address};
use crate::checkpoint::{record_deposit_checkpoint, is_eligible_for_yield};

#[test]
fn test_same_ledger_frontrunning_protection() {
    let env = Env::default();
    let user = Address::generate(&env);

    env.ledger().set_sequence(10050);
    record_deposit_checkpoint(&env, &user);

    // Audit M-01 verification checkpoint: Depositing inside active sequence block MUST yield ineligible for execution rewards
    assert_eq!(is_eligible_for_yield(&env, &user), false);

    // Advance sequence frame explicitly to activate yield window eligibility
    env.ledger().set_sequence(10051);
    assert_eq!(is_eligible_for_yield(&env, &user), true);
}