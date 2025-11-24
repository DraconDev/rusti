#!/bin/bash

# Setup script to install automated testing
# Usage: ./setup-automated-tests.sh

echo "🚀 Setting up automated testing for go-templ-htmx-ex..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}📋 This script will set up:${NC}"
echo "  ✅ GitHub Actions workflow for CI/CD"
echo "  ✅ Pre-commit hook for local testing"
echo "  ✅ Test validation and formatting checks"
echo ""

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo -e "${RED}❌ Not in a git repository. Please run this from the project root.${NC}"
    exit 1
fi

# Check if .github/workflows directory exists
if [ ! -d ".github/workflows" ]; then
    echo -e "${YELLOW}📁 Creating .github/workflows directory...${NC}"
    mkdir -p .github/workflows
fi

# Check if GitHub Actions workflow exists
if [ -f ".github/workflows/tests.yml" ]; then
    echo -e "${GREEN}✅ GitHub Actions workflow already exists${NC}"
else
    echo -e "${RED}❌ GitHub Actions workflow not found!${NC}"
    echo "Please ensure .github/workflows/tests.yml exists"
    exit 1
fi

# Install pre-commit hook
echo -e "${BLUE}🪝 Setting up pre-commit hook...${NC}"

# Check if pre-commit hook already exists
if [ -f ".git/hooks/pre-commit" ]; then
    echo -e "${YELLOW}⚠️  Pre-commit hook already exists${NC}"
    read -p "Do you want to overwrite it? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Skipping pre-commit hook installation"
    else
        cp pre-commit-hook.sh .git/hooks/pre-commit
        chmod +x .git/hooks/pre-commit
        echo -e "${GREEN}✅ Pre-commit hook installed successfully${NC}"
    fi
else
    cp pre-commit-hook.sh .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
    echo -e "${GREEN}✅ Pre-commit hook installed successfully${NC}"
fi

# Test the pre-commit hook
echo -e "${BLUE}🧪 Testing pre-commit hook...${NC}"
echo "Run this to test: ./.git/hooks/pre-commit"

echo ""
echo -e "${GREEN}🎉 Automated testing setup complete!${NC}"
echo ""
echo -e "${BLUE}📋 What happens now:${NC}"
echo "  🔄 GitHub Actions: Tests run automatically on every push/PR"
echo "  🪝 Pre-commit: Tests run locally before each commit"
echo ""
echo -e "${BLUE}🚀 Next steps:${NC}"
echo "  1. Commit and push your changes to trigger GitHub Actions"
echo "  2. Test the pre-commit hook: ./.git/hooks/pre-commit"
echo "  3. View test results in GitHub Actions tab"
echo ""
echo -e "${BLUE}📚 For more info:${NC}"
echo "  - GitHub Actions: Check the Actions tab in your repo"
echo "  - Pre-commit hook: Runs automatically on 'git commit'"
echo "  - Manual testing: ./test_auth_endpoints.sh"