package services

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"

	"github.com/DraconDev/go-templ-htmx-ex/internal/models"
	"github.com/DraconDev/go-templ-htmx-ex/internal/utils/config"
)

// AuthService handles session management with the auth microservice
type AuthService struct {
	config  *config.Config
	client  *http.Client
	timeout time.Duration
}

// NewAuthService creates a new auth service instance
func NewAuthService(cfg *config.Config) *AuthService {
	return &AuthService{
		config:  cfg,
		client:  &http.Client{Timeout: 5 * time.Second}, // Reduced timeout
		timeout: 5 * time.Second,
	}
}

// CreateSession returns map with session_id and user_context for session establishment
func (s *AuthService) CreateSession(auth_code string) (map[string]interface{}, error) {
	return s.callAuthServiceGeneric("/auth/session/create", map[string]string{
		"auth_code": auth_code,
	})
}

// ExchangeCodeForTokens exchanges OAuth authorization code for session tokens
func (s *AuthService) ExchangeCodeForTokens(auth_code string) (*models.TokenExchangeResponse, error) {
	fmt.Printf("🔐 AUTH-SERVICE: Exchanging code for tokens...\n")

	// Call auth service and extract session_id like the working reference
	return s.extractSessionFromResponse(auth_code)
}

// extractSessionFromResponse extracts session_id from auth service response
func (s *AuthService) extractSessionFromResponse(auth_code string) (*models.TokenExchangeResponse, error) {
	// Get raw response like working reference
	response, err := s.makeRequest("/auth/session/create", map[string]string{
		"auth_code": auth_code,
	})
	if err != nil {
		return &models.TokenExchangeResponse{
			Success: false,
			Error:   "Failed to call auth service: " + err.Error(),
		}, err
	}

	// Parse response as map to extract session_id
	var respData map[string]interface{}
	if err := json.Unmarshal(response, &respData); err != nil {
		return &models.TokenExchangeResponse{
			Success: false,
			Error:   "Failed to parse response: " + err.Error(),
		}, err
	}

	// Extract session_id like working reference
	var sessionID string
	var hasSessionID bool

	if sessionInterface, exists := respData["session_id"]; exists {
		if sessionStr, ok := sessionInterface.(string); ok {
			sessionID = sessionStr
			hasSessionID = true
		}
	}

	if !hasSessionID || sessionID == "" {
		return &models.TokenExchangeResponse{
			Success: false,
			Error:   "Missing session_id in auth service response",
		}, fmt.Errorf("missing session_id")
	}

	// Return session_id as IdToken like working reference
	return &models.TokenExchangeResponse{
		Success: true,
		IdToken: sessionID,
	}, nil
}

// RefreshSession refreshes an existing session_id
func (s *AuthService) RefreshSession(session_id string) (*models.AuthResponse, error) {
	return s.callAuthService("/auth/session/refresh", map[string]string{
		"session_id": session_id,
	})
}

// GetUserInfo retrieves user information using session_id by refreshing the session
// This calls /auth/session/refresh which returns SessionRefreshResponse with user_context
func (s *AuthService) GetUserInfo(session_id string) (*models.UserSessionContext, error) {
	bodyBytes, err := s.makeRequest("/auth/session/refresh", map[string]string{
		"session_id": session_id,
	})
	if err != nil {
		return nil, err
	}

	var refreshResp models.SessionRefreshResponse
	if err := json.Unmarshal(bodyBytes, &refreshResp); err != nil {
		return nil, err
	}

	return &refreshResp.UserContext, nil
}

// ValidateSession validates a session_id and returns user information for middleware
func (s *AuthService) ValidateSession(session_id string) (*models.AuthResponse, error) {
	return s.callAuthService("/auth/session/refresh", map[string]string{
		"session_id": session_id,
	})
}

// Logout logs out a user using session_id
func (s *AuthService) Logout(session_id string) error {
	fmt.Printf("User logged out with session_id: %s\n", session_id)
	return nil
}

// callAuthService makes a request to the auth microservice
func (s *AuthService) callAuthService(endpoint string, params map[string]string) (*models.AuthResponse, error) {
	bodyBytes, err := s.makeRequest(endpoint, params)
	if err != nil {
		return nil, err
	}

	var authResp models.AuthResponse
	if err := json.Unmarshal(bodyBytes, &authResp); err != nil {
		return nil, err
	}

	return &authResp, nil
}

// callAuthServiceGeneric makes a request and returns generic response
func (s *AuthService) callAuthServiceGeneric(endpoint string, params map[string]string) (map[string]interface{}, error) {
	bodyBytes, err := s.makeRequest(endpoint, params)
	if err != nil {
		return nil, err
	}

	var response map[string]interface{}
	if err := json.Unmarshal(bodyBytes, &response); err != nil {
		return nil, err
	}

	return response, nil
}

// makeRequest handles the HTTP request to auth microservice
func (s *AuthService) makeRequest(endpoint string, params map[string]string) ([]byte, error) {
	jsonData, err := json.Marshal(params)
	if err != nil {
		return nil, err
	}

	url := s.config.AuthServiceURL + endpoint
	fmt.Printf("🔍 AUTH-SERVICE: Making request to %s\n", url)
	fmt.Printf("🔍 AUTH-SERVICE: Payload: %s\n", string(jsonData))

	req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonData))
	if err != nil {
		return nil, err
	}
	req.Header.Set("Content-Type", "application/json")

	fmt.Printf("🔍 AUTH-SERVICE: Sending HTTP request...\n")
	resp, err := s.client.Do(req)
	if err != nil {
		fmt.Printf("🔍 AUTH-SERVICE: ❌ Request failed: %v\n", err)
		return nil, fmt.Errorf("failed to connect to auth service at %s: %v", s.config.AuthServiceURL, err)
	}
	defer resp.Body.Close()

	// Debug logging
	fmt.Printf("🔍 AUTH-SERVICE: ✅ Response received - Status: %s\n", resp.Status)
	fmt.Printf("🔍 AUTH-SERVICE: Response headers: %v\n", resp.Header)

	// Read response body
	bodyBytes, err := io.ReadAll(resp.Body)
	if err != nil {
		fmt.Printf("🔍 AUTH-SERVICE: ❌ Failed to read response body: %v\n", err)
		return nil, err
	}

	fmt.Printf("🔍 AUTH-SERVICE: Raw response body: %s\n", string(bodyBytes))
	fmt.Printf("🔍 AUTH-SERVICE: Body length: %d\n", len(bodyBytes))

	// Check if response is successful
	if resp.StatusCode >= 400 {
		fmt.Printf("🔍 AUTH-SERVICE: ❌ HTTP error %d: %s\n", resp.StatusCode, string(bodyBytes))
		return nil, fmt.Errorf("auth service error: %s - %s", resp.Status, string(bodyBytes))
	}

	return bodyBytes, nil
}
