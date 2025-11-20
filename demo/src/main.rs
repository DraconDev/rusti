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

// Home page with example cards
fn home_page() -> impl rusti::Component {
    let year = 2025;

    rusti! {
        <html>
            @page_head("Rusti Demo")
            <body class="bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-7xl">
                    @page_header("Rusti Demo", "Comprehensive Rust HTML templating demos")
                    <main>
                        <section class="mb-12">
                            <h2 class="text-4xl font-bold text-gray-800 mb-8 text-center">Explore Features</h2>
                            <div class="grid md:grid-cols-3 gap-6">
                                @clickable_card("Components", "Reusable UI building blocks", "/")
                                @clickable_card("HTMX", "Server-side interactivity", "/htmx")
                                @clickable_card("Conditionals", "If/else logic rendering", "/conditionals")
                                @clickable_card("Lists", "For loops and iteration", "/lists")
                                @clickable_card("Match", "Pattern matching", "/match")
                                @clickable_card("XSS Protection", "Automatic HTML escaping", "/xss")
                                @clickable_card("Attributes", "Dynamic attribute values", "/attributes")
                            </div>
                        </section>
                    </main>
                    @page_footer(year)
                </div>
            </body>
        </html>
    }
}

// Comprehensive conditionals demonstration
fn conditionals_page() -> impl rusti::Component {
    let year = 2025;
    let is_premium = true;
    let user_role = "admin";
    let score = 85;
    let stock_count = 3;

    rusti! {
        <html>
            @page_head("Conditionals Demo")
            <body class="bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-7xl">
                    @page_header("Conditionals & Logic", "Comprehensive guide to conditional rendering in Rusti")
                    <main>
                        <div class="space-y-8">
                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">1. Simple Boolean Conditional</h2>
                                <div class="space-y-4">
                                    @if is_premium {
                                        <div class="p-4 bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-lg">
                                            <p class="text-lg font-bold">Premium User</p>
                                            <p>You have access to all features!</p>
                                        </div>
                                    } else {
                                        <div class="p-4 bg-gray-200 text-gray-700 rounded-lg">
                                            <p class="text-lg font-bold">Free User</p>
                                            <p>Upgrade to premium for more features</p>
                                        </div>
                                    }
                                    <pre class="bg-gray-800 text-green-400 p-3 rounded mt-4 text-sm"><code>{"@if is_premium { ... } else { ... }"}</code></pre>
                                </div>
                            </div>

                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">2. String Comparison</h2>
                                <div class="space-y-4">
                                    @if user_role == "admin" {
                                        <div class="p-4 bg-red-100 border-l-4 border-red-500 text-red-700">
                                            <p class="font-bold">Admin Access</p>
                                            <p>Full system permissions granted</p>
                                        </div>
                                    } else {
                                        <div class="p-4 bg-blue-100 border-l-4 border-blue-500 text-blue-700">
                                            <p class="font-bold">User Access</p>
                                            <p>Limited permissions</p>
                                        </div>
                                    }
                                    <pre class="bg-gray-800 text-green-400 p-3 rounded mt-4 text-sm"><code>{"@if role == \"admin\" { ... }"}</code></pre>
                                </div>
                            </div>

                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">3. Numeric Comparison</h2>
                                <div class="flex items-center gap-4">
                                    <div class="text-6xl font-black text-gray-800">{ score }</div>
                                    @if score >= 90 {
                                        <div class="p-4 bg-green-500 text-white rounded-lg flex-1">
                                            <p class="text-2xl font-bold">Excellent!</p>
                                            <p>Outstanding performance</p>
                                        </div>
                                    } else {
                                        <div class="p-4 bg-yellow-500 text-white rounded-lg flex-1">
                                            <p class="text-2xl font-bold">Good Job!</p>
                                            <p>Keep improving</p>
                                        </div>
                                    }
                                </div>
                                <pre class="bg-gray-800 text-green-400 p-3 rounded mt-4 text-sm"><code>{"@if score >= 90 { ... } else { ... }"}</code></pre>
                            </div>

                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">4. Stock Level Indicator</h2>
                                <div class="flex items-center gap-4">
                                    <span class="text-4xl font-bold">Stock: { stock_count }</span>
                                    @if stock_count > 10 {
                                        <span class="px-4 py-2 bg-green-500 text-white rounded-full font-bold">In Stock</span>
                                    } else {
                                        @if stock_count > 0 {
                                            <span class="px-4 py-2 bg-orange-500 text-white rounded-full font-bold">Low Stock</span>
                                        } else {
                                            <span class="px-4 py-2 bg-red-500 text-white rounded-full font-bold">Out of Stock</span>
                                        }
                                    }
                                </div>
                                <pre class="bg-gray-800 text-green-400 p-3 rounded mt-4 text-sm"><code>{"@if count > 10 { ... } else { @if count > 0 { ... } }"}</code></pre>
                            </div>

                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">Syntax Reference</h2>
                                <div class="space-y-4 text-sm">
                                    <div>
                                        <h3 class="font-bold mb-2">Supported Comparisons:</h3>
                                        <ul class="list-disc list-inside space-y-1 text-gray-700">
                                            <li>Equality: ==, !=</li>
                                            <li>Numeric: &lt;, &gt;, &lt;=, &gt;=</li>
                                            <li>Boolean: true, false</li>
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </main>
                    @page_footer(year)
                </div>
            </body>
        </html>
    }
}

