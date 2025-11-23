// Datastar example demo
use crate::{page_footer, page_head};
use rusti::rusti;

pub fn datastar_demo() -> impl rusti::Component {
    rusti! {
        <html lang="en">
            <script src="https://cdn.tailwindcss.com"></script>
            <script src="https://unpkg.com/htmx.org@1.9.10"></script>
            <script src="https://unpkg.com/datastar@1.0.0"></script>
            @page_head("Datastar Demo")
            <body class="bg-slate-900 min-h-screen text-slate-200 flex items-center justify-center">
                <h1 class="text-4xl font-bold text-purple-400">Datastar Integration Demo</h1>
                
            </body>
        </html>
    }
}

pub async fn datastar_demo_handler() -> impl axum::response::IntoResponse {
    use axum::response::Html;
    Html(rusti::render_to_string(&datastar_demo()))
}
