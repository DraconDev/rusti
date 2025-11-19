use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use rusti::rusti;
use serde::Deserialize;

mod examples;

#[derive(Deserialize)]
struct CounterForm {
    count: i32,
}

fn page<'a>(title: &'a str, body: &'a str) -> impl rusti::Component + 'a {
    let year = 2025;    

    
    rusti! {
        <html>
            <head>
                <title>{ title }</title>
                <link rel="stylesheet" href="https://cdn.tailwindcss.com"></link>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-7xl">
                    <header class="text-center mb-16">
                        <div class="inline-block">
                            <h1 class="text-6xl font-black text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-purple-600 mb-4">{ title }</h1>
                            <div class="h-1 bg-gradient-to-r from-blue-600 to-purple-600 rounded-full"></div>
                        </div>
                        <p class="text-xl text-gray-600 mt-6 max-w-2xl mx-auto">{ body }</p>
                        <div class="flex gap-4 justify-center mt-8">
                            <span class="px-4 py-2 bg-blue-100 text-blue-700 rounded-full text-sm font-semibold">Type-Safe</span>
                            <span class="px-4 py-2 bg-purple-100 text-purple-700 rounded-full text-sm font-semibold">Zero-Cost</span>
                            <span class="px-4 py-2 bg-green-100 text-green-700 rounded-full text-sm font-semibold">XSS Protected</span>
                        </div>
                    </header>
                    <main>
                        <section class="mb-12">
                            <h2 class="text-4xl font-bold text-gray-800 mb-8 text-center">Explore Examples</h2>
                            <div class="grid md:grid-cols-2 gap-8">
                                <div class="group bg-white rounded-2xl p-8 shadow-lg hover:shadow-2xl transition-all duration-300 border border-blue-100 hover:border-blue-300">
                                    <div class="flex items-center gap-3 mb-6">
                                        <div class="w-12 h-12 bg-gradient-to-br from-blue-400 to-blue-600 rounded-xl flex items-center justify-center text-white text-xl font-bold shadow-lg">B</div>
                                        <h3 class="text-2xl font-bold text-gray-800">Basic Examples</h3>
                                    </div>
                                    <ul class="space-y-3">
                                        <li class="group/item">
                                            <a href="/" class="flex items-center gap-2 text-blue-600 hover:text-blue-800 font-medium transition-colors p-3 rounded-lg hover:bg-blue-50">
                                                <span class="text-blue-400 group-hover/item:translate-x-1 transition-transform">&gt;</span>
                                                <span>Simple component composition</span>
                                            </a>
                                        </li>
                                    </ul>
                                </div>
                                <div class="group bg-white rounded-2xl p-8 shadow-lg hover:shadow-2xl transition-all duration-300 border border-purple-100 hover:border-purple-300">
                                    <div class="flex items-center gap-3 mb-6">
                                        <div class="w-12 h-12 bg-gradient-to-br from-purple-400 to-purple-600 rounded-xl flex items-center justify-center text-white text-xl font-bold shadow-lg">A</div>
                                        <h3 class="text-2xl font-bold text-gray-800">Advanced Features</h3>
                                    </div>
                                    <ul class="space-y-3">
                                        <li class="group/item">
                                            <a href="/advanced" class="flex items-center gap-2 text-purple-600 hover:text-purple-800 font-medium transition-colors p-3 rounded-lg hover:bg-purple-50">
                                                <span class="text-purple-400 group-hover/item:translate-x-1 transition-transform">&gt;</span>
                                                <span>Complex nested components</span>
                                            </a>
                                        </li>
                                        <li class="group/item">
                                            <a href="/advanced/dashboard" class="flex items-center gap-2 text-purple-600 hover:text-purple-800 font-medium transition-colors p-3 rounded-lg hover:bg-purple-50">
                                                <span class="text-purple-400 group-hover/item:translate-x-1 transition-transform">&gt;</span>
                                                <span>Dynamic user dashboard</span>
                                            </a>
                                        </li>
                                        <li class="group/item">
                                            <a href="/advanced/user/123" class="flex items-center gap-2 text-purple-600 hover:text-purple-800 font-medium transition-colors p-3 rounded-lg hover:bg-purple-50">
                                                <span class="text-purple-400 group-hover/item:translate-x-1 transition-transform">&gt;</span>
                                                <span>User profile template</span>
                                            </a>
                                        </li>
                                    </ul>
                                </div>
                                <div class="group bg-white rounded-2xl p-8 shadow-lg hover:shadow-2xl transition-all duration-300 border border-green-100 hover:border-green-300">
                                    <div class="flex items-center gap-3 mb-6">
                                        <div class="w-12 h-12 bg-gradient-to-br from-green-400 to-green-600 rounded-xl flex items-center justify-center text-white text-xl font-bold shadow-lg">C</div>
                                        <h3 class="text-2xl font-bold text-gray-800">Conditional Rendering</h3>
                                    </div>
                                    <ul class="space-y-3">
                                        <li class="group/item">
                                            <a href="/conditional/active" class="flex items-center gap-2 text-green-600 hover:text-green-800 font-medium transition-colors p-3 rounded-lg hover:bg-green-50">
                                                <span class="text-green-400 group-hover/item:translate-x-1 transition-transform">&gt;</span>
                                                <span>If/else patterns</span>
                                            </a>
                                        </li>
                                        <li class="group/item">
                                            <a href="/conditional/pending" class="flex items-center gap-2 text-green-600 hover:text-green-800 font-medium transition-colors p-3 rounded-lg hover:bg-green-50">
                                                <span class="text-green-400 group-hover/item:translate-x-1 transition-transform">&gt;</span>
                                                <span>Multiple conditions</span>
                                            </a>
                                        </li>
                                        <li class="group/item">
                                            <a href="/conditional/inactive" class="flex items-center gap-2 text-green-600 hover:text-green-800 font-medium transition-colors p-3 rounded-lg hover:bg-green-50">
                                                <span class="text-green-400 group-hover/item:translate-x-1 transition-transform">&gt;</span>
                                                <span>Default case handling</span>
                                            </a>
                                        </li>
                                    </ul>
                                </div>
                                <div class="group bg-white rounded-2xl p-8 shadow-lg hover:shadow-2xl transition-all duration-300 border border-orange-100 hover:border-orange-300">
                                    <div class="flex items-center gap-3 mb-6">
                                        <div class="w-12 h-12 bg-gradient-to-br from-orange-400 to-orange-600 rounded-xl flex items-center justify-center text-white text-xl font-bold shadow-lg">M</div>
                                        <h3 class="text-2xl font-bold text-gray-800">Match Expressions</h3>
                                    </div>
                                    <ul class="space-y-3">
                                        <li class="group/item">
                                            <a href="/match/admin/95" class="flex items-center gap-2 text-orange-600 hover:text-orange-800 font-medium transition-colors p-3 rounded-lg hover:bg-orange-50">
                                                <span class="text-orange-400 group-hover/item:translate-x-1 transition-transform">&gt;</span>
                                                <span>String pattern matching</span>
                                            </a>
                                        </li>
                                        <li class="group/item">
                                            <a href="/match/moderator/82" class="flex items-center gap-2 text-orange-600 hover:text-orange-800 font-medium transition-colors p-3 rounded-lg hover:bg-orange-50">
                                                <span class="text-orange-400 group-hover/item:translate-x-1 transition-transform">&gt;</span>
                                                <span>Role-based rendering</span>
                                            </a>
                                        </li>
                                        <li class="group/item">
                                            <a href="/match/user/67" class="flex items-center gap-2 text-orange-600 hover:text-orange-800 font-medium transition-colors p-3 rounded-lg hover:bg-orange-50">
                                                <span class="text-orange-400 group-hover/item:translate-x-1 transition-transform">&gt;</span>
                                                <span>Range matching</span>
                                            </a>
                                        </li>
                                    </ul>
                                </div>
                            </div>
                        </section>
                    </main>
                    <footer class="text-center mt-16 pt-8 border-t border-gray-200">
                        <p class="text-gray-500">Copyright { year } - Built with Rusti</p>
                    </footer>
                </div>
            </body>
        </html>
    }
}

