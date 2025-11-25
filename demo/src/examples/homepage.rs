use axum::response::{Html, IntoResponse};
use azumi::html;

/// Homepage showcasing all Azumi 2.0 examples
pub fn homepage() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Azumi 2.0 - Type-Safe HTML Templates"</title>
                <style src="demo/static/homepage.css" />
                // <script>
                //     console.log("Homepage loaded");
                // </script>
                // <style>
                //        body {
                //             background-color: #f0f0f0;
                //         }
                // </style>
            </head>
            <body>
                <div class="container">
                    <header class="hero">
                        <h1>"⚡ Azumi 2.0"</h1>
                        <p class="tagline">"Type-safe HTML templates for Rust + Axum"</p>
                        <div class="badges">
                            <span class="badge">"Zero-cost"</span>
                            <span class="badge">"Compile-time checked"</span>
                            <span class="badge">"External CSS/JS"</span>
                        </div>
                    </header>

                    <section class="examples">
                        <h2>"Examples"</h2>
                        <div class="grid">
                            @example_card(
                                "Hello World",
                                "Basic quoting and external files",
                                "/hello",
                                "🌍"
                            )
                            @example_card(
                                "Components",
                                "Composition patterns and control flow",
                                "/components",
                                "🧩"
                            )
                            @example_card(
                                "HTMX Todo",
                                "Server-side rendering with HTMX",
                                "/htmx-todo",
                                "✅"
                            )
                            @example_card(
                                "Tailwind CSS",
                                "Utility-first styling with Tailwind",
                                "/tailwind",
                                "🎨"
                            )
                        </div>
                    </section>

                    <section class="features">
                        <h2>"Why Azumi?"</h2>
                        <div class="feature-grid">
                            <div class="feature">
                                <h3>"🔒 Rigorous"</h3>
                                <p>"Mandatory quoting prevents lexer edge cases"</p>
                            </div>
                            <div class="feature">
                                <h3>"🛠️ Full Tooling"</h3>
                                <p>"External CSS/JS get IDE support"</p>
                            </div>
                            <div class="feature">
                                <h3>"⚡ Zero-Cost"</h3>
                                <p>"Compile-time templates, no runtime overhead"</p>
                            </div>
                            <div class="feature">
                                <h3>"🎯 Type-Safe"</h3>
                                <p>"Catch errors at compile time"</p>
                            </div>
                        </div>
                    </section>

                    <footer class="footer">
                        <p>"Built with Azumi 2.0 • Designed for Axum"</p>
                    </footer>
                </div>
            </body>
        </html>
    }
}

fn example_card<'a>(
    title: &'a str,
    description: &'a str,
    href: &'a str,
    icon: &'a str,
) -> impl azumi::Component + 'a {
    html! {
        <a href={href} class="card">
            <div class="card-icon">{icon}</div>
            <h3 class="card-title">{title}</h3>
            <p class="card-description">{description}</p>
            <span class="card-link">"View Example →"</span>
        </a>
    }
}

pub async fn homepage_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&homepage()))
}
