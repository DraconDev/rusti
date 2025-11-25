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

Most CSS units work fine, but **avoid `em` units and hex colors containing `e`** in inline styles - Rust's lexer may interpret `e` as scientific notation:
- ⚠️ `padding: 2em;` — May cause lexer issues (2e interpreted as number)
- ⚠️ `color: #2e2e2e;` — May cause lexer issues (contains 'e')
- ✅ `padding: "2em";` — Quote the entire value to avoid parsing issues
- ✅ `color: "#2e2e2e";` — Quote hex colors with 'e'
- ✅ `padding: 2rem;` — Use `rem` instead of `em`
- ✅ `padding: 32px;` — Use `px` or other units
- ✅ `color: rgb(46, 46, 46);` — Use `rgb()` instead of hex

**Best Practice**: Use Tailwind CSS (recommended) or external stylesheets to completely avoid these edge cases.

**Important**: Scripts must use **double quotes only** (`""`), never single quotes (`''`).

---

## 🎬 Script Variable Injection

Rusti supports **dynamic variable injection** into `<script>` tags using the `@{ }` syntax. This allows you to safely pass Rust values into your JavaScript code at compile time.

### ✅ Basic Variable Injection

```rust
fn counter_app() -> impl rusti::Component {
    let count = 42;
    let max_count = 100;
    
    rusti! {
        <script>
            const currentCount = @{ count };
            const maxCount = @{ max_count };
            
            console.log("Count:", currentCount);
            console.log("Max:", maxCount);
        </script>
    }
}
```

**Output JavaScript:**
```javascript
const currentCount = 42;
const maxCount = 100;
```

### 📝 String Injection

String variables require special handling - they must be converted to `String` type:

```rust
fn app() -> impl rusti::Component {
    rusti! {
        <script>
            // ✅ Correct - use @let to create String in template scope
            @let message = "Hello, World!".to_string();
            const msg = @{ message };
            
            // ✅ Also correct - format! creates String
            @let user = format!("User_{}", 123);
            const username = @{ user };
            
            console.log(msg, username);
        </script>
    }
}
```

**Output JavaScript:**
```javascript
const msg = "Hello, World!";
const username = "User_123";
```

### 🔄 Control Flow in Scripts

You can use Rusti's control flow (`@if`, `@for`, `@match`, `@let`) **inside** `<script>` tags:

```rust
fn dynamic_script() -> impl rusti::Component {
    let items = vec!["apple", "banana", "cherry"];
    let debug_mode = true;
    
    rusti! {
        <script>
            const items = [];
            
            @for item in &items {
                items.push(@{ item });
            }
            
            @if debug_mode {
                console.log("Debug: Items loaded:", items);
            }
            
            @let total = items.len();
            console.log("Total items:", @{ total });
        </script>
    }
}
```

### ⚠️ How It Works: Debug Formatting

Variables injected with `@{ }` are formatted using Rust's `Debug` trait (`{:?}`):
- **Numbers**: `42` → `42`
- **Strings**: `"hello".to_string()` → `"hello"` (with quotes)
- **Booleans**: `true` → `true`
- **Arrays/Vecs**: `vec![1,2,3]` → `[1, 2, 3]`

This is why **strings must be `String` type** - they need Debug formatting to include quotes in the JavaScript output.

### 🚫 Common Pitfalls

```rust
// ❌ WRONG - &str doesn't have the right Debug format
let msg = "Hello";
// <script>const x = @{ msg };</script>
// Result: const x = Hello; // ← Syntax error! Missing quotes

// ✅ CORRECT - String adds quotes via Debug
let msg = "Hello".to_string();
// <script>const x = @{ msg };</script>
// Result: const x = "Hello"; // ✓ Valid JavaScript

// ❌ WRONG - Cannot inject complex Rust structs directly
struct User { name: String }
let user = User { name: "Alice".to_string() };
// <script>const u = @{ user };</script>
// Result: const u = User { name: "Alice" }; // ← Invalid JS!

// ✅ CORRECT - Serialize to JSON or extract fields
let user_json = serde_json::to_string(&user).unwrap();
// <script>const u = @{ user_json };</script>
```

### 💡 Best Practice: Use HTMX Instead

For most interactive UIs, **use HTMX** instead of client-side JavaScript:

```rust
// ❌ Avoid: Complex client-side JS
// <script>
//     @for item in &items {
//         document.createElement("div").innerHTML = @{ item };
//     }
// </script>

// ✅ Prefer: Server-side rendering with HTMX
rusti! {
    <div hx-get="/api/items" hx-trigger="load">
        @for item in &items {
            <div>{item}</div>
        }
    </div>
}
```

