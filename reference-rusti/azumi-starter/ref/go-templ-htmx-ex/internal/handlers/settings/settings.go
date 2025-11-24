package settings

import (
	"fmt"
	"net/http"

	"github.com/DraconDev/go-templ-htmx-ex/internal/clients/paymentms"
	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers/auth/session"
	"github.com/DraconDev/go-templ-htmx-ex/internal/models"
	"github.com/DraconDev/go-templ-htmx-ex/internal/repositories"
	"github.com/DraconDev/go-templ-htmx-ex/internal/utils/config"
	"github.com/DraconDev/go-templ-htmx-ex/templates/pages"
)

type SettingsHandler struct {
	config         *config.Config
	sessionHandler *session.SessionHandler
	userRepo       *repositories.UserRepository
	prefsRepo      *repositories.PreferencesRepository
	paymentClient  *paymentms.Client
}

func NewSettingsHandler(
	cfg *config.Config,
	sessionHandler *session.SessionHandler,
	userRepo *repositories.UserRepository,
	prefsRepo *repositories.PreferencesRepository,
	paymentClient *paymentms.Client,
) *SettingsHandler {
	return &SettingsHandler{
		config:         cfg,
		sessionHandler: sessionHandler,
		userRepo:       userRepo,
		prefsRepo:      prefsRepo,
		paymentClient:  paymentClient,
	}
}

func (h *SettingsHandler) SettingsPageHandler(w http.ResponseWriter, r *http.Request) {
	// 1. Get session user info
	userInfo := h.sessionHandler.GetUserInfo(r)
	if !userInfo.LoggedIn {
		http.Redirect(w, r, "/login", http.StatusSeeOther)
		return
	}

	// 2. Get local user record to get UUID
	user, err := h.userRepo.GetUserByEmail(r.Context(), userInfo.Email)
	if err != nil {
		http.Error(w, "User record not found", http.StatusInternalServerError)
		return
	}

	// 3. Get preferences
	prefs, err := h.prefsRepo.GetPreferences(r.Context(), user.ID)
	if err != nil {
		// If error (and not just not found which is handled by repo), log it
		fmt.Printf("Error fetching preferences: %v\n", err)
		// Create default prefs object for display
		prefs = &models.UserPreferences{
			UserID:             user.ID,
			Theme:              "dark",
			Language:           "en",
			Timezone:           "UTC",
			EmailNotifications: true,
			EmailBilling:       true,
			PushNotifications:  true,
		}
	}

	// 4. Render template
	component := pages.Settings(userInfo, prefs)
	if err := component.Render(r.Context(), w); err != nil {
		http.Error(w, "Failed to render settings page", http.StatusInternalServerError)
	}
}

func (h *SettingsHandler) UpdateSettingsHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// 1. Get session user info
	userInfo := h.sessionHandler.GetUserInfo(r)
	if !userInfo.LoggedIn {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	// 2. Get local user record
	user, err := h.userRepo.GetUserByEmail(r.Context(), userInfo.Email)
	if err != nil {
		http.Error(w, "User record not found", http.StatusInternalServerError)
		return
	}

	// 3. Parse form
	if err := r.ParseForm(); err != nil {
		http.Error(w, "Invalid form data", http.StatusBadRequest)
		return
	}

	// 4. Update preferences
	prefs := &models.UserPreferences{
		UserID:             user.ID,
		Timezone:           r.FormValue("timezone"),
		EmailNotifications: r.FormValue("email_notifications") == "on",
		EmailBilling:       r.FormValue("email_billing") == "on",
		// Preserve others or update if form has them
		Theme:    "dark", // Default for now
		Language: "en",   // Default for now
	}

	// We use Upsert logic: try update, if fails (no rows), create
	// But our repo has Update and Create.
	// Let's try Update first.
	updatedPrefs, err := h.prefsRepo.UpdatePreferences(r.Context(), prefs)
	if err != nil {
		// Try create
		_, err = h.prefsRepo.CreatePreferences(r.Context(), user.ID)
		if err != nil {
			http.Error(w, "Failed to update settings", http.StatusInternalServerError)
			return
		}
		// Now update with values
		updatedPrefs, err = h.prefsRepo.UpdatePreferences(r.Context(), prefs)
		if err != nil {
			http.Error(w, "Failed to update settings", http.StatusInternalServerError)
			return
		}
	}

	// 5. Render success message or re-render form (HTMX)
	// For now, just return a success message
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<div class='p-4 bg-green-500/20 text-green-400 rounded-lg'>Settings saved successfully!</div>")
	_ = updatedPrefs
}

func (h *SettingsHandler) BillingPortalHandler(w http.ResponseWriter, r *http.Request) {
	// 1. Get session user info
	userInfo := h.sessionHandler.GetUserInfo(r)
	if !userInfo.LoggedIn {
		http.Redirect(w, r, "/login", http.StatusSeeOther)
		return
	}

	// 2. Get local user for user_id
	user, err := h.userRepo.GetUserByEmail(r.Context(), userInfo.Email)
	if err != nil {
		http.Error(w, "User record not found", http.StatusInternalServerError)
		return
	}

	// 3. Call Payment MS to create portal session
	returnURL := h.config.RedirectURL + "/settings"

	portalURL, err := h.paymentClient.CreateCustomerPortal(r.Context(), user.ID, returnURL)
	if err != nil {
		fmt.Printf("Error creating portal session: %v\n", err)
		http.Redirect(w, r, "/settings?error=portal_failed", http.StatusSeeOther)
		return
	}

	// 4. Redirect to portal
	http.Redirect(w, r, portalURL, http.StatusSeeOther)
}
