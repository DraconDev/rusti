use rusti::rusti;

pub fn script_support_demo() -> impl rusti::Component {
    rusti! {
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <title>Script Support Demo</title>
            <script src="https://cdn.tailwindcss.com"></script>
        </head>
        <body class="bg-slate-900 min-h-screen text-slate-200 flex items-center justify-center">
            <div class="text-center">
                <h1 class="text-4xl font-bold mb-4">Script Support Demo</h1>
                <button class="px-6 py-3 bg-blue-600 text-white rounded hover:bg-blue-700" onclick="showAlert()">Click me</button>
            </div>
            <script>
                function showAlert() {
                    alert('Hello from Rusti script support!');
                }
            </script>
        </body>
        </html>
    }
}

// Async handler for the route
async fn script_support_demo_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(rusti::render_to_string(&script_support_demo()))
}
