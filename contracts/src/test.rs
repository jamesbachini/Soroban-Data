#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, Env};

#[test]
fn test_operations() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StarterContract);
    let client = StarterContractClient::new(&env, &contract_id);

    let key = symbol_short!("key1");
    let value = symbol_short!("hello");
    
    // Test set and get
    client.set(&key, &value);
    let value2 = client.get(&key);
    assert_eq!(value, value2.expect("hello"));

    // Test TTL extension
    client.extend(&key);
    /*
        let max_ttl = env.storage().max_ttl();
        let ttl = env.storage().persistent().get_ttl(&key);
        env.as_contract(&contract_id, || {
            assert_eq!(ttl, max_ttl);
        });

    */
}
