package session

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/DraconDev/go-templ-htmx-ex/internal/models"
	"github.com/DraconDev/go-templ-htmx-ex/internal/utils/errors"
)

// SetSessionHandler handles setting a new session cookie
// This handler is responsible for:
// 1. Setting session cookies from a provided session ID
// 2. Syncing user data from Auth MS to local DB
func (h *SessionHandler) SetSessionHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")

	var req struct {
		SessionID string `json:"session_id"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		handleJSONError(w, "Invalid request body", err, errors.NewBadRequestError)
		return
	}

	if req.SessionID == "" {
		handleJSONError(w, "Missing session_id", nil, errors.NewBadRequestError)
		return
	}

	// 1. Fetch user info from Auth MS (via session refresh)
	userContext, err := h.AuthService.GetUserInfo(req.SessionID)
	if err != nil {
		// If we can't get user info, we shouldn't set the session
		handleJSONError(w, "Failed to validate session with Auth Service", err, errors.NewUnauthorizedError)
		return
	}

	// 2. Sync user to local DB
	if h.UserRepository != nil {
		// Convert UserSessionContext to User model
		user := &models.User{
			AuthID:  userContext.UserID,
			Email:   userContext.Email,
			Name:    userContext.Name,
			Picture: userContext.Picture,
			IsAdmin: false, // Default to false, admin can update later
		}

		// Upsert user
		// We use a background context or the request context
		_, err := h.UserRepository.UpsertUser(r.Context(), user)
		if err != nil {
			fmt.Printf("⚠️ SESSION: Failed to sync user to local DB: %v\n", err)
			// We continue even if sync fails, to allow login
		} else {
			fmt.Printf("✅ SESSION: Synced user %s to local DB\n", user.Email)
		}
	}

	// Use session utility to set the cookie
	sessionConfig := DefaultSessionCookieConfig()
	SetSessionCookie(w, req.SessionID, sessionConfig)

	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"message": "Server session established successfully",
		"user":    userContext, // Optional: return user info
	}); err != nil {
		_ = err
	}
}

// handleJSONError is a helper to standardize error responses
func handleJSONError(w http.ResponseWriter, message string, err error, errorType func(string) *errors.AppError) {
	if err != nil {
		fmt.Printf("Error in handleJSONError: %v\n", err)
	}
	w.WriteHeader(errorType(message).Code)
	_ = json.NewEncoder(w).Encode(map[string]interface{}{
		"error": message,
	})
}
