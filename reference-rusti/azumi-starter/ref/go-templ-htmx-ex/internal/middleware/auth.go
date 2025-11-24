package middleware

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/DraconDev/go-templ-htmx-ex/templates/layouts"
)

// UserContextKey is the key used to store user info in request context
type UserContextKey string

const userContextKey UserContextKey = "user"

// AuthMiddleware validates server sessions for protected routes
func AuthMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		path := r.URL.Path
		category := getRouteCategory(path)

		fmt.Printf("🔐 MIDDLEWARE: Processing route %s [Category: %s]\n", path, category)

		// Skip session validation for auth callback route (no session yet during callback flow)
		var userInfo layouts.UserInfo
		if path != "/auth/callback" {
			userInfo = validateSession(r)
		} else {
			userInfo = layouts.UserInfo{LoggedIn: false}
		}
		ctx := context.WithValue(r.Context(), userContextKey, userInfo)

		// Check if this route requires authentication
		if requiresAuthentication(path) {
			// If route requires auth but user is not logged in, redirect
			if !userInfo.LoggedIn {
				if r.URL.Path[:5] == "/api/" {
					// For API routes, return JSON error
					w.Header().Set("Content-Type", "application/json")
					w.WriteHeader(http.StatusUnauthorized)
					if err := json.NewEncoder(w).Encode(map[string]interface{}{
						"error": "Authentication required",
					}); err != nil {
						fmt.Printf("🔐 MIDDLEWARE: Failed to encode error response: %v\n", err)
					}
					return
				}

				// For web routes, redirect to login
				http.Redirect(w, r, "/login", http.StatusFound)
				return
			}
		}

		next.ServeHTTP(w, r.WithContext(ctx))
	})
}

// GetUserFromContext gets user info from request context
func GetUserFromContext(r *http.Request) layouts.UserInfo {
	userInfo, ok := r.Context().Value(userContextKey).(layouts.UserInfo)
	if !ok {
		return layouts.UserInfo{LoggedIn: false}
	}
	return userInfo
}

// getRouteCategory returns the category of a route for debugging
func getRouteCategory(path string) string {
	// Protected routes that require authentication
	if path == "/profile" || path == "/admin" || hasPrefix(path, "/api/admin") {
		return "PROTECTED"
	}

	// Public routes
	if path == "/" || path == "/health" || path == "/login" || path == "/test" || path == "/auth/callback" || hasPrefix(path, "/auth/") {
		return "PUBLIC"
	}

	// Auth API routes
	if hasPrefix(path, "/api/auth/") {
		return "AUTH_API"
	}

	return "UNKNOWN"
}

// requiresAuthentication checks if a route requires authentication
func requiresAuthentication(path string) bool {
	// Authentication API routes should NOT require authentication (they handle auth tokens)
	if hasPrefix(path, "/api/auth/") {
		return false
	}

	return path == "/profile" || path == "/admin" || hasPrefix(path, "/api/admin")
}

// hasPrefix is a simple string prefix check
func hasPrefix(s, prefix string) bool {
	return len(s) >= len(prefix) && s[:len(prefix)] == prefix
}
