package handlers

import (
	"fmt"
	"net/http"
	"time"

	"github.com/DraconDev/go-templ-htmx-ex/internal/middleware"
	"github.com/DraconDev/go-templ-htmx-ex/templates/layouts"
	"github.com/DraconDev/go-templ-htmx-ex/templates/pages"
	"github.com/a-h/templ"
)

// HealthHandler handles health check requests
func HealthHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	if _, err := w.Write([]byte(`{"status": "healthy", "timestamp": "` + time.Now().Format(time.RFC3339) + `"}`)); err != nil {
		fmt.Printf("Error writing health check response: %v\n", err)
	}
}

func HomeHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/html")

	// Get user info from middleware context
	userInfo := middleware.GetUserFromContext(r)

	fmt.Printf("🏠 HOME: User info - LoggedIn: %v, Name: %s, Email: %s\n",
		userInfo.LoggedIn, userInfo.Name, userInfo.Email)

	var navigation templ.Component
	if userInfo.LoggedIn {
		fmt.Printf("🏠 HOME: Rendering NavigationLoggedIn\n")
		navigation = layouts.NavigationLoggedIn(userInfo)
	} else {
		fmt.Printf("🏠 HOME: Rendering NavigationLoggedOut\n")
		navigation = layouts.NavigationLoggedOut()
	}

	component := layouts.Layout("Home", "Production-ready startup platform with Google OAuth, PostgreSQL database, and admin dashboard. Built with Go + HTMX + Templ.", navigation, pages.HomeContent())
	if err := component.Render(r.Context(), w); err != nil {
		fmt.Printf("Error rendering Home page: %v\n", err)
	}
}

// ProfileHandler handles the user profile page
func ProfileHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/html")
	// Get user info from middleware context
	userInfo := middleware.GetUserFromContext(r)
	if !userInfo.LoggedIn {
		// Redirect to home if not logged in
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}

	// Create profile content with real user data
	navigation := layouts.NavigationLoggedIn(userInfo)
	component := layouts.Layout("Profile", "User profile page with authentication details and account management.", navigation, pages.ProfileContent(userInfo.Name, userInfo.Email, userInfo.Picture))
	if err := component.Render(r.Context(), w); err != nil {
		http.Error(w, "Failed to render profile page", http.StatusInternalServerError)
		return
	}
}

// LoginHandler handles the login page
func LoginHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/html")
	component := layouts.Layout("Login", "Secure authentication page with Google OAuth integration for user access.", layouts.NavigationLoggedOut(), pages.LoginContent())
	if err := component.Render(r.Context(), w); err != nil {
		http.Error(w, "Failed to render login page", http.StatusInternalServerError)
		return
	}
}
