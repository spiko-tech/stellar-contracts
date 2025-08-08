#!/bin/bash

# Default to dev environment
ENVIRONMENT=${1:-dev}
STELLAR_PROFILE=$2
NETWORK=${3:-testnet}

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed. Please install jq first."
    echo "On macOS: brew install jq"
    echo "On Ubuntu/Debian: sudo apt-get install jq"
    return 1
fi

# Check if stellar profile provided
if [ -z "$STELLAR_PROFILE" ]; then
    echo "Error: Stellar profile is required. Please provide a stellar profile."
    exit 1
fi

# Check if the JSON file exists for the specified environment
JSON_FILE="address/${ENVIRONMENT}.json"
if [ ! -f "$JSON_FILE" ]; then
    echo "Error: $JSON_FILE not found. Available environments:"
    ls -1 address/*.json 2>/dev/null | sed 's|address/||' | sed 's|.json||' || echo "No address files found"
    return 1
fi

PERMISSION_MANAGER_ADDRESS=$(jq -r '.permissionManager' "$JSON_FILE")
REDEMPTION_ADDRESS=$(jq -r '.redemption' "$JSON_FILE")
EUTBL_ADDRESS=$(jq -r '.tokens.EUTBL' "$JSON_FILE")
USTBL_ADDRESS=$(jq -r '.tokens.USTBL' "$JSON_FILE")
EUR_USTBL_ADDRESS=$(jq -r '.tokens.eurUSTBL' "$JSON_FILE")
UKTBL_ADDRESS=$(jq -r '.tokens.UKTBL' "$JSON_FILE")
SPKCC_ADDRESS=$(jq -r '.tokens.SPKCC' "$JSON_FILE")
EUR_SPKCC_ADDRESS=$(jq -r '.tokens.eurSPKCC' "$JSON_FILE")

echo "🌍 Environment: $ENVIRONMENT"
echo "🌍 Stellar Profile: $STELLAR_PROFILE"
echo "📋 Loaded Addresses:"
echo "  Permission Manager: $PERMISSION_MANAGER_ADDRESS"
echo "  Redemption: $REDEMPTION_ADDRESS"
echo "  EUTBL: $EUTBL_ADDRESS"
echo "  USTBL: $USTBL_ADDRESS"
echo "  eurUSTBL: $EUR_USTBL_ADDRESS"
echo "  UKTBL: $UKTBL_ADDRESS"
echo "  SPKCC: $SPKCC_ADDRESS"
echo "  eurSPKCC: $EUR_SPKCC_ADDRESS"

echo "🔄 Setup Permission Manager"
echo "---> Initialize"
set -x
stellar contract invoke \
  --id $PERMISSION_MANAGER_ADDRESS \
  --source $STELLAR_PROFILE \
  --network ${NETWORK} \
  -- \
  initialize
{ set +x; } 2>/dev/null

echo "🔄 Setup Redemption"
echo "---> Set Permission Manager"
set -x
stellar contract invoke \
  --id $REDEMPTION_ADDRESS \
  --source $STELLAR_PROFILE \
  --network testnet \
  -- \
  set_permission_manager \
  --permission_manager $PERMISSION_MANAGER_ADDRESS
{ set +x; } 2>/dev/null

echo "---> Add Token EUTBL"
set -x
stellar contract invoke \
  --id $REDEMPTION_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  add_token \
  --token_contract_address $EUTBL_ADDRESS
{ set +x; } 2>/dev/null

echo "---> Add Token USTBL"
set -x
stellar contract invoke \
  --id $REDEMPTION_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  add_token \
  --token_contract_address $USTBL_ADDRESS
{ set +x; } 2>/dev/null

echo "---> Add Token EUR_USTBL"
set -x
stellar contract invoke \
  --id $REDEMPTION_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  add_token \
  --token_contract_address $EUR_USTBL_ADDRESS
{ set +x; } 2>/dev/null

echo "---> Add Token UKTBL"
set -x
stellar contract invoke \
  --id $REDEMPTION_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  add_token \
  --token_contract_address $UKTBL_ADDRESS
{ set +x; } 2>/dev/null

echo "---> Add Token SPKCC"
set -x
stellar contract invoke \
  --id $REDEMPTION_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  add_token \
  --token_contract_address $SPKCC_ADDRESS
{ set +x; } 2>/dev/null

echo "---> Add Token EUR_SPKCC"
set -x
stellar contract invoke \
  --id $REDEMPTION_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  add_token \
  --token_contract_address $EUR_SPKCC_ADDRESS
{ set +x; } 2>/dev/null

echo "🔄 Setup EUTBL"
echo "---> Set Permission Manager"
set -x
stellar contract invoke \
  --id $EUTBL_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  set_permission_manager \
  --permission_manager $PERMISSION_MANAGER_ADDRESS
{ set +x; } 2>/dev/null
echo "---> Set Redemption"
set -x
stellar contract invoke \
  --id $EUTBL_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  set_redemption \
  --redemption $REDEMPTION_ADDRESS
{ set +x; } 2>/dev/null

echo "🔄 Setup USTBL"
echo "---> Set Permission Manager"
set -x
stellar contract invoke \
  --id $USTBL_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  set_permission_manager \
  --permission_manager $PERMISSION_MANAGER_ADDRESS
{ set +x; } 2>/dev/null
echo "---> Set Redemption"
set -x
stellar contract invoke \
  --id $USTBL_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  set_redemption \
  --redemption $REDEMPTION_ADDRESS
{ set +x; } 2>/dev/null

echo "🔄 Setup EUR_USTBL"
echo "---> Set Permission Manager"
set -x
stellar contract invoke \
  --id $EUR_USTBL_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  set_permission_manager \
  --permission_manager $PERMISSION_MANAGER_ADDRESS
{ set +x; } 2>/dev/null
echo "---> Set Redemption"
set -x
stellar contract invoke \
  --id $EUR_USTBL_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  set_redemption \
  --redemption $REDEMPTION_ADDRESS
{ set +x; } 2>/dev/null

echo "🔄 Setup UKTBL"
echo "---> Set Permission Manager"
set -x
stellar contract invoke \
  --id $UKTBL_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  set_permission_manager \
  --permission_manager $PERMISSION_MANAGER_ADDRESS
{ set +x; } 2>/dev/null
echo "---> Set Redemption"
set -x
stellar contract invoke \
  --id $UKTBL_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  set_redemption \
  --redemption $REDEMPTION_ADDRESS
{ set +x; } 2>/dev/null

echo "🔄 Setup SPKCC"
echo "---> Set Permission Manager"
set -x
stellar contract invoke \
  --id $SPKCC_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  set_permission_manager \
  --permission_manager $PERMISSION_MANAGER_ADDRESS
{ set +x; } 2>/dev/null
echo "---> Set Redemption"
set -x
stellar contract invoke \
  --id $SPKCC_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  set_redemption \
  --redemption $REDEMPTION_ADDRESS
{ set +x; } 2>/dev/null

echo "🔄 Setup EUR_SPKCC"
echo "---> Set Permission Manager"
set -x
stellar contract invoke \
  --id $EUR_SPKCC_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  set_permission_manager \
  --permission_manager $PERMISSION_MANAGER_ADDRESS
{ set +x; } 2>/dev/null
echo "---> Set Redemption"
set -x
stellar contract invoke \
  --id $EUR_SPKCC_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  set_redemption \
  --redemption $REDEMPTION_ADDRESS
{ set +x; } 2>/dev/null


echo "🔄 Setup Role"
echo "---> Give WHITELISTED_ROLE to redemption"
set -x
stellar contract invoke \
  --id $PERMISSION_MANAGER_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  grant_role \
  --caller $STELLAR_PROFILE \
  --account $REDEMPTION_ADDRESS \
  --role WLISTED
{ set +x; } 2>/dev/null
echo "---> Give REDEMPTION_EXECUTOR_ROLE to redemption"
set -x
stellar contract invoke \
  --id $PERMISSION_MANAGER_ADDRESS \
  --source $STELLAR_PROFILE \
  --network $NETWORK \
  -- \
  grant_role \
  --caller $STELLAR_PROFILE \
  --account $REDEMPTION_ADDRESS \
  --role REXECUTOR
{ set +x; } 2>/dev/null

