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

#### Permission Manager
```
stellar contract invoke \
  --id permission_manager_dev \
  --source nicolas \
  --network testnet \
  -- \
  initialize
```

#### Redemption
```
stellar contract invoke \
  --id redemption_dev \
  --source nicolas \
  --network testnet \
  -- \
  set_permission_manager \
  --permission_manager CA2OGL5LVNIMZ35L4PUS342ZQX2NW2OX62SYBSCEPXYXNXXYDBDZDLU6

stellar contract invoke \
  --id redemption_dev \
  --source nicolas \
  --network testnet \
  -- \
  add_token \
  --token_contract_address CBWO2NZMJMFCAQGFL6P3OEL3XU2UNSYK4X3DKOJJYBWFI2SDLKQJ2J4E

stellar contract invoke \
  --id redemption_dev \
  --source nicolas \
  --network testnet \
  -- \
  add_token \
  --token_contract_address CDPSCPGHVF5SQZXPFHNY365LVS3SZIIMNJJPCJDVZB3FOK4ZCVRUQYG3

stellar contract invoke \
  --id redemption_dev \
  --source nicolas \
  --network testnet \
  -- \
  add_token \
  --token_contract_address CCSDGXMVAJ454NALIXYN7H3ALF25W4AF7N2USXXNGI2ECCANEV6BGY53

stellar contract invoke \
  --id redemption_dev \
  --source nicolas \
  --network testnet \
  -- \
  add_token \
  --token_contract_address CCOYYP3NB25R3ST75WYPQRG2XWJ536SUBWE4YCXO7FUUFFEPYTYOSBSB

stellar contract invoke \
  --id redemption_dev \
  --source nicolas \
  --network testnet \
  -- \
  add_token \
  --token_contract_address CD5XCPVVUOXU6SY7HG62Q6VC2MBYGCZ54UOOFTFEKMGOF34DZE7Z6I3C

stellar contract invoke \
  --id redemption_dev \
  --source nicolas \
  --network testnet \
  -- \
  add_token \
  --token_contract_address CBGQXPX3VYDWWJQUKCEUDGVJPXENSWAOLRDIERLA44V5FPFTWBT4AALQ
```

#### Token
```

```

To invoke a contract function
```
stellar contract invoke \
  --id permission_manager_v0_3 \
  --source alice \
  --network testnet \
  -- \
  get_admin

stellar contract invoke \
  --id permission_manager_v0_3 \
  --source nicolas \
  --network testnet \
  -- \
  grant_role \
  --caller GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H \
  --account GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H \
  --role WLISTER

stellar contract invoke \
  --id permission_manager_v0_3 \
  --source nicolas \
  --network testnet \
  -- \
  has_role \
  --account GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H \
  --role WLISTER

// To get help on a command
stellar contract invoke \
  --id permission_manager_v0_3 \
  --source alice \
  --network testnet \
  -- \
  has_role \
  --help
```

### Utils

To get the compiled wasm code for integration test
```
stellar contract fetch --id token_v0_1 > token.wasm
```
