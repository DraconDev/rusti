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
                <style src="/static/homepage.css" />
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
                        @tailwind_card()
                        @TestCard2()
                        <div class="grid">
                            @example_card(
                                title = "Hello World",
                                description = "Basic quoting and external files",
                                href = "/hello",
                                icon = "🌍"
                            )
                            @example_card(
                                title = "Tailwind CSS",
                                description = "Utility-first styling with Tailwind",
                                href = "/tailwind",
                                icon = "🎨"
                            )
                            @example_card(
                                title = "Components",
                                description = "Composition patterns and control flow",
                                href = "/components",
                                icon = "🧩"
                            )
                            @example_card(
                                title = "Layouts",
                                description = "Component composition patterns",
                                href = "/layouts",
                                icon = "📐"
                            )
                            @example_card(
                                title = "Control Flow",
                                description = "If, For, and Match expressions",
                                href = "/control-flow",
                                icon = "🔀"
                            )
                            @example_card(
                                title = "Forms",
                                description = "Input handling and structure",
                                href = "/forms",
                                icon = "📝"
                            )
                            @example_card(
                                title = "HTMX Todo",
                                description = "Server-side rendering with HTMX",
                                href = "/htmx-todo",
                                icon = "✅"
                            )
                            @example_card(
                                title = "Dashboard",
                                description = "Complex layout and data visualization",
                                href = "/dashboard",
                                icon = "📊"
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

#[azumi::component]
fn example_card(
    #[prop(default = "\"Example\"")] title: &'static str,
    #[prop(default = "\"Description\"")] description: &'static str,
    #[prop(default = "\"/\"")] href: &'static str,
    #[prop(default = "\"📦\"")] icon: &'static str,
) -> impl azumi::Component {
    html! {
        <>
            <style src="/static/example_card.css" />
            <a href={href} class="card">
                <div class="card-icon">{icon}</div>
                <h3 class="card-title">{title}</h3>
                <p class="card-description">{description}</p>
                <span class="card-link">"View Example →"</span>
            </a>
        </>
    }
}

/// Example: Tailwind CSS (inline utility classes)
#[azumi::component]
fn tailwind_card() -> impl azumi::Component {
    html! {
        <div class="bg-blue-500 rounded-xl shadow-lg p-6 hover:scale-105 transition">
            <h3 class="text-2xl font-bold text-white mb-2">"Tailwind Example"</h3>
            <p class="text-blue-100">"This card uses Tailwind utility classes"</p>
        </div>
    }
}

#[azumi::component]
fn TestCard2() -> impl azumi::Component {
    html! {
        <style src="/static/test_card2.css" />
        <div class="card2">
            <h2>"Test Card 2 (Scoped File)"</h2>
            <p>"This card uses a separate CSS file scoped to this component."</p>
            <button>"Click Me"</button>
        </div>
    }
}

pub async fn homepage_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&homepage()))
}
