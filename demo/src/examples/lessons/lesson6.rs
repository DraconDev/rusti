use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 6: HTMX Integration
pub fn lesson6() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 6: HTMX Integration - Azumi"</title>
                <style src="lessons.css" />
                <style src="lesson6.css" />
                <script src="https://unpkg.com/htmx.org@1.9.10"></script>
            </head>
            <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-number">"Lesson 6"</div>
                        <h1 class="lesson-title">"HTMX Integration"</h1>
                        <p class="lesson-subtitle">"Build interactive apps with server-side rendering"</p>
                    </header>

                    <section class="section">
                        <h2 class="section-title">"🎯 What is HTMX?"</h2>
                        <p>"HTMX lets you add interactivity to server-rendered pages without JavaScript frameworks. Azumi + HTMX = <strong>powerful full-stack applications</strong> with zero client-side JavaScript!"</p>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"💻 Basic HTMX Request"</h2>
                        <p>"Send AJAX requests with HTML attributes:"</p>
                        <pre class="code-block">"html! {\n    <button\n        hx-post=\"/clicked\"\n        hx-swap=\"outerHTML\"\n        class=\"btn\">\n        \"Click Me\"\n    </button>\n}"</pre>
                        
                        <div class="highlight-box">
                            <p><strong>"Common HTMX attributes:"</strong></p>
                            <ul>
                                <li>"<code>hx-post</code>, <code>hx-get</code>, <code>hx-put</code>, <code>hx-delete</code> - HTTP method"</li>
                                <li>"<code>hx-url</code> - URL to request"</li>
                                <li>"<code>hx-swap</code> - How to update the DOM"</li>
                                <li>"<code>hx-target</code> - Which element to update"</li>
                                <li>"<code>hx-trigger</code> - What event triggers the request"</li>
                            </ul>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎮 Live Demo: Interactive Button"</h2>
                        <p>"Click this button to see HTMX in action:"</p>
                        
                        <div class="htmx-demo">
                            <button 
                                hx-post="/api/click"
                                hx-swap="outerHTML"
                                class="demo-btn"
                                id="demo-button">
                                "Click me!"
                            </button>
                            <div class="demo-status" id="demo-status">
                                "Ready to click"
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🔄 HTMX Swap Options"</h2>
                        <p>"Different ways to update the page:"</p>
                        
                        <div class="swap-examples">
                            <div class="swap-demo">
                                <h4>"innerHTML"</h4>
                                <button 
                                    hx-post="/api/innerhtml"
                                    hx-swap="innerHTML"
                                    hx-target=".swap-result-1"
                                    class="demo-btn small">
                                    "Update Content"
                                </button>
                                <div class="swap-result-1">
                                    "This will be replaced"
                                </div>
                            </div>

                            <div class="swap-demo">
                                <h4>"beforeend"</h4>
                                <button 
                                    hx-post="/api/append"
                                    hx-swap="beforeend"
                                    hx-target=".swap-result-2"
                                    class="demo-btn small">
                                    "Add Item"
                                </button>
                                <div class="swap-result-2">
                                    "Item 1"
                                </div>
                            </div>

                            <div class="swap-demo">
                                <h4>"outerHTML"</h4>
                                <button 
                                    hx-post="/api/replace"
                                    hx-swap="outerHTML"
                                    class="demo-btn small">
                                    "Replace Me"
                                </button>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"✨ Advanced HTMX"</h2>
                        <p>"HTMX with custom headers and validation:"</p>
                        <pre class="code-block">"html! {\n    <form hx-post=\"/api/submit\" hx-swap=\"none\">\n        <input type=\"email\" name=\"email\" required />\n        <button type=\"submit\">\"Submit\"</button>\n    </form>\n}"</pre>
                        
                        <div class="highlight-box">
                            <p><strong>"HTMX benefits with Azumi:"</strong></p>
                            <ul>
                                <li>"✅ Server-side rendering for SEO and performance"</li>
                                <li>"✅ Progressive enhancement - works without JS"</li>
                                <li>"✅ Type-safe HTML generation with Azumi"</li>
                                <li>"✅ No complex client-side state management"</li>
                                <li>"✅ Smaller JavaScript bundle"</li>
                            </ul>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎨 Styling HTMX"</h2>
                        <p>"HTMX automatically adds <code>htmx-request</code> class during requests:"</p>
                        <pre class="code-block">".demo-btn {\n    background: var(--azumi-primary);\n    color: white;\n    padding: 0.75rem 1.5rem;\n    border: none;\n    border-radius: var(--radius-md);\n    cursor: pointer;\n    transition: all 0.2s;\n}\n\n.demo-btn.htmx-request {\n    opacity: 0.6;\n    cursor: wait;\n}"</pre>
                    </section>

                    <nav class="nav-buttons">
                        <a href="/lesson-5" class="btn btn-secondary">"← Previous: Components"</a>
                        <a href="/lesson-7" class="btn">"Next: Layouts →"</a>
                    </nav>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson6_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson6()))
}