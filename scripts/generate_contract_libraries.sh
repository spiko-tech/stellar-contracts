#!/bin/bash

# Default to dev environment
ENVIRONMENT=${1:-dev}
NETWORK=${2:-testnet}

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed. Please install jq first."
    echo "On macOS: brew install jq"
    echo "On Ubuntu/Debian: sudo apt-get install jq"
    return 1
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

generate_permission_manager_library() {
    set -x
    stellar contract bindings typescript \
      --network ${NETWORK} \
      --contract-id $PERMISSION_MANAGER_ADDRESS \
      --output-dir .packages/$ENVIRONMENT/permission_manager
    { set +x; } 2>/dev/null
}

generate_redemption_library() {
    set -x
    stellar contract bindings typescript \
      --network ${NETWORK} \
      --contract-id $REDEMPTION_ADDRESS \
      --output-dir .packages/$ENVIRONMENT/redemption
    { set +x; } 2>/dev/null
}

generate_eutbl_library() {
    set -x
    stellar contract bindings typescript \
      --network ${NETWORK} \
      --contract-id $EUTBL_ADDRESS \
      --output-dir .packages/$ENVIRONMENT/eutbl
    { set +x; } 2>/dev/null
}

generate_ustbl_library() {
    set -x
    stellar contract bindings typescript \
      --network ${NETWORK} \
      --contract-id $USTBL_ADDRESS \
      --output-dir .packages/$ENVIRONMENT/ustbl
    { set +x; } 2>/dev/null
}

generate_eur_ustbl_library() {
    set -x
    stellar contract bindings typescript \
      --network ${NETWORK} \
      --contract-id $EUR_USTBL_ADDRESS \
      --output-dir .packages/$ENVIRONMENT/eur_ustbl
    { set +x; } 2>/dev/null
}

generate_uktbl_library() {
    set -x
    stellar contract bindings typescript \
      --network ${NETWORK} \
      --contract-id $UKTBL_ADDRESS \
      --output-dir .packages/$ENVIRONMENT/uktbl
    { set +x; } 2>/dev/null
}

generate_spkcc_library() {
    set -x
    stellar contract bindings typescript \
      --network ${NETWORK} \
      --contract-id $SPKCC_ADDRESS \
      --output-dir .packages/$ENVIRONMENT/spkcc
    { set +x; } 2>/dev/null
}

generate_eur_spkcc_library() {
    set -x
    stellar contract bindings typescript \
      --network ${NETWORK} \
      --contract-id $EUR_SPKCC_ADDRESS \
      --output-dir .packages/$ENVIRONMENT/eur_spkcc
    { set +x; } 2>/dev/null
}

echo "🌍 Environment: $ENVIRONMENT"
echo "📋 Loaded Addresses:"
echo "  Permission Manager: $PERMISSION_MANAGER_ADDRESS"
echo "  Redemption: $REDEMPTION_ADDRESS"
echo "  EUTBL: $EUTBL_ADDRESS"
echo "  USTBL: $USTBL_ADDRESS"
echo "  eurUSTBL: $EUR_USTBL_ADDRESS"
echo "  UKTBL: $UKTBL_ADDRESS"
echo "  SPKCC: $SPKCC_ADDRESS"
echo "  eurSPKCC: $EUR_SPKCC_ADDRESS"

echo "🧹 Clean old libraries"
rm -rf .packages/$ENVIRONMENT

echo "🔄 Generate Permission Manager Library"
generate_permission_manager_library

echo "🔄 Generate Redemption Library"
generate_redemption_library

echo "🔄 Generate EUTBL Library"
generate_eutbl_library

echo "🔄 Generate USTBL Library"
add_ustbl_to_redemption

echo "🔄 Generate EUR_USTBL Library"
generate_eur_ustbl_library

echo "🔄 Generate UKTBL Library"
add_uktbl_to_redemption

echo "🔄 Generate SPKCC Library"
generate_spkcc_library

echo "🔄 Generate EUR_SPKCC Library"
generate_eur_spkcc_library