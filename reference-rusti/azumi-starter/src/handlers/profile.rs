use axum::{response::Html, Extension};
use rusti::rusti;

use crate::{error::Result, services::session::AuthenticatedUser, templates};

pub async fn profile_page(
    Extension(auth_user): Extension<AuthenticatedUser>,
) -> Result<Html<String>> {
    let user = auth_user.user_context();
    let page = rusti! {
        @templates::base_layout(title = "Profile - Azumi Starter".to_string(), is_authenticated = true) {
            @templates::profile_page(user = user.clone())
        }
    };
    Ok(Html(rusti::render_to_string(&page)))
}
