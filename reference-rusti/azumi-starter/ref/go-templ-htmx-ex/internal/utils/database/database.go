package database

import (
	"database/sql"

	"github.com/dracondev/go-templ-htmx-ex/libs/dbx"
)

// Re-export dbx functions and variables for backward compatibility
var DB *sql.DB

func init() {
	// Sync our DB variable with dbx.DB
	DB = dbx.DB
}

// InitDatabase initializes the database connection using DB_URL environment variable
func InitDatabase() error {
	err := dbx.InitDatabase()
	if err == nil {
		DB = dbx.GetDB()
	}
	return err
}

// InitDatabaseFromConnString initializes database from a connection string
func InitDatabaseFromConnString(connString string) error {
	err := dbx.InitDatabaseFromConnString(connString)
	if err == nil {
		DB = dbx.GetDB()
	}
	return err
}

// CloseDatabase closes the database connection
func CloseDatabase() error {
	return dbx.CloseDatabase()
}

// GetDB returns the database connection pool
func GetDB() *sql.DB {
	DB = dbx.GetDB()
	return DB
}

// IsInitialized checks if the database connection pool is initialized
func IsInitialized() bool {
	return dbx.IsInitialized()
}
