use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};

// mod examples; // Disabled for now

use rusti::rusti;
use serde::Deserialize;

#[derive(Deserialize)]
struct CounterForm {
    count: i32,
}

// Head component - contains page metadata and stylesheets
fn page_head<'a>(title: &'a str) -> impl rusti::Component + 'a {
    rusti! {
        <head>
            <title>{ title }</title>
            <script src="https://cdn.tailwindcss.com"></script>
            <script src="https://unpkg.com/htmx.org@1.9.10"></script>
        </head>
    }
}

// Header component - displays page title and subtitle
fn page_header<'a>(title: &'a str, subtitle: &'a str) -> impl rusti::Component + 'a {
    rusti! {
        <header class="text-center mb-16">
            <div class="inline-block">
                <h1 class="text-6xl font-black text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-purple-600 mb-4">{ title }</h1>
                <div class="h-1 bg-gradient-to-r from-blue-600 to-purple-600 rounded-full"></div>
            </div>
            <p class="text-xl text-gray-600 mt-6 max-w-2xl mx-auto">{ subtitle }</p>
            <div class="flex gap-4 justify-center mt-8">
                <span class="px-4 py-2 bg-blue-100 text-blue-700 rounded-full text-sm font-semibold">Type-Safe</span>
                <span class="px-4 py-2 bg-purple-100 text-purple-700 rounded-full text-sm font-semibold">Zero-Cost</span>
                <span class="px-4 py-2 bg-green-100 text-green-700 rounded-full text-sm font-semibold">XSS Protected</span>
            </div>
        </header>
    }
}

// Footer component - displays copyright and attribution
fn page_footer(year: i32) -> impl rusti::Component {
    rusti! {
        <footer class="text-center mt-16 pt-8 border-t border-gray-200">
            <p class="text-gray-500">Copyright { year } - Built with Rusti</p>
        </footer>
    }
}

// Flexible layout component that accepts any content
fn full_page_layout<'a, C: rusti::Component + 'a>(
    title: &'a str,
    subtitle: &'a str,
    content: C,
) -> impl rusti::Component + 'a {
    let year = 2025;

    rusti! {
        <html>
            @page_head(title)
            <body class="bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-7xl">
                    @page_header(title, subtitle)
                    <main>
                        @content()
                    </main>
                    @page_footer(year)
                </div>
            </body>
        </html>
    }
}

fn clickable_card<'a>(
    title: &'a str,
    description: &'a str,
    href: &'a str,
) -> impl rusti::Component + 'a {
    rusti! {
        <a href={href} class="block group bg-white rounded-2xl p-8 shadow-lg hover:shadow-2xl transition-all duration-300 border border-blue-100 hover:border-blue-300 no-underline">
            <div class="flex items-center gap-3 mb-6">
                <div class="w-12 h-12 bg-gradient-to-br from-blue-400 to-blue-600 rounded-xl flex items-center justify-center text-white text-xl font-bold shadow-lg group-hover:scale-110 transition-transform">
                    &rarr;
                </div>
                <h3 class="text-2xl font-bold text-gray-800 group-hover:text-blue-600 transition-colors">{ title }</h3>
            </div>
            <p class="text-gray-600">{ description }</p>
        </a>
    }
}

// HTMX Interactive Demo
fn htmx_page() -> impl rusti::Component {
    let content = rusti! {
        <div class="space-y-8">
            <div class="bg-white rounded-2xl p-8 shadow-lg">
                <h2 class="text-2xl font-bold text-gray-800 mb-6">Interactive Counter</h2>
                <div id="counter" class="text-center">
                    @counter_partial(0)
                </div>
            </div>
            <div class="bg-white rounded-2xl p-8 shadow-lg">
                <h2 class="text-2xl font-bold text-gray-800 mb-4">How It Works</h2>
                <ul class="space-y-2 text-gray-700">
                    <li class="flex items-start gap-2">
                        <span class="text-pink-500 font-bold">&gt;</span>
                        <span>HTMX attributes enable interactivity without JavaScript</span>
                    </li>
                    <li class="flex items-start gap-2">
                        <span class="text-pink-500 font-bold">&gt;</span>
                        <span>Rusti attribute support makes HTMX integration seamless</span>
                    </li>
                    <li class="flex items-start gap-2">
                        <span class="text-pink-500 font-bold">&gt;</span>
                        <span>Server renders partial HTML updates</span>
                    </li>
                </ul>
            </div>
        </div>
    };

    full_page_layout(
        "HTMX + Rusti Demo",
        "Interactive counter with server-side rendering",
        content,
    )
}

