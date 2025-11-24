package payment

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/DraconDev/go-templ-htmx-ex/internal/clients/paymentms"
	"github.com/DraconDev/go-templ-htmx-ex/internal/middleware"
	"github.com/DraconDev/go-templ-htmx-ex/internal/utils/config"
	"github.com/DraconDev/go-templ-htmx-ex/templates/layouts"
	"github.com/DraconDev/go-templ-htmx-ex/templates/pages"
)

// PaymentHandler handles payment-related requests
type PaymentHandler struct {
	Config *config.Config
	Client *paymentms.Client
}

// NewPaymentHandler creates a new payment handler
func NewPaymentHandler(config *config.Config, client *paymentms.Client) *PaymentHandler {
	return &PaymentHandler{
		Config: config,
		Client: client,
	}
}

// PricingPageHandler handles the pricing page display
func (h *PaymentHandler) PricingPageHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/html")

	// Get user info from middleware context (optional for pricing page)
	userInfo := middleware.GetUserFromContext(r)

	// Render pricing page
	component := pages.Pricing(userInfo)
	if err := component.Render(r.Context(), w); err != nil {
		http.Error(w, "Failed to render pricing page", http.StatusInternalServerError)
		return
	}
}

// PaymentPageHandler handles the payment page display
func (h *PaymentHandler) PaymentPageHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/html")

	// Get user info from middleware context
	userInfo := middleware.GetUserFromContext(r)
	if !userInfo.LoggedIn {
		// Redirect to home if not logged in
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}

	// Create payment page content with user data
	navigation := layouts.NavigationLoggedIn(userInfo)
	component := layouts.Layout("Payment", "Subscribe to access premium features and content.", navigation, pages.PaymentContent(userInfo))
	if err := component.Render(r.Context(), w); err != nil {
		http.Error(w, "Failed to render payment page", http.StatusInternalServerError)
		return
	}
}

// CheckoutHandler handles payment checkout initiation
func (h *PaymentHandler) CheckoutHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")

	// Get user info from middleware context
	userInfo := middleware.GetUserFromContext(r)
	if !userInfo.LoggedIn {
		w.WriteHeader(http.StatusUnauthorized)
		json.NewEncoder(w).Encode(map[string]interface{}{
			"error": "Authentication required",
		})
		return
	}

	// Parse request body
	var req struct {
		PriceID    string `json:"price_id"`
		ProductID  string `json:"product_id"`
		SuccessURL string `json:"success_url"`
		CancelURL  string `json:"cancel_url"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		w.WriteHeader(http.StatusBadRequest)
		json.NewEncoder(w).Encode(map[string]interface{}{
			"error": "Invalid request body",
		})
		return
	}

	// Validate required fields
	if req.PriceID == "" || req.ProductID == "" {
		w.WriteHeader(http.StatusBadRequest)
		json.NewEncoder(w).Encode(map[string]interface{}{
			"error": "Missing required fields: price_id, product_id",
		})
		return
	}

	// Set default URLs if not provided
	baseURL := h.Config.RedirectURL // Use configured redirect URL
	if req.SuccessURL == "" {
		req.SuccessURL = baseURL + "/payment/success"
	}
	if req.CancelURL == "" {
		req.CancelURL = baseURL + "/payment/cancel"
	}

	// Call payment microservice using client
	checkoutReq := paymentms.SubscriptionCheckoutRequest{
		UserID:     userInfo.Email, // Using email as user ID for now
		Email:      userInfo.Email,
		ProductID:  req.ProductID,
		PriceID:    req.PriceID,
		SuccessURL: req.SuccessURL,
		CancelURL:  req.CancelURL,
	}

	checkoutResp, err := h.Client.CreateSubscriptionCheckout(r.Context(), checkoutReq)
	if err != nil {
		fmt.Printf("❌ PAYMENT: Failed to create checkout session: %v\n", err)
		w.WriteHeader(http.StatusInternalServerError)
		json.NewEncoder(w).Encode(map[string]interface{}{
			"error": "Failed to create checkout session",
		})
		return
	}

	// Return checkout URL to frontend
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]interface{}{
		"checkout_url":        checkoutResp.CheckoutURL,
		"checkout_session_id": checkoutResp.CheckoutSessionID,
	})
}

// SuccessHandler handles successful payment redirects
func (h *PaymentHandler) SuccessHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/html")

	userInfo := middleware.GetUserFromContext(r)
	navigation := layouts.NavigationLoggedIn(userInfo)
	component := layouts.Layout("Payment Success", "Thank you for your purchase!", navigation, pages.PaymentSuccessContent())
	if err := component.Render(r.Context(), w); err != nil {
		http.Error(w, "Failed to render success page", http.StatusInternalServerError)
		return
	}
}

// CancelHandler handles cancelled payment redirects
func (h *PaymentHandler) CancelHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/html")

	userInfo := middleware.GetUserFromContext(r)
	navigation := layouts.NavigationLoggedIn(userInfo)
	component := layouts.Layout("Payment Cancelled", "Payment was cancelled. You can try again.", navigation, pages.PaymentCancelContent())
	if err := component.Render(r.Context(), w); err != nil {
		http.Error(w, "Failed to render cancel page", http.StatusInternalServerError)
		return
	}
}