// Lists and iteration demo
fn lists_page() -> impl rusti::Component {
    let year = 2025;
    let fruits = vec!["🍎 Apples", "🍌 Bananas", "🍊 Oranges", "🍇 Grapes"];
    let scores = vec![95, 87, 92, 78, 88, 96];
    let tasks = vec![
        ("Setup project", true),
        ("Write code", true),
        ("Add tests", false),
        ("Deploy", false),
    ];

    rusti! {
        <html>
            @page_head("Lists Demo")
            <body class="bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-7xl">
                    @page_header("Lists & Iteration", "Dynamic list rendering with @for loops")
                    <main>
                        <div class="space-y-8">
                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">Simple List</h2>
                                <ul class="space-y-2">
                                    @for fruit in &fruits {
                                        <li class="p-3 bg-blue-50 rounded-lg">{ fruit }</li>
                                    }
                                </ul>
                                <pre class="bg-gray-800 text-green-400 p-4 rounded mt-4 text-sm"><code>{"@for fruit in fruits {\n    <li>{ fruit }</li>\n}"}</code></pre>
                            </div>

                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">Graded Scores</h2>
                                <div class="grid md:grid-cols-2 gap-4">
                                    @for score in &scores {
                                        <div class="flex items-center justify-between p-4 bg-gradient-to-r from-purple-50 to-pink-50 rounded-lg">
                                            <span class="text-3xl font-bold text-purple-600">{ score }</span>
                                            @if *score >= 90 {
                                                <span class="px-4 py-2 bg-green-500 text-white rounded-full font-bold">A</span>
                                            } else {
                                                <span class="px-4 py-2 bg-yellow-500 text-white rounded-full font-bold">B</span>
                                            }
                                        </div>
                                    }
                                </div>
                                <pre class="bg-gray-800 text-green-400 p-4 rounded mt-4 text-sm"><code>{"@for score in scores {\n    @if score >= 90 { <span>A</span> }\n}"}</code></pre>
                            </div>

                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">Task List</h2>
                                <div class="space-y-2">
                                    @for task in &tasks {
                                        <div class="flex items-center gap-3 p-4 bg-gray-50 rounded-lg">
                                            @if task.1 {
                                                <span class="text-2xl">{ "✅" }</span>
                                                <span class="flex-1 line-through text-gray-500">{ task.0 }</span>
                                            } else {
                                                <span class="text-2xl">{ "⬜" }</span>
                                                <span class="flex-1 font-semibold text-gray-800">{ task.0 }</span>
                                            }
                                        </div>
                                    }
                                </div>
                            </div>
                        </div>
                    </main>
                    @page_footer(year)
                </div>
            </body>
        </html>
    }
}