// Advanced example components

fn advanced_page<'a>(page_title: &'a str, user_count: i32) -> impl rusti::Component + 'a {
    rusti! {
        <html>
            <head>
                <title>{ page_title }</title>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="bg-gradient-to-br from-purple-50 to-pink-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-6xl">
                    <header class="text-center mb-12">
                        <h1 class="text-5xl font-black text-transparent bg-clip-text bg-gradient-to-r from-purple-600 to-pink-600 mb-4">{ page_title }</h1>
                        <p class="text-lg text-gray-600">Complex component composition in action</p>
                    </header>
                    <main class="space-y-8">
                        <div class="bg-white rounded-2xl p-8 shadow-lg">
                            <h2 class="text-2xl font-bold text-gray-800 mb-4">Live Stats</h2>
                            <div class="flex items-center gap-4">
                                <div class="text-5xl font-bold text-purple-600">{ user_count }</div>
                                <div class="text-gray-600">Active users right now</div>
                            </div>
                        </div>
                        <div class="bg-white rounded-2xl p-8 shadow-lg">
                            <h2 class="text-2xl font-bold text-gray-800 mb-6">Key Features</h2>
                            <ul class="space-y-3">
                                <li class="flex items-start gap-3">
                                    <span class="text-green-500 text-xl">&gt;</span>
                                    <span class="text-gray-700">User count is dynamically injected</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <span class="text-green-500 text-xl">&gt;</span>
                                    <span class="text-gray-700">All text is automatically HTML escaped</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <span class="text-green-500 text-xl">&gt;</span>
                                    <span class="text-gray-700">Components can be nested arbitrarily deep</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <span class="text-green-500 text-xl">&gt;</span>
                                    <span class="text-gray-700">Full Rust type system integration</span>
                                </li>
                            </ul>
                        </div>
                    </main>
                    <footer class="text-center mt-12 text-gray-500">
                        <a href="/" class="text-purple-600 hover:underline">Back to Home</a>
                    </footer>
                </div>
            </body>
        </html>
    }
}

