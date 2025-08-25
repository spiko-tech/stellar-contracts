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

# Check if wasm files exist
if [ ! -f "target/wasm32v1-none/release/permission_manager.wasm" ] || [ ! -f "target/wasm32v1-none/release/redemption.wasm" ] || [ ! -f "target/wasm32v1-none/release/token.wasm" ]; then
    echo "Error: Required WASM files not found. Please build the contracts first:"
    echo "  stellar contract build"
    exit 1
fi

echo "🚀 Starting contract deployment..."
echo "🌍 Environment: $ENVIRONMENT"
echo "👤 Stellar Profile: $STELLAR_PROFILE"
echo "🌐 Network: $NETWORK"
echo "👑 Admin/Owner: $ADMIN_ADDRESS"
echo "📁 Config File: $CONFIG_FILE"
echo ""

# Function to deploy permission manager
deploy_permission_manager() {
    echo "🔐 Deploying Permission Manager..."
    set -x
    PERMISSION_MANAGER_ID=$(stellar contract deploy \
        --source $STELLAR_PROFILE \
        --network $NETWORK \
        --alias permission_manager_${ENVIRONMENT} \
        --wasm target/wasm32v1-none/release/permission_manager.wasm \
        -- \
        --admin $ADMIN_ADDRESS)
    { set +x; } 2>/dev/null
    
    if [ $? -eq 0 ]; then
        echo "✅ Permission Manager deployed successfully"
        echo "📋 Permission Manager Contract ID: $PERMISSION_MANAGER_ID"
    else
        echo "❌ Failed to deploy Permission Manager"
        exit 1
    fi
}

# Function to deploy redemption contract
deploy_redemption() {
    echo "🔄 Deploying Redemption Contract..."
    set -x
    REDEMPTION_ID=$(stellar contract deploy \
        --source $STELLAR_PROFILE \
        --network $NETWORK \
        --alias redemption_${ENVIRONMENT} \
        --wasm target/wasm32v1-none/release/redemption.wasm \
        -- \
        --owner $ADMIN_ADDRESS)
    { set +x; } 2>/dev/null
    
    if [ $? -eq 0 ]; then
        echo "✅ Redemption Contract deployed successfully"
        echo "📋 Redemption Contract ID: $REDEMPTION_ID"
    else
        echo "❌ Failed to deploy Redemption Contract"
        exit 1
    fi
}

# Function to deploy token contracts
deploy_tokens() {
    echo "🪙 Deploying Token Contracts..."
    
    # Initialize arrays to store token data
    TOKEN_SYMBOLS=()
    TOKEN_KEYS=()
    TOKEN_IDS=()
    
    # Read tokens from config file (new format: object with named keys)
    TOKEN_KEYS_LIST=$(jq -r '.tokens | keys[]' "$CONFIG_FILE")
    
    for token_key in $TOKEN_KEYS_LIST; do
        # Get token data for this key
        TOKEN_NAME=$(jq -r ".tokens.$token_key.name" "$CONFIG_FILE")
        TOKEN_SYMBOL=$(jq -r ".tokens.$token_key.symbol" "$CONFIG_FILE")
        TOKEN_DECIMALS=$(jq -r ".tokens.$token_key.decimals" "$CONFIG_FILE")
        
        echo "  🪙 Deploying $TOKEN_SYMBOL ($TOKEN_NAME)..."
        
        set -x
        TOKEN_ID=$(stellar contract deploy \
            --source $STELLAR_PROFILE \
            --network $NETWORK \
            --alias $(echo "$TOKEN_SYMBOL" | tr '[:upper:]' '[:lower:]')_${ENVIRONMENT} \
            --wasm target/wasm32v1-none/release/token.wasm \
            -- \
            --owner $ADMIN_ADDRESS \
            --name "$TOKEN_NAME" \
            --symbol $TOKEN_SYMBOL \
            --decimals $TOKEN_DECIMALS)
        { set +x; } 2>/dev/null
        
        if [ $? -eq 0 ]; then
            echo "    ✅ $TOKEN_SYMBOL deployed successfully"
            echo "    📋 $TOKEN_SYMBOL Contract ID: $TOKEN_ID"
            
            # Store token data for later use
            TOKEN_SYMBOLS+=("$TOKEN_SYMBOL")
            TOKEN_KEYS+=("$token_key")
            TOKEN_IDS+=("$TOKEN_ID")
            
            echo "    🔍 Debug: Added $token_key -> $TOKEN_ID to arrays"
            echo "    🔍 Debug: Arrays now contain ${#TOKEN_KEYS[@]} elements"
        else
            echo "    ❌ Failed to deploy $TOKEN_SYMBOL"
            exit 1
        fi
    done
    
    # Store token data in global variables for other functions
    TOKEN_SYMBOLS_STR="${TOKEN_SYMBOLS[*]}"
    TOKEN_KEYS_STR="${TOKEN_KEYS[*]}"
    TOKEN_IDS_STR="${TOKEN_IDS[*]}"
    
    echo "🔍 Debug: Final arrays contain ${#TOKEN_KEYS[@]} elements"
    echo "🔍 Debug: TOKEN_KEYS_STR: $TOKEN_KEYS_STR"
    echo "🔍 Debug: TOKEN_IDS_STR: $TOKEN_IDS_STR"
}

