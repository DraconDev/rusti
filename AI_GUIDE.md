# Comprehensive AI Guide to Writing Azumi Code

> A complete reference guide for AI assistants helping developers write Azumi applications, covering all features and patterns.

## 🎯 Philosophy & Core Concepts

Azumi is a **compiler-driven optimistic UI framework** that generates client-side predictions from Rust code. Write logic once, get instant UI updates everywhere.

- **Single Source of Truth**: Rust code is the only source of truth
- **Compile-Time Safety**: Macros catch errors before runtime
- **Zero JavaScript**: Compiler generates all client logic
- **Server-Side Truth**: Server always wins, client predictions are optimistic

---

## 🚨 Critical Rules (Absolute Must-Know)

### 1. CSS Values MUST Be Double-Quoted

```rust
// ✅ CORRECT - all values quoted
.btn {
    padding: "1rem";
    background: "#4CAF50";
    color: "white";
    border-radius: "8px";
}

// ❌ WRONG - will cause compile errors or lexer issues
.btn {
    padding: 1rem;      // Lexer can't parse "1rem" as token
    background: #4CAF50; // # causes parsing issues
    color: white;        // Unquoted identifier
}
```

### 2. CSS Classes Become Rust Variables

```rust
html! {
    <style>
        .my_button { padding: "1rem"; }  // Defines variable `my_button`
    </style>
    <button class={my_button}>"Click"</button>  // Uses variable
}
```

**Naming rules:**
- Use `snake_case` for class names (becomes Rust identifier)
- Avoid hyphens in class names (`my-button` → use `my_button`)

### 3. Live State Requires Component Link

```rust
// State definition
#[azumi::live]
pub struct Counter { pub count: i32 }

// Implementation MUST specify component
#[azumi::live_impl(component = "counter_view")]  // ← Required!
impl Counter {
    pub fn increment(&mut self) { self.count += 1; }
}

// Component MUST match the name
#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! { /* ... */ }
}
```

### 4. Event Binding Syntax

```rust
// ✅ CORRECT - use on:event={state.method}
<button on:click={state.increment}>"Click"</button>

// ❌ WRONG - don't use closures or function calls
<button on:click={|| state.increment()}>"Click"</button>
<button on:click={state.increment()}>"Click"</button>
```

### 5. Text Content Must Be Quoted

```rust
// ✅ CORRECT - all text in quotes
<p>"Hello world"</p>
<p>"Count: " {count}</p>

// ❌ WRONG - will cause parsing errors
<p>Hello world</p>
```

---

## 🏗️ Component Fundamentals

### Basic Component Structure

```rust
#[azumi::component]
pub fn MyComponent(title: &str, count: i32) -> impl Component {
    html! {
        <style>
            .container { padding: "1rem"; }
            .title { font-size: "1.5rem"; color: "#333"; }
        </style>
        <div class={container}>
            <h1 class={title}>{title}</h1>
            <p>"Count: " {count}</p>
        </div>
    }
}
```

### Component with Children

```rust
#[azumi::component]
pub fn Container(children: impl Component) -> impl Component {
    html! {
        <style>
            .container { padding: "1rem"; border: "1px solid #ddd"; }
        </style>
        <div class={container}>
            {children}
        </div>
    }
}

// Usage with curly braces
@Container {
    <p>"Content inside container"</p>
    <div>"Multiple elements"</div>
}
```

### Component Composition

```rust
#[azumi::component]
pub fn Card<'a>(title: &'a str, content: &'a str) -> impl Component + 'a {
    html! {
        <style>
            .card { border: "1px solid #eee"; padding: "1rem"; }
            .title { font-weight: "bold"; margin-bottom: "0.5rem"; }
        </style>
        <div class={card}>
            <h3 class={title}>{title}</h3>
            <p>{content}</p>
        </div>
    }
}

#[azumi::component]
pub fn Dashboard() -> impl Component {
    html! {
        <div>
            @Card(title="Welcome", content="Getting started")
            @Card(title="Features", content="Type-safe components")
        </div>
    }
}
```

