package paymentms

import "time"

// CartCheckoutRequest represents a request to create a checkout session for multiple items.
type CartCheckoutRequest struct {
	CancelURL  string     `json:"cancel_url"`
	Email      string     `json:"email"`
	Items      []CartItem `json:"items"`
	SuccessURL string     `json:"success_url"`
	UserID     string     `json:"user_id"`
}

// CartItem represents an item in the cart checkout request.
type CartItem struct {
	PriceID  string `json:"price_id"`
	Quantity int    `json:"quantity"`
}

// ItemCheckoutRequest represents a request to create a checkout session for a single item.
type ItemCheckoutRequest struct {
	CancelURL  string `json:"cancel_url"`
	Email      string `json:"email"`
	PriceID    string `json:"price_id"`
	Quantity   int    `json:"quantity"`
	SuccessURL string `json:"success_url"`
	UserID     string `json:"user_id"`
}

// SubscriptionCheckoutRequest represents a request to create a checkout session for a subscription.
type SubscriptionCheckoutRequest struct {
	CancelURL  string `json:"cancel_url"`
	Email      string `json:"email"`
	PriceID    string `json:"price_id"`
	ProductID  string `json:"product_id"`
	SuccessURL string `json:"success_url"`
	UserID     string `json:"user_id"`
}

// CheckoutResponse represents the response from a checkout creation request.
type CheckoutResponse struct {
	CheckoutSessionID string `json:"checkout_session_id"`
	CheckoutURL       string `json:"checkout_url"`
}

// SubscriptionStatusResponse represents the response for a subscription status check.
type SubscriptionStatusResponse struct {
	CurrentPeriodEnd time.Time `json:"current_period_end"`
	ProductID        string    `json:"product_id"`
	Status           string    `json:"status"`
	SubscriptionID   string    `json:"subscription_id"`
}

// PortalRequest represents a request to create a customer portal session.
type PortalRequest struct {
	ReturnURL string `json:"return_url"`
}

// PortalResponse represents the response from a portal creation request.
type PortalResponse struct {
	URL string `json:"url"`
}