**Why?**
- Server-side rendering is faster and more reliable
- Better SEO and accessibility
- Type-safe Rust code instead of stringly-typed JavaScript
- Automatic XSS protection

---

## 🎨 Scoped CSS

Rusti automatically scopes CSS when you include `<style>` tags as **direct children** of an element. This prevents style pollution and naming conflicts.

### How It Works

```rust
fn scoped_card() -> impl rusti::Component {
    rusti! {
        <div>
            <style>
                .card {
                    padding: 2em;
                    background: #f0f0f0;
                    border-radius: 8px;
                }
                .title {
                    font-size: 1.5em;
                    font-weight: bold;
                }
            </style>
            
            <div class="card">
                <h2 class="title">Scoped Card</h2>
                <p>These styles only affect this component!</p>
            </div>
        </div>
    }
}
```

**Generated HTML:**
```html
<div>
    <style data-scope="s0">
        .card[data-s0] { padding: 2em; background: #f0f0f0; border-radius: 8px; }
        .title[data-s0] { font-size: 1.5em; font-weight: bold; }
    </style>
    <div class="card" data-s0>
        <h2 class="title" data-s0>Scoped Card</h2>
        <p data-s0>These styles only affect this component!</p>
    </div>
</div>
```

### Key Features

- **Automatic Scoping**: Rusti adds unique `data-scope` attributes to both styles and elements
- **Selector Transformation**: All CSS selectors get the scope attribute appended
- **No Global Pollution**: Styles cannot leak to other components
- **Zero Configuration**: Just nest `<style>` as a direct child

### Multiple Components

Each component gets its own unique scope:

```rust
fn app() -> impl rusti::Component {
    rusti! {
        <div>
            @scoped_card()  // Gets scope "s0"
            @scoped_card()  // Gets scope "s1" - completely isolated!
        </div>
    }
}
```

### When NOT to Use Scoped CSS

- **Global styles** (use external stylesheet)
- **Tailwind CSS** (already scoped by design)
- **Simple components** (inline styles may be clearer)

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

When writing CSS directly in `<style>` tags, **avoid `em` units and hex colors containing `e`** as Rust's lexer may interpret `e` as scientific notation:

**Units:**
- ✅ `padding: 2rem;` - `rem` works fine
- ✅ `padding: 32px;` - `px`, `%`, and most other units work fine
- ⚠️ `padding: 2em;` - May cause "invalid number" error (2e = scientific notation)
- ✅ `padding: "2em";` - Quote if you must use `em`

**Colors:**
- ✅ `color: #fff;` - Hex colors without 'e' work fine
- ✅ `color: #2d3748;` - Works fine ('d' is not 'e')
- ⚠️ `color: #2e2e2e;` - May cause lexer issues (contains 'e')
- ✅ `color: "#2e2e2e";` - Quote hex colors with 'e'
- ✅ `color: rgb(46, 46, 46);` - Use `rgb()` instead

**💡 Simplicity Recommendation:**

For the easiest development experience, **use Tailwind CSS or external stylesheets** to completely avoid these edge cases. If you must write inline CSS:
1. **Use `rem` instead of `em`**
2. **Use `rgb()` instead of hex colors with `e`**
3. **Or quote all CSS values** if you prefer consistency

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

## 🧠 Design Decisions

Understanding Rusti's design philosophy helps you write idiomatic, maintainable code.

### Component Naming: PascalCase vs snake_case vs @component

Rusti supports **three component patterns**, each with different calling conventions:

#### 1. **PascalCase** → Builder Pattern (Complex Components)

Use `PascalCase` with the `#[component]` macro for components with multiple or optional props:

```rust
use rusti::component;

#[component]
fn Card(
    title: String,
    #[prop(default = "false")] highlighted: bool,
    #[prop(default = "\"\".to_string())] subtitle: String
) -> impl rusti::Component {
    rusti! {
        <div class={ if highlighted { "card highlighted" } else { "card" } }>
            <h2>{title}</h2>
            @if !subtitle.is_empty() {
                <p>{subtitle}</p>
            }
        </div>
    }
}

// Usage: Named arguments with builder pattern
rusti! {
    @Card(title = "Hello".to_string())
    @Card(title = "Featured".to_string(), highlighted = true)
    @Card(
        title = "Full".to_string(), 
        highlighted = true,
        subtitle = "With subtitle".to_string()
    )
}
```