---

## 🎨 Styling System

### CSS Scoping: Component vs Global

```rust
#[azumi::component]
pub fn StyledComponent() -> impl Component {
    html! {
        // Global styles - NOT scoped (use string literals)
        <style global>
            body { font-family: "Inter, sans-serif"; }
            .global_heading { color: "purple"; }
        </style>

        // Component styles - automatically scoped (become variables)
        <style>
            .local_heading { color: "blue"; }
            .container { padding: "1rem"; }
        </style>

        <div class={container}>
            <h1 class={local_heading}>"Scoped (blue)"</h1>
            <h2 class="global_heading">"Global (purple)"</h2>
        </div>
    }
}
```

### Dynamic CSS with Custom Properties

```rust
#[azumi::component]
pub fn ProgressMeter(completion: f64, accent_color: &str) -> impl Component {
    html! {
        <style>
            .meter {
                width: "100%";
                height: "20px";
                background: "#eee";
                border-radius: "10px";
                overflow: "hidden";
            }
            .fill {
                height: "100%";
                background: "var(--accent, #2196f3)";
                width: "calc(var(--progress) * 100%)";
                transition: "width 0.3s ease";
            }
        </style>

        <div class={meter}>
            // style="" ONLY allows CSS custom properties (--variables)
            <div class={fill} style="--progress: {completion}; --accent: {accent_color}"></div>
        </div>
    }
}
```

**⚠️ Important**: Only CSS custom properties (`--var-name`) are allowed in `style=""` attributes. Direct properties like `style="width: 50%"` cause compile errors.

### CSS Validation Rules

Azumi validates all CSS at compile time:

**Allowed:**
- Standard CSS properties with quoted values
- CSS custom properties (`--variable-name`)
- CSS functions like `calc()`, `var()`, etc.

**Not Allowed:**
- External CSS imports (`@import`)
- Inline CSS properties without custom properties
- Invalid CSS syntax

---

## 🧮 Control Flow & Logic

### @let Pattern (Local Variables)

```rust
#[azumi::component]
pub fn LetExample() -> impl Component {
    html! {
        <style>
            .result { background: "#f0f0f0"; padding: "0.5rem"; }
        </style>
        <div>
            // Basic variable declaration
            @let name = "Azumi";
            <p>"Hello, " {name} "!"</p>

            // Calculated values
            @let items = vec!["Item 1", "Item 2", "Item 3"];
            @let item_count = items.len();
            <p>"Total items: " {item_count}</p>

            // Derived calculations
            @let base_price = 100.0;
            @let tax_rate = 0.08;
            @let total_price = base_price * (1.0 + tax_rate);
            <div class={result}>
                <p>"Total: $" {total_price:.2}</p>
            </div>

            // With conditional logic
            @let score = 85;
            @let grade = if score >= 90 {
                "A"
            } else if score >= 80 {
                "B"
            } else {
                "C"
            };
            <p>"Grade: " {grade}</p>
        </div>
    }
}
```

### Conditional Rendering (@if)

```rust
html! {
    // Simple conditional
    @if state.active {
        <div>"Active"</div>
    }

    // With else
    @if state.user_type == "admin" {
        <div>"Admin Panel"</div>
    } else {
        <div>"User Dashboard"</div>
    }

    // Conditional class application
    <div class={if state.active { "active_class" } else { "inactive_class" }}>
        "Content"
    </div>
}
```

### Loops (@for)

```rust
html! {
    // Basic loop
    @for item in &state.items {
        <div>{item}</div>
    }

    // Loop with index
    @for (i, item) in state.items.iter().enumerate() {
        <div>{i + 1}". " {item}</div>
    }

    // Empty state handling
    @if state.items.is_empty() {
        <p>"No items found"</p>
    } else {
        @for item in &state.items {
            <div>{item}</div>
        }
    }
}
```

### Pattern Matching (@match)

