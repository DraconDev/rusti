//! Lesson 34: Capstone - Social Profile
//!
//! A comprehensive example integrating Head, Schema, Forms, CSS Vars, and HTMX.

use azumi::{head, html};
use serde::{Deserialize, Serialize};

// 1. Data Structures with Schema Support
#[derive(Serialize, Deserialize, azumi::Schema, Clone)]
#[schema(type = "Person")]
pub struct UserProfile {
    pub full_name: String,
    pub handle: String,
    pub bio: String,
    pub location: String,
    pub website: String,
    pub stats: UserStats,
    pub theme_color: String,
}

#[derive(Serialize, Deserialize, azumi::Schema, Clone)]
#[schema(skip)] // Don't generate separate schema for stats
pub struct UserStats {
    pub followers: u32,
    pub following: u32,
    pub posts: u32,
}

// 2. Components

#[azumi::component]
fn stat_card(label: &'static str, value: u32) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson34.css" />
        <div class="stat-item">
            <span class="stat-value">{value}</span>
            <span class="stat-label">{label}</span>
        </div>
    }
}

#[azumi::component]
pub fn social_profile() -> impl azumi::Component {
    
    html! {
        <html lang="en">
        <head>
        // 3. SEO & Meta Tags
        // Mock Data
        @let user = UserProfile {
                    full_name: "Alex Rivera".to_string(),
                    handle: "@arivera".to_string(),
                    bio: "Rust Developer & UI Enthusiast. Building the future of web apps with Azumi."
                        .to_string(),
                    location: "San Francisco, CA".to_string(),
                    website: "https://azumi.rs".to_string(),
                    stats: UserStats {
                        followers: 1250,
                        following: 420,
                        posts: 89,
                    },
                    theme_color: "#8b5cf6".to_string(),
    };      
                {head! {
                    title: user.full_name.clone(),
                    description: user.bio.clone(),
                    image: "/static/images/default-avatar.png",
                    type: "profile"
                }}

                // 4. JSON-LD Schema
                <script type="application/ld+json">
                    {user.to_schema_script()}
                </script>

                <style src="/static/pages/lesson34.css" />
            </head>
            <body>
                <style src="/static/pages/lesson34.css" />

                // 5. CSS Variables for Theming
                <div class="profile-container" --primary-color={&user.theme_color}>

                    <div class="profile-header">
                        <div class="cover-image"></div>
                        <div class="profile-info">
                            <img src="/static/images/default-avatar.png" alt={&user.full_name} class="avatar" />

                            <div class="user-details">
                                <div class="name-section">
                                    <h1>{&user.full_name}</h1>
                                    <div class="handle">{&user.handle}</div>
                                </div>
                                <button class="save-btn" hx-post="/api/follow" hx-swap="outerHTML">
                                    "Follow"
                                </button>
                            </div>

                            <p class="bio">{&user.bio}</p>

                            <div class="stats-grid">
                                @stat_card(label="Followers", value=user.stats.followers)
                                @stat_card(label="Following", value=user.stats.following)
                                @stat_card(label="Posts", value=user.stats.posts)
                            </div>
                        </div>
                    </div>

                    // 6. Type-Safe Form Binding
                    <div class="edit-section">
                        <h3>"Edit Profile"</h3>
                        <form bind={UserProfile} hx-post="/api/profile/update" hx-swap="none">
                            <div class="form-grid">
                                <div class="form-group">
                                    <label for="full_name">"Full Name"</label>
                                    <input type="text" id="full_name" name="full_name" value={&user.full_name} />
                                </div>

                                <div class="form-group">
                                    <label for="location">"Location"</label>
                                    <input type="text" id="location" name="location" value={&user.location} />
                                </div>

                                <div class="form-group">
                                    <label for="website">"Website"</label>
                                    <input type="url" id="website" name="website" value={&user.website} />
                                </div>
                            </div>

                            <div class="form-group">
                                <label for="bio">"Bio"</label>
                                <textarea id="bio" name="bio" rows="3">{&user.bio}</textarea>
                            </div>

                            <button type="submit" class="save-btn">"Save Changes"</button>
                        </form>
                    </div>

                </div>
            </body>
        </html>
    }
}

pub async fn lesson34_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @social_profile() }))
}
