use axum::response::{Html, IntoResponse};
use rusti::rusti;

/// Demonstrates handling of quotes, raw strings, and edge cases
pub fn quote_demo() -> impl rusti::Component {
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Quotes & Special Cases Demo</title>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="bg-gray-900 text-gray-100 p-8">
                <div class="max-w-4xl mx-auto space-y-8">
                    <h1 class="text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-pink-500 to-violet-500">
                        Quotes & Special Cases
                    </h1>
                    
                    <!-- Example 1: Attributes always use double quotes -->
                    <section class="bg-gray-800 p-6 rounded-lg">
                        <h2 class="text-2xl font-bold mb-4">1. Attributes Use Double Quotes</h2>
                        <p class="text-gray-300">All HTML attributes must use double quotes in Rusti.</p>
                        <code class="block bg-gray-700 p-4 mt-2 rounded">class="container"</code>
                    </section>
                    
                    <!-- Example 2: JSON in attributes needs raw strings -->
                    <section class="bg-gray-800 p-6 rounded-lg">
                        <h2 class="text-2xl font-bold mb-4">2. JSON in Attributes</h2>
                        <p class="text-gray-300 mb-2">For HTMX, Alpine.js, or Datastar:</p>
                        <div data-config={r#"{"theme": "dark", "timeout": 1000}"#} 
                             class="bg-gray-700 p-4 rounded">
                            This div has JSON in data-config (check the source!)
                        </div>
                    </section>
                    
                    <!-- Example 3: Inline scripts with raw strings -->
                    <section class="bg-gray-800 p-6 rounded-lg">
                        <h2 class="text-2xl font-bold mb-4">3. Inline JavaScript</h2>
                        <p class="text-gray-300">Always use raw strings for script content:</p>
                        <button 
                            onclick={r#"alert('Single quotes work in raw strings!')"#}
                            class="bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded mt-2">
                            Click Me
                        </button>
                    </section>
                    
                    <!-- Example 4: CSS units -->
                    <section class="bg-gray-800 p-6 rounded-lg">
                        <h2 class="text-2xl font-bold mb-4">4. CSS Units</h2>
                        <style>{r#"
                            .demo-box {
                                margin: 2em;        /* Raw string = no quotes needed */
                                padding: 1rem;
                                background: #3b82f6;
                                color: white;
                            }
                        "#}</style>
                        <div class="demo-box">
                            This uses 2em margin (via raw string in style tag)
                        </div>
                    </section>
                </div>
            </body>
        </html>
    }
}

pub async fn quote_demo_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&quote_demo()))
}
