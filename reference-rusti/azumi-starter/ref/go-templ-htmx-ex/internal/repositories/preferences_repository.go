package repositories

import (
	"context"
	"database/sql"

	dbSqlc "github.com/DraconDev/go-templ-htmx-ex/database/sqlc"
	"github.com/DraconDev/go-templ-htmx-ex/internal/models"
	"github.com/google/uuid"
)

// PreferencesRepository handles user preferences data access operations
type PreferencesRepository struct {
	queries *dbSqlc.Queries
}

// NewPreferencesRepository creates a new preferences repository
func NewPreferencesRepository(queries *dbSqlc.Queries) *PreferencesRepository {
	return &PreferencesRepository{
		queries: queries,
	}
}

// GetPreferences retrieves preferences for a user
func (r *PreferencesRepository) GetPreferences(ctx context.Context, userID string) (*models.UserPreferences, error) {
	if r.queries == nil {
		return nil, models.ErrDatabaseNotConnected
	}

	uuidID, err := uuid.Parse(userID)
	if err != nil {
		return nil, err
	}

	dbPrefs, err := r.queries.GetUserPreferences(ctx, uuid.NullUUID{UUID: uuidID, Valid: true})
	if err != nil {
		if err == sql.ErrNoRows {
			// Return default preferences if not found
			return &models.UserPreferences{
				UserID:             userID,
				Theme:              "dark",
				Language:           "en",
				Timezone:           "UTC",
				EmailNotifications: true,
				EmailBilling:       true,
				PushNotifications:  true,
			}, nil
		}
		return nil, err
	}

	return &models.UserPreferences{
		ID:                 dbPrefs.ID.String(),
		UserID:             dbPrefs.UserID.UUID.String(),
		Theme:              dbPrefs.Theme.String,
		Language:           dbPrefs.Language.String,
		Timezone:           dbPrefs.Timezone.String,
		EmailNotifications: dbPrefs.EmailNotifications.Bool,
		EmailBilling:       dbPrefs.EmailBilling.Bool,
		PushNotifications:  dbPrefs.PushNotifications.Bool,
		CreatedAt:          dbPrefs.CreatedAt.Time,
		UpdatedAt:          dbPrefs.UpdatedAt.Time,
	}, nil
}

// CreatePreferences creates default preferences for a user
func (r *PreferencesRepository) CreatePreferences(ctx context.Context, userID string) (*models.UserPreferences, error) {
	if r.queries == nil {
		return nil, models.ErrDatabaseNotConnected
	}

	uuidID, err := uuid.Parse(userID)
	if err != nil {
		return nil, err
	}

	dbPrefs, err := r.queries.CreateUserPreferences(ctx, dbSqlc.CreateUserPreferencesParams{
		UserID:             uuid.NullUUID{UUID: uuidID, Valid: true},
		Theme:              sql.NullString{String: "dark", Valid: true},
		Language:           sql.NullString{String: "en", Valid: true},
		Timezone:           sql.NullString{String: "UTC", Valid: true},
		EmailNotifications: sql.NullBool{Bool: true, Valid: true},
		EmailBilling:       sql.NullBool{Bool: true, Valid: true},
		PushNotifications:  sql.NullBool{Bool: true, Valid: true},
	})
	if err != nil {
		return nil, err
	}

	return &models.UserPreferences{
		ID:                 dbPrefs.ID.String(),
		UserID:             dbPrefs.UserID.UUID.String(),
		Theme:              dbPrefs.Theme.String,
		Language:           dbPrefs.Language.String,
		Timezone:           dbPrefs.Timezone.String,
		EmailNotifications: dbPrefs.EmailNotifications.Bool,
		EmailBilling:       dbPrefs.EmailBilling.Bool,
		PushNotifications:  dbPrefs.PushNotifications.Bool,
		CreatedAt:          dbPrefs.CreatedAt.Time,
		UpdatedAt:          dbPrefs.UpdatedAt.Time,
	}, nil
}

// UpdatePreferences updates user preferences
func (r *PreferencesRepository) UpdatePreferences(ctx context.Context, prefs *models.UserPreferences) (*models.UserPreferences, error) {
	if r.queries == nil {
		return nil, models.ErrDatabaseNotConnected
	}

	uuidID, err := uuid.Parse(prefs.UserID)
	if err != nil {
		return nil, err
	}

	dbPrefs, err := r.queries.UpdateUserPreferences(ctx, dbSqlc.UpdateUserPreferencesParams{
		UserID:             uuid.NullUUID{UUID: uuidID, Valid: true},
		Theme:              sql.NullString{String: prefs.Theme, Valid: true},
		Language:           sql.NullString{String: prefs.Language, Valid: true},
		Timezone:           sql.NullString{String: prefs.Timezone, Valid: true},
		EmailNotifications: sql.NullBool{Bool: prefs.EmailNotifications, Valid: true},
		EmailBilling:       sql.NullBool{Bool: prefs.EmailBilling, Valid: true},
		PushNotifications:  sql.NullBool{Bool: prefs.PushNotifications, Valid: true},
	})
	if err != nil {
		return nil, err
	}

	return &models.UserPreferences{
		ID:                 dbPrefs.ID.String(),
		UserID:             dbPrefs.UserID.UUID.String(),
		Theme:              dbPrefs.Theme.String,
		Language:           dbPrefs.Language.String,
		Timezone:           dbPrefs.Timezone.String,
		EmailNotifications: dbPrefs.EmailNotifications.Bool,
		EmailBilling:       dbPrefs.EmailBilling.Bool,
		PushNotifications:  dbPrefs.PushNotifications.Bool,
		CreatedAt:          dbPrefs.CreatedAt.Time,
		UpdatedAt:          dbPrefs.UpdatedAt.Time,
	}, nil
}
