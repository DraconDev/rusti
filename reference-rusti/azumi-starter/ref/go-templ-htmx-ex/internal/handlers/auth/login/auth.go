package login

import (
	"github.com/DraconDev/go-templ-htmx-ex/internal/services"
	"github.com/DraconDev/go-templ-htmx-ex/internal/utils/config"
)

// LoginHandler handles authentication-related HTTP requests
type LoginHandler struct {
	Config      *config.Config        // App configuration
	AuthService *services.AuthService // Auth service for session management (includes HTTP client)
}

// NewLoginHandler creates a new authentication handler
func NewLoginHandler(config *config.Config) *LoginHandler {
	return &LoginHandler{
		Config:      config,
		AuthService: services.NewAuthService(config),
	}
}
