# Azumi

**Type-Safe, Compile-Time Validated HTML Templates for Rust & Axum.**

Azumi is a **strict** HTML template system for Rust. It validates your CSS at compile time with **location-specific errors**, enforces component-scoped styling, and ensures **type safety**. Every class you use must be defined. No exceptions.

---

## 🎯 What is Azumi?

Azumi is a **compile-time HTML template macro** for Rust that:

-   ✅ **Validates every CSS class** at compile time with **location-specific errors**
-   ✅ **Enforces accessibility** - img alt attributes, valid input types, ARIA roles
-   ✅ **Supports CSS variables** - pass dynamic values with `--variable={value}` syntax
-   ✅ Enforces **strict quoting** to eliminate lexer ambiguity
-   ✅ Provides **automatic CSS scoping** for component isolation
-   ✅ Integrates seamlessly with **Axum** and **HTMX**
-   ✅ Offers **zero runtime overhead** (everything happens at compile time)
-   ✅ Enables **full IDE support** for CSS through external files

---

## ❌ What Azumi is NOT

-   ❌ **Not a JavaScript Framework** - Azumi is server-side only. Use it with HTMX or Alpine.js for interactivity.
-   ❌ **Not "HTML in Rust"** - It's a **macro**, not a parser. Text must be quoted.
-   ❌ **Not a CSS Framework** - Azumi **validates** your custom CSS. No Tailwind, no utility classes, no framework dependencies. Write real CSS.
-   ❌ **Not a Style Soup** - Stop mixing your structure, behavior, and presentation in one file. This creates impossible-to-maintain codebases.
-   ❌ **Not Flexible About Styles** - Inline `<style>` tags are **blocked**. All CSS must be in external files with `<style src>`.
-   ❌ **Not Lenient** - If you break the rules, it won't compile. This is intentional.

---

## 🧭 Design Philosophy

### Why So Strict?

Azumi makes **opinionated** choices because most "flexible" approaches **create technical debt**:

| Approach                   | Problem                                                                                          | Azumi's Solution                                                                              |
| -------------------------- | ------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------- |
| **Inline styles**          | Undescriptive, error-prone, typos are invisible, no IDE support, poor separation of concerns     | External CSS files with full IDE support, autocomplete, linting, and compile-time validation  |
| **Utility CSS (Tailwind)** | Only saves a few characters but creates hard-to-read code, poor separation, framework dependency | Real CSS classes that are readable, maintainable, and framework-independent                   |
| **Big style blocks**       | Mixing structure, behavior, and presentation creates unmaintainable "style soup"                 | Clean separation: HTML structure in Rust, CSS presentation in separate files                  |
| **Quoted text**            | Requires extra typing, poorer syntax highlighting                                                | Prevents lexer ambiguity, enables arbitrary content, and provides type safety at compile time |
| **No CSS validation**      | Typos, dead CSS, missing definitions all go unnoticed until runtime                              | Every class validated at compile time with exact location errors                              |
| **Global CSS leakage**     | Styles from one component accidentally affect others                                             | Automatic component-scoped CSS prevents styling conflicts                                     |

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

-   **Clear distinction**: `@` means "this is Rust code", `<>` means "this is HTML".
-   **No ambiguity**: You instantly know what's being rendered vs. what's executing logic.
-   **Familiar syntax**: Similar to Razor (`@`), Blade (`@`), and JSX (`<Component>`).

That's it. No complex rules about capitalization—just use `@` for Rust, `<>` for HTML.

---

## 📊 Comparison: Azumi vs. The World

Azumi is designed specifically for **Rust SSR** with a focus on **correctness and maintainability**.

| Lib              | Compile Safety | CSS Validation |  CSS Scoping   | Ergonomics | Runtime Perf | Strictness |  SSR/HTMX  |
| ---------------- | :------------: | :------------: | :------------: | :--------: | :----------: | :--------: | :--------: |
| **Azumi**        |   ⭐⭐⭐⭐⭐   |   ✅ **Yes**   |   ✅ **Yes**   |  ⭐⭐⭐⭐  |  ⭐⭐⭐⭐⭐  | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Templ** (Go)   |    ⭐⭐⭐⭐    |     ❌ No      |     ❌ No      |  ⭐⭐⭐⭐  |  ⭐⭐⭐⭐⭐  |    ⭐⭐    |  ⭐⭐⭐⭐  |
| **Maud**         |     ⭐⭐⭐     |     ❌ No      |     ❌ No      |   ⭐⭐⭐   |   ⭐⭐⭐⭐   |    ⭐⭐    |   ⭐⭐⭐   |
| **Askama**       |    ⭐⭐⭐⭐    |     ❌ No      |     ❌ No      |   ⭐⭐⭐   |  ⭐⭐⭐⭐⭐  |    ⭐⭐    |  ⭐⭐⭐⭐  |
| **Leptos**       |   ⭐⭐⭐⭐⭐   |     ❌ No      |     ❌ No      |  ⭐⭐⭐⭐  |    ⭐⭐⭐    |   ⭐⭐⭐   |  ⭐⭐⭐⭐  |
| **React** (Next) |      ⭐⭐      |     ❌ No      | ❌ (CSS-in-JS) | ⭐⭐⭐⭐⭐ |     ⭐⭐     |     ⭐     |    ⭐⭐    |

**Azumi Wins On:**

1.  **CSS Validation**: The only library that validates CSS classes at compile time.
2.  **Scoping**: Built-in, zero-config CSS scoping.
3.  **Zero Runtime**: Compiles to pure Rust code that writes strings. No VDOM, no runtime overhead.

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

