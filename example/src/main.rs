use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use rusti::rusti;

fn header(title: &str) -> impl rusti::Component + '_ {
    rusti! {
        <header>
            <h1>{ title }</h1>
        </header>
    }
}

fn footer(year: i32) -> impl rusti::Component {
    rusti! {
        <footer>
            <p>Copyright { year }</p>
        </footer>
    }
}

fn page<'a>(title: &'a str, body: &'a str) -> impl rusti::Component + 'a {
    rusti! {
        <html>
            <head>
                <title>{ title }</title>
                <link></link>
            </head>
            <body>
                @header(title)
                <main>
                    <p>{ body }</p>
                    <section>
                        <h2>Explore Examples</h2>
                        <div>
                            <h3>Basic Examples</h3>
                            <ul>
                                <li><a>Home - Simple component composition</a></li>
                            </ul>
                        </div>
                        <div>
                            <h3>Advanced Features</h3>
                            <ul>
                                <li><a>Advanced Demo - Complex nested components</a></li>
                                <li><a>Dashboard - Dynamic user count</a></li>
                                <li><a>User Profile - Template with parameters</a></li>
                            </ul>
                        </div>
                        <div>
                            <h3>Conditional Rendering</h3>
                            <ul>
                                <li><a>Active Status - If/else example</a></li>
                                <li><a>Pending Status - Multiple conditions</a></li>
                                <li><a>Inactive Status - Default case</a></li>
                            </ul>
                        </div>
                        <div>
                            <h3>Match Expressions</h3>
                            <ul>
                                <li><a>Admin Role - Match with string patterns</a></li>
                                <li><a>Moderator Role - Role-based rendering</a></li>
                                <li><a>User Role - Match with ranges</a></li>
                            </ul>
                        </div>
                    </section>
                </main>
                @footer(2025)
            </body>
        </html>
    }
}

// Advanced example components
fn card<'a>(title: &'a str, content: &'a str, badge: &'a str) -> impl rusti::Component + 'a {
    rusti! {
        <div>
            <h3>{ title }</h3>
            <span>{ badge }</span>
            <p>{ content }</p>
        </div>
    }
}

fn advanced_page<'a>(page_title: &'a str, user_count: i32) -> impl rusti::Component + 'a {
    rusti! {
        <html>
            <head>
                <title>{ page_title }</title>
                <link></link>
            </head>
            <body>
                @header("Rusti Advanced Demo")
                <main>
                    <section>
                        <h2>{ page_title }</h2>
                        <p>This demonstrates complex component composition in Rusti</p>
                        <div>
                            <p>Active users:</p>
                            <strong>{ user_count }</strong>
                        </div>
                    </section>
                    <section>
                        <h2>Feature Cards</h2>
                        <div>
                            @card("Type Safety", "All components are fully type-checked at compile time", "Feature") @card("Zero Cost", "Templates compile to efficient fmt::Write calls", "Performance") @card("Composable", "Nest components infinitely with full Rust type safety", "Architecture")
                        </div>
                    </section>
                    <section>
                        <h2>Dynamic Content</h2>
                        <ul>
                            <li>User count is dynamically injected</li>
                            <li>All text is automatically HTML escaped</li>
                            <li>Components can be nested arbitrarily deep</li>
                            <li>Full Rust type system integration</li>
                        </ul>
                    </section>
                </main>
                @footer(2025)
                <script></script>
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
            </head>
            <body>
                @header("User Profile")
                <main>
                    <h2>Profile Details</h2>
                    <dl>
                        <dt>User ID</dt>
                        <dd>{ user_id }</dd>
                        <dt>Username</dt>
                        <dd>{ username }</dd>
                        <dt>Email</dt>
                        <dd>{ email }</dd>
                    </dl>
                    <div>
                        @card("Account Status", "Active and verified", "Status")
                        @card("Member Since", "January 2025", "Info")
                    </div>
                </main>
                @footer(2025)
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

    // Conditional rendering with if
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
            </head>
            <body>
                @header("If/Else Examples")
                <main>
                    <h2>Conditional Rendering Patterns</h2>
                    <section>
                        <h3>Status Badge</h3>
                        <p>Current status:</p>
                        <strong>{ badge_text }</strong>
                    </section>
                    <section>
                        <h3>Item Count</h3>
                        <p>You have { item_count } items</p>
                        <p>{ message }</p>
                    </section>
                </main>
                @footer(2025)
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

    // Match on numeric ranges
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
            </head>
            <body>
                @header("Match Examples")
                <main>
                    <h2>Match Expression Patterns</h2>
                    <section>
                        <h3>User Role</h3>
                        <p>Role:</p>
                        <strong>{ role }</strong>
                        <p>{ description }</p>
                        <p>Badge color: { color }</p>
                    </section>
                    <section>
                        <h3>Score Grading</h3>
                        <p>Score: { score }</p>
                        <p>Grade: { grade }</p>
                    </section>
                </main>
                @footer(2025)
            </body>
        </html>
    }
}

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
        .route("/match/:role/:score", get(match_demo));

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
