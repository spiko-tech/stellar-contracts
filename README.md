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
  --source nicolas \
  --network testnet \
  --alias redemption_v0_1 \
  --wasm target/wasm32v1-none/release/redemption.wasm \
  -- \
  --owner GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H
```

To invoke a contract function
```
stellar contract invoke \
  --id redemption_v0_1 \
  --source alice \
  --network testnet \
  -- \
  get_owner
```

To get the compiled wasm code for integration test
```
stellar contract fetch --id redemption_v0_1 > redemption.wasm
```

### Testnet deployment

My freighter address: `GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H`

#### Permission manager contract

**Release**

- Testnet
    - permission_manager_v0_2: `CADJ2TJ6M23OJWHX5DISWPFWEGYSWTQ5QMVWAURIBKXIUDUP6HEDCMML`
    - permission_manager_v0_1: `CAGVGJ6MDBJI5KD62UISPCFP2Q5GFTDCUZUY23WVMCT7PQGQRVJXVTWU`
    - permission_manager_v0_0: `CCAANR7HZNOXYZD7SXS2WQLV5BAOXSRZ7V4O43IANSB355QN2RNG7WVC`

#### Redemption

**Release**

- Testnet
    - redemption_v0_1: `CBLVJKK34MHHXIHNYPMSDE3O3L24D6PTQXBVKQDTRAKHJLJBBP2CH26B`
    - redemption_v0_0: `CBLVJKK34MHHXIHNYPMSDE3O3L24D6PTQXBVKQDTRAKHJLJBBP2CH26B`
