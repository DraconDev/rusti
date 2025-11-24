# cachex

A generic, type-safe TTL cache library for Go with automatic cleanup and thread-safe operations.

## Features

- **Type-Safe Generics**: Uses Go 1.18+ generics for compile-time type safety
- **Configurable TTL**: Set default TTL per cache or custom TTL per entry
- **Thread-Safe**: RWMutex for concurrent read/write operations
- **Automatic Cleanup**: Background goroutine for expired entry removal
- **Zero Dependencies**: Pure Go standard library

## Installation

```bash
go get github.com/dracondev/go-templ-htmx-ex/libs/cachex
```

## Usage

### Basic Example

```go
package main

import (
    "fmt"
    "time"
    "github.com/dracondev/go-templ-htmx-ex/libs/cachex"
)

func main() {
    // Create a cache with 5-second TTL
    cache := cachex.New[string](5 * time.Second)

    // Set a value
    cache.Set("key1", "value1")

    // Get a value
    if value, ok := cache.Get("key1"); ok {
        fmt.Println(value) // "value1"
    }

    // Wait for expiration
    time.Sleep(6 * time.Second)

    // Value is now expired
    if _, ok := cache.Get("key1"); !ok {
        fmt.Println("Key expired")
    }
}
```

### Custom TTL Per Entry

```go
cache := cachex.New[int](5 * time.Second)

// Use default TTL
cache.Set("short", 100)

// Use custom TTL for this entry
cache.SetWithTTL("long", 200, 1 * time.Hour)
```

### With Structs

```go
type User struct {
    ID    string
    Name  string
    Email string
}

cache := cachex.New[User](15 * time.Second)

cache.Set("user:123", User{
    ID:    "123",
    Name:  "Alice",
    Email: "alice@example.com",
})

if user, ok := cache.Get("user:123"); ok {
    fmt.Printf("User: %s (%s)\n", user.Name, user.Email)
}
```

### Automatic Cleanup

```go
cache := cachex.New[string](5 * time.Second)

// Start background cleanup every 30 seconds
stop := cache.StartCleanupRoutine(30 * time.Second)

// ... use cache ...

// Stop cleanup when done
close(stop)
```

### Manual Cleanup

```go
cache := cachex.New[string](5 * time.Second)

// Add entries
cache.Set("key1", "value1")
cache.Set("key2", "value2")

// Wait for expiration
time.Sleep(6 * time.Second)

// Manually remove expired entries
removed := cache.Cleanup()
fmt.Printf("Removed %d expired entries\n", removed)
```

## API Reference

### Constructor

```go
func New[T any](ttl time.Duration) *Cache[T]
```

Creates a new cache with the specified default TTL.

### Methods

| Method | Description |
|--------|-------------|
| `Get(key string) (T, bool)` | Get value if exists and not expired |
| `Set(key string, value T)` | Set value with default TTL |
| `SetWithTTL(key string, value T, ttl time.Duration)` | Set value with custom TTL |
| `Delete(key string)` | Remove a value |
| `Clear()` | Remove all values |
| `Size() int` | Get number of entries (including expired) |
| `Cleanup() int` | Remove expired entries, returns count removed |
| `StartCleanupRoutine(interval time.Duration) chan struct{}` | Start background cleanup |

## Thread Safety

All operations are thread-safe using `sync.RWMutex`:
- `Get`, `Size` use read locks (concurrent reads allowed)
- `Set`, `Delete`, `Clear`, `Cleanup` use write locks (exclusive access)

## Performance

- **Get**: O(1) average
- **Set**: O(1) average
- **Delete**: O(1) average
- **Cleanup**: O(n) where n is number of entries

## Testing

```bash
cd libs/cachex
go test -v
```

**Test Coverage**: 13/13 tests passing
- Basic operations (Set, Get, Delete, Clear)
- TTL expiration
- Custom TTL per entry
- Cleanup routines
- Generic types (string, int, struct, pointer)
- Concurrent operations

## License

MIT
