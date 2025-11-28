//! Lesson 18: CSS Variables & Dynamic Theming
//!
//! Using CSS custom properties with Rust values for dynamic styling

use azumi::html;

#[azumi::component]
pub fn css_variables_demo() -> impl azumi::Component {
    // Theme configuration
    let primary_color = "#3498db";
    let secondary_color = "#2ecc71";
    let danger_color = "#e74c3c";
    let success_color = "#27ae60";
    
    // Dynamic values
    let completion_percent = 75;
    let loading_duration = 2;
    let card_elevation = 4;
    
    html! {
        <style src="/static/pages/lesson18.css" />
        <div class="lesson18-container">
            <header class="lesson18-header">
                <h1 class="lesson18-title">"Lesson 18: CSS Variables & Dynamic Theming"</h1>
                <p class="lesson18-subtitle">
                    "Passing Rust values directly to CSS custom properties"
                </p>
            </header>

            <section class="lesson18-section">
                <h2 class="section-title">"Color Theming"</h2>
                <p class="section-text">
                    "Define your theme colors in Rust and pass them to CSS variables:"
                </p>

                <div 
                    class="theme-demo"
                    --primary={primary_color}
                    --secondary={secondary_color}
                    --danger={danger_color}
                    --success={success_color}
                >
                    <div class="color-box primary">"Primary"</div>
                    <div class="color-box secondary">"Secondary"</div>
                    <div class="color-box danger">"Danger"</div>
                    <div class="color-box success">"Success"</div>
                </div>

                <div class="code-example">
                    <code>
                        "--primary=" {primary_color} "\n"
                        "--secondary=" {secondary_color}
                    </code>
                </div>
            </section>

            <section class="lesson18-section">
                <h2 class="section-title">"Progress Indicators"</h2>
                <p class="section-text">
                    "Use variables for dynamic progress bars and loading states:"
                </p>

                <div class="progress-demo" --completion={completion_percent}>
                    <div class="progress-bar">
                        <div class="progress-fill"></div>
                        <span class="progress-label">{completion_percent} "%"</span>
                    </div>
                </div>

                <div class="loading-demo" --duration={loading_duration}>
                    <div class="spinner"></div>
                    <p>"Loading... (" {loading_duration} "s animation)"</p>
                </div>
            </section>

            <section class="lesson18-section">
                <h2 class="section-title">"Elevation & Shadows"</h2>
                <p class="section-text">
                    "Dynamic material design elevation based on Rust state:"
                </p>

                <div class="elevation-demo">
                    @for level in [2, 4, 8, 16] {
                        <div class="card" --elevation={level}>
                            <h3>"Elevation " {level}</h3>
                            <p>"Shadow depth: " {level} "px"</p>
                        </div>
                    }
                </div>
            </section>

            <section class="lesson18-section">
                <h2 class="section-title">"How It Works"</h2>
                <div class="explanation">
                    <h3>"HTML Side (Azumi)"</h3>
                    <pre class="code-block">
"<div class=\"card\" --elevation="{card_elevation}\">\n"
"  <!-- content -->\n"
"</div>"
                    </pre>

                    <h3>"CSS Side"</h3>
                    <pre class="code-block">
".card {\n"
"  box-shadow: 0 var(--elevation) calc(var(--elevation) * 2) rgba(0,0,0,0.2);\n"
"}"
                    </pre>

                    <h3>"Key Benefits"</h3>
                    <ul class="benefits-list">
                        <li>"✅ Type-safe theme configuration"</li>
                        <li>"✅ No JavaScript required"</li>
                        <li>"✅ Server-side dynamic styling"</li>
                        <li>"✅ Full CSS custom property support"</li>
                        <li>"✅ Can be computed from database/user preferences"</li>
                    </ul>
                </div>
            </section>

            <section class="lesson18-section">
                <h2 class="section-title">"Advanced Example: User Preferences"</h2>
                <p class="section-text">
                    "Real-world example: theme from user preferences"
                </p>

                <div class="code-example">
                    <pre>
"// From database or session\n"
"let user_theme = UserTheme {\n"
"    primary: \"#ff6b6b\",\n"
"    font_size: 16,\n"
"    dark_mode: true,\n"
"};\n\n"
"html! {\n"
"    <div\n"
"        --primary={user_theme.primary}\n"
"        --font-size={user_theme.font_size}\n"
"        --bg={if user_theme.dark_mode { \"#222\" } else { \"#fff\" }}\n"
"    >\n"
"        <!-- Fully themed content -->\n"
"    </div>\n"
"}"
                    </pre>
                </div>
            </section>

            <section class="lesson18-section">
                <h2 class="section-title">"Key Takeaways"</h2>
                <ul class="takeaways-list">
                    <li>"✓ CSS variables bridge Rust and CSS"</li>
                    <li>"✓ Perfect for theming, progress bars, animations"</li>
                    <li>"✓ No runtime JavaScript needed"</li>
                    <li>"✓ Type-safe on the Rust side"</li>
                    <li>"✓ Combine with user data for personalization"</li>
                    <li>"✓ Works with responsive design and media queries"</li>
                </ul>
            </section>
        </div>
    }
}

/// Axum handler for Lesson 18
pub async fn lesson18_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @css_variables_demo() }))
}
