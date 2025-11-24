use axum::{
    extract::State,
    response::{Html, IntoResponse, Json, Redirect},
};
use rusti::rusti;
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};

use crate::{
    db::CreateUser, error::Result, repositories::UserRepository, services::auth::AuthService,
    templates, AppState,
};

pub async fn login_page(State(state): State<AppState>) -> Html<String> {
    let url = &state.config.auth_service_url;
    let page = rusti! {
        @templates::base_layout(title = "Login - Azumi Starter".to_string(), is_authenticated = false) {
            @templates::login_page(auth_service_url = url.to_string())
        }
    };
    Html(rusti::render_to_string(&page))
}

pub async fn auth_callback() -> Html<String> {
    let page = rusti! {
        @templates::base_layout(title = "Login - Azumi Starter".to_string(), is_authenticated = false) {
            @templates::auth_callback()
        }
    };
    Html(rusti::render_to_string(&page))
}

#[derive(Debug, Deserialize)]
pub struct ExchangeCodeRequest {
    auth_code: String,
}

#[derive(Debug, Serialize)]
pub struct ExchangeCodeResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

pub async fn exchange_code(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<ExchangeCodeRequest>,
) -> Result<impl IntoResponse> {
    tracing::info!("Exchanging auth code: {}", payload.auth_code);

    // Exchange code with auth service
    let auth_service = AuthService::new(
        state.config.auth_service_url.clone(),
        state.http_client.clone(),
    );

    let auth_response = auth_service.exchange_code(&payload.auth_code).await?;

    tracing::info!(
        "Auth exchange successful for user: {}",
        auth_response.user_context.email
    );

    // Set session cookie
    let mut cookie = Cookie::new("session_id", auth_response.session_id);
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);

    // Create or update user in database
    let user_repo = UserRepository::new(state.db.clone());

    match user_repo
        .find_by_auth_id(&auth_response.user_context.user_id)
        .await?
    {
        Some(user) => {
            tracing::info!("Existing user logged in: {}", user.email);
        }
        None => {
            let new_user = CreateUser {
                auth_id: auth_response.user_context.user_id.clone(),
                email: auth_response.user_context.email.clone(),
                name: auth_response.user_context.name.clone(),
                picture: auth_response.user_context.picture.clone(),
            };
            let user = user_repo.create(new_user).await?;
            tracing::info!("New user created: {}", user.email);
        }
    }

    Ok(Json(ExchangeCodeResponse {
        success: true,
        error: None,
    }))
}

pub async fn logout(State(state): State<AppState>, cookies: Cookies) -> Result<impl IntoResponse> {
    // Get session ID from cookies
    if let Some(cookie) = cookies.get("session_id") {
        let session_id = cookie.value();

        // Call auth service to invalidate session
        let auth_service = AuthService::new(
            state.config.auth_service_url.clone(),
            state.http_client.clone(),
        );

        let _ = auth_service.logout(session_id).await; // Ignore errors
    }

    // Remove session cookie
    let mut cookie = Cookie::new("session_id", "");
    cookie.set_max_age(time::Duration::seconds(0));
    cookie.set_path("/");
    cookies.add(cookie);

    Ok(Redirect::to("/"))
}
