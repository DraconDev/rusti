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
                <div data-star-scope="{ count: 0 }" class="flex flex-col items-center gap-4 p-8 bg-slate-800 rounded-lg shadow-lg">
                    <h2 class="text-2xl font-semibold text-blue-400">Datastar Counter</h2>
                    <p class="text-xl">Count: <span data-star-bind="count" class="font-bold text-green-400"></span></p>
                    <button data-star-on:click="count++" class="px-6 py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-md shadow-sm transition duration-150 ease-in-out">
                        Increment
                    </button>
                </div>
                @page_footer(2025)
            </body>
        </html>
    }
}

pub async fn datastar_demo_handler() -> impl axum::response::IntoResponse {
    use axum::response::Html;
    Html(rusti::render_to_string(&datastar_demo()))
}
