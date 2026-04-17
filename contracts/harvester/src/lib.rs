#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct Harvester;

#[contractimpl]
impl Harvester {
    // harvest() — permissionless; claims AMM trading fees and compounds into StrategyVault.
    // Caller receives 10 bps bounty on harvested yield.
    // Full implementation in subsequent commits.
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn contract_instantiates() {
        let env = Env::default();
        let _id = env.register_contract(None, Harvester);
    }
}