```rust
#[azumi::component]
pub fn StatusDisplay(status: &str) -> impl Component {
    html! {
        <style>
            .loading { color: "blue"; }
            .success { color: "green"; }
            .error { color: "red"; }
        </style>
        <div>
            @match status {
                "loading" => {
                    <p class={loading}>"Loading..."</p>
                }
                "success" => {
                    <p class={success}>"Operation successful!"</p>
                }
                "error" => {
                    <p class={error}>"An error occurred"</p>
                }
                _ => {
                    <p>"Unknown status"</p>
                }
            }
        </div>
    }
}
```

---

## ⚡ Live Interactive Components

### Live State Structure

```rust
#[azumi::live]
pub struct Counter {
    pub count: i32,
    pub active: bool,
}

#[azumi::live_impl(component = "counter_view")]
impl Counter {
    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
    }
}
```

### Live Component View

```rust
#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! {
        <style>
            .counter { padding: "2rem"; text-align: "center"; }
            .value { font-size: "3rem"; margin: "1rem 0"; }
            .btn { padding: "1rem 2rem"; cursor: "pointer"; }
        </style>
        <div class={counter}>
            <div class={value}>{state.count}</div>
            <button class={btn} on:click={state.increment}>
                "Increment"
            </button>
            <p>"Status: " {if state.active { "Active" } else { "Inactive" }}</p>
        </div>
    }
}
```

### Auto-Detection of Live Components

When the first parameter is `state: &T`, the component automatically detects live mode:

```rust
// Auto-wraps in <div az-scope="...">
#[azumi::component]
pub fn auto_detected_view<'a>(state: &'a MyState) -> impl Component + 'a {
    html! {
        <button on:click={state.my_action}>"Click me"</button>
    }
}
```

---

## 🎯 Event Binding Systems

### Modern: on:event Syntax (Recommended)

```rust
<button on:click={state.increment}>"Click"</button>
<input on:input={state.update_text} />
<form on:submit={state.submit_form}>"Submit"</form>
```

### Legacy: az-on DSL

For backward compatibility, the original `az-on` syntax still works:

```rust
<div az-scope={serde_json::to_string(&state).unwrap()}>
    <button az-on="click call increment -> this">
        "Increment"
    </button>
</div>
```

---

## 📋 Form Handling

### Basic Form with Live State

```rust
#[azumi::live]
pub struct ContactForm {
    pub submitted: bool,
}

#[azumi::live_impl]
impl ContactForm {
    pub fn submit(&mut self) {
        self.submitted = true;
    }

    pub fn reset(&mut self) {
        self.submitted = false;
    }
}

#[azumi::component]
pub fn contact_form_view<'a>(state: &'a ContactForm) -> impl Component + 'a {
    html! {
        <style>
            .form { display: "grid"; gap: "1rem"; max-width: "400px"; }
            .field { display: "grid"; gap: "0.5rem"; }
            .input { padding: "0.5rem"; border: "1px solid #ddd"; }
            .btn { padding: "0.75rem"; background: "#2196f3"; color: "white"; }
        </style>

        @if state.submitted {
            <div>"Thank you for your message!"</div>
            <button on:click={state.reset}>"Send Another"</button>
        }

        @if !state.submitted {
            <form class={form}>
                <div class={field}>
                    <label>"Name"</label>
                    <input class={input} type="text" name="name" />
                </div>
                <div class={field}>
                    <label>"Email"</label>
                    <input class={input} type="email" name="email" />
                </div>
                <button class={btn} type="button" on:click={state.submit}>
                    "Submit"
                </button>
            </form>
        }
    }
}
```

---

## 🔄 Server Actions (Advanced)

For complex server-side logic beyond simple predictions:

