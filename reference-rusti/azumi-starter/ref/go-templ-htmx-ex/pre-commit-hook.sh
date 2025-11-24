#!/bin/bash

# Pre-commit hook to run tests before allowing commit
# Copy this file to .git/hooks/pre-commit and make it executable

echo "🔍 Running pre-commit tests..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Run authentication service tests
echo "🧪 Testing Authentication Services..."
if go test ./internal/services/ -v; then
    echo -e "${GREEN}✅ Authentication Service Tests: PASSED${NC}"
else
    echo -e "${RED}❌ Authentication Service Tests: FAILED${NC}"
    echo -e "${YELLOW}Fix the failing tests before committing${NC}"
    exit 1
fi

# Run authentication middleware tests  
echo "🧪 Testing Authentication Middleware..."
if go test ./internal/middleware/ -v; then
    echo -e "${GREEN}✅ Authentication Middleware Tests: PASSED${NC}"
else
    echo -e "${RED}❌ Authentication Middleware Tests: FAILED${NC}"
    echo -e "${YELLOW}Fix the failing tests before committing${NC}"
    exit 1
fi

# Run all internal tests
echo "🧪 Testing All Internal Components..."
if go test ./internal/... -v; then
    echo -e "${GREEN}✅ All Internal Tests: PASSED${NC}"
else
    echo -e "${RED}❌ Internal Tests: FAILED${NC}"
    echo -e "${YELLOW}Fix the failing tests before committing${NC}"
    exit 1
fi

# Build project to check for compilation errors
echo "🔨 Building Project..."
if go build ./...; then
    echo -e "${GREEN}✅ Project Build: SUCCESS${NC}"
else
    echo -e "${RED}❌ Project Build: FAILED${NC}"
    echo -e "${YELLOW}Fix compilation errors before committing${NC}"
    exit 1
fi

# Check Go formatting
echo "📏 Checking Code Formatting..."
if [ "$(gofmt -s -l . | wc -l)" -eq 0 ]; then
    echo -e "${GREEN}✅ Code Formatting: CORRECT${NC}"
else
    echo -e "${YELLOW}⚠️  Code is not properly formatted${NC}"
    echo "Run 'go fmt ./...' to fix formatting"
    gofmt -s -l .
    exit 1
fi

# Check if go.mod is tidy
echo "📦 Checking Module Dependencies..."
if [ -z "$(go mod tidy && git diff --exit-code go.mod go.sum)" ]; then
    echo -e "${GREEN}✅ Dependencies: TIDY${NC}"
else
    echo -e "${YELLOW}⚠️  Dependencies are not tidy${NC}"
    echo "Run 'go mod tidy' to fix"
    exit 1
fi

echo -e "${GREEN}🎉 All tests passed! Commit allowed.${NC}"
exit 0