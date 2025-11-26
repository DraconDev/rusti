use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 7: Layouts & Composition
pub fn lesson7() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 7: Layouts & Composition - Azumi"</title>
                <style src="/static/lessons.css" />
                <style src="/static/lesson7.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-number">"Lesson 7"</div>
                        <h1 class="lesson-title">"Layouts & Composition"</h1>
                        <p class="lesson-subtitle">"Build reusable layouts with function composition"</p>
                    </header>

                    <section class="section">
                        <h2 class="section-title">"🎯 What You'll Learn"</h2>
                        <p>"Azumi supports composition through function composition. Create layouts by passing components as arguments to other components. No complex template inheritance needed!"</p>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"💻 Basic Layout Function"</h2>
                        <p>"Create a layout function that takes content as a parameter:"</p>
                        <pre class="code-block">"fn main_layout(title: &str, content: impl azumi::Component) -> impl azumi::Component {\n    html! {\n        <html>\n            <head>\n                <title>{title}</title>\n                <style src=\"global.css\" />\n                <style src=\"main_layout.css\" />\n            </head>\n            <body>\n                <nav class=\"navbar\">\n                    <a href=\"/\">\"Home\"</a>\n                    <a href=\"/about\">\"About\"</a>\n                </nav>\n                <main class=\"main-content\">\n                    {content}\n                </main>\n            </body>\n        </html>\n    }\n}"</pre>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🏠 Live Layout Example"</h2>
                        <p>"This page uses a layout! Notice the consistent navigation and structure:"</p>

                        <div class="layout-demo">
                            <div class="layout-nav">
                                <span class="layout-logo">"Azumi Layout"</span>
                                <div class="layout-nav-links">
                                    <a href="#">"Home"</a>
                                    <a href="#">"About"</a>
                                    <a href="#">"Contact"</a>
                                </div>
                            </div>
                            <div class="layout-content">
                                <h3>"Layout Content Area"</h3>
                                <p>"This is the main content that changes between pages."</p>
                                <p>"The navigation and structure stay consistent!"</p>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🔧 Using Layouts"</h2>
                        <p>"Use your layout function with any content:"</p>
                        <pre class="code-block">"fn home_page() -> impl azumi::Component {\n    main_layout(\"Home\", html! {\n        <div class=\"hero\">\n            <h1>\"Welcome to Azumi\"</h1>\n            <p>\"Building the future of web development\"</p>\n        </div>\n    })\n}\n\nfn about_page() -> impl azumi::Component {\n    main_layout(\"About\", html! {\n        <div class=\"about-content\">\n            <h1>\"About Azumi\"</h1>\n            <p>\"Type-safe HTML templates for Rust\"</p>\n        </div>\n    })\n}"</pre>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎨 Nested Layouts"</h2>
                        <p>"Compose layouts for complex applications:"</p>
                        <pre class="code-block">"fn admin_layout(content: impl azumi::Component) -> impl azumi::Component {\n    main_layout(\"Admin\", html! {\n        <div class=\"admin-container\">\n            <aside class=\"sidebar\">\n                <nav>\n                    <a href=\"/admin/users\">\"Users\"</a>\n                    <a href=\"/admin/settings\">\"Settings\"</a>\n                </nav>\n            </aside>\n            <div class=\"admin-content\">\n                {content}\n            </div>\n        </div>\n    })\n}"</pre>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"✨ Layout Composition Patterns"</h2>

                        <div class="pattern-examples">
                            <div class="pattern-demo">
                                <h4>"Wrapper Pattern"</h4>
                                <pre class="code-block small">"fn with_sidebar(\n    sidebar: impl azumi::Component,\n    content: impl azumi::Component,\n) -> impl azumi::Component"</pre>
                            </div>

                            <div class="pattern-demo">
                                <h4>"Decorator Pattern"</h4>
                                <pre class="code-block small">"fn with_breadcrumbs(\n    breadcrumbs: Vec<&str>,\n    content: impl azumi::Component,\n) -> impl azumi::Component"</pre>
                            </div>

                            <div class="pattern-demo">
                                <h4>"Provider Pattern"</h4>
                                <pre class="code-block small">"fn with_theme(\n    theme: &Theme,\n    content: impl azumi::Component,\n) -> impl azumi::Component"</pre>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🚀 Advanced Composition"</h2>
                        <p>"Combine multiple layout functions:"</p>
                        <pre class="code-block">"fn page_with_theme(\n    theme: &Theme,\n    breadcrumbs: Vec<&str>,\n    title: &str,\n    content: impl azumi::Component,\n) -> impl azumi::Component {\n    with_theme(theme, \n        with_breadcrumbs(breadcrumbs,\n            with_page_title(title, content)\n        )\n    )\n}"</pre>

                        <div class="highlight-box">
                            <p><strong>"Layout benefits:"</strong></p>
                            <ul>
                                <li>"✅ Reusable structure across pages"</li>
                                <li>"✅ Type-safe composition"</li>
                                <li>"✅ No template inheritance complexity"</li>
                                <li>"✅ Easy to test and maintain"</li>
                                <li>"✅ Flexible - combine layouts however you want"</li>
                            </ul>
                        </div>
                    </section>

                    <nav class="nav-buttons">
                        <a href="/lesson-6" class="btn btn-secondary">"← Previous: HTMX Integration"</a>
                        <a href="/lesson-8" class="btn">"Next: Real-World Examples →"</a>
                    </nav>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson7_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson7()))
}
