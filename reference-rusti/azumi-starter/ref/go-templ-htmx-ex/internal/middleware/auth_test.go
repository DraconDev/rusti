package middleware

import (
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestAuthMiddlewareBehavior(t *testing.T) {
	fmt.Println("🧪 Testing AuthMiddleware Behavior")

	// Test that middleware properly chains handlers
	t.Run("handler_chain", func(t *testing.T) {
		req := httptest.NewRequest("GET", "/", nil)
		rr := httptest.NewRecorder()

		// Create a test handler that always succeeds
		testHandler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			w.WriteHeader(http.StatusOK)
			_, _ = w.Write([]byte("OK"))
		})

		// Apply middleware
		middleware := AuthMiddleware(testHandler)
		middleware.ServeHTTP(rr, req)

		// Should succeed for public routes
		if rr.Code != http.StatusOK {
			t.Errorf("Expected status 200 for public route, got %d", rr.Code)
		}
	})

	// Test public routes behavior
	t.Run("public_routes", func(t *testing.T) {
		publicRoutes := []string{"/", "/login", "/health", "/test"}

		for _, route := range publicRoutes {
			t.Run("route_"+route, func(t *testing.T) {
				req := httptest.NewRequest("GET", route, nil)
				rr := httptest.NewRecorder()

				testHandler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
					w.WriteHeader(http.StatusOK)
					_, _ = w.Write([]byte("OK"))
				})

				AuthMiddleware(testHandler).ServeHTTP(rr, req)

				// Public routes should not redirect
				if rr.Code == http.StatusFound {
					t.Errorf("Public route %q should not redirect, got redirect to %q", route, rr.Header().Get("Location"))
				}
			})
		}
	})

	// Test protected routes redirect
	t.Run("protected_routes_redirect", func(t *testing.T) {
		protectedRoutes := []string{"/profile", "/admin"}

		for _, route := range protectedRoutes {
			t.Run("route_"+route, func(t *testing.T) {
				req := httptest.NewRequest("GET", route, nil)
				rr := httptest.NewRecorder()

				testHandler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
					w.WriteHeader(http.StatusOK)
					_, _ = w.Write([]byte("OK"))
				})

				AuthMiddleware(testHandler).ServeHTTP(rr, req)

				// Protected routes should redirect when no session
				if rr.Code != http.StatusFound {
					t.Errorf("Protected route %q should redirect, got status %d", route, rr.Code)
				}

				location := rr.Header().Get("Location")
				if location != "/login" {
					t.Errorf("Expected redirect to /login, got %q", location)
				}
			})
		}
	})

	// Test API routes return 401
	t.Run("api_routes_401", func(t *testing.T) {
		apiRoutes := []string{"/api/admin/users", "/api/admin/analytics"}

		for _, route := range apiRoutes {
			t.Run("route_"+route, func(t *testing.T) {
				req := httptest.NewRequest("GET", route, nil)
				rr := httptest.NewRecorder()

				testHandler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
					w.WriteHeader(http.StatusOK)
					if _, err := w.Write([]byte("OK")); err != nil {
						t.Errorf("Failed to write response: %v", err)
					}
					
				})

				AuthMiddleware(testHandler).ServeHTTP(rr, req)

				// API routes should return 401 when no session
				if rr.Code != http.StatusUnauthorized {
					t.Errorf("Protected API route %q should return 401, got status %d", route, rr.Code)
				}

				contentType := rr.Header().Get("Content-Type")
				if contentType != "application/json" {
					t.Errorf("Expected JSON response, got %q", contentType)
				}
			})
		}
	})
}

func TestGetUserFromContextBehavior(t *testing.T) {
	fmt.Println("🧪 Testing GetUserFromContext Behavior")

	// Test with empty context
	t.Run("empty_context", func(t *testing.T) {
		req := httptest.NewRequest("GET", "/", nil)

		userInfo := GetUserFromContext(req)

		if userInfo.LoggedIn {
			t.Error("Expected user to not be logged in with empty context")
		}
	})

	// Test context with user info (simulating middleware behavior)
	t.Run("with_user_context", func(t *testing.T) {
		req := httptest.NewRequest("GET", "/", nil)

		// In the actual middleware, this would be set by validateSession
		// We can't easily test this without accessing internal middleware logic

		userInfo := GetUserFromContext(req)

		// Since we didn't set the context, user should not be logged in
		if userInfo.LoggedIn {
			t.Error("Expected user to not be logged in without proper context setup")
		}

		t.Log("GetUserFromContext behaves correctly with empty context")
	})
}

func TestMiddlewareIntegration(t *testing.T) {
	fmt.Println("🧪 Testing Middleware Integration")

	// Test the complete middleware flow
	t.Run("complete_flow", func(t *testing.T) {
		// Step 1: Test public route flow
		req := httptest.NewRequest("GET", "/login", nil)
		rr := httptest.NewRecorder()

		handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			w.WriteHeader(http.StatusOK)
		})

		AuthMiddleware(handler).ServeHTTP(rr, req)

		if rr.Code != http.StatusOK {
			t.Errorf("Login route should be accessible, got status %d", rr.Code)
		}

		// Step 2: Test protected route redirection
		req = httptest.NewRequest("GET", "/profile", nil)
		rr = httptest.NewRecorder()

		AuthMiddleware(handler).ServeHTTP(rr, req)

		if rr.Code != http.StatusFound {
			t.Errorf("Profile should redirect when not authenticated, got status %d", rr.Code)
		}

		// Step 3: Test API route 401 response
		req = httptest.NewRequest("GET", "/api/admin/users", nil)
		rr = httptest.NewRecorder()

		AuthMiddleware(handler).ServeHTTP(rr, req)

		if rr.Code != http.StatusUnauthorized {
			t.Errorf("Admin API should return 401 when not authenticated, got status %d", rr.Code)
		}
	})

	// Test auth API routes are accessible
	t.Run("auth_api_accessibility", func(t *testing.T) {
		authAPIRoutes := []string{
			"/api/auth/exchange-code",
			"/api/auth/set-session",
			"/api/auth/logout",
		}

		for _, route := range authAPIRoutes {
			t.Run("route_"+route, func(t *testing.T) {
				req := httptest.NewRequest("POST", route, nil)
				rr := httptest.NewRecorder()

				handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
					w.WriteHeader(http.StatusOK)
					if _, err := w.Write([]byte("OK")); err != nil {
						t.Errorf("Failed to write response: %v", err)
					}
				})

				AuthMiddleware(handler).ServeHTTP(rr, req)

				// Auth API routes should not redirect or return 401
				if rr.Code == http.StatusFound {
					t.Errorf("Auth API route %q should not redirect, got redirect", route)
				}
				if rr.Code == http.StatusUnauthorized {
					t.Errorf("Auth API route %q should not require authentication, got 401", route)
				}
			})
		}
	})
}

// Benchmark tests for performance
func BenchmarkAuthMiddleware(b *testing.B) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	})

	middleware := AuthMiddleware(handler)

	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		req := httptest.NewRequest("GET", "/", nil)
		rr := httptest.NewRecorder()

		middleware.ServeHTTP(rr, req)
	}
}

func BenchmarkGetUserFromContext(b *testing.B) {
	req := httptest.NewRequest("GET", "/", nil)

	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		GetUserFromContext(req)
	}
}