**When to use:**
- Multiple props (3+)
- Optional props with defaults
- Complex configuration
- Follows Rust convention: PascalCase for types

#### 2. **snake_case** → Positional Arguments (Simple Components)

Use `snake_case` regular functions for simple components with 1-2 required props:

```rust
fn button(label: &str, class: &str) -> impl rusti::Component + '_ {
    rusti! {
        <button class={class}>{label}</button>
    }
}

// Usage: Positional arguments (like normal function calls)
rusti! {
    @button("Click me", "btn-primary")
    @button("Cancel", "btn-secondary")
}
```

**When to use:**
- 1-2 required props
- No optional props
- Simple, focused components
- Follows Rust convention: snake_case for functions

#### 3. **@component** → Pre-built Component Variable

Use `@component_var` to render a component variable:

```rust
fn app() -> impl rusti::Component {
    let my_header = header("My App");
    let my_footer = footer(2025);
    
    rusti! {
        <div>
            @my_header
            <main>Content here</main>
            @my_footer
        </div>
    }
}
```

**When to use:**
- Component is built conditionally
- Component is reused multiple times
- Component needs complex setup logic

### Why This Design?

**Rationale**: The naming convention automatically determines the calling syntax:
- **PascalCase** triggers builder pattern detection (line 404-410 in `macros/src/lib.rs`)
- **snake_case** uses direct function call
- **@variable** renders pre-built component

This leverages Rust's existing conventions and provides **compile-time type safety** for all three patterns.

---

### HTMX-First Philosophy

Rusti is designed for **server-side rendering with HTMX**, not heavy client-side JavaScript.

#### Why HTMX > Client JS?

```rust
// ❌ Client-Side Approach (More complex, less reliable)
rusti! {
    <div id="items"></div>
    <script>
        @let items = vec!["a", "b", "c"];
        const container = document.getElementById("items");
        
        @for item in &items {
            const div = document.createElement("div");
            div.textContent = @{ item };
            container.appendChild(div);
        }
    </script>
}

// ✅ Server-Side Approach (Simpler, faster, type-safe)
rusti! {
    <div hx-get="/api/items" hx-trigger="load" hx-swap="innerHTML">
        @for item in &items {
            <div>{item}</div>
        }
    </div>
}
```

**Benefits:**
1. **Type Safety**: Rust's type system vs stringly-typed JavaScript
2. **Performance**: Server renders once vs client manipulation
3. **SEO**: Content is in initial HTML
4. **Reliability**: No JS errors, no async race conditions
5. **Simplicity**: One language (Rust) instead of two (Rust + JS)

#### When to Use Client JS

Client-side JavaScript is appropriate for:
- **UI Interactions**: Dropdown menus, modals, tooltips
- **Real-time Features**: WebSocket updates, live notifications
- **Performance**: Very frequent updates (e.g., animations)

For these cases, use `@{ }` injection to pass server data to client:

```rust
rusti! {
    <script>
        @let api_key = env::var("API_KEY").unwrap();
        @let user_id = user.id;
        
        const ws = new WebSocket(`wss://api.example.com?key=${@{ api_key }}&user=${@{ user_id }}`);
    </script>
}
```

---

### Scoped CSS Philosophy

**Goal**: Component-level style isolation without configuration.

#### The Problem

Traditional CSS has global scope:

```css
/* component_a.css */
.button { background: blue; }

/* component_b.css */  
.button { background: red; } /* ← Conflicts with component_a! */
```

#### Rusti's Solution

Automatic scoping when `<style>` is a direct child:

```rust
fn component_a() -> impl rusti::Component {
    rusti! {
        <div>
            <style>.button { background: blue; }</style>
            <button class="button">Blue</button>
        </div>
    }
}

fn component_b() -> impl rusti::Component {
    rusti! {
        <div>
            <style>.button { background: red; }</style>
            <button class="button">Red</button>  
        </div>
    }
}
```

Both render correctly with **no conflicts**. Each gets a unique `data-scope` attribute.

#### Trade-offs

**Pros:**
- Zero configuration
- Complete isolation
- Works with any CSS

**Cons:**
- Slightly larger HTML (data attributes)
- Cannot share styles across components (use external CSS for that)
- Only works for direct children

**Recommendation**: Use scoped CSS for component-specific styles, external stylesheets for global/shared styles, and Tailwind for utility-first styling.

---

## 🎯 Best Practices

### 1. Component Design Patterns

**Choose the right component pattern for your use case:**

```rust
// ✅ PascalCase for complex components with optional props
#[component]
fn UserCard(
    name: String,
    #[prop(default = "\"\".to_string())] avatar_url: String,
    #[prop(default = "false")] is_verified: bool
) -> impl rusti::Component { /* ... */ }

