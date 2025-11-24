package session

import (
	"encoding/json"
	"net/http"
)

// LogoutHandler handles user logout
// This handler is responsible ONLY for clearing session cookies
func (h *SessionHandler) LogoutHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")

	// Use session utility to clear the cookie
	sessionConfig := DefaultSessionCookieConfig()
	ClearSessionCookie(w, sessionConfig)

	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"message": "Logged out successfully",
	}); err != nil {
		// Log error but don't fail logout process
		// In production, you might want to log this to a proper logger
		// For now, we'll just continue since logout was successful
		_ = err // Suppress unused variable warning
	}
}
