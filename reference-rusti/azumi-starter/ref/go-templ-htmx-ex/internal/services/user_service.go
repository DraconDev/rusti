package services

import (
	"context"

	dbSqlc "github.com/DraconDev/go-templ-htmx-ex/database/sqlc"
	"github.com/DraconDev/go-templ-htmx-ex/internal/models"
	"github.com/DraconDev/go-templ-htmx-ex/internal/repositories"
)

// UserService provides user-related business logic
type UserService struct {
	userRepo *repositories.UserRepository
}

// NewUserService creates a new user service
func NewUserService(queries *dbSqlc.Queries) *UserService {
	return &UserService{
		userRepo: repositories.NewUserRepository(queries),
	}
}

// CreateUser creates a new user
func (s *UserService) CreateUser(ctx context.Context, user *models.User) (*models.User, error) {
	return s.userRepo.CreateUser(ctx, user)
}

// GetUserByEmail retrieves a user by email
func (s *UserService) GetUserByEmail(ctx context.Context, email string) (*models.User, error) {
	return s.userRepo.GetUserByEmail(ctx, email)
}

// GetAllUsers retrieves all users
func (s *UserService) GetAllUsers(ctx context.Context) ([]models.User, error) {
	return s.userRepo.GetAllUsers(ctx)
}

// CountUsers returns the total number of users
func (s *UserService) CountUsers(ctx context.Context) (int64, error) {
	return s.userRepo.CountUsers(ctx)
}

// UpdateUser updates user information
func (s *UserService) UpdateUser(ctx context.Context, user *models.User) (*models.User, error) {
	return s.userRepo.UpdateUser(ctx, user)
}

// GetRecentUsers returns recently created users
func (s *UserService) GetRecentUsers(ctx context.Context) ([]models.User, error) {
	return s.userRepo.GetRecentUsers(ctx)
}

// CountUsersCreatedToday returns count of users created today
func (s *UserService) CountUsersCreatedToday(ctx context.Context) (int64, error) {
	return s.userRepo.CountUsersCreatedToday(ctx)
}

// CountUsersCreatedThisWeek returns count of users created this week
func (s *UserService) CountUsersCreatedThisWeek(ctx context.Context) (int64, error) {
	return s.userRepo.CountUsersCreatedThisWeek(ctx)
}