// Match pattern demo
fn match_page() -> impl rusti::Component {
    let year = 2025;
    let status = "active";
    let role = "admin";
    let priority = 1;

    rusti! {
        <html>
            @page_head("Match Demo")
            <body class="bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-7xl">
                    @page_header("Pattern Matching", "Powerful @match expressions")
                    <main>
                        <div class="space-y-8">
                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">String Matching</h2>
                                <div class="p-6 bg-gray-50 rounded-lg">
                                    <p class="text-gray-600 mb-2">Status: { status }</p>
                                    @match status {
                                        "active" => {
                                            <div class="p-4 bg-green-100 border-l-4 border-green-500 text-green-700">
                                                <p class="font-bold">{ "✓ Active" }</p>
                                                <p>System is running normally</p>
                                            </div>
                                        }
                                        "pending" => {
                                            <div class="p-4 bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700">
                                                <p class="font-bold">{ "⏳ Pending" }</p>
                                                <p>Waiting for activation</p>
                                            </div>
                                        }
                                        "inactive" => {
                                            <div class="p-4 bg-red-100 border-l-4 border-red-500 text-red-700">
                                                <p class="font-bold">{ "✗ Inactive" }</p>
                                                <p>System is stopped</p>
                                            </div>
                                        }
                                        _ => {
                                            <div class="p-4 bg-gray-100 border-l-4 border-gray-500 text-gray-700">
                                                <p class="font-bold">? Unknown</p>
                                                <p>Status not recognized</p>
                                            </div>
                                        }
                                    }
                                </div>
                                <pre class="bg-gray-800 text-green-400 p-4 rounded mt-4 text-sm"><code>{"@match status {\n    \"active\" => { ... }\n    \"pending\" => { ... }\n    _ => { ... }\n}"}</code></pre>
                            </div>

                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">Role-Based UI</h2>
                                <div class="p-6 bg-gray-50 rounded-lg">
                                    <p class="text-gray-600 mb-2">Role: { role }</p>
                                    @match role {
                                        "admin" => {
                                            <div class="p-6 bg-red-500 text-white rounded-lg">
                                                <h3 class="text-2xl font-bold mb-2">{ "👑 Administrator" }</h3>
                                                <p>Full system access granted</p>
                                                <button class="mt-4 px-4 py-2 bg-white text-red-600 rounded font-bold">Manage Users</button>
                                            </div>
                                        }
                                        "editor" => {
                                            <div class="p-6 bg-blue-500 text-white rounded-lg">
                                                <h3 class="text-2xl font-bold mb-2">{ "✏️ Editor" }</h3>
                                                <p>Content management access</p>
                                                <button class="mt-4 px-4 py-2 bg-white text-blue-600 rounded font-bold">Edit Content</button>
                                            </div>
                                        }
                                        _ => {
                                            <div class="p-6 bg-gray-500 text-white rounded-lg">
                                                <h3 class="text-2xl font-bold mb-2">{ "👤 Viewer" }</h3>
                                                <p>Read-only access</p>
                                                <button class="mt-4 px-4 py-2 bg-white text-gray-600 rounded font-bold">Browse</button>
                                            </div>
                                        }
                                    }
                                </div>
                            </div>

                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">Priority Levels</h2>
                                <div class="p-6 bg-gray-50 rounded-lg">
                                    <p class="text-gray-600 mb-2">Priority: { priority }</p>
                                    @match priority {
                                        1 => {
                                            <div class="p-4 bg-red-100 rounded">
                                                <span class="text-4xl">{ "🔥" }</span>
                                                <span class="ml-3 text-xl font-bold text-red-700">Critical Priority</span>
                                            </div>
                                        }
                                        2 => {
                                            <div class="p-4 bg-orange-100 rounded">
                                                <span class="text-4xl">{ "⚠️" }</span>
                                                <span class="ml-3 text-xl font-bold text-orange-700">High Priority</span>
                                            </div>
                                        }
                                        3 => {
                                            <div class="p-4 bg-yellow-100 rounded">
                                                <span class="text-4xl">{ "📌" }</span>
                                                <span class="ml-3 text-xl font-bold text-yellow-700">Medium Priority</span>
                                            </div>
                                        }
                                        _ => {
                                            <div class="p-4 bg-blue-100 rounded">
                                                <span class="text-4xl">{ "ℹ️" }</span>
                                                <span class="ml-3 text-xl font-bold text-blue-700">Low Priority</span>
                                            </div>
                                        }
                                    }
                                </div>
                            </div>
                        </div>
                    </main>
                    @page_footer(year)
                </div>
            </body>
        </html>
    }
}

