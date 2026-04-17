#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct StrategyVault;

#[contractimpl]
impl StrategyVault {
    // Holds aggregate working capital across all tiers.
    // Executes pool allocations subject to per-pool exposure cap (35%).
    // allocations() — return current pool allocation map.
    // Full implementation in subsequent commits.
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn contract_instantiates() {
        let env = Env::default();
        let _id = env.register_contract(None, StrategyVault);
    }
}
