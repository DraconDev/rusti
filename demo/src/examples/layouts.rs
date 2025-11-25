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

fn page_content() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Layouts Demo"</title>
                <style src="/static/homepage.css" />
                <style src="/static/layouts.css" />
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
                        <h1>"Layout Composition"</h1>
                        <p>"This page demonstrates a common layout pattern with sidebar navigation."</p>
                        <p>"In production, you'd extract this into a reusable function that takes content as a parameter."</p>

                        <div class="card">
                            <h3>"Pattern Options"</h3>
                            <ul>
                                <li>"Pass content as a static string or component reference"</li>
                                <li>"Use builder patterns for complex layouts"</li>
                                <li>"Compose multiple smaller layout components"</li>
                            </ul>
                        </div>
                    </main>
                </div>
            </body>
        </html>
    }
}
