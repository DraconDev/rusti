use axum::response::{Html, IntoResponse};
use azumi::html;

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
                        <h2>"Learning Path: From Basic to Advanced"</h2>
                        <p class="learning-intro">"Start with the basics and work your way through increasingly complex patterns."</p>
                        
                        <h3>"🚀 Getting Started"</h3>
                        <div class="grid">
                            @ExampleCard(
                                title = "Hello World",
                                description = "Basic quoting rules, mandatory strings, and external files",
                                href = "/hello",
                                icon = "🌍"
                            )
                            @ExampleCard(
                                title = "Tailwind CSS",
                                description = "Utility-first styling with Tailwind CSS classes",
                                href = "/tailwind",
                                icon = "🎨"
                            )
                        </div>

                        <h3>"🧩 Core Concepts"</h3>
                        <div class="grid">
                            @ExampleCard(
                                title = "Components",
                                description = "Component composition with props and defaults",
                                href = "/components",
                                icon = "🧩"
                            )
                            @ExampleCard(
                                title = "Control Flow",
                                description = "If/else, for loops, and match expressions",
                                href = "/control-flow",
                                icon = "🔀"
                            )
                            @ExampleCard(
                                title = "Layouts",
                                description = "Component composition and layout patterns",
                                href = "/layouts",
                                icon = "📐"
                            )
                        </div>

                        <h3>"🛠️ Interactive Features"</h3>
                        <div class="grid">
                            @ExampleCard(
                                title = "Forms",
                                description = "Input handling and form structure",
                                href = "/forms",
                                icon = "📝"
                            )
                            @ExampleCard(
                                title = "HTMX Todo",
                                description = "Server-side rendering with HTMX interactivity",
                                href = "/htmx-todo",
                                icon = "✅"
                            )
                        </div>

                        <h3>"📊 Complex Applications"</h3>
                        <div class="grid">
                            @ExampleCard(
                                title = "Dashboard",
                                description = "Complex layout with real-world data visualization",
                                href = "/dashboard",
                                icon = "📊"
                            )
                        </div>

                        <h3>"🔧 Advanced Examples" <span class="coming-soon">"Coming Soon"</span></h3>
                        <div class="grid disabled">
                            <div class="example-card disabled">
                                <div class="card-icon">"⚡"</div>
                                <h3 class="card-title">"Advanced Components"</h3>
                                <p class="card-description">"Forms, validation, modals, and nested composition"</p>
                                <span class="card-link">"In Development"</span>
                            </div>
                            <div class="example-card disabled">
                                <div class="card-icon">"🎯"</div>
                                <h3 class="card-title">"@let Patterns"</h3>
                                <p class="card-description">"Variable bindings and data transformation"</p>
                                <span class="card-link">"In Development"</span>
                            </div>
                            <div class="example-card disabled">
                                <div class="card-icon">"📊"</div>
                                <h3 class="card-title">"Data Processing"</h3>
                                <p class="card-description">"Advanced data manipulation and visualization"</p>
                                <span class="card-link">"In Development"</span>
                            </div>
                            <div class="example-card disabled">
                                <div class="card-icon">"🎭"</div>
                                <h3 class="card-title">"Advanced Patterns"</h3>
                                <p class="card-description">"Complex Rust pattern matching in templates"</p>
                                <span class="card-link">"In Development"</span>
                            </div>
                            <div class="example-card disabled">
                                <div class="card-icon">"🏪"</div>
                                <h3 class="card-title">"Real-World Apps"</h3>
                                <p class="card-description">"E-commerce, social media, and project management"</p>
                                <span class="card-link">"In Development"</span>
                            </div>
                            <div class="example-card disabled">
                                <div class="card-icon">"🧩"</div>
                                <h3 class="card-title">"UI Library"</h3>
                                <p class="card-description">"Reusable UI components and patterns"</p>
                                <span class="card-link">"In Development"</span>
                            </div>
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
fn ExampleCard(
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
fn TailwindCard() -> impl azumi::Component {
    html! {
        <div class="bg-blue-500 rounded-xl shadow-lg p-6 hover:scale-105 transition">
            <h3 class="text-2xl font-bold text-white mb-2">"Tailwind Example"</h3>
            <p class="text-blue-100">"This card uses Tailwind utility classes"</p>
        </div>
    }
}

pub async fn homepage_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&homepage()))
}
