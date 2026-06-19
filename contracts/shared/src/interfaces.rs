use soroban_sdk::{contractclient, Address, Env, Map};

#[contractclient(name = "TierVaultClient")]
pub trait TierVaultInterface {
    fn deposit(env: Env, user: Address, amount: i128);
    fn withdraw(env: Env, user: Address) -> i128;
    fn early_exit(env: Env, user: Address) -> i128;
}

#[contractclient(name = "StrategyVaultClient")]
pub trait StrategyVaultInterface {
    fn deposit_capital(env: Env, amount: i128);
    fn withdraw_capital(env: Env, amount: i128);
    fn allocations(env: Env) -> Map<Address, i128>;
}

#[contractclient(name = "HarvesterClient")]
pub trait HarvesterInterface {
    fn harvest(env: Env);
}