use soroban_sdk::{Env, Address, Symbol, symbol_short, panic_with_error};

const ADMIN_KEY: Symbol = symbol_short!("admin");

pub fn init_allowlist(env: &Env, admin: &Address, initial_assets: soroban_sdk::Vec<Address>) {
    assert!(!env.storage().instance().has(&ADMIN_KEY), "Allowlist already initialized");
    env.storage().instance().set(&ADMIN_KEY, admin);
    
    for asset in initial_assets.iter() {
        env.storage().instance().set(&asset, &true);
    }
}

pub fn is_asset_allowed(env: &Env, asset: &Address) -> bool {
    env.storage().instance().get::<Address, bool>(asset).unwrap_or(false)
}

pub fn add_asset(env: &Env, admin: &Address, asset: &Address) {
    let saved_admin: Address = env.storage().instance().get(&ADMIN_KEY).expect("Allowlist not initialized");
    admin.require_auth();
    assert!(admin == &saved_admin, "Allowlist: Admin authorization mismatch");
    env.storage().instance().set(asset, &true);
}

pub fn remove_asset(env: &Env, admin: &Address, asset: &Address) {
    let saved_admin: Address = env.storage().instance().get(&ADMIN_KEY).expect("Allowlist not initialized");
    admin.require_auth();
    assert!(admin == &saved_admin, "Allowlist: Admin authorization mismatch");
    if env.storage().instance().has(asset) {
        env.storage().instance().set(asset, &false);
    }
}