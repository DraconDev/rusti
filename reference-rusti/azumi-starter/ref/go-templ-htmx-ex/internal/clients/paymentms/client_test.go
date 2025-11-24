package paymentms

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

func TestGetSubscriptionStatus(t *testing.T) {
	// Mock server
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Verify request
		if r.Method != "GET" {
			t.Errorf("Expected GET request, got %s", r.Method)
		}
		if r.Header.Get("X-API-Key") != "test-key" {
			t.Errorf("Expected API key header")
		}
		if r.URL.Path != "/api/v1/subscriptions/user123/prod456" {
			t.Errorf("Expected path /api/v1/subscriptions/user123/prod456, got %s", r.URL.Path)
		}

		// Mock response
		resp := SubscriptionStatusResponse{
			Status:           "active",
			ProductID:        "prod456",
			SubscriptionID:   "sub789",
			CurrentPeriodEnd: time.Now().Add(24 * time.Hour),
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(resp)
	}))
	defer server.Close()

	// Create client
	client := New(server.URL, "test-key")

	// Test
	status, err := client.GetSubscriptionStatus(context.Background(), "user123", "prod456")
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	if status.Status != "active" {
		t.Errorf("Expected status active, got %s", status.Status)
	}
	if status.ProductID != "prod456" {
		t.Errorf("Expected product ID prod456, got %s", status.ProductID)
	}
}

func TestCreateSubscriptionCheckout(t *testing.T) {
	// Mock server
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Verify request
		if r.Method != "POST" {
			t.Errorf("Expected POST request, got %s", r.Method)
		}
		if r.Header.Get("X-API-Key") != "test-key" {
			t.Errorf("Expected API key header")
		}
		if r.URL.Path != "/api/v1/checkout/subscription" {
			t.Errorf("Expected path /api/v1/checkout/subscription, got %s", r.URL.Path)
		}

		var req SubscriptionCheckoutRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			t.Fatalf("Failed to decode request body: %v", err)
		}

		if req.UserID != "user123" {
			t.Errorf("Expected user ID user123, got %s", req.UserID)
		}

		// Mock response
		resp := CheckoutResponse{
			CheckoutSessionID: "cs_test_123",
			CheckoutURL:       "https://checkout.stripe.com/pay/cs_test_123",
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(resp)
	}))
	defer server.Close()

	// Create client
	client := New(server.URL, "test-key")

	// Test
	req := SubscriptionCheckoutRequest{
		UserID:     "user123",
		Email:      "test@example.com",
		PriceID:    "price_123",
		ProductID:  "prod_456",
		SuccessURL: "http://localhost/success",
		CancelURL:  "http://localhost/cancel",
	}

	resp, err := client.CreateSubscriptionCheckout(context.Background(), req)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	if resp.CheckoutSessionID != "cs_test_123" {
		t.Errorf("Expected session ID cs_test_123, got %s", resp.CheckoutSessionID)
	}
	if resp.CheckoutURL != "https://checkout.stripe.com/pay/cs_test_123" {
		t.Errorf("Expected checkout URL, got %s", resp.CheckoutURL)
	}
}

func TestCreateCustomerPortal(t *testing.T) {
	// Mock server
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != "POST" {
			t.Errorf("Expected POST request, got %s", r.Method)
		}
		if r.URL.Path != "/api/v1/portal" {
			t.Errorf("Expected path /api/v1/portal, got %s", r.URL.Path)
		}

		// Mock response
		resp := PortalResponse{
			URL: "https://billing.stripe.com/p/session/test_123",
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(resp)
	}))
	defer server.Close()

	client := New(server.URL, "test-key")

	url, err := client.CreateCustomerPortal(context.Background(), "user123", "http://localhost/return")
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	if url != "https://billing.stripe.com/p/session/test_123" {
		t.Errorf("Expected portal URL, got %s", url)
	}
}
