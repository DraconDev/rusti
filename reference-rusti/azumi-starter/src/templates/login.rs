use rusti::{component, rusti};

#[component]
pub fn login_page(auth_service_url: String) -> impl rusti::Component {
    rusti! {
        <div class="max-w-md mx-auto mt-16">
            <div class="glass-card rounded-2xl shadow-2xl p-8">
                <h2 class="text-3xl font-bold text-center mb-6 bg-gradient-to-r from-cyan-400 to-purple-500 bg-clip-text text-transparent">
                    "Welcome Back"
                </h2>
                <p class="text-gray-300 text-center mb-8">
                    "Sign in with your preferred provider"
                </p>

                <div class="space-y-4">
                    @oauth_button(&auth_service_url, "google", "🌐 Sign in with Google", "from-blue-500 to-blue-600")
                    @oauth_button(&auth_service_url, "github", "💻 Sign in with GitHub", "from-gray-700 to-gray-800")
                    @oauth_button(&auth_service_url, "discord", "💬 Sign in with Discord", "from-indigo-500 to-indigo-600")
                    @oauth_button(&auth_service_url, "microsoft", "🔷 Sign in with Microsoft", "from-blue-600 to-blue-700")
                </div>

                <div class="mt-8 text-center text-sm text-gray-400">
                    <p>"By signing in, you agree to our Terms of Service and Privacy Policy"</p>
                </div>
            </div>

            <div class="mt-8 text-center">
                <a href="/" class="text-cyan-400 hover:text-cyan-300 transition-colors">
                    "← Back to Home"
                </a>
            </div>
        </div>

        <script>
            "const code = new URLSearchParams(window.location.search).get('code'); if (code) fetch('/api/auth/exchange-code', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ auth_code: code }) }).then(() => window.location.href = '/profile').catch(() => window.location.href = '/login');"
        </script>
    }
}

fn oauth_button<'a>(
    auth_service_url: &'a str,
    provider: &'a str,
    label: &'a str,
    gradient: &'a str,
) -> impl rusti::Component + 'a {
    let callback_url = format!("{}/auth/{}", auth_service_url, provider);
    let btn_class = match gradient {
        "from-blue-500 to-blue-600" => "bg-gradient-to-r from-blue-500 to-blue-600 hover:opacity-90 text-white font-semibold py-3 px-6 rounded-lg transition-all duration-300 transform hover:scale-105 w-full text-center block",
        "from-gray-700 to-gray-800" => "bg-gradient-to-r from-gray-700 to-gray-800 hover:opacity-90 text-white font-semibold py-3 px-6 rounded-lg transition-all duration-300 transform hover:scale-105 w-full text-center block",
        "from-indigo-500 to-indigo-600" => "bg-gradient-to-r from-indigo-500 to-indigo-600 hover:opacity-90 text-white font-semibold py-3 px-6 rounded-lg transition-all duration-300 transform hover:scale-105 w-full text-center block",
        "from-blue-600 to-blue-700" => "bg-gradient-to-r from-blue-600 to-blue-700 hover:opacity-90 text-white font-semibold py-3 px-6 rounded-lg transition-all duration-300 transform hover:scale-105 w-full text-center block",
        _ => "bg-gradient-to-r from-gray-500 to-gray-600 hover:opacity-90 text-white font-semibold py-3 px-6 rounded-lg transition-all duration-300 transform hover:scale-105 w-full text-center block",
    };

    rusti! {
        <a href={&callback_url} class={btn_class}>
            {label}
        </a>
    }
}

pub fn auth_callback() -> impl rusti::Component {
    rusti! {
        <div class="flex items-center justify-center min-h-screen">
            <div class="text-center">
                <div class="inline-block animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-cyan-500 mb-4"></div>
                <p class="text-xl text-gray-300">"Completing login..."</p>
            </div>
        </div>
    }
}
