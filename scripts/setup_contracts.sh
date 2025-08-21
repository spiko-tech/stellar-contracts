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

RELAYER_ADDRESS=GB7BUX5B2UCSPTBC3UX4O6MRO5OPEZV2CK7FEVONU5Q7WEISLRRNT3S7

initialize_permission_manager() {
    set -x
    stellar contract invoke \
      --id $PERMISSION_MANAGER_ADDRESS \
      --source $STELLAR_PROFILE \
      --network ${NETWORK} \
      -- \
      initialize
    { set +x; } 2>/dev/null
}

set_permission_manager_on_redemption() {
    set -x
    stellar contract invoke \
      --id $REDEMPTION_ADDRESS \
      --source $STELLAR_PROFILE \
      --network testnet \
      -- \
      set_permission_manager \
      --permission_manager $PERMISSION_MANAGER_ADDRESS
    { set +x; } 2>/dev/null
}

add_eutbl_to_redemption() {
    set -x
    stellar contract invoke \
      --id $REDEMPTION_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      add_token \
      --token_contract_address $EUTBL_ADDRESS
    { set +x; } 2>/dev/null
}

add_ustbl_to_redemption() {
    set -x
    stellar contract invoke \
      --id $REDEMPTION_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      add_token \
      --token_contract_address $USTBL_ADDRESS
    { set +x; } 2>/dev/null
}

add_eur_ustbl_to_redemption() {
    set -x
    stellar contract invoke \
      --id $REDEMPTION_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      add_token \
      --token_contract_address $EUR_USTBL_ADDRESS
    { set +x; } 2>/dev/null
}

add_uktbl_to_redemption() {
    set -x
    stellar contract invoke \
      --id $REDEMPTION_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      add_token \
      --token_contract_address $UKTBL_ADDRESS
    { set +x; } 2>/dev/null
}

add_spkcc_to_redemption() {
    set -x
    stellar contract invoke \
      --id $REDEMPTION_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      add_token \
      --token_contract_address $SPKCC_ADDRESS
    { set +x; } 2>/dev/null
}

add_eur_spkcc_to_redemption() {
    set -x
    stellar contract invoke \
      --id $REDEMPTION_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      add_token \
      --token_contract_address $EUR_SPKCC_ADDRESS
    { set +x; } 2>/dev/null
}

set_permission_manager_on_eutbl() {
    set -x
    stellar contract invoke \
      --id $EUTBL_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      set_permission_manager \
      --permission_manager $PERMISSION_MANAGER_ADDRESS
    { set +x; } 2>/dev/null
}

set_redemption_on_eutbl() {
    set -x
    stellar contract invoke \
      --id $EUTBL_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      set_redemption \
      --redemption $REDEMPTION_ADDRESS
    { set +x; } 2>/dev/null
}

set_permission_manager_on_ustbl() {
    set -x
    stellar contract invoke \
      --id $USTBL_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      set_permission_manager \
      --permission_manager $PERMISSION_MANAGER_ADDRESS
    { set +x; } 2>/dev/null
}

set_redemption_on_ustbl() {
    set -x
    stellar contract invoke \
      --id $USTBL_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      set_redemption \
      --redemption $REDEMPTION_ADDRESS
    { set +x; } 2>/dev/null
}

set_permission_manager_on_eur_ustbl() {
    set -x
    stellar contract invoke \
      --id $EUR_USTBL_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      set_permission_manager \
      --permission_manager $PERMISSION_MANAGER_ADDRESS
    { set +x; } 2>/dev/null
}

set_redemption_on_eur_ustbl() {
    set -x
    stellar contract invoke \
      --id $EUR_USTBL_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      set_redemption \
      --redemption $REDEMPTION_ADDRESS
    { set +x; } 2>/dev/null
}

set_permission_manager_on_uktbl() {
    set -x
    stellar contract invoke \
      --id $UKTBL_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      set_permission_manager \
      --permission_manager $PERMISSION_MANAGER_ADDRESS
    { set +x; } 2>/dev/null
}

set_redemption_on_uktbl() {
    set -x
    stellar contract invoke \
      --id $UKTBL_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      set_redemption \
      --redemption $REDEMPTION_ADDRESS
    { set +x; } 2>/dev/null
}

