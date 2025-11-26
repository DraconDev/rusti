# Azumi 2.0

**Type-Safe, Compile-Time Validated HTML Templates for Rust & Axum.**

Azumi is the **strictest** HTML template system for Rust. It validates your CSS at compile time with **location-specific errors**, enforces component-scoped styling, and ensures **zero dead CSS**. Every class you use must be defined. Every class you define must be used. No exceptions.

---

## 🎯 What is Azumi?

Azumi is a **compile-time HTML template macro** for Rust that:

- ✅ **Validates every CSS class** at compile time with **location-specific errors**
- ✅ Enforces **strict quoting** to eliminate lexer ambiguity
- ✅ Provides **automatic CSS scoping** for component isolation
- ✅ Prevents **dead CSS** - every class must be used
- ✅ Integrates seamlessly with **Axum** and **HTMX**
- ✅ Offers **zero runtime overhead** (everything happens at compile time)
- ✅ Enables **full IDE support** for CSS through external files

---

## ❌ What Azumi is NOT

- ❌ **Not a JavaScript Framework** - Azumi is server-side only. Use it with HTMX or Alpine.js for interactivity.
- ❌ **Not "HTML in Rust"** - It's a **macro**, not a parser. Text must be quoted.
- ❌ **Not a CSS Framework** - Azumi **validates** your custom CSS. No Tailwind, no utility classes, no framework dependencies. Write real CSS.
- ❌ **Not a Style Soup** - Stop mixing your structure, behavior, and presentation in one file. This creates impossible-to-maintain codebases.
- ❌ **Not Flexible About Styles** - Inline `<style>` tags are **blocked**. All CSS must be in external files with `<style src>`.
- ❌ **Not Lenient** - If you break the rules, it won't compile. This is intentional.

---

## 🧭 Design Philosophy

### Why So Strict?

Azumi makes **opinionated** choices to prevent common mistakes:

| Rule                     | Reason                                                                                             |
| ------------------------ | -------------------------------------------------------------------------------------------------- |
| **Mandatory Quoting**    | Prevents Rust lexer confusion with special characters like `<`, `>`, `{`, `}`.                     |
| **No Inline `<style>`**  | Forces you into **external CSS files** which get full IDE support and compile-time validation.     |
| **Component-Scoped CSS** | Automatic scoping prevents style leakage. No more "why is this button blue?" debugging.            |
| **CSS Must Match HTML**  | Every class you use must be defined in CSS. Compile error with **exact location** if not.          |
| **No Dead CSS**          | Every class you define must be used in HTML. Warning at compile time if not.                       |
| **CDN-Only `<link>`**    | Local files must use `<style src>` for scoping and validation. `<link>` is reserved for externals. |

### Design Decisions Explained

#### **Why `@` instead of `<>`?**

Azumi uses `@` to invoke Rust code (components, functions, control flow) to distinguish it from HTML tags:

```rust
<input type="text" />       // HTML element
@UserCard(name="Alice")     // Rust component
@icon("user")               // Rust function
@if logged_in { ... }       // Control flow
```

**Benefits:**

- **Clear distinction**: `@` means "this is Rust code", `<>` means "this is HTML".
- **No ambiguity**: You instantly know what's being rendered vs. what's executing logic.
- **Familiar syntax**: Similar to Razor (`@`), Blade (`@`), and JSX (`<Component>`).

That's it. No complex rules about capitalization—just use `@` for Rust, `<>` for HTML.

---

## ⚡ The Rules

Azumi is strict. Follow these rules or it won't compile.

### ✅ Must Do

1. **Quote all text:** `<h1>"Hello World"</h1>`

   - _Why?_ Prevents lexer ambiguity and enables arbitrary text content.

2. **Quote all attribute values:** `<div class="container">`

   - _Why?_ Consistent syntax, no guessing if quotes are needed.

3. **External CSS:** `<style src="styles.css" />`

   - _Why?_ **Automatic Scoping + Validation.** CSS is scoped to the component and validated at compile time.
   - _Bonus:_ Full IDE support (linting, colors, autocomplete) for your CSS files.

4. **All classes must be defined in CSS:**

   ```rust
   // ❌ Compile Error: class 'btn' not defined
   <button class="btn">"Click"</button>
   ```

   - _Why?_ Catches typos at compile time. **Error shows exact location** in your code.

5. **All CSS must be used in HTML:**

   ```css
   /* ⚠️ Warning: class 'unused' is never used */
   .unused {
     color: red;
   }
   ```

   - _Why?_ Prevents dead CSS that bloats your bundle.

6. **External JS:** `<script src="/static/app.js" />`
   - _Why?_ Use the right tools (TypeScript, ESLint, Prettier) for JavaScript.

### ❌ Not Allowed

1. **Unquoted text:** `<h1>Hello</h1>` → Compile Error
2. **Unquoted attributes:** `<div class=box>` → Compile Error
3. **Undefined CSS classes:** `<div class="typo">` → **Compile Error with location**
4. **Inline styles:** `<style>.box { color: red; }</style>` → Compile Error
5. **Inline scripts:** `<script>console.log("hi")</script>` → Compile Error
6. **Local file `<link>`:** `<link rel="stylesheet" href="/static/local.css">` → Compile Error
   - Use `<style src="/static/local.css" />` instead for automatic scoping and validation.

### ⚠️ Exceptions

1. **Boolean Attributes:** `<input disabled checked />`

   - Standard HTML behavior. No value required.

2. **JSON Data Injection:** `<script type="application/json">{json_data}</script>`

   - The **only** allowed inline script. Safe way to pass server data to client-side code.

3. **CDN Stylesheets:** `<link rel="stylesheet" href="https://cdn.example.com/font-awesome.css">`
   - External CDN links are allowed (no validation). Only _local_ files must use `<style src>`.

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
} else {
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

**Let Bindings:**

```rust
@let formatted_date = format_date(&post.created_at);
<p>"Published on " {formatted_date}</p>
```

### 3. Components with Props

Use the `#[azumi::component]` macro to create reusable components with type-safe props.

```rust
#[azumi::component]
fn UserCard(
    name: &str,
    #[prop(default = "\"Member\"")] role: &str,
) -> impl azumi::Component {
    html! {
        <style src="/static/user_card.css" />
        <div class="user-card">
            <h2>{name}</h2>
            <span class="role">{role}</span>
        </div>
    }
}

// Usage
@UserCard(name="Alice", role="Admin")
@UserCard(name="Bob")  // Uses default role="Member"
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

- Files named `global.css` are **NOT scoped** - no `[data-scopeid]` added
- Injected **before** scoped styles (lower specificity)
- **Skips validation** - opt-out of strict class checking
- Perfect for design tokens that all components share

**Use global.css for:**

- ✅ CSS variables (`:root { --var: value; }`)
- ✅ Resets (`*`, `html`, `body` styles)
- ✅ Font faces (`@font-face`)
- ✅ Design tokens shared across components

**Don't use global.css for:**

- ❌ Component-specific classes (use scoped CSS)
- ❌ Layouts (use scoped CSS per component)

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

- ✅ At the **exact line and column** in your editor
- ✅ With IDE underlining (red squiggly)
- ✅ Click to jump directly to the problem
- ✅ Same experience as regular Rust compiler errors

**Prevents:**

- Typos in class names
- Removed CSS that's still used
- Dead CSS that's never used

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

**button.css:**

```css
.btn {
  background: #4f46e5;
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 0.375rem;
}
.btn:hover {
  background: #4338ca;
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
