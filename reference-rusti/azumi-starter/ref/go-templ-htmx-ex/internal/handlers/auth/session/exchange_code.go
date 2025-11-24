package session

import (
	"encoding/json"
	"fmt"
	"net/http"
)

// ExchangeCodeHandler exchanges OAuth authorization code for tokens
// This handler is responsible ONLY for exchanging auth codes for session tokens
func (h *SessionHandler) ExchangeCodeHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Printf("🔄 CODE: === Exchange authorization code STARTED ===\n")
	fmt.Printf("🔄 CODE: Request URL: %s\n", r.URL.String())
	fmt.Printf("🔄 CODE: Method: %s\n", r.Method)
	fmt.Printf("🔄 CODE: Headers: %v\n", r.Header)

	w.Header().Set("Content-Type", "application/json")

	// STEP 1: Decode the request body
	fmt.Printf("🔄 CODE: Decoding request body...\n")
	var req struct {
		AuthCode string `json:"auth_code"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		fmt.Printf("🔄 CODE: ❌ Failed to decode request: %v\n", err)
		w.WriteHeader(http.StatusBadRequest)
		_ = json.NewEncoder(w).Encode(map[string]interface{}{
			"error": "Invalid request body",
		})
		return
	}

	fmt.Printf("🔄 CODE: ✅ Request decoded successfully: %s\n", req.AuthCode)

	if req.AuthCode == "" {
		fmt.Printf("🔄 CODE: ❌ Missing authorization code\n")
		w.WriteHeader(http.StatusBadRequest)
		_ = json.NewEncoder(w).Encode(map[string]interface{}{
			"error": "Missing authorization code",
		})
		return
	}

	fmt.Printf("🔄 CODE: ✅ Authorization code received, length: %d\n", len(req.AuthCode))

	// STEP 2: Call the auth service
	fmt.Printf("🔄 CODE: Calling auth service to exchange code for tokens...\n")
	fmt.Printf("🔄 CODE: AuthService address: %p\n", h.AuthService)

	authResp, err := h.AuthService.ExchangeCodeForTokens(req.AuthCode)
	if err != nil {
		fmt.Printf("🔄 CODE: ❌ Auth service call failed: %v\n", err)
		w.WriteHeader(http.StatusInternalServerError)
		_ = json.NewEncoder(w).Encode(map[string]interface{}{
			"error": err.Error(),
		})
		return
	}

	fmt.Printf("🔄 CODE: ✅ Auth service call completed successfully\n")
	fmt.Printf("🔄 CODE: Auth response: success=%v, id_token length=%d\n", authResp.Success, len(authResp.IdToken))

	if !authResp.Success {
		fmt.Printf("🔄 CODE: ❌ Auth service returned failure: %s\n", authResp.Error)
		w.WriteHeader(http.StatusInternalServerError)
		_ = json.NewEncoder(w).Encode(map[string]interface{}{
			"error": authResp.Error,
		})
		return
	}

	fmt.Printf("🔄 CODE: ✅ Auth service returned success: %v\n", authResp.Success)

	// STEP 3: Set the session cookie
	fmt.Printf("🔄 CODE: Setting session cookie with session_id: %s\n", authResp.IdToken)
	sessionCookie := &http.Cookie{
		Name:     "session_id",
		Value:    authResp.IdToken, // Using session_id from response
		Path:     "/",
		MaxAge:   2592000, // 30 days
		HttpOnly: true,
		Secure:   false, // Set to true in production with HTTPS
	}

	http.SetCookie(w, sessionCookie)
	fmt.Printf("🔄 CODE: ✅ Session cookie set successfully\n")

	// STEP 4: Return success response
	fmt.Printf("🔄 CODE: Returning success response...\n")
	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"message": "Tokens exchanged successfully",
	}); err != nil {
		fmt.Printf("🔄 CODE: ❌ Error encoding success response: %v\n", err)
	}

	fmt.Printf("🔄 CODE: === Token exchange COMPLETED ===\n")
}
