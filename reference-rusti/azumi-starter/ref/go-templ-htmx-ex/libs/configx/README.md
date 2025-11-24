# configx

A flexible, reusable configuration management library for Go applications.

## Features

- **Environment Variable Loading**: Automatic .env file support
- **Type-Safe Configuration**: Structured configuration with validation
- **Default Values**: Fallback values for missing configuration
- **Required Fields**: Enforce critical configuration presence
- **Flexible Options**: Customizable loading behavior

## Installation

```bash
go get github.com/dracondev/go-templ-htmx-ex/libs/configx
```

## Usage

### Basic Example

```go
package main

import (
    "log"
    "github.com/dracondev/go-templ-htmx-ex/libs/configx"
)

func main() {
    // Define configuration fields
    fields := []configx.ConfigField{
        {
            Key:          "PORT",
            DefaultValue: "8080",
            Required:     false,
            Description:  "Server port",
        },
        {
            Key:          "DATABASE_URL",
            DefaultValue: "",
            Required:     true,
            Description:  "Database connection string",
        },
    }

    // Load configuration
    config, err := configx.Load(fields, configx.DefaultOptions())
    if err != nil {
        log.Fatal(err)
    }

    // Use configuration
    port := config.Get("PORT")
    dbURL := config.Get("DATABASE_URL")
    
    log.Printf("Server starting on port %s", port)
    log.Printf("Connecting to database: %s", dbURL)
}
```

### Custom Options

```go
opts := &configx.LoadOptions{
    EnvFile:       ".env.production",
    FailOnMissing: true,  // Fail if .env file is missing
}

config, err := configx.Load(fields, opts)
```

### Configuration Methods

```go
// Get a value
value := config.Get("KEY")

// Get with fallback
value := config.GetOrDefault("KEY", "fallback")

// Check if key exists
if config.Has("KEY") {
    // ...
}

// Set a value (useful for testing)
config.Set("KEY", "value")

// Get all configuration
all := config.All()
```

## API Reference

### ConfigField

```go
type ConfigField struct {
    Key          string  // Environment variable name
    DefaultValue string  // Default value if not set
    Required     bool    // Whether field is required
    Description  string  // Field description
}
```

### LoadOptions

```go
type LoadOptions struct {
    EnvFile       string  // Path to .env file (default: ".env")
    FailOnMissing bool    // Fail if .env file is missing (default: false)
}
```

### Config Methods

- `Get(key string) string` - Get configuration value
- `GetOrDefault(key, defaultValue string) string` - Get value with fallback
- `Set(key, value string)` - Set configuration value
- `Has(key string) bool` - Check if key exists
- `All() map[string]string` - Get all configuration fields

## License

MIT
