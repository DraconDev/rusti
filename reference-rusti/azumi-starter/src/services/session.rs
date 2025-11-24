use axum::extract::Request;
use tower_cookies::Cookies;

use crate::services::auth::UserContext;

pub struct SessionService;

impl SessionService {
    pub fn extract_session_id(cookies: &Cookies) -> Option<String> {
        cookies
            .get("session_id")
            .map(|cookie| cookie.value().to_string())
    }

    pub fn extract_session_from_request(req: &Request) -> Option<String> {
        req.headers()
            .get("cookie")
            .and_then(|value| value.to_str().ok())
            .and_then(|cookie_str| {
                cookie_str
                    .split(';')
                    .find(|s| s.trim().starts_with("session_id="))
                    .and_then(|s| s.split('=').nth(1))
                    .map(|s| s.trim().to_string())
            })
    }
}

// Extension type for storing user context in request
#[derive(Clone)]
pub struct AuthenticatedUser(pub UserContext);

impl AuthenticatedUser {
    pub fn user_context(&self) -> &UserContext {
        &self.0
    }
}
