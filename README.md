# stellar-contracts

### Guide

To build all the contract
```
stellar contract build
```

To add an account to the stellar cli (it will ask you your pass phrase)
```
stellar keys add ACCOUNT_NAME --secure-store
```

You can verify that it is the right account by looking at the public key
```
stellar keys public-key ACCOUNT_NAME
```

To deploy a contract (put as many parameters as you need instead of admin)
```
stellar contract deploy \
  --source ACCOUNT_NAME \
  --network testnet \
  --alias CONTRACT_NAME \
  --wasm target/wasm32v1-none/release/permission_manager.wasm \
  -- \
  --admin GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H
```

To invoke a contract function
```
stellar contract invoke \
  --id CONTRACT_NAME \
  --source ACCOUNT_NAME \
  --network testnet \
  -- \
  get_admin
```

To get the compiled wasm code for integration test
```
stellar contract fetch --id permission_manager_v0_0 > permission_manager.wasm
```

### Testnet deployment

My freighter address: `GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H`

#### Permission manager contract

**Release**

- Testnet
    - v0_0: `CCAANR7HZNOXYZD7SXS2WQLV5BAOXSRZ7V4O43IANSB355QN2RNG7WVC`
