package routes

import (
	"net/http"

	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers"
	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers/admin"
	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers/auth/login"
	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers/auth/session"
	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers/dashboard"
	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers/payment"
	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers/settings"
	"github.com/gorilla/mux"
)

// HandlerInstances holds all handler instances for route registration
type HandlerInstances struct {
	AdminHandler     *admin.AdminHandler
	LoginHandler     *login.LoginHandler
	SessionHandler   *session.SessionHandler
	PaymentHandler   *payment.PaymentHandler
	DashboardHandler *dashboard.DashboardHandler
	SettingsHandler  *settings.SettingsHandler
}

// SetupRoutes configures and returns the router with all routes
func SetupRoutes(handlerInstances *HandlerInstances) *mux.Router {
	router := mux.NewRouter()

	// =============================================================================
	// PUBLIC ROUTES - No authentication required
	// =============================================================================

	// Homepage - Main landing page with platform showcase
	router.HandleFunc("/", handlers.HomeHandler).Methods("GET")

	// Health check - API health monitoring endpoint
	router.HandleFunc("/health", handlers.HealthHandler).Methods("GET")

	// Login page - OAuth provider selection UI
	router.HandleFunc("/login", handlers.LoginHandler).Methods("GET")

	// Pricing page - Public pricing information
	if handlerInstances.PaymentHandler != nil {
		router.HandleFunc("/pricing", handlerInstances.PaymentHandler.PricingPageHandler).Methods("GET")
	}

	// =============================================================================
	// OAUTH AUTHENTICATION FLOW
	// =============================================================================

	// OAuth Login Route - Consolidated with provider parameter
	if handlerInstances.LoginHandler != nil {
		router.HandleFunc("/auth/login", handlerInstances.LoginHandler.LoginHandler).Methods("GET")
		router.HandleFunc("/auth/callback", handlerInstances.LoginHandler.AuthCallbackHandler).Methods("GET")
	}

	// =============================================================================
	// PROTECTED USER ROUTES - Authentication required
	// =============================================================================

	// Dashboard - Main user interface
	if handlerInstances.DashboardHandler != nil {
		router.HandleFunc("/dashboard", handlerInstances.DashboardHandler.DashboardHandler).Methods("GET")
	}

	// User profile page - Display user information and account details
	router.HandleFunc("/profile", handlers.ProfileHandler).Methods("GET")

	// Settings Routes
	if handlerInstances.SettingsHandler != nil {
		router.HandleFunc("/settings", handlerInstances.SettingsHandler.SettingsPageHandler).Methods("GET")
		router.HandleFunc("/settings/update", handlerInstances.SettingsHandler.UpdateSettingsHandler).Methods("POST")
		router.HandleFunc("/settings/billing", handlerInstances.SettingsHandler.BillingPortalHandler).Methods("POST", "GET")
	}

	// Payment page - Subscription and billing management
	if handlerInstances.PaymentHandler != nil {
		router.HandleFunc("/payment", handlerInstances.PaymentHandler.PaymentPageHandler).Methods("GET")
		router.HandleFunc("/payment/success", handlerInstances.PaymentHandler.SuccessHandler).Methods("GET")
		router.HandleFunc("/payment/cancel", handlerInstances.PaymentHandler.CancelHandler).Methods("GET")
	}

	// =============================================================================
	// ADMIN ROUTES - Admin authentication required
	// =============================================================================

	// Admin dashboard - Main admin interface for platform management
	if handlerInstances.AdminHandler != nil {
		router.HandleFunc("/admin", handlerInstances.AdminHandler.AdminDashboardHandler).Methods("GET")
		router.HandleFunc("/api/admin/users", handlerInstances.AdminHandler.GetUsersHandler).Methods("GET")
		router.HandleFunc("/api/admin/analytics", handlerInstances.AdminHandler.GetAnalyticsHandler).Methods("GET")
		router.HandleFunc("/api/admin/settings", handlerInstances.AdminHandler.GetSettingsHandler).Methods("GET")
		router.HandleFunc("/api/admin/logs", handlerInstances.AdminHandler.GetLogsHandler).Methods("GET")
	}

	// =============================================================================
	// SESSION MANAGEMENT API - Authentication required
	// =============================================================================

	if handlerInstances.SessionHandler != nil {
		// Logout user - Destroy current session and clear cookies
		router.HandleFunc("/api/auth/logout", handlerInstances.SessionHandler.LogoutHandler).Methods("POST")

		// Set session - Create new server session with provided session ID
		router.HandleFunc("/api/auth/set-session", handlerInstances.SessionHandler.SetSessionHandler).Methods("POST")

		// Exchange code - Exchange OAuth authorization code for session tokens
		router.HandleFunc("/api/auth/exchange-code", handlerInstances.SessionHandler.ExchangeCodeHandler).Methods("POST")
	}

	// =============================================================================
	// PAYMENT API - Payment processing endpoints
	// =============================================================================

	if handlerInstances.PaymentHandler != nil {
		router.HandleFunc("/api/payment/checkout", handlerInstances.PaymentHandler.CheckoutHandler).Methods("POST")
	}

	// Static files (for CSS, JS, etc.)
	router.PathPrefix("/static/").Handler(http.StripPrefix("/static/", http.FileServer(http.Dir("static/"))))

	return router
}

