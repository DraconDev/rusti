use axum::response::{Html, IntoResponse};
use azumi::html;

/// Tailwind CSS example - utility-first styling
pub fn tailwind_demo() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Tailwind with Azumi"</title>
                <script src="https://cdn.tailwindcss.com" />
            </head>
            <body class="bg-gradient-to-br from-purple-500 to-pink-500 min-h-screen p-8">
                <div class="max-w-4xl mx-auto">
                    <header class="bg-white rounded-2xl shadow-2xl p-8 mb-8">
                        <h1 class="text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-purple-600 to-pink-600 mb-4">
                            "🎨 Tailwind + Azumi"
                        </h1>
                        <p class="text-gray-600 text-lg">
                            "Utility-first CSS meets type-safe templates"
                        </p>
                    </header>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                        @card(
                            "⚡ Fast",
                            "No custom CSS needed",
                            "bg-blue-500"
                        )
                        @card(
                            "🎯 Precise",
                            "Exact control over every pixel",
                            "bg-green-500"
                        )
                        @card(
                            "🔧 Flexible",
                            "Compose utilities on the fly",
                            "bg-yellow-500"
                        )
                        @card(
                            "📱 Responsive",
                            "Mobile-first by default",
                            "bg-red-500"
                        )
                    </div>

                    <div class="bg-white rounded-2xl shadow-2xl p-8">
                        <h2 class="text-3xl font-bold text-gray-800 mb-6">"Example Form"</h2>
                        <form class="space-y-4">
                            <div>
                                <label class="block text-sm font-medium text-gray-700 mb-2">
                                    "Email Address"
                                </label>
                                <input
                                    type="email"
                                    class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent transition"
                                    placeholder="you@example.com"
                                />
                            </div>
                            <div>
                                <label class="block text-sm font-medium text-gray-700 mb-2">
                                    "Message"
                                </label>
                                <textarea
                                    class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent transition"
                                    rows="4"
                                    placeholder="Your message here..."
                                />
                            </div>
                            <button
                                type="submit"
                                class="w-full bg-gradient-to-r from-purple-600 to-pink-600 text-white font-bold py-3 px-6 rounded-lg hover:shadow-lg transform hover:-translate-y-0.5 transition"
                            >
                                "Send Message"
                            </button>
                        </form>
                    </div>

                    <footer class="text-center mt-8 text-white">
                        <p class="text-sm opacity-90">"Built with Azumi + Tailwind CSS"</p>
                    </footer>
                </div>
            </body>
        </html>
    }
}

fn card<'a>(title: &'a str, description: &'a str, color: &'a str) -> impl azumi::Component + 'a {
    html! {
        <div class={format!("bg-white rounded-xl shadow-lg p-6 transform hover:scale-105 transition border-l-4 {}", color)}>
            <h3 class="text-2xl font-bold text-gray-800 mb-2">{title}</h3>
            <p class="text-gray-600">{description}</p>
        </div>
    }
}

pub async fn tailwind_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&tailwind_demo()))
}
