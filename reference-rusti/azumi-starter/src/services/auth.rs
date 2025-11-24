use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AuthService {
    auth_service_url: String,
    http_client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub session_id: String,
    pub user_context: UserContext,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserContext {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub picture: Option<String>,
}

#[derive(Debug, Serialize)]
struct ExchangeCodeRequest {
    auth_code: String,
}

impl AuthService {
    pub fn new(auth_service_url: String, http_client: reqwest::Client) -> Self {
        Self {
            auth_service_url,
            http_client,
        }
    }

    pub async fn exchange_code(&self, auth_code: &str) -> Result<AuthResponse> {
        let url = format!("{}/api/auth/exchange-code", self.auth_service_url);

        let response = self
            .http_client
            .post(&url)
            .json(&ExchangeCodeRequest {
                auth_code: auth_code.to_string(),
            })
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to call auth service: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "Auth service returned {}: {}",
                status, text
            )));
        }

        let auth_response = response
            .json::<AuthResponse>()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse auth response: {}", e)))?;

        Ok(auth_response)
    }

    pub async fn validate_session(&self, session_id: &str) -> Result<UserContext> {
        let url = format!("{}/api/auth/validate", self.auth_service_url);

        let response = self
            .http_client
            .get(&url)
            .header("Cookie", format!("session_id={}", session_id))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to validate session: {}", e)))?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized);
        }

        if !response.status().is_success() {
            return Err(AppError::Internal(format!(
                "Session validation failed: {}",
                response.status()
            )));
        }

        let user_context = response
            .json::<UserContext>()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse user context: {}", e)))?;

        Ok(user_context)
    }

    pub async fn logout(&self, session_id: &str) -> Result<()> {
        let url = format!("{}/api/auth/logout", self.auth_service_url);

        let response = self
            .http_client
            .post(&url)
            .header("Cookie", format!("session_id={}", session_id))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to logout: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Internal(format!(
                "Logout failed: {}",
                response.status()
            )));
        }

        Ok(())
    }
}