// ✅ snake_case for simple, focused components
fn icon(name: &str, size: u32) -> impl rusti::Component { /* ... */ }

// ✅ @variable for conditional/reusable components
let header = if user.is_admin() {
    admin_header()
} else {
    user_header()
};
```

### 2. CSS Strategy Hierarchy

**Follow this priority order for styling:**

1. **Tailwind CSS** (Recommended) - Utility-first, no conflicts
   ```rust
   <div class="p-8 bg-gradient-to-r from-blue-500 to-purple-600">
   ```

2. **External Stylesheets** - Global/shared styles
   ```rust
   <style src="styles/global.css" />
   ```

3. **Scoped CSS** - Component-specific styles
   ```rust
   <div>
       <style>.card { padding: 2em; }</style>
       <div class="card">...</div>
   </div>
   ```

4. **Inline Styles** - One-off styling
   ```rust
   <div style="margin: 2em; color: #ff0000;">
   ```

### 3. Script Injection Guidelines

**Prefer HTMX over client-side JavaScript:**

```rust
// ❌ Avoid: Heavy client-side logic
<script>
    @for item in &items {
        // Complex DOM manipulation
    }
</script>

// ✅ Prefer: Server-side rendering with HTMX
<div hx-get="/api/items" hx-trigger="load">
    @for item in &items {
        <div>{item}</div>
    }
</div>
```

**When you must use `<script>`, remember:**
- Use `@let` to create `String` variables
- ` @{ }` uses Debug formatting
- Keep scripts minimal and focused

### 4. Component Composition

**Build complex UIs from small, reusable components:**

```rust
// ✅ Good: Small, focused components
#[component]
fn Page(title: String, children: impl rusti::Component) -> impl rusti::Component {
    rusti! {
        <div class="page">
            @header(title)
            @children
            @footer()
        </div>
    }
}

// ❌ Avoid: Monolithic components with everything inline
fn giant_page() -> impl rusti::Component {
    rusti! {
        <div>
            // 500 lines of HTML...
        </div>
    }
}
```

### 5. Type Safety

**Leverage Rust's type system:**

```rust
// ✅ Pass typed structs instead of primitives
struct User {
    name: String,
    role: Role,
}

fn user_profile(user: &User) -> impl rusti::Component { /* ... */ }

// ❌ Avoid: Primitive obsession
fn user_profile(name: &str, role: &str, id: i32) -> impl rusti::Component { /* ... */ }
```

### 6. Testing

**Test component rendering with unit tests:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_button_renders() {
        let html = rusti::render_to_string(&button("Click me", "btn-primary"));
        assert!(html.contains("Click me"));
        assert!(html.contains("btn-primary"));
    }
    
    #[test]
    fn test_conditional_rendering() {
        let html = rusti::render_to_string(&status_badge(true));
        assert!(html.contains("Active"));
    }
}
```

### 7. Performance

**Rusti is zero-cost, but follow these guidelines:**

- **Avoid unnecessary allocations**: Use `&str` when possible
  ```rust
  fn label(text: &str) -> impl rusti::Component + '_ { /* ... */ }
  ```

- **Use `impl rusti::Component`**: Enables return type optimization
  ```rust
  // ✅ Good
  fn card() -> impl rusti::Component { /* ... */ }
  
  // ❌ Avoid
  fn card() -> Box<dyn rusti::Component> { /* ... */ }
  ```

- **Render to string once**: Don't repeatedly render the same component
  ```rust
  // ✅ Good
  let html = rusti::render_to_string(&page());
  
  // ❌ Avoid in loops
  for _ in 0..100 {
      let html = rusti::render_to_string(&page()); // Wasteful!
  }
  ```

### 8. Error Handling

**Handle errors gracefully in components:**

```rust
fn user_profile(user_id: i32) -> impl rusti::Component {
    let user = match fetch_user(user_id) {
        Ok(u) => u,
        Err(_) => return rusti! {
            <div class="error">User not found</div>
        },
    };
    
    rusti! {
        <div class="profile">
            <h2>{user.name}</h2>
        </div>
    }
}
```

### 9. Accessibility

**Always include semantic HTML and ARIA attributes:**

```rust
fn dialog(title: &str) -> impl rusti::Component {
    rusti! {
        <div role="dialog" aria-labelledby="dialog-title">
            <h2 id="dialog-title">{title}</h2>
            <button aria-label="Close dialog">×</button>
        </div>
    }
}
```

