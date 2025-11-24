package configx

import (
	"fmt"
	"log"
	"os"

	"github.com/joho/godotenv"
)

// ConfigField defines a configuration field with metadata
type ConfigField struct {
	Key          string
	DefaultValue string
	Required     bool
	Description  string
}

// Config holds application configuration with flexible field management
type Config struct {
	fields map[string]string
}

// LoadOptions configures how configuration is loaded
type LoadOptions struct {
	EnvFile      string
	FailOnMissing bool
}

// DefaultOptions returns sensible defaults for loading configuration
func DefaultOptions() *LoadOptions {
	return &LoadOptions{
		EnvFile:      ".env",
		FailOnMissing: false,
	}
}

// Load creates a new Config by loading environment variables
func Load(fields []ConfigField, opts *LoadOptions) (*Config, error) {
	if opts == nil {
		opts = DefaultOptions()
	}

	// Load environment variables from .env file if it exists
	if opts.EnvFile != "" {
		if err := godotenv.Load(opts.EnvFile); err != nil {
			if opts.FailOnMissing {
				return nil, fmt.Errorf("failed to load env file %s: %w", opts.EnvFile, err)
			}
			log.Printf("Warning: .env file not found or could not be loaded: %v", err)
		}
	}

	config := &Config{
		fields: make(map[string]string),
	}

	// Load each field
	for _, field := range fields {
		value := os.Getenv(field.Key)
		if value == "" {
			if field.Required {
				return nil, fmt.Errorf("required configuration field %s is not set", field.Key)
			}
			value = field.DefaultValue
		}
		config.fields[field.Key] = value
	}

	return config, nil
}

// Get retrieves a configuration value by key
func (c *Config) Get(key string) string {
	return c.fields[key]
}

// GetOrDefault retrieves a configuration value or returns a default
func (c *Config) GetOrDefault(key, defaultValue string) string {
	if value, exists := c.fields[key]; exists && value != "" {
		return value
	}
	return defaultValue
}

// Set updates a configuration value (useful for testing)
func (c *Config) Set(key, value string) {
	c.fields[key] = value
}

// Has checks if a configuration key exists
func (c *Config) Has(key string) bool {
	_, exists := c.fields[key]
	return exists
}

// All returns all configuration fields
func (c *Config) All() map[string]string {
	result := make(map[string]string)
	for k, v := range c.fields {
		result[k] = v
	}
	return result
}