// RouteInfo provides information about application routes
type RouteInfo struct {
	Name        string `json:"name"`
	Method      string `json:"method"`
	Pattern     string `json:"pattern"`
	Description string `json:"description"`
}

// GetAllRoutes returns information about all application routes
func GetAllRoutes() []RouteInfo {
	return []RouteInfo{
		// Public Routes
		{Name: "home", Method: "GET", Pattern: "/", Description: "Main landing page"},
		{Name: "health", Method: "GET", Pattern: "/health", Description: "Health check endpoint"},
		{Name: "login", Method: "GET", Pattern: "/login", Description: "Login page"},
		{Name: "pricing", Method: "GET", Pattern: "/pricing", Description: "Pricing page"},

		// OAuth Routes
		{Name: "oauth_login", Method: "GET", Pattern: "/auth/login", Description: "OAuth provider login"},
		{Name: "oauth_callback", Method: "GET", Pattern: "/auth/callback", Description: "OAuth callback handler"},

		// Protected Routes
		{Name: "dashboard", Method: "GET", Pattern: "/dashboard", Description: "User dashboard"},
		{Name: "profile", Method: "GET", Pattern: "/profile", Description: "User profile page"},
		{Name: "payment", Method: "GET", Pattern: "/payment", Description: "Payment and subscription page"},
		{Name: "payment_success", Method: "GET", Pattern: "/payment/success", Description: "Payment success page"},
		{Name: "payment_cancel", Method: "GET", Pattern: "/payment/cancel", Description: "Payment cancelled page"},

		// Admin Routes
		{Name: "admin_dashboard", Method: "GET", Pattern: "/admin", Description: "Admin dashboard"},
		{Name: "admin_get_users", Method: "GET", Pattern: "/api/admin/users", Description: "Get users API"},
		{Name: "admin_get_analytics", Method: "GET", Pattern: "/api/admin/analytics", Description: "Get analytics API"},
		{Name: "admin_get_settings", Method: "GET", Pattern: "/api/admin/settings", Description: "Get settings API"},
		{Name: "admin_get_logs", Method: "GET", Pattern: "/api/admin/logs", Description: "Get logs API"},

		// Auth API Routes
		{Name: "logout", Method: "POST", Pattern: "/api/auth/logout", Description: "User logout"},
		{Name: "set_session", Method: "POST", Pattern: "/api/auth/set-session", Description: "Set session"},
		{Name: "exchange_code", Method: "POST", Pattern: "/api/auth/exchange-code", Description: "Exchange auth code"},

		// Payment API Routes
		{Name: "payment_checkout", Method: "POST", Pattern: "/api/payment/checkout", Description: "Create payment checkout session"},
	}
}

// RouteSummary provides a summary of all registered routes
type RouteSummary struct {
	TotalRoutes      int `json:"total_routes"`
	PublicRoutes     int `json:"public_routes"`
	ProtectedRoutes  int `json:"protected_routes"`
	AdminRoutes      int `json:"admin_routes"`
	AuthAPIRoutes    int `json:"auth_api_routes"`
	PaymentAPIRoutes int `json:"payment_api_routes"`
}

// CountRoutes provides a count of all route types
func CountRoutes() RouteSummary {
	return RouteSummary{
		TotalRoutes:      17,
		PublicRoutes:     3,
		ProtectedRoutes:  4,
		AdminRoutes:      5,
		AuthAPIRoutes:    4,
		PaymentAPIRoutes: 1,
	}
}
