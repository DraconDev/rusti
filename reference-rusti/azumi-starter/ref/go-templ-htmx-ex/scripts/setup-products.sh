#!/bin/bash

# =============================================================================
# STRIPE PRODUCT SETUP SCRIPT
# =============================================================================
# This script registers your SaaS products with the Payment Microservice,
# which creates them in Stripe and returns the Product/Price IDs.
#
# Usage:
#   ./scripts/setup-products.sh
#
# Prerequisites:
#   - Payment MS must be running
#   - Payment MS must have /admin/products/register endpoint implemented
#   - PAYMENT_MS_URL and PAYMENT_MS_ADMIN_KEY must be set in .env
# =============================================================================

set -e  # Exit on error

echo "🚀 SaaS Starter - Stripe Product Setup"
echo "======================================="
echo ""

# Check if .env exists
if [ ! -f .env ]; then
    echo "❌ Error: .env file not found!"
    echo "📝 Please copy .env.example to .env first:"
    echo "   cp .env.example .env"
    exit 1
fi

# Load .env
source .env

# Check required variables
if [ -z "$PAYMENT_MS_URL" ]; then
    echo "❌ Error: PAYMENT_MS_URL not set in .env"
    exit 1
fi

if [ -z "$PAYMENT_MS_API_KEY" ]; then
    echo "❌ Error: PAYMENT_MS_API_KEY not set in .env"
    echo "💡 This is the same API key you use for regular Payment MS operations"
    exit 1
fi

# Check if config file exists
CONFIG_FILE="scripts/setup-config.json"

if [ -f "$CONFIG_FILE" ]; then
    echo "📄 Found config file: $CONFIG_FILE"
    echo "   Reading product configuration..."
    echo ""
    
    # Check if jq is installed
    if ! command -v jq &> /dev/null; then
        echo "❌ Error: jq is required to read config file but is not installed"
        echo "💡 Install jq: sudo apt-get install jq (Ubuntu) or brew install jq (Mac)"
        echo "   Or remove $CONFIG_FILE to use interactive mode"
        exit 1
    fi
    
    # Read and validate config
    if ! jq empty "$CONFIG_FILE" 2>/dev/null; then
        echo "❌ Error: Invalid JSON in $CONFIG_FILE"
        exit 1
    fi
    
    PROJECT_NAME=$(jq -r '.project_name' "$CONFIG_FILE")
    
    # Display config summary
    echo "📋 Product Configuration from $CONFIG_FILE:"
    echo "   Project: $PROJECT_NAME"
    jq -r '.plans[] | "   Plan: \(.name) - Monthly: $\(.pricing.monthly / 100) / Yearly: $\(.pricing.yearly / 100)"' "$CONFIG_FILE"
    echo ""
    
    read -p "✅ Proceed with this configuration? (y/n): " CONFIRM
    
    if [ "$CONFIRM" != "y" ] && [ "$CONFIRM" != "Y" ]; then
        echo "❌ Setup cancelled"
        exit 0
    fi
    
    # Use config file as request body
    REQUEST_BODY=$(cat "$CONFIG_FILE")
else
    echo "� No config file found. Using interactive mode..."
    echo "💡 Tip: Create scripts/setup-config.json to skip prompts next time"
    echo ""
    
    # Prompt for project name
    read -p "📦 Enter your project name (e.g., my-awesome-saas): " PROJECT_NAME
    
    if [ -z "$PROJECT_NAME" ]; then
        echo "❌ Error: Project name is required"
        exit 1
    fi
    
    echo ""
    echo "📋 Product Configuration:"
    echo "   Project: $PROJECT_NAME"
    echo "   Plan: Pro Plan"
    echo "   Monthly: $29/month"
    echo "   Yearly: $290/year (save $58)"
    echo ""
    
    read -p "✅ Proceed with this configuration? (y/n): " CONFIRM
    
    if [ "$CONFIRM" != "y" ] && [ "$CONFIRM" != "Y" ]; then
        echo "❌ Setup cancelled"
        exit 0
    fi
    
    # Build request body
    REQUEST_BODY="{
        \"project_name\": \"$PROJECT_NAME\",
        \"plans\": [{
            \"name\": \"Pro Plan\",
            \"description\": \"Professional features for power users\",
            \"features\": [\"unlimited_projects\", \"advanced_analytics\", \"priority_support\", \"api_access\"],
            \"pricing\": {
                \"monthly\": 2900,
                \"yearly\": 29000
            }
        }]
    }"
