#!/bin/bash

# Integration tests for authentication endpoints
echo "🧪 Starting Authentication Integration Tests"
echo "========================================"

# Test 1: Health check endpoint
echo "Test 1: Health Check Endpoint"
echo "----------------------------"
curl -s -w "Status: %{http_code}\n" http://localhost:8080/health
echo ""

# Test 2: Login page endpoint  
echo "Test 2: Login Page Endpoint"
echo "--------------------------"
curl -s -w "Status: %{http_code}\n" http://localhost:8080/login
echo ""

# Test 3: OAuth callback endpoint (simulated)
echo "Test 3: OAuth Callback Endpoint"
echo "-------------------------------"
curl -s -w "Status: %{http_code}\n" "http://localhost:8080/auth/callback?auth_code=github_test123"
echo ""

# Test 4: Exchange code API endpoint (should not require authentication)
echo "Test 4: Exchange Code API Endpoint"
echo "---------------------------------"
curl -s -X POST -H "Content-Type: application/json" \
     -d '{"auth_code":"github_test123"}' \
     -w "Status: %{http_code}\n" \
     http://localhost:8080/api/auth/exchange-code
echo ""

# Test 5: Protected endpoint without authentication
echo "Test 5: Protected Profile Endpoint (No Auth)"
echo "-------------------------------------------"
curl -s -w "Status: %{http_code}\n" http://localhost:8080/profile
echo ""

# Test 6: Protected admin endpoint without authentication  
echo "Test 6: Protected Admin Endpoint (No Auth)"
echo "-----------------------------------------"
curl -s -w "Status: %{http_code}\n" http://localhost:8080/admin
echo ""

# Test 7: Admin API endpoint without authentication
echo "Test 7: Admin API Endpoint (No Auth)"
echo "----------------------------------"
curl -s -w "Status: %{http_code}\n" http://localhost:8080/api/admin/users
echo ""

echo "========================================"
echo "✅ Integration Tests Complete"
echo ""
echo "Expected Results:"
echo "- Health: 200 OK"
echo "- Login: 200 OK" 
echo "- Callback: 200 OK (renders HTML)"
echo "- Exchange Code: Should NOT redirect to login (middleware fix working)"
echo "- Profile: Should redirect to login"
echo "- Admin: Should redirect to login"  
echo "- Admin API: Should return 401 JSON"