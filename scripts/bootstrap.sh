#!/bin/bash

#### Usage: ./scripts/bootstrap.sh dev nicolas testnet GB7BUX5B2UCSPTBC3UX4O6MRO5OPEZV2CK7FEVONU5Q7WEISLRRNT3S7

# Default to dev environment
ENVIRONMENT=${1}
STELLAR_PROFILE=${2}
NETWORK=${3}
RELAYER_ADDRESS=${4}

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
    echo "Usage: $0 [ENVIRONMENT] [STELLAR_PROFILE] [NETWORK] [RELAYER_ADDRESS]"
    echo "Example: $0 dev nicolas testnet"
    exit 1
fi

# Check if relayer address provided
if [ -z "$RELAYER_ADDRESS" ]; then
    echo "Error: Relayer address is required. Please provide a relayer address."
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
./scripts/setup_contracts.sh $ENVIRONMENT $STELLAR_PROFILE $NETWORK $RELAYER_ADDRESS
./scripts/generate_contract_libraries.sh $ENVIRONMENT $NETWORK
