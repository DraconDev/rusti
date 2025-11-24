package login

import (
	"fmt"
	"net/http"

	"github.com/DraconDev/go-templ-htmx-ex/templates/layouts"
	"github.com/DraconDev/go-templ-htmx-ex/templates/pages"
)

// =============================================================================
// OAUTH LOGIN HANDLERS
// =============================================================================
// These handlers manage the OAuth login flow for all supported providers:
// - Google, GitHub, Discord, Microsoft
// - Handles redirects to external providers
// - Processes OAuth callbacks
// =============================================================================

// LoginHandler handles OAuth login for any provider
// Flow: User clicks "Login with [Provider]" -> Redirect to our auth service ->
//
//	Auth service handles OAuth -> Returns to our callback with session token
//
// Usage: /auth/login?provider=google|github|discord|microsoft
func (h *LoginHandler) LoginHandler(w http.ResponseWriter, r *http.Request) {
	// Get provider from query parameter
	provider := r.URL.Query().Get("provider")
	if provider == "" {
		fmt.Printf("🔐 LOGIN ERROR: Missing provider parameter\n")
		http.Redirect(w, r, "/login?error=missing_provider", http.StatusFound)
		return
	}

	// Validate provider
	validProviders := map[string]bool{
		"google":    true,
		"github":    true,
		"discord":   true,
		"microsoft": true,
	}

	if !validProviders[provider] {
		fmt.Printf("🔐 LOGIN ERROR: Invalid provider '%s'\n", provider)
		http.Redirect(w, r, "/login?error=invalid_provider", http.StatusFound)
		return
	}

	fmt.Printf("🔐 LOGIN: Starting %s OAuth flow\n", provider)
	fmt.Printf("🔐 LOGIN: AuthServiceURL = %s\n", h.Config.AuthServiceURL)
	fmt.Printf("🔐 LOGIN: RedirectURL = %s\n", h.Config.RedirectURL)

	// Redirect to our auth microservice with redirect_uri parameter
	// The auth service will handle the actual OAuth flow for the specified provider
	authURL := fmt.Sprintf("%s/auth/%s?redirect_uri=%s/auth/callback",
		h.Config.AuthServiceURL, provider, h.Config.RedirectURL)

	fmt.Printf("🔐 LOGIN: Redirecting to: %s\n", authURL)
	http.Redirect(w, r, authURL, http.StatusFound)
}

// AuthCallbackHandler handles the OAuth callback
// Flow: OAuth provider redirects here with authorization code in URL
//
//	Client-side JS extracts token and calls /api/auth/set-session
func (h *LoginHandler) AuthCallbackHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Printf("🔐 CALLBACK: === OAuth callback STARTED ===\n")
	fmt.Printf("🔐 CALLBACK: URL = %s\n", r.URL.String())
	fmt.Printf("🔐 CALLBACK: Query params = %v\n", r.URL.Query())
	fmt.Printf("🔐 CALLBACK: Fragment = %s\n", r.URL.Fragment)

	fmt.Printf("🔐 CALLBACK: Setting content type and rendering template...\n")
	w.Header().Set("Content-Type", "text/html")

	// STEP 2: Render callback page with JavaScript to extract session token from URL fragment
	// The fragment (#access_token=...) is not sent to server, so JS must handle it
	component := layouts.Layout("Authenticating", "Authentication processing page for OAuth callback and session establishment.", layouts.NavigationLoggedOut(), pages.AuthCallbackContent())

	fmt.Printf("🔐 CALLBACK: About to render component...\n")
	if err := component.Render(r.Context(), w); err != nil {
		fmt.Printf("🚨 CALLBACK: Error rendering component: %v\n", err)
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		return
	}
	fmt.Printf("🔐 CALLBACK: Component rendered successfully\n")
	fmt.Printf("🔐 CALLBACK: === OAuth callback COMPLETED ===\n")
}