fn user_profile<'a>(
    user_id: &'a str,
    username: &'a str,
    email: &'a str,
) -> impl rusti::Component + 'a {
    rusti! {
        <html>
            <head>
                <title>User Profile</title>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="bg-gradient-to-br from-blue-50 to-cyan-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-4xl">
                    <header class="text-center mb-12">
                        <h1 class="text-5xl font-black text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-cyan-600 mb-2">User Profile</h1>
                    </header>
                    <main class="bg-white rounded-2xl p-8 shadow-xl">
                        <h2 class="text-3xl font-bold text-gray-800 mb-6">Profile Details</h2>
                        <dl class="grid grid-cols-1 md:grid-cols-3 gap-6">
                            <div class="bg-blue-50 rounded-lg p-4">
                                <dt class="text-sm font-semibold text-blue-900 mb-2">User ID</dt>
                                <dd class="text-xl font-bold text-blue-600">{ user_id }</dd>
                            </div>
                            <div class="bg-blue-50 rounded-lg p-4">
                                <dt class="text-sm font-semibold text-blue-900 mb-2">Username</dt>
                                <dd class="text-xl font-bold text-blue-600">{ username }</dd>
                            </div>
                            <div class="bg-blue-50 rounded-lg p-4">
                                <dt class="text-sm font-semibold text-blue-900 mb-2">Email</dt>
                                <dd class="text-xl font-bold text-blue-600">{ email }</dd>
                            </div>
                        </dl>
                    </main>
                    <footer class="text-center mt-12 text-gray-500">
                        <a href="/" class="text-blue-600 hover:underline">Back to Home</a>
                    </footer>
                </div>
            </body>
        </html>
    }
}

async fn hello_world() -> impl IntoResponse {
    let title = "Rusti - Type-Safe HTML Templates";
    let body = "Welcome to Rusti! A Go templ-like library for Rust with component composition, XSS protection, and full type safety.";
    let component = page(title, body);
    Html(rusti::render_to_string(&component))
}

async fn advanced_demo() -> impl IntoResponse {
    let component = advanced_page("Advanced Features Demo", 42);
    Html(rusti::render_to_string(&component))
}

