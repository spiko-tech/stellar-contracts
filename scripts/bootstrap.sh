#!/bin/bash

# Default to dev environment
ENVIRONMENT=${1:-dev}
STELLAR_PROFILE=${2:-nicolas}
NETWORK=${3:-testnet}

# Constant admin/owner address - change this as needed
ADMIN_ADDRESS="GBYIQXBKEB655EB3WTRITS6RR5GXEP6SQRBLPREZHNFYKT7WBMTMPR3H"

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed. Please install jq first."
    echo "On macOS: brew install jq"
    echo "On Ubuntu/Debian: sudo apt-get install jq"
    exit 1
fi

# Check if stellar profile provided
if [ -z "$STELLAR_PROFILE" ]; then
    echo "Error: Stellar profile is required. Please provide a stellar profile."
    echo "Usage: $0 [ENVIRONMENT] [STELLAR_PROFILE] [NETWORK]"
    echo "Example: $0 dev nicolas testnet"
    exit 1
fi

# Check if the config file exists for the specified environment
CONFIG_FILE="config/${ENVIRONMENT}.json"
if [ ! -f "$CONFIG_FILE" ]; then
    echo "Error: $CONFIG_FILE not found. Available environments:"
    ls -1 config/*.json 2>/dev/null | sed 's|config/||' | sed 's|.json||' || echo "No config files found"
    exit 1
fi

cargo test
stellar contract build
./scripts/deploy_contracts.sh $ENVIRONMENT $STELLAR_PROFILE $NETWORK
./scripts/setup_contracts.sh $ENVIRONMENT $STELLAR_PROFILE $NETWORK
./scripts/generate_contract_libraries.sh