### 10. Documentation

**Document complex components:**

```rust
/// Renders a paginated list with navigation controls.
///
/// # Arguments
/// * `items` - The items to display
/// * `page` - Current page number (1-indexed)
/// * `page_size` - Items per page
///
/// # Example
/// ```rust
/// let items = vec!["a", "b", "c"];
/// paginated_list(&items, 1, 10)
/// ```
fn paginated_list<T: Display>(
    items: &[T],
    page: usize,
    page_size: usize
) -> impl rusti::Component { /* ... */ }
```

---

## 🔧 Troubleshooting

### Script Injection Issues

#### Problem: "Uncaught SyntaxError: Invalid or unexpected token"

**Cause**: String variable not converted to `String` type.

```rust
// ❌ Wrong
let msg = "Hello";  // &str
<script>const x = @{ msg };</script>
// Output: const x = Hello; // ← Missing quotes!

// ✅ Fix
let msg = "Hello".to_string();  // String
<script>const x = @{ msg };</script>
// Output: const x = "Hello"; // ✓ Valid JS
```

**Solution**: Always use `.to_string()` or `format!()` for strings in scripts.

---

#### Problem: "Variable interpolation not working"

**Cause**: Using wrong syntax or context.

```rust
// ❌ Wrong: Using { } instead of @{ }
<script>const x = { my_var };</script>  // Treated as JS object!

// ✅ Fix: Use @{ }
<script>const x = @{ my_var };</script>

// ❌ Wrong: Using @{ } outside script tags
<div>@{ my_var }</div>  // Use { } in HTML!

// ✅ Fix: Use { } in HTML context
<div>{my_var}</div>
```

---

### CSS Unit Parsing

#### Problem: "2em" causes lexer error

**Cause**: Rust's lexer interprets `2em` as invalid syntax.

```rust
// ❌ Problematic
<style>
    .card { padding: 2em; }  // May cause lexer issues
</style>

// ✅ Solution 1: Add space (parser fixes it automatically)
<style>
    .card { padding: 2 em; }  // → Becomes "2em" in output
</style>

// ✅ Solution 2: Use quoted string
<style>
    .card { padding: "2em"; }  // → Becomes "2em" in output
</style>

// ✅ Solution 3: Use different units
<style>
    .card { padding: 2rem; }  // rem works fine
    .card { padding: 32px; }  // px works fine
</style>
```

---

### Lifetime Issues

#### Problem: "Borrowed value does not live long enough"

**Cause**: Borrowing temporary values.

```rust
// ❌ Wrong
fn card() -> impl rusti::Component + '_ {
    rusti! {
        <div>{format!("Hello")}</div>  // format!() creates temporary String
    }
}

// ✅ Fix 1: Use @let
fn card() -> impl rusti::Component {
    rusti! {
        @let greeting = format!("Hello");
        <div>{greeting}</div>
    }
}

// ✅ Fix 2: Remove lifetime bound if not borrowing
fn card() -> impl rusti::Component {  // No '_ needed
    rusti! {
        <div>{"Hello"}</div>
    }
}
```

---

### Quote Handling

#### Problem: Quotes appearing in output when they shouldn't

**Cause**: Rusti auto-strips outer quotes in text positions.

```rust
// Both produce: <h1>Hello</h1>
<h1>"Hello"</h1>
<h1>Hello</h1>

// To show literal quotes, use raw strings with { }
<p>{r#""This shows quotes""#}</p>  // → "This shows quotes"
```

---

### Component Not Found

#### Problem: `@MyComponent(...)` not working

**Cause**: Missing `#[component]` macro or typo in name.

```rust
// ❌ Wrong: Regular function with PascalCase name
fn MyComponent(title: String) -> impl rusti::Component { /* ... */ }

// Usage fails:
// @MyComponent(title = "Hello".to_string())  // ← Error!

// ✅ Fix: Add #[component] macro
#[component]
fn MyComponent(title: String) -> impl rusti::Component { /* ... */ }

// ✅ Or use snake_case for regular functions
fn my_component(title: String) -> impl rusti::Component { /* ... */ }

// Usage with positional args:
// @my_component("Hello".to_string())
```

---

### HTMX Attributes Not Working

#### Problem: HTMX namespaced attributes cause errors

**Cause**: Using double colon `::` instead of single colon `:`.

```rust
// ❌ Wrong
<button hx-on::click="handleClick()">  // Double colon!

// ✅ Fix: Single colon
<button hx-on:click="handleClick()">
```



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