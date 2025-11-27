use azumi::html;

/// Lesson 1: Your First Azumi Template
/// Learn the basics of the html! macro and component creation

/// Example 1: The Absolute Basics
pub fn hello_world() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson1.css" />
        <div class="lesson1-container">
            <h1 class="lesson1-title">"Hello, Azumi!"</h1>
            <p class="lesson1-text">"This is your first template using the html! macro."</p>
        </div>
    }
}

/// Example 2: Templates are Functions
pub fn greeting(name: &str) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson1.css" />
        <div class="lesson1-box">
            <h2 class="lesson1-subtitle">"Dynamic Content"</h2>
            <p class="lesson1-text">"Hello, " {name} "!"</p>
            <p class="lesson1-small">"Templates are regular Rust functions that accept parameters."</p>
        </div>
    }
}

/// Example 3: Multiple Elements (Fragments)
pub fn card_example() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson1.css" />
        <div class="lesson1-card">
            <h3 class="lesson1-card-title">"Card Title"</h3>
            <p class="lesson1-card-text">"This card is built with Azumi."</p>
            <div class="lesson1-card-footer">
                <span class="lesson1-badge">"Azumi"</span>
                <span class="lesson1-badge">"Rust"</span>
            </div>
        </div>
    }
}

/// Example 4: Integration with Axum
pub fn integration_example() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson1.css" />
        <div class="lesson1-code-example">
            <h3 class="lesson1-subtitle">"Using with Axum"</h3>
            <pre class="lesson1-code">
"// In your handler:
pub async fn my_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&hello_world()))
}"
            </pre>
            <p class="lesson1-small">"Azumi components integrate seamlessly with Axum handlers."</p>
        </div>
    }
}

/// Main lesson component
pub fn lesson1() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson1.css" />
        <div class="lesson1-container">
            <header class="lesson1-header">
                <h1 class="lesson1-main-title">"Lesson 1: Your First Azumi Template"</h1>
                <p class="lesson1-subtitle">"Learn the basics of the html! macro"</p>
            </header>

            <section class="lesson1-section">
                <h2 class="lesson1-section-title">"What is Azumi?"</h2>
                <div class="lesson1-info-box">
                    <p class="lesson1-text">
                        "Azumi is a compile-time HTML template system for Rust. "
                        "It uses a macro called " <code>"html!"</code> " to let you write HTML-like syntax "
                        "that gets validated and compiled into pure Rust code."
                    </p>
                </div>
            </section>

            <section class="lesson1-section">
                <h2 class="lesson1-section-title">"The html! Macro"</h2>
                <div class="lesson1-code-demo">
                    <h3 class="lesson1-demo-title">"Basic Syntax"</h3>
                    <pre class="lesson1-code">
"html! {
    <div class=\\\"container\\\">
        <h1>\\\"Hello, World!\\\"</h1>
    </div>
}"
                    </pre>
                    <div class="lesson1-key-points">
                        <h4>"Key Points:"</h4>
                        <ul class="lesson1-list">
                            <li>"All text must be quoted: " <code>"\"like this\""</code></li>
                            <li>"All attribute values must be quoted: " <code>"class=\"container\""</code></li>
                            <li>"Use curly braces for Rust expressions: " <code>"{variable}"</code></li>
                        </ul>
                    </div>
                </div>
            </section>

            <section class="lesson1-section">
                <h2 class="lesson1-section-title">"Live Examples"</h2>

                <div class="lesson1-example">
                    <h3 class="lesson1-example-title">"Example 1: Hello World"</h3>
                    @hello_world()
                </div>

                <div class="lesson1-example">
                    <h3 class="lesson1-example-title">"Example 2: Dynamic Content"</h3>
                    @greeting("Alice")
                </div>

                <div class="lesson1-example">
                    <h3 class="lesson1-example-title">"Example 3: Card Component"</h3>
                    @card_example()
                </div>

                <div class="lesson1-example">
                    <h3 class="lesson1-example-title">"Example 4: Axum Integration"</h3>
                    @integration_example()
                </div>
            </section>

            <section class="lesson1-section">
                <h2 class="lesson1-section-title">"Try This!"</h2>
                <div class="lesson1-challenge">
                    <h3>"Challenge"</h3>
                    <p class="lesson1-text">
                        "Create a function that takes a " <code>"title"</code> " and " <code>"description"</code>
                        " as parameters and returns a card with both values displayed."
                    </p>
                    <details class="lesson1-hint">
                        <summary>"Hint"</summary>
                        <pre class="lesson1-code">
"fn my_card(title: &str, desc: &str) -> impl Component {
    html! {
        <div class=\\\"card\\\">
            <h2>{title}</h2>
            <p>{desc}</p>
        </div>
    }
}"
                        </pre>
                    </details>
                </div>
            </section>

            <footer class="lesson1-footer">
                <a href="/lesson-0" class="lesson1-nav-link prev">"← Lesson 0: Fragments"</a>
                <a href="/lesson-2" class="lesson1-nav-link next">"Lesson 2: The Quoting Rule →"</a>
            </footer>
        </div>
    }
}

// Axum handler
pub async fn lesson1_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson1()))
}
