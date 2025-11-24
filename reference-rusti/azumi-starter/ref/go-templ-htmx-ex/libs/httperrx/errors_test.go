package httperrx

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestNewBadRequestError(t *testing.T) {
	err := NewBadRequestError("bad request")
	if err.Code != http.StatusBadRequest {
		t.Errorf("Code = %d, want %d", err.Code, http.StatusBadRequest)
	}
	if err.Message != "bad request" {
		t.Errorf("Message = %s, want bad request", err.Message)
	}
}

func TestNewUnauthorizedError(t *testing.T) {
	err := NewUnauthorizedError("unauthorized")
	if err.Code != http.StatusUnauthorized {
		t.Errorf("Code = %d, want %d", err.Code, http.StatusUnauthorized)
	}
}

func TestNewForbiddenError(t *testing.T) {
	err := NewForbiddenError("forbidden")
	if err.Code != http.StatusForbidden {
		t.Errorf("Code = %d, want %d", err.Code, http.StatusForbidden)
	}
}

func TestNewNotFoundError(t *testing.T) {
	err := NewNotFoundError("not found")
	if err.Code != http.StatusNotFound {
		t.Errorf("Code = %d, want %d", err.Code, http.StatusNotFound)
	}
}

func TestNewInternalServerError(t *testing.T) {
	err := NewInternalServerError("internal error")
	if err.Code != http.StatusInternalServerError {
		t.Errorf("Code = %d, want %d", err.Code, http.StatusInternalServerError)
	}
}

func TestNewError(t *testing.T) {
	err := NewError(418, "I'm a teapot")
	if err.Code != 418 {
		t.Errorf("Code = %d, want 418", err.Code)
	}
	if err.Message != "I'm a teapot" {
		t.Errorf("Message = %s, want I'm a teapot", err.Message)
	}
}

func TestAppErrorError(t *testing.T) {
	err := NewBadRequestError("test error")
	if err.Error() != "test error" {
		t.Errorf("Error() = %s, want test error", err.Error())
	}
}

func TestAppErrorWriteJSON(t *testing.T) {
	err := NewBadRequestError("test error")
	w := httptest.NewRecorder()

	err.WriteJSON(w)

	if w.Code != http.StatusBadRequest {
		t.Errorf("Status code = %d, want %d", w.Code, http.StatusBadRequest)
	}

	contentType := w.Header().Get("Content-Type")
	if contentType != "application/json" {
		t.Errorf("Content-Type = %s, want application/json", contentType)
	}

	var response AppError
	if err := json.NewDecoder(w.Body).Decode(&response); err != nil {
		t.Fatalf("Failed to decode response: %v", err)
	}

	if response.Code != http.StatusBadRequest {
		t.Errorf("Response code = %d, want %d", response.Code, http.StatusBadRequest)
	}

	if response.Message != "test error" {
		t.Errorf("Response message = %s, want test error", response.Message)
	}
}

func TestWriteError(t *testing.T) {
	tests := []struct {
		name       string
		err        error
		wantCode   int
		wantMsg    string
	}{
		{
			name:     "AppError",
			err:      NewNotFoundError("not found"),
			wantCode: http.StatusNotFound,
			wantMsg:  "not found",
		},
		{
			name:     "Generic error",
			err:      &testError{msg: "generic error"},
			wantCode: http.StatusInternalServerError,
			wantMsg:  "generic error",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			w := httptest.NewRecorder()
			WriteError(w, tt.err)

			if w.Code != tt.wantCode {
				t.Errorf("Status code = %d, want %d", w.Code, tt.wantCode)
			}

			var response AppError
			if err := json.NewDecoder(w.Body).Decode(&response); err != nil {
				t.Fatalf("Failed to decode response: %v", err)
			}

			if response.Message != tt.wantMsg {
				t.Errorf("Response message = %s, want %s", response.Message, tt.wantMsg)
			}
		})
	}
}

func TestErrorHandler(t *testing.T) {
	handler := ErrorHandler(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		panic("test panic")
	}))

	w := httptest.NewRecorder()
	r := httptest.NewRequest("GET", "/", nil)

	handler.ServeHTTP(w, r)

	if w.Code != http.StatusInternalServerError {
		t.Errorf("Status code = %d, want %d", w.Code, http.StatusInternalServerError)
	}

	var response AppError
	if err := json.NewDecoder(w.Body).Decode(&response); err != nil {
		t.Fatalf("Failed to decode response: %v", err)
	}

	if response.Code != http.StatusInternalServerError {
		t.Errorf("Response code = %d, want %d", response.Code, http.StatusInternalServerError)
	}
}

func TestErrorHandlerNoPanic(t *testing.T) {
	handler := ErrorHandler(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("OK"))
	}))

	w := httptest.NewRecorder()
	r := httptest.NewRequest("GET", "/", nil)

	handler.ServeHTTP(w, r)

	if w.Code != http.StatusOK {
		t.Errorf("Status code = %d, want %d", w.Code, http.StatusOK)
	}

	if w.Body.String() != "OK" {
		t.Errorf("Body = %s, want OK", w.Body.String())
	}
}

// testError is a helper type for testing generic errors
type testError struct {
	msg string
}

func (e *testError) Error() string {
	return e.msg
}
