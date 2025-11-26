use axum::response::{Html, IntoResponse};
use azumi::html;

/// Homepage showcasing all Azumi examples
pub fn homepage() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Azumi - Type-Safe HTML Templates"</title>
                // <link rel="stylesheet" href="/static/homepage.css" />
                <script src="https://cdn.tailwindcss.com" />
            </head>
            <body>
                <div class="container">
                    <header class="hero">
                        <h1>"⚡ Azumi"</h1>
                        <p class="tagline">"Type-safe HTML templates for Rust + Axum"</p>
                        <div class="badges">
                            <span class="badge">"Zero-cost"</span>
                            <span class="badge">"Compile-time checked"</span>
                            <span class="badge">"External CSS/JS"</span>
                        </div>
                    </header>

                    <section class="examples">
                        <h2>"Examples"</h2>
                        @test_card()
                        @test_card2()
                        <div class="grid">
                            @example_card(
                                "Hello World",
                                "Basic quoting and external files",
                                "/hello",
                                "🌍"
                            )
                            @example_card(
                                "Tailwind CSS",
                                "Utility-first styling with Tailwind",
                                "/tailwind",
                                "🎨"
                            )
                            @example_card(
                                "Components",
                                "Composition patterns and control flow",
                                "/components",
                                "🧩"
                            )
                            @example_card(
                                "Layouts",
                                "Component composition patterns",
                                "/layouts",
                                "📐"
                            )
                            @example_card(
                                "Control Flow",
                                "If, For, and Match expressions",
                                "/control-flow",
                                "🔀"
                            )
                            @example_card(
                                "Forms",
                                "Input handling and structure",
                                "/forms",
                                "📝"
                            )
                            @example_card(
                                "HTMX Todo",
                                "Server-side rendering with HTMX",
                                "/htmx-todo",
                                "✅"
                            )
                            @example_card(
                                "Dashboard",
                                "Complex layout and data visualization",
                                "/dashboard",
                                "📊"
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
                        <p>"Built with Azumi • Designed for Axum"</p>
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
        <>
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>"Azumi - Type-Safe HTML Templates"</title>
            // big footgun make the styles global
            // <link rel="stylesheet" href="/static/test_card2.css" />
        </head>
        <a href={href} class="card">
            <div class="card-icon">{icon}</div>
            <h3 class="card-title">{title}</h3>
            <p class="card-description">{description}</p>
            <span class="card-link">"View Example →"</span>
        </a>
        </>
    }
}

fn test_card<'a>() -> impl azumi::Component + 'a {
    html! {
        <div class="bg-red-500 rounded-xl shadow-lg p-6 transform hover:scale-105 transition border-l-4">
            <h3 class="text-2xl font-bold text-gray-800 mb-2">"Test Card"</h3>
            <p class="text-gray-600">"This is a test card"</p>
        </div>
    }
}

fn test_card2<'a>() -> impl azumi::Component + 'a {
    html! {
        <div>
            <head>
                <meta charset="UTF-8" />
                <title>"Test Card 2"</title>
                <style src="/static/test_card2.css" />
            </head>
            <div class="card2">
                <div class="">"🎨"</div>
                <h3 class="">"Test Card 2"</h3>
                <p class="">"This is a test card 2"</p>
                <span class="">"View Example →"</span>
            </div>
        </div>
    }
}

pub async fn homepage_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&homepage()))
}