async fn advanced_dashboard() -> impl IntoResponse {
    let component = advanced_page("Dashboard", 1337);
    Html(rusti::render_to_string(&component))
}

async fn advanced_user(Path(user_id): Path<String>) -> impl IntoResponse {
    let username = format!("User{}", &user_id);
    let email = format!("user{}@example.com", &user_id);
    let component = user_profile(&user_id, &username, &email);
    Html(rusti::render_to_string(&component))
}

// Demonstrate if/else conditional rendering
fn status_badge(status: &str) -> &str {
    // Use Rust if/else to choose content
    if status == "active" {
        "Active"
    } else if status == "pending" {
        "Pending Review"
    } else {
        "Inactive"
    }
}

fn conditional_page<'a>(user_status: &'a str, item_count: i32) -> impl rusti::Component + 'a {
    let badge_text = status_badge(user_status);

    let message = if item_count > 10 {
        "You have many items!"
    } else if item_count > 0 {
        "You have some items"
    } else {
        "No items yet"
    };

    rusti! {
        <html>
            <head>
                <title>Conditional Rendering Demo</title>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="bg-gradient-to-br from-green-50 to-emerald-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-4xl">
                    <header class="text-center mb-12">
                        <h1 class="text-5xl font-black text-transparent bg-clip-text bg-gradient-to-r from-green-600 to-emerald-600 mb-4">If/Else Examples</h1>
                    </header>
                    <main class="space-y-6">
                        <div class="bg-white rounded-2xl p-8 shadow-lg">
                            <h2 class="text-2xl font-bold text-gray-800 mb-6">Status Badge</h2>
                            <div class="inline-block px-6 py-3 bg-green-100 text-green-800 rounded-full text-lg font-semibold">
                                { badge_text }
                            </div>
                        </div>
                        <div class="bg-white rounded-2xl p-8 shadow-lg">
                            <h2 class="text-2xl font-bold text-gray-800 mb-4">Item Count</h2>
                            <p class="text-lg text-gray-700 mb-2">You have <span class="font-bold text-green-600">{ item_count }</span> items</p>
                            <p class="text-gray-600 italic">{ message }</p>
                        </div>
                    </main>
                    <footer class="text-center mt-12 text-gray-500">
                        <a href="/" class="text-green-600 hover:underline">Back to Home</a>
                    </footer>
                </div>
            </body>
        </html>
    }
}

// Demonstrate match expression rendering
fn role_description(role: &str) -> &str {
    match role {
        "admin" => "Full system access",
        "moderator" => "Content moderation access",
        "user" => "Standard user access",
        _ => "Unknown role",
    }
}

fn role_color(role: &str) -> &str {
    match role {
        "admin" => "red",
        "moderator" => "orange",
        "user" => "green",
        _ => "gray",
    }
}

fn match_page<'a>(role: &'a str, score: i32) -> impl rusti::Component + 'a {
    let description = role_description(role);
    let color = role_color(role);

    let grade = match score {
        90..=100 => "A - Excellent",
        80..=89 => "B - Good",
        70..=79 => "C - Average",
        60..=69 => "D - Below Average",
        _ => "F - Failing",
    };

    rusti! {
        <html>
            <head>
                <title>Match Expression Demo</title>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="bg-gradient-to-br from-orange-50 to-red-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-4xl">
                    <header class="text-center mb-12">
                        <h1 class="text-5xl font-black text-transparent bg-clip-text bg-gradient-to-r from-orange-600 to-red-600 mb-4">Match Examples</h1>
                    </header>
                    <main class="space-y-6">
                        <div class="bg-white rounded-2xl p-8 shadow-lg">
                            <h2 class="text-2xl font-bold text-gray-800 mb-6">User Role</h2>
                            <div class="space-y-4">
                                <div>
                                    <span class="text-sm text-gray-500">Role:</span>
                                    <div class="text-2xl font-bold text-orange-600 mt-1">{ role }</div>
                                </div>
                                <p class="text-gray-700">{ description }</p>
                                <div class="inline-block px-4 py-2 rounded-lg font-semibold" style="background-color: { color }20; color: { color };">{ color } badge</div>
                            </div>
                        </div>
                        <div class="bg-white rounded-2xl p-8 shadow-lg">
                            <h2 class="text-2xl font-bold text-gray-800 mb-6">Score Grading</h2>
                            <div class="flex items-center gap-6">
                                <div class="text-5xl font-black text-orange-600">{ score }</div>
                                <div>
                                    <div class="text-sm text-gray-500">Grade</div>
                                    <div class="text-xl font-bold text-gray-800">{ grade }</div>
                                </div>
                            </div>
                        </div>
                    </main>
                    <footer class="text-center mt-12 text-gray-500">
                        <a href="/" class="text-orange-600 hover:underline">Back to Home</a>
                    </footer>
                </div>
            </body>
        </html>
    }
}