set_permission_manager_on_spkcc() {
    set -x
    stellar contract invoke \
      --id $SPKCC_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      set_permission_manager \
      --permission_manager $PERMISSION_MANAGER_ADDRESS
    { set +x; } 2>/dev/null
}

set_redemption_on_spkcc() {
    set -x
    stellar contract invoke \
      --id $SPKCC_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      set_redemption \
      --redemption $REDEMPTION_ADDRESS
    { set +x; } 2>/dev/null
}

set_permission_manager_on_eur_spkcc() {
    set -x
    stellar contract invoke \
      --id $EUR_SPKCC_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      set_permission_manager \
      --permission_manager $PERMISSION_MANAGER_ADDRESS
    { set +x; } 2>/dev/null
}

set_redemption_on_eur_spkcc() {
    set -x
    stellar contract invoke \
      --id $EUR_SPKCC_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      set_redemption \
      --redemption $REDEMPTION_ADDRESS
    { set +x; } 2>/dev/null
}

give_whitelisted_role_to_redemption() {
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
}

give_redemption_executor_role_to_redemption() {
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
}

give_whitelister_role_to_relayer() {
    set -x
    stellar contract invoke \
      --id $PERMISSION_MANAGER_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      grant_role \
      --caller $STELLAR_PROFILE \
      --account $RELAYER_ADDRESS \
      --role WLISTER
    { set +x; } 2>/dev/null
}

give_minter_role_to_relayer() {
    set -x
    stellar contract invoke \
      --id $PERMISSION_MANAGER_ADDRESS \
      --source $STELLAR_PROFILE \
      --network $NETWORK \
      -- \
      grant_role \
      --caller $STELLAR_PROFILE \
      --account $RELAYER_ADDRESS \
      --role MINTER
    { set +x; } 2>/dev/null
}

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
initialize_permission_manager

echo "🔄 Setup Redemption"
echo "---> Set Permission Manager"
set_permission_manager_on_redemption

echo "---> Add Token EUTBL"
add_eutbl_to_redemption

echo "---> Add Token USTBL"
add_ustbl_to_redemption

echo "---> Add Token EUR_USTBL"
add_eur_ustbl_to_redemption

echo "---> Add Token UKTBL"
add_uktbl_to_redemption

echo "---> Add Token SPKCC"
add_spkcc_to_redemption

echo "---> Add Token EUR_SPKCC"
add_eur_spkcc_to_redemption

echo "🔄 Setup EUTBL"
echo "---> Set Permission Manager"
set_permission_manager_on_eutbl
echo "---> Set Redemption"
set_redemption_on_eutbl

echo "🔄 Setup USTBL"
echo "---> Set Permission Manager"
set_permission_manager_on_ustbl
echo "---> Set Redemption"
set_redemption_on_ustbl

echo "🔄 Setup EUR_USTBL"
echo "---> Set Permission Manager"
set_permission_manager_on_eur_ustbl
echo "---> Set Redemption"
set_redemption_on_eur_ustbl

echo "🔄 Setup UKTBL"
echo "---> Set Permission Manager"
set_permission_manager_on_uktbl
echo "---> Set Redemption"
set_redemption_on_uktbl

echo "🔄 Setup SPKCC"
echo "---> Set Permission Manager"
set_permission_manager_on_spkcc
echo "---> Set Redemption"
set_redemption_on_spkcc

echo "🔄 Setup EUR_SPKCC"
echo "---> Set Permission Manager"
set_permission_manager_on_eur_spkcc
echo "---> Set Redemption"
set_redemption_on_eur_spkcc

echo "🔄 Setup Role"
echo "---> Give WHITELISTED_ROLE to redemption"
give_whitelisted_role_to_redemption
echo "---> Give REDEMPTION_EXECUTOR_ROLE to redemption"
give_redemption_executor_role_to_redemption
echo "---> Give WHITELISTER_ROLE to relayer"
give_whitelister_role_to_relayer
echo "---> Give MINTER_ROLE to relayer"
give_minter_role_to_relayer