// Demo for base_layout and navbar using rusti macro

use axum::response::{Html, IntoResponse};
use rusti::rusti;

/// Base layout component that wraps provided content with a full HTML page.
pub fn base_layout<'a>(
    title: &'a str,
    is_authenticated: bool,
    content: impl rusti::Component + 'a,
) -> impl rusti::Component + 'a {
    // Using format! for parts the parser struggles with
    let head_html = format!(
        r#"<html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>{}</title><script src="https://unpkg.com/htmx.org@1.9.10"></script><script src="https://cdn.tailwindcss.com"></script><style>body{{background:linear-gradient(135deg,#0f172a 0%,#1e293b 100%)}}.glass-card{{background:rgba(30,41,59,0.7);backdrop-filter:blur(10px);border:1px solid rgba(255,255,255,0.1)}}.glow-effect{{box-shadow:0 0 20px rgba(6,182,212,0.3)}}</style></head><body class="bg-gray-900 text-white min-h-screen">"#,
        title
    );

    rusti! {
        {head_html}
        @navbar(is_authenticated)
        <div class="container mx-auto px-4 py-8">
            @content
        </div>
        "</body></html>"
    }
}

/// Navbar component used by the base layout.
fn navbar(is_authenticated: bool) -> impl rusti::Component {
    rusti! {
        <nav class="glass-card rounded-2xl shadow-2xl p-4 mb-8">
            <div class="flex justify-between items-center">
                <div class="flex items-center space-x-6">
                    <a href="/" class="text-2xl font-bold bg-gradient-to-r from-cyan-400 to-purple-500 bg-clip-text text-transparent">
                        "Azumi"
                    </a>
                    <a href="/" class="text-gray-300 hover:text-white transition-colors">
                        "Home"
                    </a>
                    @if is_authenticated {
                        <a href="/profile" class="text-gray-300 hover:text-white transition-colors">
                            "Profile"
                        </a>
                        <a href="/settings" class="text-gray-300 hover:text-white transition-colors">
                            "Settings"
                        </a>
                    }
                </div>
                <div>
                    @if is_authenticated {
                        <form action="/api/auth/logout" method="post" class="inline">
                            <button type="submit" class="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded-lg transition-colors">
                                "Logout"
                            </button>
                        </form>
                    } @else {
                        <a href="/login" class="bg-gradient-to-r from-cyan-500 to-purple-600 hover:from-cyan-400 hover:to-purple-500 text-white px-6 py-2 rounded-lg transition-all">
                            "Login"
                        </a>
                    }
                </div>
            </div>
        </nav>
    }
}

/// Wrapper function to expose a demo page using the base layout.
pub fn base_layout_demo() -> impl rusti::Component {
    let title = "Base Layout Demo";
    let is_auth = false;
    let content = rusti! {
        <h1 class="text-4xl font-bold text-center text-white">"Welcome to the Base Layout Demo"</h1>
        <p class="mt-4 text-center text-gray-300">"This page demonstrates the base_layout component with navbar."</p>
    };
    base_layout(title, is_auth, content)
}

/// Axum handler for the `/base-layout` route.
pub async fn base_layout_demo_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&base_layout_demo()))
}
