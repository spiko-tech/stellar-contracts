# stellar-contracts

My freighter address: `GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H`

## Guide

### Add account

To add an account to the stellar cli (it will ask you your pass phrase)
```
stellar keys add nicolas --seed-phrase // Deprecated but the only one which works for now
stellar keys add ACCOUNT_NAME --secure-store // New version but unable to sign a transaction then
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
  --source nicolas \
  --network testnet \
  --alias permission_manager_dev \
  --wasm target/wasm32v1-none/release/permission_manager.wasm \
  -- \
  --admin GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H 

# REDEMPTION
stellar contract deploy \
  --source nicolas \
  --network testnet \
  --alias redemption_dev \
  --wasm target/wasm32v1-none/release/redemption.wasm \
  -- \
  --owner GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H 

# TOKEN EUTBL
stellar contract deploy \
  --source nicolas \
  --network testnet \
  --alias eutbl_dev \
  --wasm target/wasm32v1-none/release/token.wasm \
  -- \
  --owner GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H \
  --name "Dev EUTBL" \
  --symbol EUTBL \
  --decimals 5

# TOKEN USTBL
stellar contract deploy \
  --source nicolas \
  --network testnet \
  --alias ustbl_dev \
  --wasm target/wasm32v1-none/release/token.wasm \
  -- \
  --owner GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H \
  --name "Dev USTBL" \
  --symbol USTBL \
  --decimals 5


# TOKEN eurUSTBL
stellar contract deploy \
  --source nicolas \
  --network testnet \
  --alias eurustbl_dev \
  --wasm target/wasm32v1-none/release/token.wasm \
  -- \
  --owner GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H \
  --name "Dev eurUSTBL" \
  --symbol eurUSTBL \
  --decimals 5

# TOKEN UKTBL
stellar contract deploy \
  --source nicolas \
  --network testnet \
  --alias uktbl_dev \
  --wasm target/wasm32v1-none/release/token.wasm \
  -- \
  --owner GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H \
  --name "Dev UKTBL" \
  --symbol UKTBL \
  --decimals 5

# TOKEN SPKCC
stellar contract deploy \
  --source nicolas \
  --network testnet \
  --alias spkcc_dev \
  --wasm target/wasm32v1-none/release/token.wasm \
  -- \
  --owner GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H \
  --name "Dev SPKCC" \
  --symbol SPKCC \
  --decimals 5

# TOKEN eurSPKCC
stellar contract deploy \
  --source nicolas \
  --network testnet \
  --alias eurspkcc_dev \
  --wasm target/wasm32v1-none/release/token.wasm \
  -- \
  --owner GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H \
  --name "Dev eurSPKCC" \
  --symbol eurSPKCC \
  --decimals 5
```

### Setup

Use the custom setup script.
It is mandatory workflow for an operational setup.
Use it as an inspiration to craft your own command.
```
./scripts/setup_contracts.sh dev nicolas
```

### Utils

To get the compiled wasm code for integration test
```
stellar contract fetch --id TOKEN_ID > token.wasm
```
