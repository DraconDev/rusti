# Azumi 🌀

**Type-Safe, Compile-Time Validated HTML Templates for Rust & Axum.**

[![Crates.io](https://img.shields.io/crates/v/azumi.svg)](https://crates.io/crates/azumi)
[![Docs](https://docs.rs/azumi/badge.svg)](https://docs.rs/azumi)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Azumi is a **strict** HTML template system for Rust that **validates CSS at compile time** with **location-specific errors**, enforces **component-scoped styling**, ensures **type safety**, and provides **zero runtime overhead**. Every class you use **must be defined**. No exceptions. No runtime surprises.

## 🎯 What is Azumi? (TL;DR)

```rust
use azumi::html;

#[azumi::component]
fn Button(text: &str) -> impl azumi::Component {
    html! {
        <style src=\"/static/button.css\" />
        <button class=\"btn-primary\">{text}</button>
    }
}
```

**Key Features:**
- ✅ **Compile-time CSS validation** - Undefined classes → compile errors at **exact locations**
- ✅ **Automatic CSS scoping** - No global style leaks, component isolation out-of-the-box
- ✅ **Strict quoting** - Lexer-proof templates, arbitrary content support
- ✅ **Zero runtime** - Pure `proc_macro` expansion to `fmt::Display`
- ✅ **Full IDE support** - Autocomplete, linting for external CSS files
- ✅ **Axum + HTMX ready** - Server-side rendering powerhouse
- ✅ **Component system** - Auto-generated builder props with defaults

**Azumi is NOT:**
- ❌ Tailwind/UnoCSS - Write *real* semantic CSS classes
- ❌ JSX/Leptos - Server-side only, no client hydration complexity
- ❌ Inline styles - External files only, full separation of concerns
- ❌ Lenient - Break rules → won't compile (intentional)

## 🚀 Quick Start

### 1. Install
```toml
[dependencies]
azumi = { git = \"https://github.com/DraconDev/azumi\" } # or crates.io when stable
azumi-macros = { git = \"https://github.com/DraconDev/azumi\" }
axum = \"0.7\"
tower-http = { version = \"0.5\", features = [\"fs\"] }
```

### 2. Basic Template
```rust
use azumi::{html, component};

html! {
    <style src=\"/static/app.css\" />
    <div class=\"container\">
        <h1 class=\"title\">\"Hello Azumi!\"</h1>
        @if user.is_logged_in {
            <p>\"Welcome back, \" {user.name}!\"</p>
        }
    </div>
}
```

### 3. Run Demo
```bash
git clone https://github.com/DraconDev/azumi
cd azumi/demo
cargo run
# Visit http://localhost:8081
```

## 🧭 Design Philosophy

Azumi solves **technical debt** from \"flexible\" templating:

| Problem | Traditional Approach | Azumi Solution |
|---------|---------------------|---------------|
| **Typos in classes** | Runtime invisible fails | Compile-time errors at **exact line/col** |
| **Style leaks** | Global CSS pollution | Auto-scoped `[data-sabc123]` attributes |
| **Inline styles** | No IDE, unmaintainable | External CSS + full VSCode support |
| **Utility CSS** | Unreadable HTML | Semantic `.btn-primary`, `.card` |
| **Mixed concerns** | HTML/CSS/JS soup | Strict separation enforced |
| **Dead CSS** | Bloats bundle | Validates used vs defined (future) |

## 📖 Complete Syntax Guide

### HTML Elements & Attributes
```rust
html! {
    <div class=\"container\" id=\"main\">
        <input type=\"text\" placeholder=\"Search...\" disabled />
        <img src=\"/logo.png\" alt=\"Logo\" />
    </div>
}
```
- **All text/attrs quoted** - `<h1>\"Title\"</h1>`
- **Boolean attrs** - `<input disabled />`
- **Self-closing** - `<br />`

### Rust Interop (`@`)
```rust
@if condition {
    <p>Yes</p>
} else {
    <p>No</p>
}

@for item in items {
    <li>{item}</li>
}

@match status {
    Ok(data) => { <span class=\"success\">OK</span> }
    Err(e) => { <span class=\"error\">{e}</span> }
}

@let computed = format!(\"${:.2}\", price);
<p>{computed}</p>

@Component(name=\"Alice\")  // Component call
```

### Components (Auto-Props)
```rust
#[azumi::component]
fn UserCard(
    name: &str,
    #[prop(default = r#\"User\"#)] role: &str,
    children: impl azumi::Component,  // Optional children
) -> impl azumi::Component {
    html! {
        <div class=\"user-card\">
            <h3>{name} ({role})</h3>
            {children}
        </div>
    }
}

// Usage
@UserCard(name=\"Alice\", role=\"Admin\") {
    <p>Bio here</p>
}
```

Generates `UserCard::Props::builder().name(\"Alice\").role(\"Admin\").build()`

### CSS Integration
```rust
html! {
    <!DOCTYPE html>
    <html>
        <head>
            <style src=\"/static/global.css\" />  // Unscoped globals
            <style src=\"/static/card.css\" />    // Scoped to component
        </head>
        <body class=\"app\">  // Uses global
            <div class=\"card\">  // Scoped
                ...
            </div>
        </body>
    </html>
}
```

## 🎨 CSS System Deep Dive

### Automatic Scoping
```
card.css: .card { padding: 1rem; }
```
Expands to:
```html
<style>
.card[data-s1a2b3c] { padding: 1rem; }
</style>
<div class=\"card\" data-s1a2b3c>...</div>
```

- **Hash-based scope ID** from CSS content
- **Preserves pseudo-classes** `:hover`, `::before`
- **@-rules untouched** (`@media`, `@keyframes`)

### Validation Pipeline
1. **Collect** `<style src>` files (skip `global.css`)
2. **Parse** classes with regex (fast)
3. **Validate** every `class=\"...\"` → compile error if missing
4. **Scope** CSS + inject `<style>`

**Error Example:**
```
error[E0425]: CSS class 'btn-primry' not defined
  --> src/component.rs:10:20
   |
10 |     <button class=\"btn-primry\">
   |                      ^^^^^^^^
```

### Global CSS (`global.css`)
- **Unscoped** - No `[data-scope]`
- **Injected first** - Lower specificity
- **No validation** - Design tokens, resets
```css
/* global.css */
:root { --primary: #3b82f6; }
* { box-sizing: border-box; }
```

## 🛠️ Components API

`#[azumi::component]` auto-generates:

```rust
// Input fn
fn MyComp(name: &str, optional: Option<&str>) -> impl Component { ... }

// Output
mod my_comp {
    #[derive(Debug)]
    pub struct Props { pub name: &'static str, pub optional: Option<&'static str> }
    
    pub struct PropsBuilder { ... }
    
    impl Props { pub fn builder() -> PropsBuilder { ... } }
    
    impl PropsBuilder {
        pub fn name(self, v: &'static str) -> Self { ... }
        pub fn optional(self, v: Option<&'static str>) -> Self { ... }
        pub fn build(self) -> Result<Props, &'static str> { ... }
    }
    
    pub fn render(props: Props) -> impl Component { ... }
}
```

## 🔧 Editor Setup (VS Code)

1. **CSS Peek** - Ctrl+Click `<style src>`
```json
{
    \"cssPeek.peekFromLanguages\": [\"rust\"],
    \"cssPeek.searchFileExtensions\": [\".css\"]
}
```

2. **rust-analyzer** - Native proc_macro support
3. **Better TOML** - Cargo.toml syntax

## 📚 Interactive Lessons

Run `cargo run` in `demo/` → **20 Progressive Lessons**:

- **1-5**: Basics (HTML, data, CSS)
- **6-10**: Control flow (@if/@for/@match)
- **11-15**: Components (props, children, composition)
- **16-20**: HTMX, layouts, full apps

Each lesson: **Live demo + source code + rendered HTML**.

## 🚀 Production Axum Integration

```rust
use axum::{response::Html, routing::get, Router};

async fn home() -> Html<String> {
    let component = html! { <h1>Hello</h1> };
    Html(azumi::render_to_string(&component))
}

let app = Router::new().route(\"/\", get(home));
```

**HTMX Example:**
```rust
html! {
    <button hx-post=\"/api/action\" hx-swap=\"outerHTML\" class=\"btn\">\"Click\"</button>
}
```

## ⚠️ Strict Rules (Won't Compile Otherwise)

✅ **Must:**
- Quote text: `\"Hello\"`
- `<style src=\"/path.css\" />`
- Define all classes used

❌ **Banned:**
- Inline `<style>...</style>`
- Inline `<script>...</script>`
- Unquoted text/attrs
- `<link href=\"/local.css\">` (use `<style src>`)

**Exceptions:**
- `<script type=\"application/json\">{data}</script>`
- CDN `<link href=\"https://...\">`

## 🔍 Internals (For AIs & Experts)

### Macro Pipeline (`macros/src/lib.rs`)
```
html! { ... } ──parse──> Nodes ──validate_css──> Errors?
                          ↓
                    collect_styles ──scope_css──> Scoped CSS
                          ↓
                    generate_body ──> fmt::Display impl
```

### Key Modules:
- `token_parser.rs`: Custom lexer for `@` syntax
- `css_validator.rs`: Regex class extraction + file resolution
- `css.rs`: Selector scoping (handles `:hover`, `::before`)
- `component.rs`: Props builder gen (defaults, children)

### Validation Flow:
```
HTML classes ──> css_validator::validate_component_css ──> compile_error!()
CSS files ───> resolve_css_file_path (7 fallback paths) ──> parse_css_classes
```

## 📈 Roadmap
- [x] CSS scoping/validation
- [x] Component props builder
- [x] 20-lesson curriculum
- [ ] Dead CSS warnings
- [ ] ID validation
- [ ] TSX-like children API
- [ ] Crates.io 1.0

## 🤝 Contributing
1. `cargo test`
2. `cd demo && cargo run`
3. PRs welcome!


### 5. Automatic CSS Scoping

Azumi reads your CSS files at compile time, generates a unique hash, and scopes your styles to the component.

**Input (`card.css`):**

```css
.card {
    background: #fff;
    padding: 20px;
}
h2 {
    color: #333;
}
```

**Component:**

```rust
html! {
    <style src="card.css" />
    <div class="card">
        <h2>"Scoped Title"</h2>
    </div>
}
```

**Output:**

```html
<style>
    .card[data-s12345] {
        background: #fff;
        padding: 20px;
    }
    h2[data-s12345] {
        color: #333;
    }
</style>
<div class="card" data-s12345>
    <h2 data-s12345>Scoped Title</h2>
</div>
```

**CSS Variables Across Components:**

CSS variables defined in a component ARE accessible to child components because they inherit through the DOM:

```css
/* parent.css */
:root {
    --primary-color: #4f46e5;
    --spacing: 1rem;
}
.container {
    background: var(--primary-color);
}
```

```css
/* child.css */
.button {
    color: var(--primary-color); /* ✅ Works! Inherits from parent */
    margin: var(--spacing);
}
```

This is standard CSS behavior - scoping only affects **selectors**, not CSS custom properties.

### 6. Global Styles with `global.css`

For truly global CSS (resets, design tokens, font-faces), use the **`global.css` convention**:

```rust
html! {
    <!DOCTYPE html>
    <html>
        <head>
            <style src="global.css" />       // Unscoped, available everywhere
            <style src="button.css" />       // Scoped to component
        </head>
        <body>
            <button class="btn">"Click"</button>
        </body>
    </html>
}
```

**global.css:**

```css
:root {
    --primary: #4f46e5;
    --spacing: 1rem;
}

* {
    box-sizing: border-box;
}

body {
    margin: 0;
    font-family: system-ui;
}
```

**button.css:**

```css
.btn {
    background: var(--primary); /* ✅ Uses global variable! */
    padding: var(--spacing);
}
```

**How it works:**

-   Files named `global.css` are **NOT scoped** - no `[data-scopeid]` added
-   Injected **before** scoped styles (lower specificity)
-   **Skips validation** - opt-out of strict class checking
-   Perfect for design tokens that all components share

**Use global.css for:**

-   ✅ CSS variables (`:root { --var: value; }`)
-   ✅ Resets (`*`, `html`, `body` styles)
-   ✅ Font faces (`@font-face`)
-   ✅ Design tokens shared across components

**Don't use global.css for:**

-   ❌ Component-specific classes (use scoped CSS)
-   ❌ Layouts (use scoped CSS per component)

### 7. Compile-Time CSS Validation

Azumi validates your CSS at compile time and shows **exact locations** of errors.

**Example Error:**

```rust
html! {
    <style src="button.css" />
    <button class="btn-primary">"Click Me"</button>
    //              ^^^^^^^^^^^
    //              error: CSS class 'btn-primary' is not defined in any CSS file
}
```

**The error appears:**

-   ✅ At the **exact line and column** in your editor
-   ✅ With IDE underlining (red squiggly)
-   ✅ Click to jump directly to the problem
-   ✅ Same experience as regular Rust compiler errors

**Prevents:**

-   Typos in class names
-   Removed CSS that's still used
-   Dead CSS that's never used

### 8. HTMX Integration

Server-side rendering is back. Azumi + HTMX is a powerful combo.

```rust
html! {
    <style src="button.css" />
    <button
        hx-post="/clicked"
        hx-swap="outerHTML"
        class="btn">
        "Click Me"
    </button>
}
```

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
azumi = "0.2.0"
azumi-macros = "0.2.0"
```

---

## 🛠️ Development

To run the demo project:

```bash
cd demo
cargo run
```

Visit `http://localhost:8081` to see all examples in action.

---

## 🔧 Editor Setup

### Recommended: CSS Peek Extension (VS Code)

To get "Go to Definition" support for your `<style src>` paths:

1. Install **CSS Peek** extension
2. Add this to your `.vscode/settings.json`:

```json
{
    "cssPeek.peekFromLanguages": ["html", "rust"],
    "cssPeek.searchFileExtensions": [".css", ".scss"]
}
```

Now you can **Ctrl+Click** (Cmd+Click on Mac) on `<style src="path/to/file.css" />` to jump to the CSS file!

---

## 🏗️ Project Structure

```
azumi/
├── azumi/          # Core library
├── macros/         # Procedural macros (html!, component)
└── demo/           # Example application
    ├── src/
    │   ├── main.rs
    │   └── examples/
    │       ├── homepage.rs
    │       ├── components.rs
    │       ├── forms.rs
    │       └── ...
    └── static/
        ├── homepage.css
        ├── forms.css
        └── ...
```

---

## 📜 License

MIT
