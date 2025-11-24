package main

import (
	"context"
	"database/sql"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/gorilla/mux"

	dbSqlc "github.com/DraconDev/go-templ-htmx-ex/database/sqlc"
	"github.com/DraconDev/go-templ-htmx-ex/internal/clients/paymentms"
	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers/admin"
	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers/auth/login"
	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers/auth/session"
	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers/dashboard"
	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers/payment"
	"github.com/DraconDev/go-templ-htmx-ex/internal/handlers/settings"
	"github.com/DraconDev/go-templ-htmx-ex/internal/middleware"
	"github.com/DraconDev/go-templ-htmx-ex/internal/repositories"
	"github.com/DraconDev/go-templ-htmx-ex/internal/routes"
	"github.com/DraconDev/go-templ-htmx-ex/internal/utils/config"
	database "github.com/DraconDev/go-templ-htmx-ex/internal/utils/database"
	_ "github.com/lib/pq"
)

var adminHandler *admin.AdminHandler
var sqlDB *sql.DB
var queries *dbSqlc.Queries
var loginHandler *login.LoginHandler
var sessionHandler *session.SessionHandler
var paymentHandler *payment.PaymentHandler
var dashboardHandler *dashboard.DashboardHandler
var settingsHandler *settings.SettingsHandler

func main() {
	// Load configuration
	cfg := config.LoadConfig()

	// Initialize database if configured
	if err := database.InitDatabaseIfConfigured(); err != nil {
		log.Printf("⚠️  Database initialization failed: %v", err)
		log.Println("💡 Continuing without database functionality")
	}

	// Get database connection for runtime use
	dbURL := os.Getenv("DB_URL")
	if dbURL != "" {
		var err error
		sqlDB, err = sql.Open("postgres", dbURL)
		if err != nil {
			log.Printf("❌ Database connection failed: %v", err)
			sqlDB = nil
		} else {
			// Test connection
			if err := sqlDB.Ping(); err != nil {
				log.Printf("❌ Database ping failed: %v", err)
				sqlDB = nil
			} else {
				log.Println("✅ Database connected successfully")

				// Initialize SQLC queries
				queries = dbSqlc.New(sqlDB)
				log.Println("✅ SQLC queries initialized")
			}
		}
	}

	// Services ready (user service not currently used)
	if queries != nil {
		log.Println("✅ Database services ready")
	}

	// Create handlers with services
	if queries != nil {
		adminHandler = admin.NewAdminHandler(cfg, queries)
	} else {
		log.Println("⚠️  Admin handler not initialized - no database connection")
	}

	// Initialize repositories
	userRepo := repositories.NewUserRepository(queries)
	prefsRepo := repositories.NewPreferencesRepository(queries)
	log.Println("✅ Repositories initialized")

	// Initialize login and session handlers
	loginHandler = login.NewLoginHandler(cfg)
	sessionHandler = session.NewSessionHandler(cfg, userRepo)
	log.Println("✅ Login and session handlers initialized")

	// Initialize Payment MS Client
	paymentClient := paymentms.New(cfg.PaymentServiceURL, cfg.PaymentServiceAPIKey)
	log.Println("✅ Payment MS Client initialized")

	// Initialize payment handler
	paymentHandler = payment.NewPaymentHandler(cfg, paymentClient)
	log.Println("✅ Payment handler initialized")

	// Initialize Dashboard Handler
	dashboardHandler = dashboard.NewDashboardHandler(cfg, paymentClient, sessionHandler)
	log.Println("✅ Dashboard handler initialized")

	// Initialize Settings Handler
	settingsHandler = settings.NewSettingsHandler(cfg, sessionHandler, userRepo, prefsRepo, paymentClient)
	log.Println("✅ Settings handler initialized")

	// Create router using centralized route structure
	router := SetupRoutes()

	// Create HTTP server
	server := &http.Server{
		Addr:         cfg.GetServerAddress(),
		Handler:      router,
		ReadTimeout:  30 * time.Second,
		WriteTimeout: 30 * time.Second,
		IdleTimeout:  60 * time.Second,
	}

	// Start server in a goroutine
	go func() {
		log.Printf("Starting server on port %s", cfg.ServerPort)
		log.Printf("Visit http://localhost:%s to access the application", cfg.ServerPort)
		if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Server failed to start: %v", err)
		}
	}()

	// Wait for interrupt signal for graceful shutdown
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)
	<-c

	log.Println("Shutting down server...")

	// Graceful shutdown
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	if err := server.Shutdown(ctx); err != nil {
		log.Fatalf("Server forced to shutdown: %v", err)
	}

	log.Println("Server stopped")
}

// SetupRoutes creates and configures the router with all routes
func SetupRoutes() *mux.Router {
	// Create handler instances for the routes package
	handlerInstances := &routes.HandlerInstances{
		AdminHandler:     adminHandler,
		LoginHandler:     loginHandler,
		SessionHandler:   sessionHandler,
		PaymentHandler:   paymentHandler,
		DashboardHandler: dashboardHandler,
		SettingsHandler:  settingsHandler,
	}

	// Use centralized route setup
	router := routes.SetupRoutes(handlerInstances)

	// Add middleware after routes are set up
	router.Use(middleware.AuthMiddleware)

	return router
}