fi

echo ""
echo "🔧 Registering products with Payment MS..."
echo "   URL: $PAYMENT_MS_URL/admin/products/register"
echo ""

# Call Payment MS API
RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "$PAYMENT_MS_URL/admin/products/register" \
  -H "X-API-Key: $PAYMENT_MS_API_KEY" \
  -H "Content-Type: application/json" \
  -d "$REQUEST_BODY")

# Split response body and status code
HTTP_BODY=$(echo "$RESPONSE" | head -n -1)
HTTP_STATUS=$(echo "$RESPONSE" | tail -n 1)

# Check if request was successful
if [ "$HTTP_STATUS" -ne 200 ] && [ "$HTTP_STATUS" -ne 201 ]; then
    echo "❌ Error: Payment MS returned status $HTTP_STATUS"
    echo "Response: $HTTP_BODY"
    echo ""
    echo "💡 Troubleshooting:"
    echo "   1. Ensure Payment MS is running at $PAYMENT_MS_URL"
    echo "   2. Verify PAYMENT_MS_API_KEY is correct"
    echo "   3. Check Payment MS has /admin/products/register endpoint"
    exit 1
fi

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "⚠️  Warning: jq is not installed. Cannot auto-update .env"
    echo "📝 Response from Payment MS:"
    echo "$HTTP_BODY" | python3 -m json.tool 2>/dev/null || echo "$HTTP_BODY"
    echo ""
    echo "📋 Please manually add these to your .env file"
    exit 0
fi

# Parse response
SUCCESS=$(echo "$HTTP_BODY" | jq -r '.success // false')

if [ "$SUCCESS" != "true" ]; then
    echo "❌ Error: Payment MS returned unsuccessful response"
    echo "$HTTP_BODY" | jq '.'
    exit 1
fi

# Extract product IDs
PRODUCT_ID=$(echo "$HTTP_BODY" | jq -r '.products[0].stripe_product_id')
PRICE_MONTHLY=$(echo "$HTTP_BODY" | jq -r '.products[0].prices.monthly.stripe_price_id')
PRICE_YEARLY=$(echo "$HTTP_BODY" | jq -r '.products[0].prices.yearly.stripe_price_id')

# Validate IDs were extracted
if [ -z "$PRODUCT_ID" ] || [ "$PRODUCT_ID" == "null" ]; then
    echo "❌ Error: Could not extract Product ID from response"
    echo "$HTTP_BODY" | jq '.'
    exit 1
fi

echo "✅ Products registered successfully!"
echo ""
echo "📦 Product Details:"
echo "   Product ID: $PRODUCT_ID"
echo "   Monthly Price ID: $PRICE_MONTHLY"
echo "   Yearly Price ID: $PRICE_YEARLY"
echo ""

# Update .env file
echo "📝 Updating .env file..."

# Check if IDs already exist in .env
if grep -q "STRIPE_PRODUCT_PRO=" .env; then
    # Update existing values
    sed -i.bak "s|STRIPE_PRODUCT_PRO=.*|STRIPE_PRODUCT_PRO=$PRODUCT_ID|" .env
    sed -i.bak "s|STRIPE_PRICE_MONTHLY=.*|STRIPE_PRICE_MONTHLY=$PRICE_MONTHLY|" .env
    sed -i.bak "s|STRIPE_PRICE_YEARLY=.*|STRIPE_PRICE_YEARLY=$PRICE_YEARLY|" .env
    rm .env.bak
else
    # Append new values
    cat >> .env << EOF

# Stripe Products (auto-generated by setup-products.sh)
STRIPE_PRODUCT_PRO=$PRODUCT_ID
STRIPE_PRICE_MONTHLY=$PRICE_MONTHLY
STRIPE_PRICE_YEARLY=$PRICE_YEARLY
EOF
fi

echo "✅ .env file updated!"
echo ""
echo "🎉 Setup complete! Your Stripe products are ready."
echo ""
echo "📌 Next steps:"
echo "   1. Restart your application to load new config"
echo "   2. Visit /pricing to see the pricing page"
echo "   3. Test the checkout flow"
echo ""
