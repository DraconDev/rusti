package session

import (
	"github.com/DraconDev/go-templ-htmx-ex/internal/repositories"
	"github.com/DraconDev/go-templ-htmx-ex/internal/services"
	"github.com/DraconDev/go-templ-htmx-ex/internal/utils/config"
)

type SessionHandler struct {
	Config         *config.Config
	AuthService    *services.AuthService
	UserRepository *repositories.UserRepository
}

func NewSessionHandler(config *config.Config, userRepo *repositories.UserRepository) *SessionHandler {
	return &SessionHandler{
		Config:         config,
		AuthService:    services.NewAuthService(config),
		UserRepository: userRepo,
	}
}
