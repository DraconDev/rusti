use axum::response::{Html, IntoResponse};
use azumi::html;

pub async fn layouts_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&page_content()))
}

// A reusable layout component
// Note: In Azumi, 'children' is just another prop, usually of type `impl azumi::Component` or generic.
// For simplicity in this demo, we'll just render the page directly, but show how to wrap content.
// A true "Layout" pattern often involves passing a closure or a specific slot.
// Here we demonstrate "Composition" which is the React/Azumi way.

fn main_layout(title: &str, content: impl azumi::Component) -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>{title}</title>
                <style src="/static/homepage.css" />
                <style>
                    .sidebar { width: 200px; float: left; background: #eee; padding: 20px; height: 100vh; }
                    .main { margin-left: 220px; padding: 20px; }
                    .clearfix::after { content: ""; clear: both; display: table; }
                </style>
            </head>
            <body>
                <div class="clearfix">
                    <nav class="sidebar">
                        <h3>"Sidebar"</h3>
                        <ul>
                            <li><a href="/">"Home"</a></li>
                            <li><a href="/layouts">"Layouts"</a></li>
                            <li><a href="/components">"Components"</a></li>
                        </ul>
                    </nav>
                    <main class="main">
                        {content}
                    </main>
                </div>
            </body>
        </html>
    }
}

fn page_content() -> impl azumi::Component {
    // We use the layout by calling it as a function and passing our content
    main_layout(
        "Layouts Demo",
        html! {
            <h1>"Layout Composition"</h1>
            <p>"This page demonstrates how to create a reusable layout wrapper."</p>
            <p>"The sidebar on the left is part of the layout, while this content is passed in."</p>

            <div class="card">
                <h3>"Why Composition?"</h3>
                <p>"It avoids inheritance and makes data flow explicit."</p>
            </div>
        },
    )
}
