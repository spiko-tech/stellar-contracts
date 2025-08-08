# stellar-contracts

### Guide

#### Build
```
stellar contract build
```

#### Add account

To add an account to the stellar cli (it will ask you your pass phrase)
```
stellar keys add ACCOUNT_NAME --secure-store
```

You can verify that it is the right account by looking at the public key
```
stellar keys public-key ACCOUNT_NAME
```

#### Deployment
```
stellar contract deploy \
  --source nicolas \
  --network testnet \
  --alias permission_manager_v0_3 \
  --wasm target/wasm32v1-none/release/permission_manager.wasm \
  -- \
  --admin GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H 

stellar contract deploy \
  --source nicolas \
  --network testnet \
  --alias redemption_v0_2 \
  --wasm target/wasm32v1-none/release/redemption.wasm \
  -- \
  --owner GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H 

stellar contract deploy \
  --source nicolas \
  --network testnet \
  --alias token_v0_1 \
  --wasm target/wasm32v1-none/release/token.wasm \
  -- \
  --owner GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H \
  --name "Spiko EUR Test" \
  --symbol EUTBL \
  --decimals 6
```

To invoke a contract function
```
stellar contract invoke \
  --id token_v0_0 \
  --source alice \
  --network testnet \
  -- \
  get_owner
```

To get the compiled wasm code for integration test
```
stellar contract fetch --id token_v0_1 > token.wasm
```

### Testnet deployment

My freighter address: `GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H`

#### Permission manager contract

**Release**

- Testnet
    - permission_manager_v0_3: `CAYDAW7XOWVJ2OAOU6HIWV3XWZYPE5F4ZUXLJKODYINQZAQMFUEZBDHQ`
    - permission_manager_v0_2: `CADJ2TJ6M23OJWHX5DISWPFWEGYSWTQ5QMVWAURIBKXIUDUP6HEDCMML`
    - permission_manager_v0_1: `CAGVGJ6MDBJI5KD62UISPCFP2Q5GFTDCUZUY23WVMCT7PQGQRVJXVTWU`
    - permission_manager_v0_0: `CCAANR7HZNOXYZD7SXS2WQLV5BAOXSRZ7V4O43IANSB355QN2RNG7WVC`

#### Redemption

**Release**

- Testnet
    - redemption_v0_2: `CCWNSDJKJR7XW2LGHRBB5L6P2FBQV2XGAPVYD2EMEAHG2X7X27UJYDGP`
    - redemption_v0_1: `CBLVJKK34MHHXIHNYPMSDE3O3L24D6PTQXBVKQDTRAKHJLJBBP2CH26B`
    - redemption_v0_0: `CBLVJKK34MHHXIHNYPMSDE3O3L24D6PTQXBVKQDTRAKHJLJBBP2CH26B`

#### Token

**Release**

- Testnet
    - token_v0_1: `CDTEKYWAPFPWYL4LQK7AV3TP5W5IFN5D3VKBALF3LOKMBANSA6M2XFYW`
    - token_v0_0: `CCLQBG4PPTHSCZDI3QUGGSSPUHRH7O42GG64A36H7MAKG6Z6CU3W5MSD`
