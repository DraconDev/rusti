use axum::response::{Html, IntoResponse};
use rusti::{Component, rusti};

/// Demonstrates Tailwind CSS utilities via a Rusti component.
pub fn tailwind_demo() -> impl rusti::Component {
    rusti!(
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Tailwind Demo</title>
                <!-- Tailwind CDN -->
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="bg-gray-100 p-8">
                <div class="max-w-4xl mx-auto space-y-8">
                    <h1 class="text-4xl font-extrabold text-center text-indigo-600">Tailwind CSS Demo</h1>
                    <p class="text-lg text-gray-700 text-center">
                        This page showcases a variety of Tailwind utility classes.
                    </p>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div class="p-6 bg-white rounded-lg shadow hover:shadow-lg transition-shadow">
                            <h2 class="text-2xl font-semibold mb-2 text-blue-500">Card Title</h2>
                            <p class="text-gray-600">A simple card with padding, rounded corners and a shadow.</p>
                            <button class="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors">
                                Action
                            </button>
                        </div>
                        <div class="p-6 bg-white rounded-lg shadow hover:shadow-lg transition-shadow">
                            <h2 class="text-2xl font-semibold mb-2 text-green-500">Another Card</h2>
                            <p class="text-gray-600">Demonstrating responsive grid and spacing utilities.</p>
                            <a href="#" class="inline-block mt-4 text-green-600 hover:underline">Learn more</a>
                        </div>
                    </div>
                    <div class="flex justify-center space-x-4">
                        <span class="inline-block w-8 h-8 bg-red-500 rounded-full"></span>
                        <span class="inline-block w-8 h-8 bg-yellow-500 rounded-full"></span>
                        <span class="inline-block w-8 h-8 bg-green-500 rounded-full"></span>
                    </div>
                    <form class="bg-white p-6 rounded-lg shadow-md" action="#" method="POST">
                        <label class="block mb-2 font-medium text-gray-700" for="email">Email</label>
                        <input class="w-full border border-gray-300 rounded px-3 py-2 mb-4 focus:outline-none focus:ring-2 focus:ring-indigo-500" type="email" id="email" name="email" placeholder="you@example.com" />
                        <button class="w-full bg-indigo-600 text-white py-2 rounded hover:bg-indigo-700 transition-colors" type="submit">Subscribe</button>
                    </form>
                </div>
            </body>
        </html>
    )
}

pub async fn tailwind_demo_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&tailwind_demo()))
}