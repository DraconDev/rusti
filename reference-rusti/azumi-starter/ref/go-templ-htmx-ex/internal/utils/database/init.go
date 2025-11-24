package database

import (
	"database/sql"
	"fmt"
	"log"
	"os"

	_ "github.com/lib/pq"
)

// RunMigrations initializes the database schema and creates initial admin
func RunMigrations(dbURL string) error {
	// Connect to database
	db, err := sql.Open("postgres", dbURL)
	if err != nil {
		return fmt.Errorf("failed to connect to database: %w", err)
	}
	defer db.Close()

	// Test connection
	if err := db.Ping(); err != nil {
		return fmt.Errorf("failed to ping database: %w", err)
	}

	log.Println("✅ Connected to database for initialization")

	// Read and execute migration script
	migrationSQL, err := os.ReadFile("database/migrations/001_initial_schema.sql")
	if err != nil {
		return fmt.Errorf("failed to read migration script: %w", err)
	}

	// Execute migration
	if _, err := db.Exec(string(migrationSQL)); err != nil {
		return fmt.Errorf("failed to execute migration: %w", err)
	}

	log.Println("✅ Database schema created successfully")

	// Set initial admin if ADMIN_EMAIL is provided
	adminEmail := os.Getenv("ADMIN_EMAIL")
	if adminEmail != "" {
		// Check if any admin user already exists
		var adminCount int
		err := db.QueryRow("SELECT COUNT(*) FROM users WHERE is_admin = TRUE").Scan(&adminCount)
		if err != nil {
			log.Printf("⚠️  Could not check for existing admin users: %v", err)
		} else if adminCount == 0 {
			// Check if admin email user exists
			var userExists bool
			err := db.QueryRow("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)", adminEmail).Scan(&userExists)
			if err != nil {
				log.Printf("⚠️  Could not check for existing user: %v", err)
			} else if userExists {
				// Make existing user an admin
				_, err := db.Exec("UPDATE users SET is_admin = TRUE WHERE email = $1", adminEmail)
				if err != nil {
					log.Printf("⚠️  Could not promote user to admin: %v", err)
				} else {
					log.Printf("✅ Promoted existing user to admin: %s", adminEmail)
				}
			} else {
				// Create initial admin user
				_, err := db.Exec(`
					INSERT INTO users (auth_id, email, name, picture, is_admin)
					VALUES ($1, $2, $3, $4, $5)
					ON CONFLICT (email) DO UPDATE SET is_admin = TRUE
				`, "admin-"+adminEmail, adminEmail, "Platform Admin", "", true)

				if err != nil {
					log.Printf("⚠️  Could not create initial admin user: %v", err)
				} else {
					log.Printf("✅ Created initial admin user: %s", adminEmail)
				}
			}
		} else {
			log.Printf("✅ Admin users already exist (%d admins) - skipping initial admin creation", adminCount)
		}
	} else {
		log.Println("ℹ️  No ADMIN_EMAIL configured - using first-user-as-admin system")
	}

	return nil
}

// InitDatabaseIfConfigured initializes database if DB_URL is available
func InitDatabaseIfConfigured() error {
	dbURL := os.Getenv("DB_URL")
	if dbURL == "" {
		log.Println("ℹ️  No DB_URL configured - skipping database initialization")
		return nil
	}

	log.Println("🚀 Initializing database...")
	if err := RunMigrations(dbURL); err != nil {
		return fmt.Errorf("database initialization failed: %w", err)
	}

	log.Println("✅ Database initialization completed successfully")
	log.Println("💡 Manual admin setup: Run SQL: UPDATE users SET is_admin = TRUE WHERE email = 'dracsharp@gmail.com';")
	return nil
}
