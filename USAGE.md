# Using R usti in Your Project

## Quick Start

### 1. Add Rusti to your `Cargo.toml`

Since Rusti is not published on crates.io yet, you can use it via a local path or Git repository:

**Option A: Local Path** (if you have the source code locally)
```toml
[dependencies]
rusti = { path = "../path/to/rusti" }
# The macro is auto-imported, no need to add it separately
```

**Option B: Git Repository** (recommended for sharing)
```toml
[dependencies]
rusti = { git = "https://github.com/yourusername/rusti" }
# The macro is re-exported from rusti, so you only need to add rusti
```

**Option C: Published on crates.io** (once published)
```toml
[dependencies]
rusti = "0.1.0"
```

### 2. Import and Use

```rust
use rusti::rusti;

fn greeting(name: &str) -> impl rusti::Component + '_ {
    rusti! {
        <div class="greeting">
            <h1>Hello, { name }!</h1>
            <p>Welcome to Rusti</p>
        </div>
    }
}
```

### 3. Render to String

```rust
use rusti::Component;

fn main() {
    let html = greeting("Alice").render_to_string();
    println!("{}", html);
    // Output: <div class="greeting"><h1>Hello, Alice!</h1><p>Welcome to Rusti</p></div>
}
```

## Integration with Web Frameworks

### Axum Example

```rust
use axum::{response::Html, routing::get, Router};
use rusti::rusti;

async fn index() -> Html<String> {
    let page = rusti! {
        <html>
            <head>
                <title>My App</title>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="bg-gray-100">
                <h1 class="text-3xl font-bold">Welcome!</h1>
            </body>
        </html>
    };
    Html(rusti::Component::render_to_string(&page))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Actix-web Example

```rust
use actix_web::{get, App, HttpResponse, HttpServer};
use rusti::rusti;

#[get("/")]
async fn index() -> HttpResponse {
    let page = rusti! {
        <html>
            <body><h1>Hello from Actix!</h1></body>
        </html>
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rusti::Component::render_to_string(&page))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```

## Key Features

### ✅ Type-Safe Templates
All variables are checked at compile time:
```rust
fn user_card(name: &str, age: i32) -> impl rusti::Component + '_ {
    rusti! {
        <div class="user-card">
            <h2>{ name }</h2>
            <p>Age: { age }</p>
        </div>
    }
}
```

### ✅ Component Composition
Nest components using the `@` syntax:
```rust
fn page(title: &str, content: impl rusti::Component) -> impl rusti::Component + '_ {
    rusti! {
        <html>
            <head><title>{ title }</title></head>
            <body>
                @header(title)
                <main>@content</main>
                @footer()
            </body>
        </html>
    }
}
```

### ✅ Automatic XSS Protection
All dynamic content is HTML-escaped by default:
```rust
let unsafe_input = "<script>alert('xss')</script>";
let safe_html = rusti! {
    <div>{ unsafe_input }</div>
};
// Renders as: <div>&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;</div>
```

### ✅ Conditional Rendering
Use standard Rust `if/else` and `match`:
```rust
fn status_badge(status: &str) -> impl rusti::Component + '_ {
    let (color, text) = match status {
        "active" => ("green", "Active"),
        "pending" => ("yellow", "Pending"),
        _ => ("gray", "Inactive"),
    };
    
    rusti! {
        <span class={"badge-{color}"}>{ text }</span>
    }
}
```

### ✅ Tailwind CSS Support
Full attribute support enables Tailwind classes:
```rust
rusti! {
    <div class="flex items-center gap-4 p-6 bg-white rounded-lg shadow-xl">
        <h1 class="text-3xl font-bold text-blue-600">Styled!</h1>
    </div>
}
```

## Project Structure

When using Rusti in your project:

```
your-project/
├── Cargo.toml          # Add: rusti = { path = "../rusti" }
├── src/
│   ├── main.rs         # Use rusti! macro
│   └── components.rs   # Define reusable components
```

## Common Patterns

### Layouts
```rust
fn layout(title: &str, body: impl rusti::Component) -> impl rusti::Component + '_ {
    rusti! {
        <html>
            <head>
                <title>{ title }</title>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="min-h-screen bg-gray-50">
                @navbar()
                <main class="container mx-auto p-4">
                    @body
                </main>
                @footer()
            </body>
        </html>
    }
}
```

### Lists (using standard Rust)
```rust
fn user_list(users: &[User]) -> impl rusti::Component + '_ {
    let items: Vec<_> = users
        .iter()
        .map(|u| user_card(&u.name, u.age))
        .collect();
    
    rusti! {
        <ul class="space-y-4">
            {for item in items { @item }}
        </ul>
    }
}
```

## Known Limitations

See [LIMITATIONS.md](../LIMITATIONS.md) for current parser limitations.

## Need Help?

- Check the [demo](../demo/src/main.rs) directory for working examples
- Read the [README](../README.md) for library overview
- Report issues on GitHub
