# Soroban Data

### Experiments with data storage and state management in Soroban


```bash
rustup target add wasm32-unknown-unknown

cargo test

cargo build --target wasm32-unknown-unknown --release

stellar keys generate --global james --network testnet

soroban contract deploy  --wasm target/wasm32-unknown-unknown/release/sorobandata.wasm --source james --network testnet

stellar contract invoke --id CONTRACT_HERE --source james --network testnet -- set --key name --value jamesbachini

stellar contract invoke --id CONTRACT_HERE --source james --network testnet -- get --key name

stellar contract invoke --id CONTRACT_HERE --source james --network testnet -- extend --key name
```