package middleware

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"time"

	"github.com/DraconDev/go-templ-htmx-ex/internal/utils/config"
	"github.com/DraconDev/go-templ-htmx-ex/templates/layouts"
)

// validateSessionWithAuthService validates session by calling auth microservice
func validateSessionWithAuthService(sessionID string) (layouts.UserInfo, error) {
	fmt.Printf("🔐 MIDDLEWARE: Calling auth service to validate session %s\n", sessionID[:8]+"...")

	// Create HTTP client with timeout
	client := &http.Client{Timeout: 10 * time.Second}

	// Prepare request to auth service
	reqBody := map[string]string{"session_id": sessionID}
	jsonData, err := json.Marshal(reqBody)
	if err != nil {
		fmt.Printf("🔐 MIDDLEWARE: Failed to marshal request: %v\n", err)
		return layouts.UserInfo{LoggedIn: false}, err
	}

	req, err := http.NewRequest("POST", fmt.Sprintf("%s/auth/session/refresh", config.Current.AuthServiceURL), bytes.NewBuffer(jsonData))
	if err != nil {
		fmt.Printf("🔐 MIDDLEWARE: Failed to create request: %v\n", err)
		return layouts.UserInfo{LoggedIn: false}, err
	}

	req.Header.Set("Content-Type", "application/json")

	fmt.Printf("🔐 MIDDLEWARE: Sending validation request to auth service\n")

	// Send request to auth service
	resp, err := client.Do(req)
	if err != nil {
		fmt.Printf("🔐 MIDDLEWARE: Failed to call auth service: %v\n", err)
		// Don't fail the request if auth service is unavailable
		return layouts.UserInfo{LoggedIn: false}, nil
	}
	defer resp.Body.Close()

	fmt.Printf("🔐 MIDDLEWARE: Auth service response status: %s\n", resp.Status)

	// Parse response
	var respData map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&respData); err != nil {
		fmt.Printf("🔐 MIDDLEWARE: Failed to parse response: %v\n", err)
		return layouts.UserInfo{LoggedIn: false}, nil
	}

	fmt.Printf("🔐 MIDDLEWARE: Auth service response: %v\n", respData)

	// Check if session is valid by looking for user_context or success response
	var userInfo layouts.UserInfo

	// Try user_context first (existing format)
	if userContext, ok := respData["user_context"].(map[string]interface{}); ok && userContext != nil {
		userInfo.LoggedIn = true

		if name, ok := userContext["name"].(string); ok && name != "" {
			userInfo.Name = name
		}
		if email, ok := userContext["email"].(string); ok && email != "" {
			userInfo.Email = email
		}
		if picture, ok := userContext["picture"].(string); ok && picture != "" {
			userInfo.Picture = picture
		}
		if userID, ok := userContext["user_id"].(string); ok && userID != "" {
			fmt.Printf("🔐 MIDDLEWARE: Session valid for user: %s (%s)\n", userInfo.Name, userInfo.Email)
		}

		return userInfo, nil
	}

	// Try session_id validation response (new format from working reference)
	if success, ok := respData["success"].(bool); ok && success {
		// For session_id based validation, just mark as logged in
		// User details would come from other endpoints if needed
		userInfo.LoggedIn = true
		fmt.Printf("🔐 MIDDLEWARE: Session validated successfully (session_id format)\n")
		return userInfo, nil
	}

	fmt.Printf("🔐 MIDDLEWARE: Session validation failed\n")
	return layouts.UserInfo{LoggedIn: false}, nil
}
