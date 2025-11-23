use axum::response::{Html, IntoResponse};
use rusti::{rusti, Component};

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
                <!-- Responsive Navbar -->
                <nav class="bg-white border-b border-gray-200 px-4 py-3 sm:px-6 dark:bg-gray-800">
                    <div class="flex items-center justify-between">
                        <div class="flex items-center">
                            <a href="#" class="text-xl font-bold text-gray-800 dark:text-white">Rusti</a>
                        </div>
                        <div class="hidden md:flex space-x-4">
                            <a href="#" class="text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white transition">Home</a>
                            <a href="#" class="text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white transition">Docs</a>
                            <a href="#" class="text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white transition">GitHub</a>
                        </div>
                        <button class="md:hidden focus:outline-none" aria-label="Toggle menu">
                            <svg class="h-6 w-6 text-gray-800 dark:text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                        </button>
                    </div>
                </nav>
                <!-- Dark mode toggle -->
                <div class="flex justify-center mt-8">
                    <label class="inline-flex items-center cursor-pointer">
                        <input type="checkbox" class="sr-only" id="dark-toggle" />
                        <span class="mr-2 text-gray-700 dark:text-gray-200">Dark mode</span>
                        <div class="relative">
                            <div class="block bg-gray-600 w-14 h-8 rounded-full"></div>
                            <div class="dot absolute left-1 top-1 bg-white w-6 h-6 rounded-full transition"></div>
                        </div>
                    </label>
                </div>
                <!-- Modal example -->
                <div class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 hidden" id="modal">
                    <div class="bg-white rounded-lg p-6 w-96">
                        <h3 class="text-lg font-semibold mb-4">Modal Title</h3>
                        <p class="mb-4">This is a simple modal using Tailwind utilities.</p>
                        <button class="px-4 py-2 bg-indigo-600 text-white rounded hover:bg-indigo-700" onclick="document.getElementById('modal').classList.add('hidden')">Close</button>
                    </div>
                </div>
                <div class="flex justify-center mt-4">
                    <button class="px-4 py-2 bg-indigo-600 text-white rounded hover:bg-indigo-700" onclick="document.getElementById('modal').classList.remove('hidden')">Open Modal</button>
                </div>
                <!-- Responsive grid with hover and focus utilities -->
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mt-8">
                    <div class="p-6 bg-white rounded-lg shadow hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 transition">
                        <h4 class="text-xl font-medium mb-2">Item 1</h4>
                        <p class="text-gray-600">Hover and focus effects.</p>
                    </div>
                    <div class="p-6 bg-white rounded-lg shadow hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 transition">
                        <h4 class="text-xl font-medium mb-2">Item 2</h4>
                        <p class="text-gray-600">Hover and focus effects.</p>
                    </div>
                    <div class="p-6 bg-white rounded-lg shadow hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 transition">
                        <h4 class="text-xl font-medium mb-2">Item 3</h4>
                        <p class="text-gray-600">Hover and focus effects.</p>
                    </div>
                    <div class="p-6 bg-white rounded-lg shadow hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 transition">
                        <h4 class="text-xl font-medium mb-2">Item 4</h4>
                        <p class="text-gray-600">Hover and focus effects.</p>
                    </div>
                </div>
            </body>
        </html>
    )
}

pub async fn tailwind_demo_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&tailwind_demo()))
}
