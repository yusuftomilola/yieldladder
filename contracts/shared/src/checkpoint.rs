use soroban_sdk::{Env, Address, symbol_short, Symbol};

const LEDGER_KEY: Symbol = symbol_short!("chkpnt");

pub fn record_deposit_checkpoint(env: &Env, user: &Address) {
    // Audit M-01 Fix: yield accrual window activates on the NEXT ledger sequence
    let next_eligible_ledger = env.ledger().sequence().checked_add(1).expect("Ledger max seq overflow");
    env.storage().persistent().set(&user, &next_eligible_ledger);
}

pub fn is_eligible_for_yield(env: &Env, user: &Address) -> bool {
    if !env.storage().persistent().has(user) {
        return false;
    }
    let activation_ledger: u32 = env.storage().persistent().get(user).unwrap();
    env.ledger().sequence() >= activation_ledger
}