4. **All classes used in HTML must be defined in CSS:**

    ```rust
    // ❌ Compile Error: class 'btn' not defined
    <button class="btn">"Click"</button>
    ```

    - _Why?_ Catches typos at compile time. **Error shows exact location** in your code.

5. **Images must have `alt` attributes:**

    ```rust
    // ❌ Compile Error: missing alt attribute
    <img src="logo.png" />

    // ✅ Correct
    <img src="logo.png" alt="Company Logo" />
    <img src="decoration.png" alt="" />  // Empty for decorative images
    ```

    - _Why?_ Required by HTML spec. Essential for accessibility (screen readers).

6. **Input and button types must be valid:**

    ```rust
    // ❌ Compile Error: invalid type (typo)
    <input type="txt" />
    <button type="sumbit">"Submit"</button>

    // ✅ Correct
    <input type="text" />
    <button type="submit">"Submit"</button>
    ```

    - _Why?_ Catches common typos that cause silent failures. Provides helpful suggestions.

7. **ARIA roles must be valid:**

    ```rust
    // ❌ Compile Error: invalid role
    <div role="btn">"Click"</div>

    // ✅ Correct
    <button role="button">"Click"</button>
    <div role="navigation">"Nav"</div>
    ```

    - _Why?_ Invalid roles don't work. Enforces WAI-ARIA spec compliance.

8. **External JS:** `<script src="/static/app.js" />`
    - _Why?_ Use the right tools (TypeScript, ESLint, Prettier) for JavaScript.

### ❌ Not Allowed

1. **Unquoted text:** `<h1>Hello</h1>` → Compile Error
2. **Unquoted attributes:** `<div class=box>` → Compile Error
3. **Undefined CSS classes:** `<div class="typo">` → **Compile Error with location**
4. **Inline styles:** `<style>.box { color: red; }</style>` → Compile Error
    - **Why?** Creates "style soup" - mixing structure, behavior, and presentation. No IDE support, no validation, poor maintainability.
5. **Inline scripts:** `<script>console.log("hi")</script>` → Compile Error
6. **Local file `<link>`:** `<link rel="stylesheet" href="/static/pages/local.css">` → Compile Error
    - Use `<style src="/static/pages/local.css" />` instead for automatic scoping and validation.

### ⚠️ Exceptions

1. **Boolean Attributes:** `<input disabled checked />`

    - Standard HTML behavior. No value required.

2. **JSON Data Injection:** `<script type="application/json">{json_data}</script>`

    - The **only** allowed inline script. Safe way to pass server data to client-side code.

3. **CDN Stylesheets:** `<link rel="stylesheet" href="https://cdn.example.com/font-awesome.css">`

    - External CDN links are allowed (no validation). Only _local_ files must use `<style src>`.

4. **Global CSS:** `<style src="global.css" />`
    - Files named `global.css` are **exempt** from scoping and validation. Use for resets, variables, and global tokens.

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

Azumi supports both braced blocks and **ergonomic single-expression arms**:

```rust
@match status {
    // Single expression (No braces needed!)
    Status::Active => <span class="green">"Active"</span>,

    // Block with multiple statements
    Status::Pending => {
        @let msg = "Waiting...";
        <span class="orange">{msg}</span>
    },

    _ => <span class="gray">"Unknown"</span>,
}
```

**Let Bindings:**

```rust
@let formatted_date = format_date(&post.created_at);
<p>"Published on " {formatted_date}</p>
```

### 3. String Interpolation

Azumi supports smart string concatenation within expressions:

```rust
// Automatic concatenation of string literal + expression
<p>{"Price: $" (product.price)}</p>

// Equivalent to:
// format!("Price: ${}", product.price)
```

### 4. Components with Props

Use the `#[azumi::component]` macro to create reusable components with type-safe props.

````rust
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

### 5. CSS Variables

Pass dynamic values to your CSS using attribute syntax. Attributes starting with `--` are automatically converted to inline styles.

```rust
let progress = "75%";
let color = "blue";

html! {
    <style src="progress.css" />

    // Renders: <div class="bar" style="--width: 75%; --bg: blue">
    <div class="bar" --width={progress} --bg={color}>
        "Loading..."
    </div>
}
````

In `progress.css`:

```css
.bar {
    width: var(--width);
    background-color: var(--bg);
}
```

}

// Usage
@UserCard(name="Alice", role="Admin")
@UserCard(name="Bob") // Uses default role="Member"

````

### 5. Automatic CSS Scoping

Azumi reads your CSS files at compile time, generates a unique hash, and scopes your styles to the component.

**Input (`card.css`):**

```css
.card {
    background: #fff;
}
h2 {
    color: #333;
}
````

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
    }
    h2[data-s12345] {
        color: #333;
    }
</style>
<div class="card" data-s12345>
    <h2 data-s12345>Scoped Title</h2>
</div>
```

### 6. Compile-Time CSS Validation

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

### 7. HTMX Integration

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

## 🔮 Future Roadmap

Azumi is constantly evolving. Here's what's coming next:

-   **Accessibility Enforcement**: Compile-time checks for A11y rules (e.g., `<img>` missing `alt`, empty buttons).
-   **CSS Variable Injection**: Pass Rust variables directly to CSS (e.g., `<div --color={user_color}>`).
-   **LSP Support**: Dedicated Language Server for even better IDE integration.

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
