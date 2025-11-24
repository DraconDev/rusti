package middleware

import (
	"time"

	"github.com/DraconDev/go-templ-htmx-ex/templates/layouts"
	"github.com/dracondev/go-templ-htmx-ex/libs/cachex"
)

// SessionCache stores validation results with 15-second TTL
type SessionCache struct {
	*cachex.Cache[layouts.UserInfo]
}

// NewSessionCache creates a new session cache
func NewSessionCache() *SessionCache {
	return &SessionCache{
		Cache: cachex.New[layouts.UserInfo](15 * time.Second),
	}
}

// Get retrieves cached user info if not expired
func (c *SessionCache) Get(sessionID string) (layouts.UserInfo, bool) {
	return c.Cache.Get(sessionID)
}

// Set caches user info with 15-second TTL
func (c *SessionCache) Set(sessionID string, userInfo layouts.UserInfo) {
	c.Cache.Set(sessionID, userInfo)
}
