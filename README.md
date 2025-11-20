# Rusti 🦀

**A Type-Safe, Rust-Native HTML Templating Library**

Rusti is a powerful, zero-cost, type-safe HTML templating engine for Rust. It allows you to write HTML-like syntax directly in your Rust code using the `rusti!` macro. It compiles your templates into efficient Rust code at compile time, ensuring type safety, automatic XSS protection, and blazing fast performance.

Inspired by Go's `templ` library, Rusti brings the component model to server-side Rust.

---

## 🚀 Key Features

- **Type-Safe**: All variables and expressions are checked at compile time. If it compiles, it works.
- **Zero-Cost Abstraction**: Templates compile directly to `std::fmt::Write` calls. No runtime parsing, no virtual DOM, no overhead.
- **Rust-Native Control Flow**: Use `@if`, `@for`, and `@match` directly in your templates.
- **Component Composition**: Build complex UIs from small, reusable, testable components.
- **Automatic XSS Protection**: All dynamic content is HTML-escaped by default.
- **Framework Agnostic**: Works with Axum, Actix-web, Rocket, or any Rust program.
- **Tailwind CSS Ready**: Full support for standard HTML attributes and classes.

---

## 📦 Installation

Add `rusti` to your `Cargo.toml`.

**From a Local Path (if cloning the repo):**
```toml
[dependencies]
rusti = { path = "path/to/rusti" }
```

**From Git:**
```toml
[dependencies]
rusti = { git = "https://github.com/DraconDev/rusti" }
```

---

## 📖 Usage Guide

### 1. The `rusti!` Macro

The core of the library is the `rusti!` macro. It takes HTML-like syntax and returns a closure that implements `rusti::Component`.

```rust
use rusti::rusti;

fn hello_world() -> impl rusti::Component {
    rusti! {
        <div>
            <h1>Hello, World!</h1>
        </div>
    }
}
```

### 2. Rendering to String

To render a component, use the `.render_to_string()` method.

```rust
fn main() {
    let component = hello_world();
    println!("{}", component.render_to_string());
}
```

### 3. Dynamic Content

Inject Rust variables and expressions using curly braces `{ }`. All content is automatically escaped to prevent XSS.

```rust
fn greeting(name: &str, count: i32) -> impl rusti::Component + '_ {
    rusti! {
        <p>
            Hello, { name }! You have { count } messages.
        </p>
    }
}
```

### 4. Attributes

#### Static Attributes
Standard HTML attributes work as expected.

```rust
rusti! {
    <div class="container p-4" id="main-content">
        Content
    </div>
}
```

#### Dynamic Attributes
Use `{ }` to pass Rust values to attributes.

```rust
let is_disabled = true;
let image_url = "/profile.png";

rusti! {
    <button disabled={is_disabled}>Submit</button>
    <img src={image_url} />
}
```

### 5. Components & Composition

Components are just Rust functions that return `impl rusti::Component`. You can nest them using the `@` syntax.

```rust
// Define a reusable button component
fn my_button(text: &str) -> impl rusti::Component + '_ {
    rusti! {
        <button class="btn-primary">{ text }</button>
    }
}

// Use it in a page
fn home_page() -> impl rusti::Component {
    rusti! {
        <main>
            <h1>Welcome</h1>
            <!-- Call the component with arguments -->
            @my_button("Click Me")
            @my_button("Sign Up")
        </main>
    }
}
```

### 6. Control Flow

Rusti provides first-class support for control flow directly in your templates.

#### `@if` / `@else`
Conditionally render content.

```rust
fn user_badge(logged_in: bool, name: &str) -> impl rusti::Component + '_ {
    rusti! {
        <div class="nav">
            @if logged_in {
                <span>Welcome, { name }</span>
                <a href="/logout">Logout</a>
            } @else {
                <a href="/login">Login</a>
            }
        </div>
    }
}
```

#### `@for` Loops
Iterate over any Rust iterator.

```rust
fn todo_list(items: &[&str]) -> impl rusti::Component + '_ {
    rusti! {
        <ul>
            @for item in items {
                <li>{ item }</li>
            }
        </ul>
    }
}
```

#### `@match` Expressions
Pattern matching for complex logic.

```rust
enum Status {
    Active,
    Pending,
    Suspended,
}

fn status_indicator(status: Status) -> impl rusti::Component {
    rusti! {
        @match status {
            Status::Active => { <span class="green">Active</span> }
            Status::Pending => { <span class="yellow">Pending...</span> }
            Status::Suspended => { <span class="red">Suspended</span> }
        }
    }
}
```

---

## 🌐 Web Framework Integration

Rusti integrates easily with any web framework.

### Axum Example

```rust
use axum::{response::Html, routing::get, Router};
use rusti::rusti;

async fn handler() -> Html<String> {
    let component = rusti! {
        <h1>Hello from Axum!</h1>
    };
    Html(component.render_to_string())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

---

## ⚠️ Known Limitations & Gotchas

1.  **Script & Style Tags**:
    *   The parser can struggle with complex JS/CSS inside `<script>` or `<style>` tags because it might mistake content for Rust syntax.
    *   **Workaround**: Use external files (`<script src="...">`) or use spaced closing tags like `< / script >` if you must inline.

2.  **HTML Comments**:
    *   Standard HTML comments `<!-- ... -->` are not currently supported and will cause parse errors. Use standard Rust comments `//` outside the macro or rely on the fact that the macro is Rust code.

3.  **Rust-Analyzer Errors**:
    *   You might see "Unexpected input remaining" errors in your IDE. These are often cosmetic issues with how `rust-analyzer` expands macros. If `cargo build` passes, your code is correct.

---

## 🛠️ Project Structure

- `src/`: Runtime library (Component trait, escaping logic).
- `macros/`: The procedural macro implementation (Parser, Code Generator).
- `demo/`: A complete example application using Axum, HTMX, and Tailwind CSS.

---

## 📄 License

MIT License