// XSS protection demo
fn xss_page() -> impl rusti::Component {
    let year = 2025;
    let user_input = "<script>alert('XSS')</script>";

    rusti! {
        <html>
            @page_head("XSS Protection")
            <body class="bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-7xl">
                    @page_header("XSS Protection", "Automatic HTML escaping for security")
                    <main>
                        <div class="space-y-8">
                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">Automatic Escaping</h2>
                                <div class="space-y-4">
                                    <div class="p-4 bg-red-50 border-l-4 border-red-500">
                                        <p class="font-bold text-red-700">Malicious Input:</p>
                                        <code class="text-sm bg-white p-2 rounded block">{ user_input }</code>
                                    </div>
                                    <div class="p-4 bg-green-50 border-l-4 border-green-500">
                                        <p class="font-bold text-green-700">Safely Rendered:</p>
                                        <p class="bg-white p-2 rounded">{ user_input }</p>
                                        <p class="text-sm text-gray-600 mt-2">No script execution!</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </main>
                    @page_footer(year)
                </div>
            </body>
        </html>
    }
}

// Attributes demo
fn attributes_page() -> impl rusti::Component {
    let year = 2025;
    let btn_class = "px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700";

    rusti! {
        <html>
            @page_head("Attributes Demo")
            <body class="bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-7xl">
                    @page_header("Dynamic Attributes", "Interpolate values into HTML attributes")
                    <main>
                        <div class="space-y-8">
                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">Dynamic Classes</h2>
                                <button class={btn_class}>Styled Button</button>
                                <pre class="bg-gray-800 text-green-400 p-3 rounded mt-4 text-sm"><code>{"<button class={btn_class}>...</button>"}</code></pre>
                            </div>
                        </div>
                    </main>
                    @page_footer(year)
                </div>
            </body>
        </html>
    }
}

// HTMX Interactive Demo
fn htmx_page() -> impl rusti::Component {
    let year = 2025;

    rusti! {
        <html>
            @page_head("HTMX + Rusti Demo")
            <body class="bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-7xl">
                    @page_header("HTMX + Rusti Demo", "Interactive counter with server-side rendering")
                    <main>
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
                    </main>
                    @page_footer(year)
                </div>
            </body>
        </html>
    }
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

async fn conditionals_demo() -> impl IntoResponse {
    Html(rusti::render_to_string(&conditionals_page()))
}

async fn xss_demo() -> impl IntoResponse {
    Html(rusti::render_to_string(&xss_page()))
}

async fn attributes_demo() -> impl IntoResponse {
    Html(rusti::render_to_string(&attributes_page()))
}

async fn hello_world() -> impl IntoResponse {
    Html(rusti::render_to_string(&home_page()))
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

async fn lists_demo() -> impl IntoResponse {
    Html(rusti::render_to_string(&lists_page()))
}

async fn match_demo() -> impl IntoResponse {
    Html(rusti::render_to_string(&match_page()))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/htmx", get(htmx_demo))
        .route("/conditionals", get(conditionals_demo))
        .route("/lists", get(lists_demo))
        .route("/match", get(match_demo))
        .route("/xss", get(xss_demo))
        .route("/attributes", get(attributes_demo))
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
    println!("   http://127.0.0.1:3000/  - Home");
    println!("   http://127.0.0.1:3000/htmx - HTMX Demo");
    println!("   http://127.0.0.1:3000/conditionals - Conditionals");
    println!("   http://127.0.0.1:3000/lists - Lists & Iteration");
    println!("   http://127.0.0.1:3000/match - Pattern Matching");
    println!("   http://127.0.0.1:3000/xss - XSS Protection");
    println!("   http://127.0.0.1:3000/attributes - Dynamic Attributes");
    println!("\n✨ Features demonstrated:");
    println!("   • Component composition");
    println!("   • HTMX interactivity");
    println!("   • Conditional rendering (@if/@else)");
    println!("   • List iteration (@for)");
    println!("   • Pattern matching (@match)");
    println!("   • Auto HTML escaping (XSS protection)");
    println!("   • Dynamic attributes");
    println!("\n");

    axum::serve(listener, app).await.unwrap();
}
