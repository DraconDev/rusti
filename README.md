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

## 🚀 Quick Start

```rust
use azumi::html;
use axum::response::Html;

fn page() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Azumi 2.0"</title>
                // CSS is read at compile-time and scoped automatically!
                <style src="styles.css" />
                <script src="https://cdn.tailwindcss.com" />
            </head>
            <body>
                <h1>"Hello, World!"</h1>
                
                // Dynamic attributes
                <a href={"/login"} class="btn">"Login"</a>
                
                // Control flow
                @if logged_in {
                    <p>"Welcome back!"</p>
                }
                
                // Components
                @Footer(year=2025)
            </body>
        </html>
    }
}
```

## 📦 Installation

```toml
[dependencies]
azumi = "0.2.0"
```

## 🎨 CSS Scoping

Azumi automatically scopes your CSS.

**Input (`styles.css`):**
```css
.card { background: white; }
h1 { color: blue; }
```

**Usage:**
```rust
html! {
    <div class="card">
        <style src="styles.css" />
        <h1>"Scoped!"</h1>
    </div>
}
```

**Output (Simplified):**
```html
<div class="card" data-s1a2b3>
    <style data-scope="s1a2b3">
        .card[data-s1a2b3] { background: white; }
        h1[data-s1a2b3] { color: blue; }
    </style>
    <h1 data-s1a2b3>Scoped!</h1>
</div>
```

## ❓ FAQ

**Why mandatory quotes?**
Rust's lexer treats `class=red` as an identifier `class`, an operator `=`, and an identifier `red`. But `class=2em` fails because `2em` is invalid Rust syntax. Mandatory quotes (`class="2em"`) solve this permanently.

**Why no inline scripts?**
Inline scripts have zero tooling support. You can't lint them, type-check them, or minify them easily. Azumi forces you to put JS in `.js` files where your tools work.

**How do I pass data to JS?**
Use data attributes or JSON injection:
```rust
html! {
    // Option 1: Data Attributes
    <div id="app" data-user={user_json}></div>
    
    // Option 2: JSON Script
    <script type="application/json" id="data">
        {user_json}
    </script>
}
```