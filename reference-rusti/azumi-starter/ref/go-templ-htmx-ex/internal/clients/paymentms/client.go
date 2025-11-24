package paymentms

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"
)

// Client is the client for the Payment Microservice.
type Client struct {
	baseURL    string
	apiKey     string
	httpClient *http.Client
}

// New creates a new Payment MS client.
func New(baseURL, apiKey string) *Client {
	return &Client{
		baseURL: baseURL,
		apiKey:  apiKey,
		httpClient: &http.Client{
			Timeout: 10 * time.Second,
		},
	}
}

// CreateCartCheckout creates a checkout session for multiple items.
func (c *Client) CreateCartCheckout(ctx context.Context, req CartCheckoutRequest) (*CheckoutResponse, error) {
	return c.doRequest(ctx, "POST", "/api/v1/checkout/cart", req)
}

// CreateItemCheckout creates a checkout session for a single item.
func (c *Client) CreateItemCheckout(ctx context.Context, req ItemCheckoutRequest) (*CheckoutResponse, error) {
	return c.doRequest(ctx, "POST", "/api/v1/checkout/item", req)
}

// CreateSubscriptionCheckout creates a checkout session for a subscription.
func (c *Client) CreateSubscriptionCheckout(ctx context.Context, req SubscriptionCheckoutRequest) (*CheckoutResponse, error) {
	return c.doRequest(ctx, "POST", "/api/v1/checkout/subscription", req)
}

// GetSubscriptionStatus retrieves the subscription status for a user and product.
func (c *Client) GetSubscriptionStatus(ctx context.Context, userID, productID string) (*SubscriptionStatusResponse, error) {
	path := fmt.Sprintf("/api/v1/subscriptions/%s/%s", userID, productID)

	req, err := http.NewRequestWithContext(ctx, "GET", c.baseURL+path, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	c.addHeaders(req)

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return nil, fmt.Errorf("failed to perform request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("payment ms error (status %d): %s", resp.StatusCode, string(body))
	}

	var result SubscriptionStatusResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &result, nil
}

// CreateCustomerPortal creates a session for the customer portal.
// Note: The spec didn't explicitly detail the request/response for portal, assuming standard return_url pattern.
func (c *Client) CreateCustomerPortal(ctx context.Context, userID string, returnURL string) (string, error) {
	// Assuming the endpoint expects a JSON body with return_url or similar,
	// and returns a URL. Based on common Stripe patterns.
	// If the MS endpoint is just POST /api/v1/portal with some body:

	// NOTE: The user provided spec for /api/v1/portal was empty in details.
	// I will assume it takes a user_id and return_url in the body.
	// Adjusting based on typical needs.

	reqBody := map[string]string{
		"user_id":    userID,
		"return_url": returnURL,
	}

	reqBytes, err := json.Marshal(reqBody)
	if err != nil {
		return "", fmt.Errorf("failed to marshal request: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, "POST", c.baseURL+"/api/v1/portal", bytes.NewBuffer(reqBytes))
	if err != nil {
		return "", fmt.Errorf("failed to create request: %w", err)
	}

	c.addHeaders(req)

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return "", fmt.Errorf("failed to perform request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return "", fmt.Errorf("payment ms error (status %d): %s", resp.StatusCode, string(body))
	}

	var result PortalResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		// Fallback if response is just the URL string or different structure
		// But for now assuming JSON
		return "", fmt.Errorf("failed to decode response: %w", err)
	}

	return result.URL, nil
}

func (c *Client) doRequest(ctx context.Context, method, path string, payload interface{}) (*CheckoutResponse, error) {
	var body io.Reader
	if payload != nil {
		jsonBytes, err := json.Marshal(payload)
		if err != nil {
			return nil, fmt.Errorf("failed to marshal request body: %w", err)
		}
		body = bytes.NewBuffer(jsonBytes)
	}

	req, err := http.NewRequestWithContext(ctx, method, c.baseURL+path, body)
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	c.addHeaders(req)

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return nil, fmt.Errorf("failed to perform request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		respBody, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("payment ms error (status %d): %s", resp.StatusCode, string(respBody))
	}

	var result CheckoutResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &result, nil
}

func (c *Client) addHeaders(req *http.Request) {
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-API-Key", c.apiKey)
}
