use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_cookies::CookieManagerLayer;

use crate::{handlers, middleware::AuthMiddleware, AppState};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        // Public routes
        .route("/", get(handlers::home_handler))
        .route("/login", get(handlers::login_page))
        .route("/auth/callback", get(handlers::auth_callback))
        .route("/health", get(health_check))
        // Auth API routes (public)
        .route("/api/auth/exchange-code", post(handlers::exchange_code))
        // Protected routes
        .route("/profile", get(handlers::profile_page))
        .route("/admin", get(handlers::admin_dashboard))
        .route("/api/auth/logout", post(handlers::logout))
        // Add authentication middleware
        .layer(middleware::from_fn_with_state(
            state.clone(),
            AuthMiddleware::middleware,
        ))
        // Add cookie layer
        .layer(CookieManagerLayer::new())
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}
