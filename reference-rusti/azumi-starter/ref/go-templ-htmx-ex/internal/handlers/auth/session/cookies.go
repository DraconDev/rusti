package session

import (
	"net/http"
)

// CookieConfig holds session cookie configuration
type CookieConfig struct {
	Name     string
	MaxAge   int
	HttpOnly bool
	Secure   bool
	Path     string
}

// DefaultSessionCookieConfig returns the default session cookie configuration
func DefaultSessionCookieConfig() CookieConfig {
	return CookieConfig{
		Name:     "session_id",
		MaxAge:   2592000, // 30 days
		HttpOnly: true,
		Secure:   false, // Set to true in production with HTTPS
		Path:     "/",
	}
}

// SetSessionCookie sets a session cookie in the response
func SetSessionCookie(w http.ResponseWriter, sessionID string, config CookieConfig) {
	cookie := &http.Cookie{
		Name:     config.Name,
		Value:    sessionID,
		Path:     config.Path,
		MaxAge:   config.MaxAge,
		HttpOnly: config.HttpOnly,
		Secure:   config.Secure,
	}
	http.SetCookie(w, cookie)
}

// ClearSessionCookie clears a session cookie
func ClearSessionCookie(w http.ResponseWriter, config CookieConfig) {
	cookie := &http.Cookie{
		Name:     config.Name,
		Value:    "",
		Path:     config.Path,
		MaxAge:   -1,
		HttpOnly: config.HttpOnly,
	}
	http.SetCookie(w, cookie)
}

// GetSessionCookie retrieves a session cookie from the request
func GetSessionCookie(r *http.Request) (string, error) {
	cookie, err := r.Cookie("session_id")
	if err != nil {
		return "", err
	}
	return cookie.Value, nil
}

// IsSessionValid checks if a session cookie is present and not empty
func IsSessionValid(r *http.Request) bool {
	sessionID, err := GetSessionCookie(r)
	return err == nil && sessionID != ""
}
