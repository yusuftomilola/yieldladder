use soroban_sdk::{Env, Address};

// Generate client bindings automatically using native token framework layouts
soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/soroban_token_contract.wasm");

pub struct UsdcClient<'a> {
    pub env: &'a Env,
    pub client: TokenClient<'a>,
}

impl<'a> UsdcClient<'a> {
    pub fn new(env: &'a Env, contract_id: &Address) -> Self {
        Self {
            env,
            client: TokenClient::new(env, contract_id),
        }
    }

    pub fn balance(&self, account: &Address) -> i128 {
        self.client.balance(account)
    }

    pub fn transfer(&self, from: &Address, to: &Address, amount: i128) {
        assert!(amount > 0, "Token: Transfer amount must be positive");
        self.client.transfer(from, to, &amount);
    }

    pub fn allowance(&self, from: &Address, spender: &Address) -> i128 {
        self.client.allowance(from, spender)
    }
}