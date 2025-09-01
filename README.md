# stellar-contracts

## Guide

### Add account

To add an account to the stellar cli (it will ask you your pass phrase)
```
stellar keys add ACCOUNT_NAME --secure-store
```

You can verify that it is the right account by looking at the public key
```
stellar keys public-key ACCOUNT_NAME
```

### Build
```
stellar contract build
```

### Deployment
```
# PERMISSION MANAGER
stellar contract deploy \
  --source ACCOUNT_NAME \
  --network testnet \
  --alias permission_manager_dev \
  --wasm target/wasm32v1-none/release/permission_manager.wasm \
  -- \
  --admin ADMIN_ADDRESS 

# REDEMPTION
stellar contract deploy \
  --source ACCOUNT_NAME \
  --network testnet \
  --alias redemption_dev \
  --wasm target/wasm32v1-none/release/redemption.wasm \
  -- \
  --owner OWNER_ADDRESS 

# TOKEN EUTBL
stellar contract deploy \
  --source ACCOUNT_NAME \
  --network testnet \
  --alias eutbl_dev \
  --wasm target/wasm32v1-none/release/token.wasm \
  -- \
  --owner OWNER_ADDRESS \
  --name "Dev EUTBL" \
  --symbol EUTBL \
  --decimals 5

# TOKEN USTBL
stellar contract deploy \
  --source ACCOUNT_NAME \
  --network testnet \
  --alias ustbl_dev \
  --wasm target/wasm32v1-none/release/token.wasm \
  -- \
  --owner OWNER_ADDRESS \
  --name "Dev USTBL" \
  --symbol USTBL \
  --decimals 5


# TOKEN eurUSTBL
stellar contract deploy \
  --source ACCOUNT_NAME \
  --network testnet \
  --alias eurustbl_dev \
  --wasm target/wasm32v1-none/release/token.wasm \
  -- \
  --owner OWNER_ADDRESS \
  --name "Dev eurUSTBL" \
  --symbol eurUSTBL \
  --decimals 5

# TOKEN UKTBL
stellar contract deploy \
  --source ACCOUNT_NAME \
  --network testnet \
  --alias uktbl_dev \
  --wasm target/wasm32v1-none/release/token.wasm \
  -- \
  --owner OWNER_ADDRESS \
  --name "Dev UKTBL" \
  --symbol UKTBL \
  --decimals 5

# TOKEN SPKCC
stellar contract deploy \
  --source ACCOUNT_NAME \
  --network testnet \
  --alias spkcc_dev \
  --wasm target/wasm32v1-none/release/token.wasm \
  -- \
  --owner OWNER_ADDRESS \
  --name "Dev SPKCC" \
  --symbol SPKCC \
  --decimals 5

# TOKEN eurSPKCC
stellar contract deploy \
  --source ACCOUNT_NAME \
  --network testnet \
  --alias eurspkcc_dev \
  --wasm target/wasm32v1-none/release/token.wasm \
  -- \
  --owner OWNER_ADDRESS \
  --name "Dev eurSPKCC" \
  --symbol eurSPKCC \
  --decimals 5
```

### Setup

Use the custom setup script.
It is mandatory workflow for an operational setup.
Use it as an inspiration to craft your own command.
```
./scripts/setup_contracts.sh dev ACCOUNT_NAME
```

### Utils

To get the compiled wasm code for integration test
```
stellar contract fetch --id TOKEN_ID > token.wasm
```
