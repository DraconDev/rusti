// Datastar example demo
use crate::{page_footer, page_head};
use rusti::rusti;

pub fn datastar_demo() -> impl rusti::Component {
    rusti! {
        <html lang="en">
            <head>
                @page_head("Datastar Demo")
                <script src="https://cdn.tailwindcss.com"></script>
                <!-- Modern Datastar library -->
                <script type="module" src="https://cdn.jsdelivr.net/npm/@sudodevnull/datastar"></script>
            </head>
            <body class="bg-slate-900 min-h-screen text-slate-200 flex items-center justify-center p-8">
                <div class="max-w-2xl w-full space-y-8">
                    <h1 class="text-4xl font-bold text-purple-400 text-center">Datastar Integration Demo</h1>

                    <!-- Simple counter example -->
                    <div data-store={r#"{ count: 0 }"#}
                         class="flex flex-col items-center gap-4 p-8 bg-slate-800 rounded-lg shadow-lg">
                        <h2 class="text-2xl font-semibold text-blue-400">Datastar Counter</h2>
                        <p class="text-xl">Count: <span data-text="$count" class="font-bold text-green-400"></span></p>
                        <button data-on-click="$count++"
                                class="px-6 py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-md shadow-sm transition duration-150 ease-in-out">
                            Increment
                        </button>
                        <button data-on-click="$count--"
                                class="px-6 py-3 bg-red-600 hover:bg-red-700 text-white font-medium rounded-md shadow-sm transition duration-150 ease-in-out">
                            Decrement
                        </button>
                        <button data-on-click="$count = 0"
                                class="px-4 py-2 bg-gray-600 hover:bg-gray-700 text-white font-medium rounded-md shadow-sm transition duration-150 ease-in-out">
                            Reset
                        </button>
                    </div>

                    <!-- Input binding example -->
                    <div data-store={r#"{ name: 'World' }"#}
                         class="p-8 bg-slate-800 rounded-lg shadow-lg">
                        <h2 class="text-2xl font-semibold text-blue-400 mb-4">Input Binding</h2>
                        <input type="text"
                               data-model="name"
                               placeholder="Enter your name"
                               class="w-full px-4 py-2 bg-slate-700 border border-slate-600 rounded-md text-white mb-4" />
                        <p class="text-xl">Hello, <span data-text="$name" class="font-bold text-green-400"></span>!</p>
                    </div>
                </div>

                <div class="fixed bottom-4 right-4">
                    @page_footer(2025)
                </div>
            </body>
        </html>
    }
}

pub async fn datastar_demo_handler() -> impl axum::response::IntoResponse {
    use axum::response::Html;
    Html(rusti::render_to_string(&datastar_demo()))
}
