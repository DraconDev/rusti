use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

use crate::{
    error::AppError,
    services::{
        auth::AuthService,
        session::{AuthenticatedUser, SessionService},
    },
    AppState,
};

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub async fn middleware(
        State(state): State<AppState>,
        mut req: Request,
        next: Next,
    ) -> Result<Response, Response> {
        let path = req.uri().path();

        // Public routes that don't require authentication
        if Self::is_public_route(path) {
            return Ok(next.run(req).await);
        }

        // Extract session ID
        let session_id = match SessionService::extract_session_from_request(&req) {
            Some(sid) => sid,
            None => {
                // No session, redirect to login for HTML pages
                if Self::should_redirect_to_login(path) {
                    return Err(Redirect::to("/login").into_response());
                }
                return Err(AppError::Unauthorized.into_response());
            }
        };

        // Validate session with auth service
        let auth_service = AuthService::new(
            state.config.auth_service_url.clone(),
            state.http_client.clone(),
        );

        let user_context = match auth_service.validate_session(&session_id).await {
            Ok(ctx) => ctx,
            Err(_) => {
                // Invalid session, redirect to login
                if Self::should_redirect_to_login(path) {
                    return Err(Redirect::to("/login").into_response());
                }
                return Err(AppError::Unauthorized.into_response());
            }
        };

        // Store user context in request extensions
        req.extensions_mut().insert(AuthenticatedUser(user_context));

        Ok(next.run(req).await)
    }

    fn is_public_route(path: &str) -> bool {
        matches!(
            path,
            "/" | "/login" | "/health" | "/auth/callback" | "/api/auth/exchange-code"
        ) || path.starts_with("/static/")
            || path.starts_with("/auth/")
    }

    fn should_redirect_to_login(path: &str) -> bool {
        // Redirect HTML pages to login, return 401 for API routes
        !path.starts_with("/api/")
    }
}
