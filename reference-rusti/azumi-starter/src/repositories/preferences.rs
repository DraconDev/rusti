use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::{UpdatePreferences, UserPreference},
    error::Result,
};

#[derive(Clone)]
pub struct PreferencesRepository {
    pool: PgPool,
}

impl PreferencesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserPreference>> {
        let prefs = sqlx::query_as::<_, UserPreference>(
            "SELECT * FROM user_preferences WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(prefs)
    }

    pub async fn create_default(&self, user_id: Uuid) -> Result<UserPreference> {
        let prefs = sqlx::query_as::<_, UserPreference>(
            r#"
            INSERT INTO user_preferences (user_id)
            VALUES ($1)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(prefs)
    }

    pub async fn update(
        &self,
        user_id: Uuid,
        updates: UpdatePreferences,
    ) -> Result<UserPreference> {
        // Build dynamic query based on what fields are being updated
        let mut query = "UPDATE user_preferences SET ".to_string();
        let mut params: Vec<String> = vec![];
        let mut param_count = 1;

        if let Some(theme) = &updates.theme {
            params.push(format!("theme = ${}", param_count));
            param_count += 1;
        }
        if let Some(language) = &updates.language {
            params.push(format!("language = ${}", param_count));
            param_count += 1;
        }
        if let Some(email_notifs) = updates.email_notifications {
            params.push(format!("email_notifications = ${}", param_count));
            param_count += 1;
        }
        if let Some(push_notifs) = updates.push_notifications {
            params.push(format!("push_notifications = ${}", param_count));
            param_count += 1;
        }

        if params.is_empty() {
            // No updates, just fetch current
            return self
                .find_by_user_id(user_id)
                .await?
                .ok_or(crate::error::AppError::NotFound);
        }

        query.push_str(&params.join(", "));
        query.push_str(&format!(" WHERE user_id = ${} RETURNING *", param_count));

        let mut q = sqlx::query_as::<_, UserPreference>(&query);

        if let Some(theme) = &updates.theme {
            q = q.bind(theme);
        }
        if let Some(language) = &updates.language {
            q = q.bind(language);
        }
        if let Some(email_notifs) = updates.email_notifications {
            q = q.bind(email_notifs);
        }
        if let Some(push_notifs) = updates.push_notifications {
            q = q.bind(push_notifs);
        }

        q = q.bind(user_id);

        let prefs = q.fetch_one(&self.pool).await?;

        Ok(prefs)
    }
}
