use axum::{extract::State, response::Html, Extension};
use chrono::Utc;
use rusti::rusti;

use crate::{
    error::Result,
    repositories::UserRepository,
    services::session::AuthenticatedUser,
    templates::{self, AdminStats},
    AppState,
};

pub async fn admin_dashboard(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
) -> Result<Html<String>> {
    // Check if user is admin (you might want to fetch full user from DB)
    // For now, we'll allow all authenticated users to see admin dashboard
    // In production, add proper admin check

    let user_repo = UserRepository::new(state.db.clone());

    // Get statistics
    let total_users = user_repo.count_total().await?;

    let today_start = Utc::now()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Utc)
        .unwrap();
    let users_today = user_repo.count_since(today_start).await?;

    let week_start = Utc::now() - chrono::Duration::days(7);
    let users_this_week = user_repo.count_since(week_start).await?;

    let stats = AdminStats {
        total_users,
        users_today,
        users_this_week,
    };

    // Get recent users
    let recent_users = user_repo.list_recent(10).await?;

    let page = rusti! {
        @templates::base_layout(title = "Admin Dashboard - Azumi Starter".to_string(), is_authenticated = true) {
            @templates::admin_dashboard(stats = stats.clone(), recent_users = recent_users.clone())
        }
    };
    // let page = "<h1>Hello from Admin Dashboard!</h1>".to_string();
    Ok(Html(rusti::render_to_string(&page)))
}