fn counter_partial(count: i32) -> impl rusti::Component {
    rusti! {
        <div class="text-6xl font-black text-pink-600 mb-6">{ count }</div>
        <div class="flex gap-4 justify-center">
            <form hx-post="/htmx/counter/increment" hx-target="#counter" hx-swap="innerHTML">
                <input type="hidden" name="count" value={count.to_string().as_str()}></input>
                <button type="submit" class="px-6 py-3 bg-pink-600 text-white rounded-lg font-semibold hover:bg-pink-700 transition">
                    Increment
                </button>
            </form>
            <form hx-post="/htmx/counter/decrement" hx-target="#counter" hx-swap="innerHTML">
                <input type="hidden" name="count" value={count.to_string().as_str()}></input>
                <button type="submit" class="px-6 py-3 bg-gray-600 text-white rounded-lg font-semibold hover:bg-gray-700 transition">
                    Decrement
                </button>
            </form>
            <form hx-post="/htmx/counter/reset" hx-target="#counter" hx-swap="innerHTML">
                <button type="submit" class="px-6 py-3 bg-red-600 text-white rounded-lg font-semibold hover:bg-red-700 transition">
                    Reset
                </button>
            </form>
        </div>
    }
}

async fn hello_world() -> impl IntoResponse {
    let content = rusti! {
        <section class="mb-12">
            <h2 class="text-4xl font-bold text-gray-800 mb-8 text-center">Explore Examples</h2>
            <div class="grid md:grid-cols-2 gap-8">
                @clickable_card("Basic Examples", "Simple component composition", "/")
                @clickable_card("HTMX Interactivity", "Interactive counter demo", "/htmx")
            </div>
        </section>
    };

    let component = full_page_layout(
        "Rusti Demo",
        "Welcome to the Rusti demo application!",
        content,
    );
    Html(rusti::render_to_string(&component))
}

async fn htmx_demo() -> impl IntoResponse {
    Html(rusti::render_to_string(&htmx_page()))
}

async fn htmx_increment(Form(form): Form<CounterForm>) -> impl IntoResponse {
    let new_count = form.count + 1;
    Html(rusti::render_to_string(&counter_partial(new_count)))
}

async fn htmx_decrement(Form(form): Form<CounterForm>) -> impl IntoResponse {
    let new_count = form.count - 1;
    Html(rusti::render_to_string(&counter_partial(new_count)))
}

async fn htmx_reset() -> impl IntoResponse {
    Html(rusti::render_to_string(&counter_partial(0)))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/htmx", get(htmx_demo))
        .route("/htmx/counter/increment", post(htmx_increment))
        .route("/htmx/counter/decrement", post(htmx_decrement))
        .route("/htmx/counter/reset", post(htmx_reset));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("\n🚀 Rusti Web Server");
    println!("===================");
    println!("Server running on http://127.0.0.1:3000");
    println!("\n📍 Available routes:");
    println!("   http://127.0.0.1:3000/");
    println!("   http://127.0.0.1:3000/htmx");
    println!("\n✨ Features demonstrated:");
    println!("   • Component composition");
    println!("   • HTMX interactivity");
    println!("   • XSS protection");
    println!("\n");

    axum::serve(listener, app).await.unwrap();
}
