# Rusti 🦀
**The Most Robust HTML Parser for Rust** - Type-Safe, Feature-Rich, Production-Ready

Rusti is a powerful, zero-cost, type-safe HTML templating engine for Rust. It allows you to write HTML-like syntax directly in your Rust code using the `rusti!` macro.

Inspired by Go's `templ` library, Rusti brings the component model to server-side Rust. Because it compiles to native Rust code at build time, it delivers blazing fast performance with full type safety.

## 🚀 Key Features

- **Type-Safe**: All variables and expressions are checked at compile time
- **Zero-Cost Abstraction**: Compiles directly to `std::fmt::Write` calls - no runtime parsing, no virtual DOM
- **Rust-Native Control Flow**: Use `@if`, `@for`, `@match`, and `@let` directly in your templates
- **Component Composition**: Build complex UIs from small, reusable Rust functions
  - 🆕 **Optional Props with Defaults**: Use the Builder pattern with `#[prop(default = "...")]`
  - 🆕 **Typed Children**: Components can accept child content with `children: impl Component`
- **Namespaced Attributes**: Full support for HTMX (`hx-on:click`), Vue (`v-bind:class`), XML (`xml:lang`), and other modern frameworks
- **Automatic XSS Protection**: All dynamic content is HTML-escaped by default
- **External Styles**: Include CSS files at compile time with `<style src="path/to/file.css" />`
- **Framework Agnostic**: Works with Axum, Actix-web, Rocket, or any Rust program
- **Multiple Styling Strategies**: Choose between Tailwind, inline styles, raw CSS, or syntax-highlighted CSS blocks

---

## 📦 Installation

Add `rusti` to your `Cargo.toml`:

```toml
[dependencies]
rusti = { git = "https://github.com/DraconDev/rusti" }
```