// HTMX Interactive Demo
// fn htmx_page() -> impl rusti::Component {
//     rusti! {
//         <html>
//             <head>
//                 <title>HTMX + Rusti Demo</title>
//                 <script src="https://cdn.tailwindcss.com"></script>
//                 <script src="https://unpkg.com/htmx.org@1.9.10"></script>
//             </head>
//             <body class="bg-gradient-to-br from-pink-50 to-rose-50 min-h-screen">
//                 <div class="container mx-auto px-4 py-12 max-w-4xl">
//                     <header class="text-center mb-12">
//                         <h1 class="text-5xl font-black text-transparent bg-clip-text bg-gradient-to-r from-pink-600 to-rose-600 mb-4">HTMX with Rusti</h1>
//                         <p class="text-lg text-gray-600">Interactive examples with zero JavaScript</p>
//                     </header>
//                     <main class="space-y-8">
//                         <div class="bg-white rounded-2xl p-8 shadow-lg">
//                             <h2 class="text-2xl font-bold text-gray-800 mb-6">Interactive Counter</h2>
//                             <div id="counter" class="text-center">
//                                 <div class="text-6xl font-black text-pink-600 mb-6">0</div>
//                                 <div class="flex gap-4 justify-center">
//                                     <button hx-post="/htmx/counter/increment" hx-target="#counter" hx-swap="innerHTML" class="px-6 py-3 bg-pink-600 text-white rounded-lg font-semibold hover:bg-pink-700 transition">
//                                         Increment
//                                     </button>
//                                     <button hx-post="/htmx/counter/decrement" hx-target="#counter" hx-swap="innerHTML" class="px-6 py-3 bg-gray-600 text-white rounded-lg font-semibold hover:bg-gray-700 transition">
//                                         Decrement
//                                     </button>
//                                     <button hx-post="/htmx/counter/reset" hx-target="#counter" hx-swap="innerHTML" class="px-6 py-3 bg-red-600 text-white rounded-lg font-semibold hover:bg-red-700 transition">
//                                         Reset
//                                     </button>
//                                 </div>
//                             </div>
//                         </div>
//                         <div class="bg-white rounded-2xl p-8 shadow-lg">
//                             <h2 class="text-2xl font-bold text-gray-800 mb-4">How It Works</h2>
//                             <ul class="space-y-2 text-gray-700">
//                                 <li class="flex items-start gap-2">
//                                     <span class="text-pink-500 font-bold">&gt;</span>
//                                     <span>HTMX attributes enable interactivity without JavaScript</span>
//                                 </li>
//                                 <li class="flex items-start gap-2">
//                                     <span class="text-pink-500 font-bold">&gt;</span>
//                                     <span>Rusti attribute support makes HTMX integration seamless</span>
//                                 </li>
//                                 <li class="flex items-start gap-2">
//                                     <span class="text-pink-500 font-bold">&gt;</span>
//                                     <span>Server renders partial HTML updates</span>
//                                 </li>
//                             </ul>
//                         </div>
//                     </main>
//                     <footer class="text-center mt-12 text-gray-500">
//                         <a href="/" class="text-pink-600 hover:underline">Back to Home</a>
//                     </footer>
//                 </div>
//             </body>
//         </html>
//     }
// }

