package models

import (
	"errors"
	"time"

	"github.com/google/uuid"
)

// Database errors
var (
	ErrDatabaseNotConnected = errors.New("database not connected")
)

// UserSessionContext represents the user context from Auth MS (session/create and session/refresh)
type UserSessionContext struct {
	UserID  string `json:"user_id"`
	Name    string `json:"name"`
	Email   string `json:"email,omitempty"`
	Picture string `json:"picture,omitempty"`
	// Projects map[string]ProjectSubscription `json:"projects"` // Not needed for our BFF
}

// SessionCreateResponse represents the response from /auth/session/create
type SessionCreateResponse struct {
	SessionID   string             `json:"session_id"`
	UserContext UserSessionContext `json:"user_context"`
}

// SessionRefreshResponse represents the response from /auth/session/refresh
type SessionRefreshResponse struct {
	SessionID   string             `json:"session_id"`
	UserContext UserSessionContext `json:"user_context"`
}

// AuthResponse represents a generic auth service response (for backwards compatibility)
type AuthResponse struct {
	Success bool   `json:"success"`
	Message string `json:"message"`
	UserID  string `json:"user_id,omitempty"`
	Email   string `json:"email,omitempty"`
	Name    string `json:"name,omitempty"`
	Picture string `json:"picture,omitempty"`
	Error   string `json:"error,omitempty"`
}

// TokenExchangeResponse represents the response from exchanging auth code for session
type TokenExchangeResponse struct {
	Success bool   `json:"success"`
	Message string `json:"message"`
	IdToken string `json:"id_token"` // Server session token (session_id from auth service)
	Error   string `json:"error,omitempty"`
}

// ExchangeCodeRequest represents a request to exchange authorization code for tokens
type ExchangeCodeRequest struct {
	AuthCode string `json:"auth_code"`
}

// UserSummary represents user summary data for admin views
type UserSummary struct {
	ID        uuid.UUID `json:"id"`
	Email     string    `json:"email"`
	Name      string    `json:"name"`
	Picture   string    `json:"picture"`
	IsAdmin   bool      `json:"is_admin"`
	CreatedAt time.Time `json:"created_at"`
}

// UserStats represents user statistics for admin dashboard
type UserStats struct {
	TotalUsers    int64 `json:"total_users"`
	SignupsToday  int64 `json:"signups_today"`
	UsersThisWeek int64 `json:"users_this_week"`
	ActiveUsers   int64 `json:"active_users"`
	InactiveUsers int64 `json:"inactive_users"`
}
