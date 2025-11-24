package cachex

import (
	"sync"
	"time"
)

// Cache is a generic TTL-based cache with thread-safe operations
type Cache[T any] struct {
	sync.RWMutex
	entries map[string]*entry[T]
	ttl     time.Duration
	
}

// entry represents a cached value with expiration
type entry[T any] struct {
	value     T
	expiresAt time.Time
}

// New creates a new cache with the specified TTL
func New[T any](ttl time.Duration) *Cache[T] {
	return &Cache[T]{
		entries: make(map[string]*entry[T]),
		ttl:     ttl,
	}
}

// Get retrieves a cached value if it exists and hasn't expired
func (c *Cache[T]) Get(key string) (T, bool) {
	c.RLock()
	defer c.RUnlock()

	e, exists := c.entries[key]
	if !exists {
		var zero T
		return zero, false
	}

	if time.Now().After(e.expiresAt) {
		// Expired entry - will be cleaned up on next cleanup cycle
		var zero T
		return zero, false
	}

	return e.value, true
}

// Set stores a value in the cache with the configured TTL
func (c *Cache[T]) Set(key string, value T) {
	c.Lock()
	defer c.Unlock()

	c.entries[key] = &entry[T]{
		value:     value,
		expiresAt: time.Now().Add(c.ttl),
	}
}

// SetWithTTL stores a value with a custom TTL
func (c *Cache[T]) SetWithTTL(key string, value T, ttl time.Duration) {
	c.Lock()
	defer c.Unlock()

	c.entries[key] = &entry[T]{
		value:     value,
		expiresAt: time.Now().Add(ttl),
	}
}

// Delete removes a value from the cache
func (c *Cache[T]) Delete(key string) {
	c.Lock()
	defer c.Unlock()

	delete(c.entries, key)
}

// Clear removes all entries from the cache
func (c *Cache[T]) Clear() {
	c.Lock()
	defer c.Unlock()

	c.entries = make(map[string]*entry[T])
}

// Cleanup removes expired entries from the cache
func (c *Cache[T]) Cleanup() int {
	c.Lock()
	defer c.Unlock()

	now := time.Now()
	removed := 0

	for key, e := range c.entries {
		if now.After(e.expiresAt) {
			delete(c.entries, key)
			removed++
		}
	}

	return removed
}

// Size returns the number of entries in the cache (including expired)
func (c *Cache[T]) Size() int {
	c.RLock()
	defer c.RUnlock()

	return len(c.entries)
}

// StartCleanupRoutine starts a background goroutine that periodically cleans up expired entries
func (c *Cache[T]) StartCleanupRoutine(interval time.Duration) chan struct{} {
	stop := make(chan struct{})

	go func() {
		ticker := time.NewTicker(interval)
		defer ticker.Stop()

		for {
			select {
			case <-ticker.C:
				c.Cleanup()
			case <-stop:
				return
			}
		}
	}()

	return stop
}