```rust
// State for server action
#[derive(Serialize, Deserialize)]
pub struct TodoState {
    pub items: Vec<String>,
    pub input: String,
}

// Server action handler
#[azumi::action]
async fn add_todo(mut state: TodoState) -> impl azumi::Component {
    if !state.input.is_empty() {
        state.items.push(state.input.clone());
        state.input.clear();
    }
    todo_list(state)  // Return updated component
}

// Component using server action
pub fn todo_list(state: TodoState) -> impl azumi::Component {
    html! {
        <script src="/static/azumi.js"></script>
        <div az-scope={serde_json::to_string(&state).unwrap()}>
            <input type="text" name="input" value={state.input} />
            <button az-on="click call add_todo -> this">"Add"</button>

            @for item in &state.items {
                <div>{item}</div>
            }
        </div>
    }
}
```

---

## 🧩 Component Composition Patterns

### Multiple Live Components Together

```rust
#[azumi::live]
pub struct TabState {
    pub active_index: i32,
}

#[azumi::live_impl(component = "tabs_view")]
impl TabState {
    pub fn select_0(&mut self) { self.active_index = 0; }
    pub fn select_1(&mut self) { self.active_index = 1; }
    pub fn select_2(&mut self) { self.active_index = 2; }
}

#[azumi::component]
pub fn tabs_view<'a>(state: &'a TabState) -> impl Component + 'a {
    html! {
        <style>
            .tabs { display: "flex"; gap: "0.5rem"; }
            .tab { padding: "0.5rem 1rem"; cursor: "pointer"; }
            .active { background: "#2196f3"; color: "white"; }
        </style>

        <div>
            <div class={tabs}>
                <button class={if state.active_index == 0 { "tab active" } else { "tab" }}
                        on:click={state.select_0}>"Tab 1"</button>
                <button class={if state.active_index == 1 { "tab active" } else { "tab" }}
                        on:click={state.select_1}>"Tab 2"</button>
                <button class={if state.active_index == 2 { "tab active" } else { "tab" }}
                        on:click={state.select_2}>"Tab 3"</button>
            </div>
            <div>
                @match state.active_index {
                    0 => { <p>"Content for tab 1"</p> }
                    1 => { <p>"Content for tab 2"</p> }
                    2 => { <p>"Content for tab 3"</p> }
                    _ => { <p>"Unknown tab"</p> }
                }
            </div>
        </div>
    }
}
```

### Layout Components

```rust
#[azumi::component]
pub fn PageLayout(children: impl Component) -> impl Component {
    html! {
        <style>
            .page { max-width: "800px"; margin: "0 auto"; padding: "2rem"; }
            .header { border-bottom: "1px solid #eee"; margin-bottom: "2rem"; }
        </style>
        <div class={page}>
            <header class={header}>
                <h1>"My App"</h1>
            </header>
            <main>{children}</main>
        </div>
    }
}
```

---

## 🔍 HTML Structure Validation

Azumi validates HTML structure at compile time:

### Required Attributes

```rust
// Images require alt text
<img src="image.jpg" alt="Description" />

// Inputs require labels (accessibility)
<label for="name">"Name"</label>
<input id="name" type="text" />

// Links need href
<a href="/page">"Link text"</a>
```

### Valid HTML Structure

```rust
// ✅ Valid structure
<div>
    <h1>"Title"</h1>
    <p>"Content"</p>
</div>

// ❌ Invalid - missing required elements
<img src="test.jpg" />  // Missing alt attribute
```

---

## ♿ Accessibility Features

### Semantic HTML

```rust
// Use semantic elements
<nav> {/* Navigation */}
<main> {/* Main content */}
<article> {/* Article content */}
<section> {/* Section content */}
<aside> {/* Sidebar content */}

// Proper heading hierarchy
<h1>"Page Title"</h1>
<h2>"Section Title"</h2>
<h3>"Subsection Title"</h3>
```

### ARIA Attributes

```rust
<button aria-label="Close dialog" on:click={state.close}>
    "×"
</button>

<div role="alert" aria-live="polite">
    {state.message}
</div>
```

### Focus Management

```rust
// Ensure focusable elements
<button on:click={state.action}>"Click me"</button>
<input on:input={state.update} />

// Skip links for keyboard users
<a href="#main-content" class="skip-link">"Skip to main content"</a>
```