// fn counter_partial(count: i32) -> impl rusti::Component {
//     rusti! {
//         <div class="text-6xl font-black text-pink-600 mb-6">{ count }</div>
//         <div class="flex gap-4 justify-center">
//             <form hx-post="/htmx/counter/increment" hx-target="#counter" hx-swap="innerHTML">
//                 <input type="hidden" name="count" value={count.to_string().as_str()}></input>
//                 <button type="submit" class="px-6 py-3 bg-pink-600 text-white rounded-lg font-semibold hover:bg-pink-700 transition">
//                     Increment
//                 </button>
//             </form>
//             <form hx-post="/htmx/counter/decrement" hx-target="#counter" hx-swap="innerHTML">
//                 <input type="hidden" name="count" value={count.to_string().as_str()}></input>
//                 <button type="submit" class="px-6 py-3 bg-gray-600 text-white rounded-lg font-semibold hover:bg-gray-700 transition">
//                     Decrement
//                 </button>
//             </form>
//             <form hx-post="/htmx/counter/reset" hx-target="#counter" hx-swap="innerHTML">
//                 <button type="submit" class="px-6 py-3 bg-red-600 text-white rounded-lg font-semibold hover:bg-red-700 transition">
//                     Reset
//                 </button>
//             </form>
//         </div>
//     }
// }

// async fn htmx_demo() -> impl IntoResponse {
//     Html(rusti::render_to_string(&htmx_page()))
// }

// async fn htmx_increment(Form(form): Form<CounterForm>) -> impl IntoResponse {
//     let new_count = form.count + 1;
//     Html(rusti::render_to_string(&counter_partial(new_count)))
// }

// async fn htmx_decrement(Form(form): Form<CounterForm>) -> impl IntoResponse {
//     let new_count = form.count - 1;
//     Html(rusti::render_to_string(&counter_partial(new_count)))
// }

// async fn htmx_reset() -> impl IntoResponse {
//     Html(rusti::render_to_string(&counter_partial(0)))
// }

async fn conditional_demo(Path(status): Path<String>) -> impl IntoResponse {
    let item_count = if status == "active" { 15 } else { 3 };
    let component = conditional_page(&status, item_count);
    Html(rusti::render_to_string(&component))
}

async fn match_demo(Path((role, score)): Path<(String, i32)>) -> impl IntoResponse {
    let component = match_page(&role, score);
    Html(rusti::render_to_string(&component))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/advanced", get(advanced_demo))
        .route("/advanced/dashboard", get(advanced_dashboard))
        .route("/advanced/user/:id", get(advanced_user))
        .route("/conditional/:status", get(conditional_demo))
        // .route("/match/:role/:score", get(match_demo))
        // .route("/htmx", get(htmx_demo))
        // .route("/htmx/counter/increment", post(htmx_increment))
        // .route("/htmx/counter/decrement", post(htmx_decrement))
        // .route("/htmx/counter/reset", post(htmx_reset));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("\n🚀 Rusti Web Server");
    println!("===================");
    println!("Server running on http://127.0.0.1:3000");
    println!("\n📍 Available routes:");
    println!("   http://127.0.0.1:3000/");
    println!("   http://127.0.0.1:3000/advanced");
    println!("   http://127.0.0.1:3000/advanced/dashboard");
    println!("   http://127.0.0.1:3000/advanced/user/123");
    println!("\n🎯 Conditional rendering:");
    println!("   http://127.0.0.1:3000/conditional/active");
    println!("   http://127.0.0.1:3000/conditional/pending");
    println!("   http://127.0.0.1:3000/conditional/inactive");
    println!("\n🔀 Match expressions:");
    println!("   http://127.0.0.1:3000/match/admin/95");
    println!("   http://127.0.0.1:3000/match/moderator/82");
    println!("   http://127.0.0.1:3000/match/user/67");
    println!("\n💫 HTMX interactivity:");
    println!("   http://127.0.0.1:3000/htmx");
    println!("\n✨ Features demonstrated:");
    println!("   • Component composition");
    println!("   • Nested components");
    println!("   • Dynamic content");
    println!("   • If/else conditionals");
    println!("   • Match expressions");
    println!("   • XSS protection");
    println!("\n");

    axum::serve(listener, app).await.unwrap();
}
