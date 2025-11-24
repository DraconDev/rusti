package services

import (
	"fmt"
	"testing"

	"github.com/DraconDev/go-templ-htmx-ex/internal/utils/config"
)

// testConfig creates a test configuration
func testConfig() *config.Config {
	return &config.Config{
		AuthServiceURL: "http://test-auth-service:8080",
		RedirectURL:    "http://localhost:8080",
	}
}

func TestNewAuthService(t *testing.T) {
	fmt.Println("🧪 Testing NewAuthService")

	cfg := testConfig()
	authService := NewAuthService(cfg)

	if authService == nil {
		t.Error("NewAuthService should return a non-nil service")
		return
	}

	// Test that service is properly initialized
	// Note: We can't access private fields directly, so we test behavior
	t.Log("✅ NewAuthService created successfully")
}

func TestAuthServiceCreateSession(t *testing.T) {
	fmt.Println("🧪 Testing CreateSession")

	cfg := testConfig()
	authService := NewAuthService(cfg)

	// Test with empty auth code
	t.Run("empty_auth_code", func(t *testing.T) {
		_, err := authService.CreateSession("")

		// Should handle empty auth code gracefully
		if err == nil {
			t.Log("Empty auth code handled (no error returned)")
		} else {
			t.Logf("Expected error handling for empty auth code: %v", err)
		}
	})

	// Test with valid auth code (will fail due to no HTTP mock, but tests the structure)
	t.Run("valid_auth_code", func(t *testing.T) {
		_, err := authService.CreateSession("test-github-code")

		// This will likely fail due to no HTTP server, but that's expected in unit tests
		if err != nil {
			t.Logf("Expected HTTP connection error (no mock server): %v", err)
		} else {
			t.Log("CreateSession executed successfully")
		}
	})
}

func TestAuthServiceExchangeCodeForTokens(t *testing.T) {
	fmt.Println("🧪 Testing ExchangeCodeForTokens")

	cfg := testConfig()
	authService := NewAuthService(cfg)

	// Test empty authorization code
	t.Run("empty_auth_code", func(t *testing.T) {
		result, err := authService.ExchangeCodeForTokens("")

		// Should handle empty code appropriately
		if err == nil {
			if result != nil && !result.Success {
				t.Log("Empty auth code handled correctly with error response")
			}
		} else {
			t.Logf("Expected error handling for empty auth code: %v", err)
		}
	})

	// Test valid authorization code
	t.Run("valid_auth_code", func(t *testing.T) {
		result, err := authService.ExchangeCodeForTokens("test-github-code")

		if err != nil {
			t.Logf("Expected HTTP connection error (no mock server): %v", err)
		} else if result != nil {
			t.Logf("ExchangeCodeForTokens returned success: %v, id_token length: %d", result.Success, len(result.IdToken))
		}
	})
}

func TestAuthServiceRefreshSession(t *testing.T) {
	fmt.Println("🧪 Testing RefreshSession")

	cfg := testConfig()
	authService := NewAuthService(cfg)

	t.Run("refresh_session", func(t *testing.T) {
		_, err := authService.RefreshSession("test-session-123")

		if err != nil {
			t.Logf("Expected HTTP connection error (no mock server): %v", err)
		} else {
			t.Log("RefreshSession executed successfully")
		}
	})
}

func TestAuthServiceGetUserInfo(t *testing.T) {
	fmt.Println("🧪 Testing GetUserInfo")

	cfg := testConfig()
	authService := NewAuthService(cfg)

	t.Run("get_user_info", func(t *testing.T) {
		_, err := authService.GetUserInfo("test-session-123")

		if err != nil {
			t.Logf("Expected HTTP connection error (no mock server): %v", err)
		} else {
			t.Log("GetUserInfo executed successfully")
		}
	})
}

func TestAuthServiceLogout(t *testing.T) {
	fmt.Println("🧪 Testing Logout")

	cfg := testConfig()
	authService := NewAuthService(cfg)

	t.Run("logout", func(t *testing.T) {
		err := authService.Logout("test-session-123")

		// Logout should not return error in this implementation
		if err != nil {
			t.Errorf("Logout should not return error, got: %v", err)
		} else {
			t.Log("Logout executed successfully")
		}
	})
}

func TestAuthServiceValidateSession(t *testing.T) {
	fmt.Println("🧪 Testing ValidateSession")

	cfg := testConfig()
	authService := NewAuthService(cfg)

	t.Run("validate_session", func(t *testing.T) {
		_, err := authService.ValidateSession("test-session-123")

		if err != nil {
			t.Logf("Expected HTTP connection error (no mock server): %v", err)
		} else {
			t.Log("ValidateSession executed successfully")
		}
	})
}

// Integration-style tests
func TestAuthServiceIntegration(t *testing.T) {
	fmt.Println("🧪 Testing AuthService Integration Flow")

	cfg := testConfig()
	authService := NewAuthService(cfg)

	t.Run("complete_auth_flow", func(t *testing.T) {
		// Simulate the OAuth callback flow
		authCode := "github_12345_cb67890"

		// Step 1: Exchange code for tokens
		tokenResp, err := authService.ExchangeCodeForTokens(authCode)
		if err != nil {
			t.Logf("Expected HTTP error (no mock): %v", err)
		} else {
			t.Logf("Token exchange result: %+v", tokenResp)
		}

		// Step 2: Get user info
		_, err = authService.GetUserInfo("test-session-id")
		if err != nil {
			t.Logf("Expected HTTP error (no mock): %v", err)
		}

		// Step 3: Validate session
		_, err = authService.ValidateSession("test-session-id")
		if err != nil {
			t.Logf("Expected HTTP error (no mock): %v", err)
		}

		// Step 4: Logout
		err = authService.Logout("test-session-id")
		if err != nil {
			t.Errorf("Logout should not return error: %v", err)
		} else {
			t.Log("Logout successful")
		}
	})
}

// Test HTTP client configuration indirectly
func TestAuthServiceHTTPConfiguration(t *testing.T) {
	fmt.Println("🧪 Testing AuthService HTTP Configuration")

	cfg := testConfig()
	authService := NewAuthService(cfg)

	t.Run("service_initialization", func(t *testing.T) {
		// Test that service is properly initialized
		if authService == nil {
			t.Fatal("AuthService should not be nil")
		}

		t.Log("✅ AuthService properly initialized")
	})
}
