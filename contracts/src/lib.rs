#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct StarterContract;

#[contractimpl]
impl StarterContract {
    pub fn set(env: &Env, key: Symbol, value: Symbol) {
        env.storage().persistent().set(&key, &value);
        env.storage().instance().set(&key, &value);
        env.storage().temporary().set(&key, &value);
    }

    pub fn get(env: &Env, key: Symbol) -> Option<Symbol> {
        env.storage().persistent().get(&key)
    }

    pub fn extend(env: Env, key: Symbol) {
        let max_ttl = env.storage().max_ttl();
        env.storage().persistent().extend_ttl(&key, max_ttl, max_ttl);
        env.storage().instance().extend_ttl(max_ttl, max_ttl);
    }
}

mod test;
