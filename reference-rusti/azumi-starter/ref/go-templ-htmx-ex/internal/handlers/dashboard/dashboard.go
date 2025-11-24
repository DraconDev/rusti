package dashboard

import (
	"net/http"

	"github.com/DraconDev/go-templ-htmx-ex/internal/clients/paymentms"
	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers/auth/session"
	"github.com/DraconDev/go-templ-htmx-ex/internal/utils/config"
	"github.com/DraconDev/go-templ-htmx-ex/templates/pages"
)

type DashboardHandler struct {
	config         *config.Config
	paymentClient  *paymentms.Client
	sessionHandler *session.SessionHandler
}

func NewDashboardHandler(cfg *config.Config, paymentClient *paymentms.Client, sessionHandler *session.SessionHandler) *DashboardHandler {
	return &DashboardHandler{
		config:         cfg,
		paymentClient:  paymentClient,
		sessionHandler: sessionHandler,
	}
}

func (h *DashboardHandler) DashboardHandler(w http.ResponseWriter, r *http.Request) {
	// Get user info from session using SessionHandler
	userInfo := h.sessionHandler.GetUserInfo(r)
	if !userInfo.LoggedIn {
		http.Redirect(w, r, "/login", http.StatusSeeOther)
		return
	}

	// Get subscription status
	// Note: In a real app, you'd get the user ID from the session.
	// For now, we'll use the email or a placeholder if ID is missing.
	userID := userInfo.Email // Fallback since we might not have ID in session yet

	// Fetch subscription status
	// We use the product ID from config
	subStatus, err := h.paymentClient.GetSubscriptionStatus(r.Context(), userID, h.config.StripeProductID)

	// Prepare view model
	isPro := false
	status := "Free Plan"
	periodEnd := ""

	if err == nil && subStatus != nil {
		if subStatus.Status == "active" {
			isPro = true
			status = "Pro Plan"
			periodEnd = subStatus.CurrentPeriodEnd.Format("Jan 02, 2006")
		}
	}

	// Render template
	component := pages.Dashboard(userInfo.Name, userInfo.Email, userInfo.Picture, status, isPro, periodEnd)
	if err := component.Render(r.Context(), w); err != nil {
		http.Error(w, "Failed to render dashboard", http.StatusInternalServerError)
	}
}
