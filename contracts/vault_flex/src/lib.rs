#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct VaultFlex;

#[contractimpl]
impl VaultFlex {
    // Flex tier: no lock, 1.00x multiplier, 0% early-exit fee.
    // Holds depositor balances and forwards working capital to StrategyVault.
    // Full implementation in subsequent commits.
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn contract_instantiates() {
        let env = Env::default();
        let _id = env.register_contract(None, VaultFlex);
    }
}