---

## 📊 Prediction System Details

### How Optimistic UI Works

```
User clicks on:click={state.increment}
    ↓
1. INSTANT: Execute prediction locally (0ms latency!)
    ↓
2. ASYNC: Send request to server
    ↓
3. RECONCILE: Server returns HTML, morph into DOM
    ↓
4. VERIFY: If prediction was wrong, server wins
```

### Supported Prediction Patterns

| Rust Code Pattern | Generated Prediction |
| ------------------ | -------------------- |
| `self.x = !self.x` | `x = !x` (toggle) |
| `self.x = true` | `x = true` (literal) |
| `self.x = false` | `x = false` (literal) |
| `self.x += 1` | `x = x + 1` (increment) |
| `self.x -= 1` | `x = x - 1` (decrement) |
| `self.x = value` | `x = value` (assignment) |

### Complex Logic (No Prediction)

```rust
// This runs server-only, no client prediction
pub async fn complex_action(&mut self) {
    // Database operations
    // External API calls
    // Complex calculations
    // These cannot be predicted
}
```

---

## 📄 Head Meta Tags (SEO & Social Sharing)

The `head!` macro generates complete HTML head meta tags including title, description, and Open Graph/Twitter card tags.

### Basic Head Meta

```rust
use azumi::head;

#[azumi::component]
pub fn Page() -> impl Component {
    html! {
        <html>
        <head>
            {head! {
                title: "My Azumi App",
                description: "A type-safe Rust web framework"
            }}
        </head>
        <body>
            <h1>"Welcome to Azumi"</h1>
        </body>
        </html>
    }
}
```

### Full Meta with Social Sharing

```rust
#[azumi::component]
pub fn BlogPost() -> impl Component {
    let title = "Building with Azumi Live";
    let description = "Learn how to create reactive UI components";
    
    html! {
        <html>
        <head>
            {head! {
                title: title,
                description: description,
                image: "/static/azumi-preview.jpg",
                url: "https://myapp.com/blog/azumi-live",
                type: "article"
            }}
        </head>
        <body>
            <article>
                <h1>{title}</h1>
                <p>{description}</p>
            </article>
        </body>
        </html>
    }
}
```

### Dynamic Meta Values

```rust
#[azumi::component]
pub fn ProductPage(product: &'a Product) -> impl Component + 'a {
    html! {
        <html>
        <head>
            {head! {
                title: format!("{} - {} | My Store", product.name, product.category),
                description: format!("Buy {} for ${}. {} {}", product.name, product.price, product.brand, product.description),
                image: product.image_url,
                url: format!("https://mystore.com/products/{}", product.id),
                type: "product"
            }}
        </head>
        <body>
            <div>
                <h1>{product.name}</h1>
                <p>"$" {product.price}</p>
            </div>
        </body>
        </html>
    }
}
```

### Head Meta Field Reference

| Field | Required | Description | Generated Tags |
|-------|----------|-------------|----------------|
| `title` | ✅ Yes | Page title | `<title>`, `og:title`, `twitter:title` |
| `description` | ✅ Yes | Page description | `<meta name="description">`, `og:description`, `twitter:description` |
| `image` | ❌ No | Social sharing image | `og:image`, `twitter:image`, `twitter:card="summary_large_image"` |
| `url` | ❌ No | Canonical URL | `og:url` |
| `type` | ❌ No | Content type | `og:type` (defaults to "website") |

### Meta Tag Output Examples

**Minimal (title + description only):**
```html
<title>My Page</title>
<meta name="description" content="Page description">
<meta property="og:title" content="My Page">
<meta property="og:description" content="Page description">
<meta property="og:type" content="website">
<meta name="twitter:title" content="My Page">
<meta name="twitter:description" content="Page description">
<meta name="twitter:card" content="summary">
```

