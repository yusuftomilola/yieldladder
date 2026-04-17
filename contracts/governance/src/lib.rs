#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct Governance;

#[contractimpl]
impl Governance {
    // propose(action) — Strategist submits allocation change; starts 72-hour timelock.
    // veto(proposal_id) — Guardian Multisig cancels a pending proposal.
    // execute(proposal_id) — executes after timelock elapses if not vetoed.
    // Full implementation in subsequent commits.
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn contract_instantiates() {
        let env = Env::default();
        let _id = env.register_contract(None, Governance);
    }
}
