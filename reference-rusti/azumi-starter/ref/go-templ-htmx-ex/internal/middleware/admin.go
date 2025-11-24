package middleware

import (
	"net/http"

	"github.com/DraconDev/go-templ-htmx-ex/internal/utils/config"
)

// RequireAdmin middleware that checks if user is admin
func RequireAdmin(next http.Handler, adminEmail string) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		userInfo := GetUserFromContext(r)

		if !userInfo.LoggedIn {
			http.Redirect(w, r, "/", http.StatusFound)
			return
		}

		// Check if user is admin
		if userInfo.Email != adminEmail {
			http.Error(w, "Access denied: Admin privileges required", http.StatusForbidden)
			return
		}

		next.ServeHTTP(w, r)
	})
}

// RequireConfigAdmin middleware that uses config admin email
func RequireConfigAdmin(next http.Handler) http.Handler {
	return RequireAdmin(next, config.Current.AdminEmail)
}
