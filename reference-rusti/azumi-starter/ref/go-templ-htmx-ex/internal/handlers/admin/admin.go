package admin

import (
	dbSqlc "github.com/DraconDev/go-templ-htmx-ex/database/sqlc"
	"github.com/DraconDev/go-templ-htmx-ex/internal/services"
	"github.com/DraconDev/go-templ-htmx-ex/internal/utils/config"
)

// AdminHandler handles admin-specific operations
type AdminHandler struct {
	Config      *config.Config
	UserService *services.UserService
}

// NewAdminHandler creates a new admin handler
func NewAdminHandler(config *config.Config, queries *dbSqlc.Queries) *AdminHandler {
	return &AdminHandler{
		Config:      config,
		UserService: services.NewUserService(queries),
	}
}
