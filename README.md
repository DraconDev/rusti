# Azumi 2.0

**Type-Safe, Zero-Cost HTML Templates for Rust & Axum.**

Azumi is an opinionated, compile-time HTML macro that enforces best practices. It blocks anti-patterns (like inline styles/scripts) to ensure your code is maintainable, secure, and IDE-friendly.

## ⚡ The Rules

Azumi is strict. Follow these rules or it won't compile.

### ✅ Must Do
1. **Quote all text:** `<h1>"Hello World"</h1>`
   - *Why?* Prevents lexer ambiguity and enables arbitrary text content.
2. **Quote all attribute values:** `<div class="container">`
   - *Why?* Consistent syntax, no guessing if quotes are needed.
3. **External CSS:** `<style src="styles.css" />`
   - *Why?* **Automatic Scoping.** CSS is scoped to the component at compile time.
   - *Bonus:* Full IDE support (linting, colors, autocomplete) for your CSS files.
4. **External JS:** `<script src="/static/app.js" />`
   - *Why?* Use the right tools (TypeScript, ESLint, Prettier) for JavaScript.

### ❌ Not Allowed
1. **Unquoted text:** `<h1>Hello</h1>` (Compile Error)
2. **Unquoted attributes:** `<div class=box>` (Compile Error)
3. **Inline styles:** `<style>.box { color: red; }</style>` (Compile Error)
4. **Inline scripts:** `<script>console.log("hi")</script>` (Compile Error)

### ⚠️ Exceptions
1. **Boolean Attributes:** `<input disabled checked />`
   - Standard HTML behavior. No value required.
2. **JSON Data Injection:** `<script type="application/json">{json_data}</script>`
   - The **only** allowed inline script. Safe way to pass server data to client-side code.

---

## 🚀 Features & Examples

### 1. Basic Usage
Simple, type-safe HTML generation.

```rust
use azumi::html;

fn hello(name: &str) -> impl azumi::Component {
    html! {
        <div class="greeting">
            <h1>"Hello, " {name} "!"</h1>
            <p>"Welcome to Azumi."</p>
        </div>
    }
}
```

### 2. Control Flow
Azumi supports Rust-native control flow directly in your templates.

**If / Else:**
```rust
@if logged_in {
    <button>"Log Out"</button>
} @else {
    <button>"Log In"</button>
}
```

**For Loops:**
```rust
<ul>
    @for item in items {
        <li>{item.name}</li>
    }
</ul>
```

**Match Expressions:**
```rust
@match status {
    Status::Active => { <span class="green">"Active"</span> }
    Status::Pending => { <span class="orange">"Pending"</span> }
    _ => { <span>"Unknown"</span> }
}
```

### 3. Components & Composition
Components are just functions that return `impl azumi::Component`. You can nest them easily.

```rust
fn page() -> impl azumi::Component {
    html! {
        <div class="app">
            @header(user)
            <main>
                @sidebar()
                @content()
            </main>
            @footer()
        </div>
    }
}
```

### 4. Layouts
Use function composition to create reusable layouts.

```rust
fn main_layout(title: &str, content: impl azumi::Component) -> impl azumi::Component {
    html! {
        <html>
            <head><title>{title}</title></head>
            <body>
                <nav>"..."</nav>
                <main>{content}</main>
            </body>
        </html>
    }
}

// Usage
fn home_page() -> impl azumi::Component {
    main_layout("Home", html! {
        <h1>"Welcome Home"</h1>
    })
}
```

### 5. Automatic CSS Scoping
Azumi reads your CSS files at compile time, generates a unique hash, and scopes your styles to the component.

**Input (`card.css`):**
```css
.card { background: #fff; padding: 20px; }
h2 { color: #333; }
```

**Component:**
```rust
html! {
    <div class="card">
        <style src="card.css" />
        <h2>"Scoped Title"</h2>
    </div>
}
```

**Output:**
```html
<div class="card" data-s12345>
    <style data-scope="s12345">
        .card[data-s12345] { background: #fff; padding: 20px; }
        h2[data-s12345] { color: #333; }
    </style>
    <h2 data-s12345>Scoped Title</h2>
</div>
```

### 6. Tailwind CSS Support
Azumi works perfectly with Tailwind. Just include the CDN or your build output.

```rust
html! {
    <div class="bg-blue-500 text-white p-4 rounded-lg shadow-md">
        <h1 class="text-2xl font-bold">"Tailwind Ready"</h1>
    </div>
}
```

### 7. HTMX Integration
Server-side rendering is back. Azumi + HTMX is a powerful combo.

```rust
<button 
    hx-post="/clicked" 
    hx-swap="outerHTML" 
    class="btn">
    "Click Me"
</button>
```

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
azumi = "0.2.0"
```

## 🛠️ Development

To run the demo project:

```bash
cd demo
cargo run
```

Visit `http://localhost:8081` to see all examples in action.