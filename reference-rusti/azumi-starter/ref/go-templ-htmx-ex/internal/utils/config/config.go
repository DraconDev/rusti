package config

import (
	"fmt"
	"log"
	"strconv"

	"github.com/dracondev/go-templ-htmx-ex/libs/configx"
)

// Config holds application configuration
type Config struct {
	*configx.Config
	ServerPort           string
	AuthServiceURL       string
	RedirectURL          string
	AdminEmail           string
	PaymentServiceURL    string
	PaymentServiceAPIKey string
	StripeProductID      string
	// Stripe Product/Price Configuration
	StripeProductPro   string
	StripePriceMonthly string
	StripePriceYearly  string
	// Session Configuration
	SessionSecret  string
	SessionTimeout int
}

var (
	// Global config instance
	Current *Config
)

// LoadConfig loads configuration from environment variables
func LoadConfig() *Config {
	fields := []configx.ConfigField{
		{
			Key:          "PORT",
			DefaultValue: "8081",
			Required:     false,
			Description:  "Server port",
		},
		{
			Key:          "AUTH_SERVICE_URL",
			DefaultValue: "http://localhost:8080",
			Required:     false,
			Description:  "Authentication service URL",
		},
		{
			Key:          "REDIRECT_URL",
			DefaultValue: "http://localhost:8081",
			Required:     false,
			Description:  "OAuth redirect URL",
		},
		{
			Key:          "ADMIN_EMAIL",
			DefaultValue: "admin@startup-platform.local",
			Required:     false,
			Description:  "Admin email address",
		},
		{
			Key:          "PAYMENT_MS_URL",
			DefaultValue: "http://localhost:9000",
			Required:     false,
			Description:  "Payment service URL",
		},
		{
			Key:          "PAYMENT_MS_API_KEY",
			DefaultValue: "",
			Required:     false,
			Description:  "Payment service API Key",
		},
		{
			Key:          "STRIPE_PRODUCT_ID",
			DefaultValue: "",
			Required:     false,
			Description:  "Stripe Product ID for subscriptions",
		},
		{
			Key:          "STRIPE_PRODUCT_PRO",
			DefaultValue: "",
			Required:     false,
			Description:  "Stripe Product ID for Pro plan",
		},
		{
			Key:          "STRIPE_PRICE_MONTHLY",
			DefaultValue: "",
			Required:     false,
			Description:  "Stripe Price ID for monthly billing",
		},
		{
			Key:          "STRIPE_PRICE_YEARLY",
			DefaultValue: "",
			Required:     false,
			Description:  "Stripe Price ID for yearly billing",
		},
		{
			Key:          "SESSION_SECRET",
			DefaultValue: "change-me-in-production",
			Required:     false,
			Description:  "Session encryption secret",
		},
		{
			Key:          "SESSION_TIMEOUT",
			DefaultValue: "3600",
			Required:     false,
			Description:  "Session timeout in seconds",
		},
	}

	baseConfig, err := configx.Load(fields, configx.DefaultOptions())
	if err != nil {
		log.Fatalf("Failed to load configuration: %v", err)
	}

	// Parse session timeout with default
	sessionTimeout := 3600
	if timeoutStr := baseConfig.Get("SESSION_TIMEOUT"); timeoutStr != "" {
		if parsed, err := strconv.Atoi(timeoutStr); err == nil {
			sessionTimeout = parsed
		}
	}

	config := &Config{
		Config:               baseConfig,
		ServerPort:           baseConfig.Get("PORT"),
		AuthServiceURL:       baseConfig.Get("AUTH_SERVICE_URL"),
		RedirectURL:          baseConfig.Get("REDIRECT_URL"),
		AdminEmail:           baseConfig.Get("ADMIN_EMAIL"),
		PaymentServiceURL:    baseConfig.Get("PAYMENT_MS_URL"),
		PaymentServiceAPIKey: baseConfig.Get("PAYMENT_MS_API_KEY"),
		StripeProductID:      baseConfig.Get("STRIPE_PRODUCT_ID"),
		StripeProductPro:     baseConfig.Get("STRIPE_PRODUCT_PRO"),
		StripePriceMonthly:   baseConfig.Get("STRIPE_PRICE_MONTHLY"),
		StripePriceYearly:    baseConfig.Get("STRIPE_PRICE_YEARLY"),
		SessionSecret:        baseConfig.Get("SESSION_SECRET"),
		SessionTimeout:       sessionTimeout,
	}

	Current = config
	return config
}

// IsAdmin checks if the given email matches the admin email
func (c *Config) IsAdmin(email string) bool {
	return c.AdminEmail != "" && email == c.AdminEmail
}

// GetServerAddress returns the full server address
func (c *Config) GetServerAddress() string {
	return fmt.Sprintf(":%s", c.ServerPort)
}
