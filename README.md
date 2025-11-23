Rusti 🦀
A Type-Safe, Rust-Native HTML Templating Library
Rusti is a powerful, zero-cost, type-safe HTML templating engine for Rust. It allows you to write HTML-like syntax directly in your Rust code using the rusti! macro.
Inspired by Go's templ library, Rusti brings the component model to server-side Rust. Because it compiles to native Rust code, it is blazing fast and type-safe.
⚠️ IMPORTANT: Syntax Rules & "Gotchas"
Please read this first.
Because rusti! runs after the Rust compiler parses your code, you must respect the Rust Tokenizer. You are writing HTML inside Rust code, so Rust's syntax rules still apply.
1. The "2em" CSS Problem
Rust treats 2e... as the start of Scientific Notation (e.g., 2e10).
If you write margin: 2em;, the Rust compiler crashes before Rusti can even run.
❌ Invalid: margin: 2em; (Looks like a broken float)
✅ Valid: margin: "2em"; (Recommended - Rusti strips the quotes for you)
✅ Valid: margin: 2.0em; (The decimal point breaks the scientific notation check)
✅ Valid: margin: 20px; (Other units like px, rem, % work fine without quotes)
2. No Backticks (`) or Single Quotes (')
Backticks: Rust does not support backticks (`) in source code. They are illegal tokens.
Fix: Use Raw Strings (r#" ... "#).
Single Quotes: In Rust, 'a' is a char. You cannot use single quotes for strings or attributes.
Fix: Always use double quotes (class="foo").
3. The "Happy Path": Tailwind CSS 🌊
The absolute best way to use Rusti is with Tailwind CSS.
Since Tailwind uses standard string classes (class="p-4 text-center"), you will never hit the 2em or tokenizer issues.
🚀 Key Features
Type-Safe: All variables and expressions are checked at compile time.
Zero-Cost Abstraction: Compiles directly to std::fmt::Write calls. No runtime parsing, no virtual DOM.
Rust-Native Control Flow: Use @if, @for, and @match directly in your templates.
Component Composition: Build complex UIs from small, reusable Rust functions.
Automatic XSS Protection: All dynamic content is HTML-escaped by default.
Framework Agnostic: Works with Axum, Actix-web, Rocket, or any Rust program.
📦 Installation
Add rusti to your Cargo.toml:
code
Toml
[dependencies]
rusti = { git = "https://github.com/DraconDev/rusti" }
📖 Usage Guide
1. Basic Component
code
Rust
use rusti::rusti;

fn hello_world() -> impl rusti::Component {
    rusti! {
        // Standard HTML attributes use double quotes
        <div class="container mx-auto p-4">
            <h1 class="text-2xl font-bold">Hello, World!</h1>
        </div>
    }
}
2. Styling Strategy (CSS)
You have three ways to write CSS in Rusti.
A. The "Naked" Style (Recommended for simple CSS)
Most CSS is valid Rust syntax. You only need quotes for the 2em edge case.
code
Rust
rusti! {
    <style>
        .box {
            color: #ff0000;      /* Valid: Punctuation (#) + Ident (ff0000) */
            padding: 20px;       /* Valid: Integer (20) + Suffix (px) */
            font-size: 1.5rem;   /* Valid: Float (1.5) + Ident (rem) */
            
            margin: "2em";       /* QUOTES REQUIRED: Fixes the Scientific Notation crash */
        }
    </style>
}
B. The "Raw String" Style (Recommended for complex CSS)
If you are pasting a large block of CSS or using complex hacks, wrap it in a raw string.
code
Rust
rusti! {
    <style>{r#"
        body { margin: 0; padding: 2em; } /* No quotes needed inside r#""# */
        .card > * { display: block; }
    "#}</style>
}
C. The Tailwind Style (Highly Recommended)
Avoid writing CSS entirely.
code
Rust
rusti! {
    <div class="m-0 p-[2em] text-red-500">Tailwind is great</div>
}
3. Dynamic Content & Attributes
Inject Rust variables using curly braces { }.
code
Rust
fn greeting(name: &str, is_admin: bool) -> impl rusti::Component + '_ {
    let role_color = if is_admin { "text-red-500" } else { "text-blue-500" };
    
    rusti! {
        <div class="card">
            // Interpolate text
            <h1>Hello, { name }!</h1>
            
            // Interpolate attributes (No quotes needed for variables!)
            <span class={role_color}>User Role</span>
            
            // Complex logic in attributes requires braces
            <button disabled={!is_admin}>Delete</button>
        </div>
    }
}
4. Handling JSON / Complex Attributes
If you need to pass JSON (e.g., for HTMX or AlpineJS) that contains quotes, use Rust's Raw String syntax r#"..."#.
code
Rust
rusti! {
    // This allows you to use double quotes " inside the attribute
    <div hx-vals=r#"{"id": 42, "action": "update"}"#>
        Update
    </div>
}
5. Control Flow
Rusti provides first-class support for control flow using @.
code
Rust
fn user_list(users: Vec<&str>, role: Option<&str>) -> impl rusti::Component + '_ {
    rusti! {
        <div>
            // Pattern Matching
            @match role {
                Some("admin") => { <button>Delete User</button> }
                Some(_)       => { <span>Read Only</span> }
                None          => { <a href="/login">Log in</a> }
            }

            // Loops
            <ul>
                @for user in users {
                    <li>{ user }</li>
                }
            </ul>

            // Conditionals
            @if users.is_empty() {
                <p>No users found.</p>
            }
        </div>
    }
}
🌐 Axum Example
code
Rust
use axum::{response::{Html, IntoResponse}, routing::get, Router};
use rusti::rusti;

async fn handler() -> impl IntoResponse {
    let html = rusti! {
        <h1 class="text-4xl font-bold text-indigo-600">
            Hello from Axum!
        </h1>
    };
    Html(html.render_to_string())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
🛠️ Project Structure
src/: Runtime library (Component trait, escaping logic).
macros/: The procedural macro implementation (Parser using syn).
demo/: A complete example application using Axum, HTMX, and Tailwind CSS.
📄 License
MIT License