**Full (with image, URL, type):**
```html
<title>My Article</title>
<meta name="description" content="Article description">
<meta property="og:title" content="My Article">
<meta property="og:description" content="Article description">
<meta property="og:type" content="article">
<meta property="og:url" content="https://example.com/article">
<meta property="og:image" content="/image.jpg">
<meta name="twitter:image" content="/image.jpg">
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:title" content="My Article">
<meta name="twitter:description" content="Article description">
```

### SEO Best Practices

```rust
#[azumi::component]
pub fn SEOOptimizedPage() -> impl Component {
    let page_title = "Azumi Live Guide - Type-Safe Reactive UI";
    let description = "Complete guide to building reactive web applications with Azumi Live. Zero JavaScript, full type safety.";
    let keywords = "rust, web framework, reactive ui, type-safe, azumi";
    
    html! {
        <html lang="en">
        <head>
            {head! {
                title: page_title,
                description: description,
                image: "/static/azumi-social-preview.jpg",
                url: "https://azumi.dev/guide",
                type: "article"
            }}
            // Additional SEO meta tags
            <meta name="keywords" content={keywords} />
            <meta name="author" content="Azumi Team" />
            <meta name="robots" content="index, follow" />
        </head>
        <body>
            <main>
                <h1>"Azumi Live Guide"</h1>
                <p>{description}</p>
            </main>
        </body>
        </html>
    }
}
```

---

## 🔧 Component Props System

### Required Props

```rust
#[azumi::component]
pub fn Card(title: &str, content: &str) -> impl Component {
    html! {
        <div>
            <h3>{title}</h3>
            <p>{content}</p>
        </div>
    }
}

// Usage:
@Card(title = "Hello", content = "World")
```

### Optional Props with Defaults

```rust
#[azumi::component]
pub fn Button(
    label: &str,
    #[prop(default = "primary")] variant: &str,
    #[prop(default = false)] disabled: bool,
    #[prop(default = "medium")] size: &str,
) -> impl Component {
    html! {
        <button disabled={disabled} class={format!("btn btn-{} btn-{}", variant, size)}>
            {label}
        </button>
    }
}

// Usage:
@Button(label = "Click me")  // Uses all defaults
@Button(label = "Submit", variant = "secondary", disabled = true)
```

### Props with Lifetimes

```rust
#[azumi::component]
pub fn UserCard<'a>(user: &'a User) -> impl Component + 'a {
    html! {
        <div>
            <h3>{user.name}</h3>
            <p>{user.email}</p>
        </div>
    }
}
```

---

## 📁 File Structure & Organization

### Recommended Project Structure

```
src/
├── main.rs                 # Axum router setup
├── components/
│   ├── mod.rs             # Component exports
│   ├── button.rs          # Reusable Button component
│   ├── card.rs            # Card component
│   └── layout.rs          # Layout components
├── pages/
│   ├── mod.rs             # Page exports
│   ├── home.rs            # Home page component
│   ├── about.rs           # About page component
│   └── dashboard.rs       # Dashboard page
├── state/
│   ├── mod.rs             # State exports
│   ├── counter.rs         # Counter live state
│   └── user.rs            # User live state
└── utils/
    ├── mod.rs
    └── helpers.rs         # Utility functions
```

### Router Setup

```rust
use axum::Router;

fn main() {
    let app = Router::new()
        // Register all Azumi actions
        .merge(azumi::action::register_actions())
        // Add your routes
        .route("/", axum::routing::get(homepage_handler))
        .route("/lesson-0", axum::routing::get(lesson0_handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---

## ⚠️ Common Mistakes & Solutions

### ❌ Missing Component Link

```rust
// WRONG - missing component link
#[azumi::live_impl]
impl Counter { /* ... */ }

// CORRECT
#[azumi::live_impl(component = "counter_view")]
impl Counter { /* ... */ }
```

### ❌ Unquoted CSS Values

```rust
// WRONG - causes compile errors
.btn { padding: 1rem; }

// CORRECT
.btn { padding: "1rem"; }
```

### ❌ Using Hyphens in Class Names

```rust
// WRONG - can't be a Rust identifier
.my-button { }
<div class={my-button}></div>  // Error!

