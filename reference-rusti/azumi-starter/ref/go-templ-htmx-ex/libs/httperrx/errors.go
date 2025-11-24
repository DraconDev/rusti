package httperrx

import (
	"encoding/json"
	"log"
	"net/http"
)

// AppError represents application-specific HTTP errors
type AppError struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
}

// Error implements the error interface
func (e *AppError) Error() string {
	return e.Message
}

// NewBadRequestError creates a 400 Bad Request error
func NewBadRequestError(message string) *AppError {
	return &AppError{
		Code:    http.StatusBadRequest,
		Message: message,
	}
}

// NewUnauthorizedError creates a 401 Unauthorized error
func NewUnauthorizedError(message string) *AppError {
	return &AppError{
		Code:    http.StatusUnauthorized,
		Message: message,
	}
}

// NewForbiddenError creates a 403 Forbidden error
func NewForbiddenError(message string) *AppError {
	return &AppError{
		Code:    http.StatusForbidden,
		Message: message,
	}
}

// NewNotFoundError creates a 404 Not Found error
func NewNotFoundError(message string) *AppError {
	return &AppError{
		Code:    http.StatusNotFound,
		Message: message,
	}
}

// NewInternalServerError creates a 500 Internal Server Error
func NewInternalServerError(message string) *AppError {
	return &AppError{
		Code:    http.StatusInternalServerError,
		Message: message,
	}
}

// NewError creates a custom HTTP error with any status code
func NewError(code int, message string) *AppError {
	return &AppError{
		Code:    code,
		Message: message,
	}
}

// WriteJSON writes an AppError as JSON response
func (e *AppError) WriteJSON(w http.ResponseWriter) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(e.Code)
	if err := json.NewEncoder(w).Encode(e); err != nil {
		log.Printf("Error encoding error response: %v", err)
	}
}

// WriteError is a helper to write any error as JSON
func WriteError(w http.ResponseWriter, err error) {
	if appErr, ok := err.(*AppError); ok {
		appErr.WriteJSON(w)
		return
	}
	
	// Default to internal server error for unknown errors
	NewInternalServerError(err.Error()).WriteJSON(w)
}

// ErrorHandler middleware catches panics and converts them to 500 errors
func ErrorHandler(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		defer func() {
			if err := recover(); err != nil {
				log.Printf("Panic recovered: %v", err)
				NewInternalServerError("Internal server error").WriteJSON(w)
			}
		}()
		next.ServeHTTP(w, r)
	})
}
