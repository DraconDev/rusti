-- name: GetUserPreferences :one
SELECT * FROM user_preferences
WHERE user_id = $1 LIMIT 1;

-- name: CreateUserPreferences :one
INSERT INTO user_preferences (
    user_id, theme, language, timezone, email_notifications, email_billing, push_notifications
) VALUES (
    $1, $2, $3, $4, $5, $6, $7
)
RETURNING *;

-- name: UpdateUserPreferences :one
UPDATE user_preferences
SET 
    theme = COALESCE(sqlc.narg('theme'), theme),
    language = COALESCE(sqlc.narg('language'), language),
    timezone = COALESCE(sqlc.narg('timezone'), timezone),
    email_notifications = COALESCE(sqlc.narg('email_notifications'), email_notifications),
    email_billing = COALESCE(sqlc.narg('email_billing'), email_billing),
    push_notifications = COALESCE(sqlc.narg('push_notifications'), push_notifications),
    updated_at = NOW()
WHERE user_id = $1
RETURNING *;
