#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct VaultRouter;

#[contractimpl]
impl VaultRouter {
    // deposit(tier, amount) — validate tier rules, mint shares, forward to tier vault.
    // withdraw(tier)         — return principal + yield after lock_until.
    // early_exit(tier)       — return principal minus exit fee before lock_until.
    // Full implementation in subsequent commits.
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn contract_instantiates() {
        let env = Env::default();
        let _id = env.register_contract(None, VaultRouter);
    }
}
