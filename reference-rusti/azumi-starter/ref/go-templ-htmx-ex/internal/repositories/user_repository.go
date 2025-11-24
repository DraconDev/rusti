package repositories

import (
	"context"
	"database/sql"
	"time"

	dbSqlc "github.com/DraconDev/go-templ-htmx-ex/database/sqlc"
	"github.com/DraconDev/go-templ-htmx-ex/internal/models"
	"github.com/google/uuid"
)

// UserRepository handles user data access operations
type UserRepository struct {
	queries *dbSqlc.Queries
}

// NewUserRepository creates a new user repository
func NewUserRepository(queries *dbSqlc.Queries) *UserRepository {
	return &UserRepository{
		queries: queries,
	}
}

// CreateUser creates a new user in the database
func (r *UserRepository) CreateUser(ctx context.Context, user *models.User) (*models.User, error) {
	if r.queries == nil {
		return nil, models.ErrDatabaseNotConnected
	}

	picture := sql.NullString{String: user.Picture, Valid: user.Picture != ""}
	isAdmin := sql.NullBool{Bool: user.IsAdmin, Valid: true}

	dbUser, err := r.queries.CreateUser(ctx, dbSqlc.CreateUserParams{
		AuthID:  user.AuthID,
		Email:   user.Email,
		Name:    user.Name,
		Picture: picture,
		IsAdmin: isAdmin,
	})
	if err != nil {
		return nil, err
	}

	return &models.User{
		ID:        dbUser.ID.String(),
		AuthID:    dbUser.AuthID,
		Email:     dbUser.Email,
		Name:      dbUser.Name,
		Picture:   dbUser.Picture.String,
		IsAdmin:   dbUser.IsAdmin.Bool,
		Provider:  "", // SQLC User doesn't have Provider field
		CreatedAt: dbUser.CreatedAt.Time,
		UpdatedAt: dbUser.UpdatedAt.Time,
	}, nil
}

// GetUserByEmail retrieves a user by email
func (r *UserRepository) GetUserByEmail(ctx context.Context, email string) (*models.User, error) {
	if r.queries == nil {
		return nil, models.ErrDatabaseNotConnected
	}

	dbUser, err := r.queries.GetUserByEmail(ctx, email)
	if err != nil {
		return nil, err
	}

	return &models.User{
		ID:        dbUser.ID.String(),
		AuthID:    dbUser.AuthID,
		Email:     dbUser.Email,
		Name:      dbUser.Name,
		Picture:   dbUser.Picture.String,
		IsAdmin:   dbUser.IsAdmin.Bool,
		Provider:  "", // SQLC User doesn't have Provider field
		CreatedAt: dbUser.CreatedAt.Time,
		UpdatedAt: dbUser.UpdatedAt.Time,
	}, nil
}

// GetAllUsers retrieves all users
func (r *UserRepository) GetAllUsers(ctx context.Context) ([]models.User, error) {
	if r.queries == nil {
		return nil, models.ErrDatabaseNotConnected
	}

	dbUsers, err := r.queries.GetAllUsers(ctx)
	if err != nil {
		return nil, err
	}

	users := make([]models.User, len(dbUsers))
	for i, dbUser := range dbUsers {
		users[i] = models.User{
			ID:        dbUser.ID.String(),
			AuthID:    dbUser.AuthID,
			Email:     dbUser.Email,
			Name:      dbUser.Name,
			Picture:   dbUser.Picture.String,
			IsAdmin:   dbUser.IsAdmin.Bool,
			Provider:  "", // SQLC User doesn't have Provider field
			CreatedAt: dbUser.CreatedAt.Time,
			UpdatedAt: dbUser.UpdatedAt.Time,
		}
	}

	return users, nil
}

// CountUsers returns the total number of users
func (r *UserRepository) CountUsers(ctx context.Context) (int64, error) {
	if r.queries == nil {
		return 0, models.ErrDatabaseNotConnected
	}

	return r.queries.CountUsers(ctx)
}

// UpdateUser updates user information
func (r *UserRepository) UpdateUser(ctx context.Context, user *models.User) (*models.User, error) {
	if r.queries == nil {
		return nil, models.ErrDatabaseNotConnected
	}

	picture := sql.NullString{String: user.Picture, Valid: user.Picture != ""}

	// Convert string ID to UUID
	userID, err := uuid.Parse(user.ID)
	if err != nil {
		return nil, err
	}

	err = r.queries.UpdateUser(ctx, dbSqlc.UpdateUserParams{
		ID:      userID,
		Name:    user.Name,
		Picture: picture,
	})
	if err != nil {
		return nil, err
	}

	// Return updated user by fetching it again
	return r.GetUserByEmail(ctx, user.Email)
}

// GetRecentUsers returns recently created users
func (r *UserRepository) GetRecentUsers(ctx context.Context) ([]models.User, error) {
	if r.queries == nil {
		return nil, models.ErrDatabaseNotConnected
	}

	dbUsers, err := r.queries.GetRecentUsers(ctx)
	if err != nil {
		return nil, err
	}

	users := make([]models.User, len(dbUsers))
	for i, dbUser := range dbUsers {
		users[i] = models.User{
			ID:        dbUser.ID.String(),
			AuthID:    "", // Recent users query doesn't return auth_id
			Email:     dbUser.Email,
			Name:      dbUser.Name,
			Picture:   "",    // Recent users query doesn't return picture
			IsAdmin:   false, // Recent users query doesn't return is_admin
			Provider:  "",    // Recent users query doesn't return provider
			CreatedAt: dbUser.CreatedAt.Time,
			UpdatedAt: time.Time{}, // Recent users query doesn't return updated_at
		}
	}

	return users, nil
}

// CountUsersCreatedToday returns count of users created today
func (r *UserRepository) CountUsersCreatedToday(ctx context.Context) (int64, error) {
	if r.queries == nil {
		return 0, models.ErrDatabaseNotConnected
	}

	return r.queries.CountUsersCreatedToday(ctx)
}

// UpsertUser creates or updates a user
func (r *UserRepository) UpsertUser(ctx context.Context, user *models.User) (*models.User, error) {
	if r.queries == nil {
		return nil, models.ErrDatabaseNotConnected
	}

	picture := sql.NullString{String: user.Picture, Valid: user.Picture != ""}
	isAdmin := sql.NullBool{Bool: user.IsAdmin, Valid: true}

	dbUser, err := r.queries.UpsertUser(ctx, dbSqlc.UpsertUserParams{
		AuthID:  user.AuthID,
		Email:   user.Email,
		Name:    user.Name,
		Picture: picture,
		IsAdmin: isAdmin,
	})
	if err != nil {
		return nil, err
	}

	return &models.User{
		ID:        dbUser.ID.String(),
		AuthID:    dbUser.AuthID,
		Email:     dbUser.Email,
		Name:      dbUser.Name,
		Picture:   dbUser.Picture.String,
		IsAdmin:   dbUser.IsAdmin.Bool,
		Provider:  "", // SQLC User doesn't have Provider field
		CreatedAt: dbUser.CreatedAt.Time,
		UpdatedAt: dbUser.UpdatedAt.Time,
	}, nil
}

// CountUsersCreatedThisWeek returns the count of users created this week
func (r *UserRepository) CountUsersCreatedThisWeek(ctx context.Context) (int64, error) {
	if r.queries == nil {
		return 0, models.ErrDatabaseNotConnected
	}

	return r.queries.CountUsersCreatedThisWeek(ctx)
}