// CORRECT
.my_button { }
<div class={my_button}></div>
```

### ❌ Closure Event Handlers

```rust
// WRONG - don't use closures
<button on:click={|| state.increment()}>

// CORRECT - direct method reference
<button on:click={state.increment}>
```

### ❌ Missing State Reference in Component

```rust
// WRONG - state must be first param for live components
#[azumi::component]
pub fn counter_view() -> impl Component { }

// CORRECT
#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a { }
```

### ❌ Using style Attribute Incorrectly

```rust
// WRONG - direct CSS properties
<div style="width: 50%; background: red;"></div>

// CORRECT - only CSS custom properties
<div style="--width: 0.5; --color: red;"></div>
```

### ❌ Missing Script Include

```rust
// For interactive components, include azumi.js
<script src="/static/azumi.js"></script>
<script src="/static/idiomorph.js"></script>
```

---

## 🚀 Performance Considerations

### Optimized Patterns

```rust
// ✅ Good - minimal re-renders
#[azumi::component]
pub fn OptimizedComponent(state: &'a MyState) -> impl Component + 'a {
    html! {
        @let computed_value = expensive_calculation(&state.data);
        <div>{computed_value}</div>
    }
}

// ✅ Good - conditional rendering
@if state.visible {
    <ExpensiveComponent data={state.data} />
}
```

### Avoid Anti-Patterns

```rust
// ❌ Bad - unnecessary computations in template
<div>
    {expensive_function(&state.data)}  // Runs on every render
</div>

// ❌ Bad - large components
// Break into smaller components instead
```

---

## 🔧 Debugging & Development

### Development Server

```bash
# Run the demo to see examples
cd demo
cargo run
# Visit http://localhost:3000
```

### Error Messages

Common error patterns and solutions:

1. **CSS Parsing Errors**: Check all values are quoted
2. **Component Link Errors**: Ensure `#[azumi::live_impl(component = "name")]` matches component name
3. **Event Binding Errors**: Use `on:click={state.method}` not closures
4. **HTML Validation Errors**: Check required attributes (alt, href, labels)

---

## 📚 Quick Reference

| Feature | Syntax | Example |
|---------|--------|---------|
| Component | `#[azumi::component]` | `#[azumi::component] fn MyComp() -> impl Component` |
| Live State | `#[azumi::live]` | `#[azumi::live] pub struct State { }` |
| Live Methods | `#[azumi::live_impl]` | `#[azumi::live_impl(component = "view")] impl State` |
| Event | `on:event={state.method}` | `on:click={state.increment}` |
| Class | `class={class_name}` | `class={my_button}` |
| Style | `<style>` or `<style global>` | `<style> .btn { } </style>` |
| Dynamic Style | `style="--var: {value}"` | `style="--color: red"` |
| Conditional | `{if cond { a } else { b }}` | `{if active { "On" } else { "Off" }}` |
| Loop | `@for item in items { }` | `@for item in &list { <div>{item}</div> }` |
| Pattern Match | `@match value { }` | `@match status { "ok" => {} _ => {} }` |
| Local Var | `@let name = value;` | `@let total = price * quantity;` |
| Children | `@Component { }` | `@Container { <p>"Content"</p> }` |
| Text | Quoted strings | `"Hello world"` |

---

## 🎯 Key Takeaways

1. **Write Rust, get JavaScript**: The compiler does the heavy lifting
2. **Quote everything**: CSS values, text content, class names
3. **Component link required**: Live state needs `#[azumi::live_impl(component = "name")]`
4. **Event binding is declarative**: `on:click={state.method}` not closures
5. **Predictions are optimistic**: Server always wins if wrong
6. **CSS scoping is automatic**: No manual CSS management needed
7. **Forms are type-safe**: Compile-time validation for accessibility
8. **Performance is built-in**: Compiler optimizations and efficient rendering

This guide covers all aspects of Azumi development. Use it as your comprehensive reference for building type-safe, performant web applications with Rust!