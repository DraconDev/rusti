package dbx

import (
	"database/sql"
	"fmt"
	"log"
	"os"

	_ "github.com/lib/pq"
)

// DB holds the database connection pool
var DB *sql.DB

// Config holds database configuration
type Config struct {
	Driver         string
	ConnString     string
	MaxOpenConns   int
	MaxIdleConns   int
	ConnMaxLifetime int // in seconds
}

// DefaultConfig returns default database configuration
func DefaultConfig() *Config {
	return &Config{
		Driver:         "postgres",
		MaxOpenConns:   25,
		MaxIdleConns:   5,
		ConnMaxLifetime: 300, // 5 minutes
	}
}

// InitDatabase initializes the database connection using DB_URL environment variable
func InitDatabase() error {
	dbURL := os.Getenv("DB_URL")
	if dbURL == "" {
		return fmt.Errorf("DB_URL environment variable is required")
	}
	return InitDatabaseFromConnString(dbURL)
}

// InitDatabaseFromConnString initializes database from a connection string
func InitDatabaseFromConnString(connString string) error {
	return InitDatabaseWithConfig(&Config{
		Driver:         "postgres",
		ConnString:     connString,
		MaxOpenConns:   25,
		MaxIdleConns:   5,
		ConnMaxLifetime: 300,
	})
}

// InitDatabaseWithConfig initializes database with custom configuration
func InitDatabaseWithConfig(config *Config) error {
	var err error
	DB, err = sql.Open(config.Driver, config.ConnString)
	if err != nil {
		return fmt.Errorf("failed to open database: %w", err)
	}

	// Set connection pool settings
	if config.MaxOpenConns > 0 {
		DB.SetMaxOpenConns(config.MaxOpenConns)
	}
	if config.MaxIdleConns > 0 {
		DB.SetMaxIdleConns(config.MaxIdleConns)
	}

	// Test the connection
	if err := DB.Ping(); err != nil {
		return fmt.Errorf("failed to ping database: %w", err)
	}

	log.Println("✅ Database connection established successfully")
	return nil
}

// CloseDatabase closes the database connection
func CloseDatabase() error {
	if DB != nil {
		return DB.Close()
	}
	return nil
}

// GetDB returns the database connection pool
func GetDB() *sql.DB {
	return DB
}

// IsInitialized checks if the database connection pool is initialized
func IsInitialized() bool {
	return DB != nil
}

// HealthCheck performs a database health check
func HealthCheck() error {
	if !IsInitialized() {
		return fmt.Errorf("database not initialized")
	}
	return DB.Ping()
}