> **📊 How does Rusti compare to Maud, Askama, or Leptos?**  
> See our [detailed competitive analysis](#-competitive-analysis) for a technical breakdown of trade-offs, performance, and design philosophy.

---

## 📝 Text and Quotes

### Automatic Quote Stripping

Rusti automatically strips outer quotes from string literals in text positions for cleaner rendering:

```rust
rusti! {
    <h1>"Hello, World!"</h1>  // Renders as: Hello, World! (quotes removed)
    <h1>Hello, World!</h1>     // Also renders as: Hello, World!
}
```

Both syntaxes produce the same output. The quotes are removed automatically to prevent `"Hello"` from appearing with visible quote marks in your HTML.

### When You Need Literal Quotes

If you actually want to display quote marks, use raw strings:

```rust
rusti! {
    <p>{r#""This will show quotes""#}</p>  // Renders as: "This will show quotes"
    <blockquote>"Said the wise person"</blockquote>  // Use <q> or entities for semantic quotes
}
```

For semantic quotations, prefer HTML elements or entities:
- **`<q>`** for inline quotes: `<q>Quoted text</q>` → "Quoted text"
- **`<blockquote>`** for block quotes
- **HTML entities**: `&ldquo;` (") and `&rdquo;` (")

---

## 🎯 Quick Start

### Basic Component

```rust
use rusti::rusti;

fn hello_world() -> impl rusti::Component {
    rusti! {
        <div class="container mx-auto p-4">
            <h1 class="text-2xl font-bold">Hello, World!</h1>
        </div>
    }
}
```

### Component with Props (Named Arguments)

```rust
use rusti::component;

#[component]
fn card(title: String, #[prop(default = "false")] is_highlighted: bool) -> impl rusti::Component {
    rusti! {
        <div class={ if is_highlighted { "card highlighted" } else { "card" } }>
            <h2>{title}</h2>
        </div>
    }
}

// Usage - optional props can be omitted!
fn page() -> impl rusti::Component {
    rusti! {
        <div>
            @card(title = "My Card".to_string())
            @card(title = "Important!".to_string(), is_highlighted = true)
        </div>
    }
}
```

### Components with Children

```rust
#[component]
fn layout(title: String, children: impl rusti::Component) -> impl rusti::Component {
    rusti! {
        <div class="layout">
            <header><h1>{title}</h1></header>
            <main>
                @children
            </main>
        </div>
    }
}

// Usage - pass JSX-like children!
fn page() -> impl rusti::Component {
    rusti! {
        @layout(title = "My Page".to_string()) {
            <p>This is the page content</p>
            <button>Click me</button>
        }
    }
}
```

### HTMX Integration (Namespaced Attributes)

```rust
fn search_box() -> impl rusti::Component {
    rusti! {
        <input 
            type="search"
            name="q"
            hx-get="/search"
            hx-trigger="keyup changed delay:500ms"
            hx-target="#results"
            hx-on:htmx:after-request="console.log('Done!')"
        />
        <div id="results"></div>
    }
}
```

---

## 🧩 Component Patterns

### 1. Simple Functions (Positional Arguments)

```rust
fn button(label: &str, class: &str) -> impl rusti::Component + '_ {
    rusti! {
        <button class={class}>{ label }</button>
    }
}

fn page() -> impl rusti::Component {
    rusti! {
        <div>
            @button("Submit", "btn-primary")
            @button("Cancel", "btn-secondary")
        </div>
    }
}
```

### 2. The `#[component]` Macro (Named Arguments & Builder Pattern)

For complex components with multiple props, use the `#[component]` attribute. This generates a typed `Props` struct with a builder pattern, enabling:
- **Named arguments** for clarity
- **Optional props** with default values
- **Type safety** at compile time

```rust
use rusti::component;

#[component]
fn alert_box(
    message: String, 
    #[prop(default = "false")] is_error: bool,
    #[prop(default = "\"Alert\"".to_string())] title: String
) -> impl rusti::Component {
    let bg_color = if is_error { "bg-red-500" } else { "bg-blue-500" };
    
    rusti! {
        <div class={bg_color}>
            <h3>{title}</h3>
            <p>{message}</p>
        </div>
    }
}

fn page() -> impl rusti::Component {
    rusti! {
        <div>
            {/* All props specified */}
            @alert_box(
                message = "Operation failed!".to_string(),
                is_error = true,
                title = "Error".to_string()
            )
            
            {/* Using defaults */}
            @alert_box(message = "Info message".to_string())
        </div>
    }
}
```

### 3. Typed Children (Component Composition)

Components can accept children, enabling powerful composition patterns:

```rust
#[component]
fn card(title: String, children: impl rusti::Component) -> impl rusti::Component {
    rusti! {
        <div class="card border rounded shadow p-4">
            <h2 class="text-xl font-bold">{title}</h2>
            <div class="card-body">
                @children
            </div>
        </div>
    }
}

#[component]
fn button_group(children: impl rusti::Component) -> impl rusti::Component {
    rusti! {
        <div class="flex gap-2">
            @children
        </div>
    }
}

fn dashboard() -> impl rusti::Component {
    rusti! {
        @card(title = "User Actions".to_string()) {
            @button_group {
                <button class="btn-primary">Save</button>
                <button class="btn-secondary">Cancel</button>
                <button class="btn-danger">Delete</button>
            }
        }
    }
}
```

---

## 🎨 Control Flow

Rusti provides first-class support for control flow using `@`:

```rust
fn user_dashboard(users: Vec<&str>, role: Option<&str>) -> impl rusti::Component + '_ {
    rusti! {
        <div>
            {/* Pattern Matching */}
            @match role {
                Some("admin") => { <button>Admin Panel</button> }
                Some("moderator") => { <button>Moderate</button> }
                Some(_) => { <span>Read Only</span> }
                None => { <a href="/login">Log in</a> }
            }

            {/* Loops */}
            <ul>
                @for user in users.iter() {
                    <li>{ user }</li>
                }
            </ul>

            {/* Conditionals */}
            @if users.is_empty() {
                <p>No users found.</p>
            } else {
                <p>Found { users.len() } users.</p>
            }
        </div>
    }
}
```

---

## 📝 Variable Declarations with `@let`

Declare scoped variables directly in your templates using `@let`:

```rust
fn calculator() -> impl rusti::Component {
    rusti! {
        <div>
            @let x = 10;
            @let y = 20;
            @let sum = x + y;
            
            <p>{x} + {y} = {sum}</p>
            
            @let greeting = "Hello";
            @let name = "Rustacean";
            
            <h1>{greeting}, {name}!</h1>
        </div>
    }
}
```

**Key Points:**
- Variables are scoped from the point of declaration onward
- Full type inference - values can be primitives, strings, or complex expressions
- Computed values work: `@let total = items.iter().sum()`

---

## 🎭 Inline Scripts and Styles

### ✅ JavaScript in `<script>` Tags

You can write standard JavaScript directly in `<script>` tags:

```rust
rusti! {
    <script>
        const app = {
            count: 0,
            increment() {
                this.count++;
                console.log("Count:", this.count);
            }
        };
        
        document.addEventListener("DOMContentLoaded", () => {
            console.log("App initialized");
        });
    </script>
}
```

**Standard JavaScript works perfectly** - objects, arrays, functions, etc.

### ✅ CSS in `<style>` Tags

Write standard CSS directly in `<style>` tags:

```rust
rusti! {
    <style>
        .card {
            padding: "2em";       /* ✅ Use quoted strings for integer+unit */
            margin: 3rem;         /* ✅ Most units work fine */
            background: #f0f0f0;
            border-radius: 8px;
        }
        
        .button {
            font-size: 16px;      /* ✅ px units work fine */
            color: #007bff;
            padding: 0.5rem 1rem; /* ✅ Decimal units work fine */
        }
        
        @media (min-width: 768px) {
            .card {
                max-width: 1200px;
            }
        }
    </style>
}
```

**⚠️ Important CSS Unit Note:**

Most CSS units work fine, but in rare cases where Rust's lexer has issues:
- ⚠️ `padding: 2em;` — May cause lexer issues
- ✅ `padding: "2em";` — Use quoted string if needed
- ✅ `padding: 3rem;` — Most units work fine as-is
- ✅ `padding: 0.5em;` — Decimal units work fine
- ✅ `padding: 16px;` — px and other units work fine

**Important**: Scripts must use **double quotes only** (`""`), never single quotes (`''`).

---

## 🌐 Modern Web Framework Support

### HTMX Integration

Rusti has full support for HTMX attributes, including event handlers and namespaced attributes:

```rust
fn live_search() -> impl rusti::Component {
    rusti! {
        <div>
            <input 
                type="search"
                name="q"
                placeholder="Search..."
                hx-get="/search"
                hx-trigger="keyup changed delay:300ms"
                hx-target="#search-results"
                hx-indicator="#spinner"
                hx-on:htmx:before-request="console.log('Searching...')"
            />
            <div id="spinner" class="htmx-indicator">Loading...</div>
            <div id="search-results"></div>
        </div>
    }
}

fn delete_button(id: i32) -> impl rusti::Component {
    let confirm_msg = format!("Are you sure you want to delete item {}?", id);
    
    rusti! {
        <button
            hx-delete={format!("/api/items/{}", id)}
            hx-confirm={confirm_msg}
            hx-on:htmx:after-request="alert('Deleted!')"
            class="btn-danger"
        >
            Delete
        </button>
    }
}
```

### Vue-style Attributes

```rust
fn vue_component(is_active: bool) -> impl rusti::Component {
    rusti! {
        <div 
            v-bind:class={ if is_active { "active" } else { "" } }
            v-on:click="handleClick"
        >
            Click me
        </div>
    }
}
```

---

## 💅 Styling Strategies

### Option A: Tailwind CSS (Recommended) 🌊

```rust
rusti! {
    <div class="p-8 m-4 bg-gradient-to-r from-blue-500 to-purple-600 text-white rounded-lg shadow-2xl">
        <h1 class="text-4xl font-bold">Tailwind works perfectly.</h1>
        <p class="mt-2 text-gray-100">No compiler errors, ever.</p>
    </div>
}
```

### Option B: Inline Styles ✅

```rust
rusti! {
    <div style="margin: 2em; color: #ff0000; font-size: 16px;">
        Inline styles are bulletproof.
    </div>
}
```

### Option C: External Stylesheets 📋

Include external CSS files at compile time:

```rust
rusti! {
    <style src="styles/main.css" />
    <div class="custom-component">
        Content styled by external CSS
    </div>
}
```

### Option D: Raw String CSS

```rust
rusti! {
    <style>{r#"
        .container {
            background-color: #f4f4f4;
            padding: 2em;
            border-radius: 8px;
        }
        
        @media (min-width: 768px) {
            .container {
                max-width: 1200px;
            }
        }
    "#}</style>
}
```

---

## ⚠️ Syntax Rules

### The Golden Rules

1. **Use double quotes** for attributes: `class="foo"` (not `class='foo'`)
2. **Scripts use double quotes only**: Always `""`, never `''` in `<script>` tags
3. **Emojis in variables**: `let text = "Hello ✅"; rusti! { <p>{text}</p> }`
4. **Use Tailwind or inline styles** to avoid CSS headaches
5. **External styles**: Use `<style src="path/to/style.css" />` for external CSS
6. **Inline CSS**: Standard CSS works fine; use `"2em"` (quoted) if lexer complains
7. **Inline JavaScript**: Standard JS works perfectly in `<script>` tags

### CSS Unit Rules

When writing CSS directly in `<style>` tags, most units work fine:
- ✅ `padding: 3rem;` - Works fine
- ✅ `padding: 0.5em;` - Decimals work fine
- ✅ `padding: 16px;` - px, %, and other units work fine
- ⚠️ `padding: 2em;` - Rarely, may cause lexer issues
- ✅ `padding: "2em";` - Use quoted string if needed

**💡 Simplicity Recommendation:**

For the easiest development experience, **avoid using em units and hex colors without quotes** in inline CSS:
- ❌ `padding: 2em;` — May cause lexer issues
- ❌ `color: #fff;` — May be interpreted as Rust syntax if contains e
- ✅ `padding: "2em";` — Quoted em units work perfectly
- ✅ `color: "#fff";` — Quoted hex colors work perfectly
- ✅ `padding: 2rem;` — rem units typically work as-is
- ✅ `padding: 16px;` — px and most other units work as-is

**Best Practice:** Use Tailwind CSS (recommended) or external stylesheets to completely avoid these edge cases.

### Quick Translation Guide (JS/HTML → Rust)

| Feature | JavaScript / HTML | Rusti (Rust) |
|---------|------------------|--------------|
| **Quotes** | `<div id='app'>` | `<div id="app">` |
| **Text Content** | `<h1>Hello</h1>` | `<h1>Hello</h1>` or `<h1>"Hello"</h1>` (quotes auto-stripped) |
| **Literal Quotes** | `<p>"Hello"</p>` | `<p>{r#""Hello""#}</p>` (use raw string for literal quotes) |
| **Templates** | `` `Hello ${name}` `` | `r#"Hello {name}"#` |
| **URL** | `href=http://...` | `href="http://..."` |
| **Variables** | `id="${id}"` | `id={id}` |
| **Control** | `{% if %}` | `@if` |
| **Loops** | `{% for %}` | `@for` |
| **Variables** | `{% let %}` | `@let` |
| **JSON** | `data='{"x":1}'` | `data=r#"{"x":1}"#` |
| **HTMX Events** | `hx-on::click` | `hx-on:click` |
| **Inline Script** | `<script>...</script>` | `<script>...</script>` (standard JS works!) |
| **Inline Style** | `<style>...</style>` | `<style>...</style>` (standard CSS works!) |

---

## 🌐 Web Framework Integration

### Axum Example

```rust
use axum::{response::{Html, IntoResponse}, routing::get, Router};
use rusti::rusti;

async fn handler() -> impl IntoResponse {
    let html = rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <title>Rusti + Axum</title>
                <script src="https://cdn.tailwindcss.com"></script>
                <script src="https://unpkg.com/htmx.org@1.9.10"></script>
            </head>
            <body class="bg-gray-100 p-8">
                <h1 class="text-4xl font-bold text-indigo-600">
                    Hello from Axum + Rusti!
                </h1>
                <button 
                    hx-get="/api/data"
                    hx-target="#result"
                    class="mt-4 px-4 py-2 bg-blue-500 text-white rounded"
                >
                    Load Data
                </button>
                <div id="result"></div>
            </body>
        </html>
    };
    Html(rusti::render_to_string(&html))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
```

---

## 🎯 Best Practices

1. **Use Tailwind CSS** for styling whenever possible
2. **Use the `#[component]` macro** for components with multiple props
3. **Leverage optional props** to make components flexible without boilerplate
4. **Use typed children** for layout components and wrappers
5. **Use `@let` for computed values** - Keep template logic readable with intermediate variables
6. **For inline `<script>` tags**: 
   - ✅ Standard JavaScript works perfectly - objects, arrays, functions, etc.
   - ⚠️ Always use double quotes (`""`), never single quotes (`''`)
7. **For inline `<style>` tags**: 
   - ✅ Standard CSS works fine
   - ⚠️ If you encounter lexer issues with units like `2em`, use `"2em"` (quoted)
   - ✅ Most units work as-is: `3rem`, `0.5em`, `16px`
8. **Link external scripts** (`<script src="...">`) for larger JavaScript files
9. **Extract components early** to keep templates readable
10. **Leverage Rust's type system** - pass typed structs instead of primitives
11. **Test component rendering** with unit tests using `render_to_string()`

---

## 🧠 Lifetime Management

Rusti components follow standard Rust lifetime rules:

### Borrowed Data (The `'_` Pattern) - Recommended

```rust
fn user_card(name: &str) -> impl rusti::Component + '_ {
    rusti! { <div>{name}</div> }
}
```

### Owned Data (No Lifetimes)

```rust
fn counter(count: i32) -> impl rusti::Component {
    rusti! { <div>Count: {count}</div> }
}
```

---

## 📊 Competitive Analysis

| Feature | Rusti | Maud | Askama | Leptos |
|---------|-------|------|--------|--------|
| **Syntax** | HTML-like | Rust DSL | Jinja2-like | JSX-like |
| **Type Safety** | ✅ Full | ✅ Full | ✅ Full | ✅ Full |
| **Runtime Cost** | ✅ Zero | ✅ Zero | ✅ Zero | ⚠️ Virtual DOM |
| **Optional Props** | ✅ Builder Pattern | ❌ | ❌ | ✅ |
| **Typed Children** | ✅ Native | ❌ | ❌ | ✅ |
| **HTMX Support** | ✅ Namespaced Attrs | ⚠️ Manual | ⚠️ Manual | ❌ |
| **Learning Curve** | Low (HTML) | Medium | Low | Medium |

### Why Rusti?

- **vs Maud**: Rusti uses actual HTML syntax, making it easier to copy-paste from UI kits and work with designers
- **vs Askama**: Rusti components are Rust functions with full composition support, no external files needed
- **vs Leptos**: Rusti is lighter weight (no WASM/Signals overhead), perfect for server-side rendering with HTMX

---

## 📚 More Examples

Check out **[EXAMPLES.md](EXAMPLES.md)** and the **[demo/](demo/)** directory for:
- HTMX Integration patterns
- Complex forms with validation
- Layouts & nesting
- Dynamic styling
- External CSS integration

---

## 🛠️ Project Structure

- **`src/`**: Runtime library (Component trait, escaping logic)
- **`macros/`**: Procedural macro implementation (Parser, Code Generator)
- **`demo/`**: Complete example application using Axum, HTMX, and Tailwind CSS

---

## 📄 License

MIT License

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

## 🔗 Resources

- [Demo Application](demo/) - Full-featured examples
- [GitHub Repository](https://github.com/DraconDev/rusti)
- [Issue Tracker](https://github.com/DraconDev/rusti/issues)