# Function to save deployed addresses to address file
save_addresses() {
    echo "💾 Saving deployed addresses..."
    
    # Create address directory if it doesn't exist
    mkdir -p address
    
    # Create the address JSON structure
    ADDRESS_JSON=$(cat <<EOF
{
    "permissionManager": "$PERMISSION_MANAGER_ID",
    "redemption": "$REDEMPTION_ID",
    "tokens": {
EOF
)
    
    # Add token addresses using stored IDs
    FIRST=true
    
    echo "🔍 Debug: TOKEN_KEYS_STR: $TOKEN_KEYS_STR"
    echo "🔍 Debug: TOKEN_IDS_STR: $TOKEN_IDS_STR"
    
    # Convert space-separated strings back to arrays
    IFS=' ' read -ra TOKEN_KEYS <<< "$TOKEN_KEYS_STR"
    IFS=' ' read -ra TOKEN_IDS <<< "$TOKEN_IDS_STR"
    
    echo "🔍 Debug: Reconstructed arrays contain ${#TOKEN_KEYS[@]} elements"
    
    for i in "${!TOKEN_KEYS[@]}"; do
        TOKEN_KEY="${TOKEN_KEYS[$i]}"
        TOKEN_ID="${TOKEN_IDS[$i]}"
        
        echo "🔍 Debug: Processing token $i: $TOKEN_KEY -> $TOKEN_ID"
        
        if [ "$FIRST" = true ]; then
            FIRST=false
        else
            ADDRESS_JSON="$ADDRESS_JSON,"
        fi
        
        ADDRESS_JSON="$ADDRESS_JSON
        \"$TOKEN_KEY\": \"$TOKEN_ID\""
    done
    
    ADDRESS_JSON="$ADDRESS_JSON
    }
}"
    
    # Save to address file
    echo "$ADDRESS_JSON" > "address/${ENVIRONMENT}.json"
    echo "✅ Addresses saved to address/${ENVIRONMENT}.json"
}

# Function to display deployment summary
show_summary() {
    echo ""
    echo "🎉 Deployment completed successfully!"
    echo ""
    echo "📋 Deployment Summary:"
    echo "  🔐 Permission Manager: $PERMISSION_MANAGER_ID"
    echo "  🔄 Redemption Contract: $REDEMPTION_ID"
    echo ""
    echo "🪙 Token Contracts:"
    
    # Display token contracts using stored IDs
    # Convert space-separated strings back to arrays
    IFS=' ' read -ra TOKEN_KEYS <<< "$TOKEN_KEYS_STR"
    IFS=' ' read -ra TOKEN_SYMBOLS <<< "$TOKEN_SYMBOLS_STR"
    IFS=' ' read -ra TOKEN_IDS <<< "$TOKEN_IDS_STR"
    
    for i in "${!TOKEN_KEYS[@]}"; do
        TOKEN_KEY="${TOKEN_KEYS[$i]}"
        TOKEN_SYMBOL="${TOKEN_SYMBOLS[$i]}"
        TOKEN_ID="${TOKEN_IDS[$i]}"
        echo "    $TOKEN_KEY ($TOKEN_SYMBOL): $TOKEN_ID"
    done
    
    echo ""
    echo "📁 Addresses saved to: address/${ENVIRONMENT}.json"
    echo ""
    echo "🔄 Next steps:"
    echo "  1. Run setup script: ./scripts/setup_contracts.sh $ENVIRONMENT $STELLAR_PROFILE"
    echo "  2. Verify contracts on Stellar Explorer"
    echo ""
}

# Main deployment flow
main() {
    echo "🚀 Starting deployment process..."
    echo ""
    
    # Deploy contracts
    deploy_permission_manager
    echo ""
    
    deploy_redemption
    echo ""
    
    deploy_tokens
    echo ""
    
    # Save addresses
    save_addresses
    echo ""
    
    # Show summary
    show_summary
}

# Run main function
main
