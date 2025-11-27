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

Azumi is built for developers who want **Rust's safety guarantees applied to their frontend code**.

| Feature            |  **Azumi** (Rust)   |  **Templ** (Go)  | **Maud** (Rust) | **Askama** (Rust) | **Leptos** (Rust) | **React / Next.js**  |
| :----------------- | :-----------------: | :--------------: | :-------------: | :---------------: | :---------------: | :------------------: |
| **Syntax**         |  HTML-like (Macro)  | HTML-like (File) |    Rust DSL     |    Jinja-like     | JSX-like (Macro)  |         JSX          |
| **CSS Validation** | ✅ **Compile-Time** |      ❌ No       |      ❌ No      |       ❌ No       |       ❌ No       |        ❌ No         |
| **CSS Scoping**    |  ✅ **Automatic**   |    ❌ Manual     |    ❌ Manual    |     ❌ Manual     |     ❌ Manual     | ⚠️ Modules/CSS-in-JS |
| **A11y Checks**    | ✅ **Compile-Time** |      ❌ No       |      ❌ No      |       ❌ No       | ⚠️ Runtime/Linter |   ⚠️ ESLint Plugin   |
| **Runtime Cost**   |     🚀 **Zero**     |     🚀 Zero      |     🚀 Zero     |      🚀 Zero      |   ⚡ Low (WASM)   |    🐢 High (VDOM)    |
| **Ergonomics**     |      ⭐⭐⭐⭐       |     ⭐⭐⭐⭐     |     ⭐⭐⭐      |      ⭐⭐⭐       |     ⭐⭐⭐⭐      |        ⭐⭐⭐        |

### Why the ratings?

-   **Azumi**: Strict but helpful. You get HTML syntax with Rust's power. CSS and A11y errors are caught _before_ you run the app.
-   **Templ**: Great Go library, similar vibes. Lacks the deep CSS integration and strict validation of Azumi.
-   **Maud**: Fast, but writing HTML as Rust function calls (`div { "text" }`) can be hard for designers to read.
-   **Askama**: Familiar for Python/Jinja users, but string-based templates lose some type safety and IDE support compared to macros.
-   **Leptos**: Amazing for interactive apps (WASM), but brings reactivity overhead (Signals) that isn't needed for pure SSR.
-   **React/Next.js**: The giant. Incredible ecosystem, but heavy runtime (hydration), complex build tooling, and "useEffect" fatigue lower the ergonomics for simple SSR tasks.

---

## ⚡ The Rules & Features

Azumi enforces best practices so you don't have to remember them.

### ✅ Compile-Time Validation (New!)

Azumi doesn't just check your syntax; it checks your **semantics**.

1.  **CSS Validation**:

    ```rust
    // ❌ Compile Error: class 'btn-primry' not found in button.css
    <button class="btn-primry">"Click"</button>
    ```

2.  **Accessibility (A11y) Validation**:

    ```rust
    // ❌ Compile Error: <img> missing alt attribute
    <img src="cat.jpg" />

    // ❌ Compile Error: <button> must have text content or aria-label
    <button class="icon-only"></button>
    ```

3.  **HTML Structure Validation**:
    ```rust
    // ❌ Compile Error: <ul> can only contain <li>
    <ul>
        <div>"Invalid"</div>
    </ul>
    ```

### 🚀 Smart Interpolation

Azumi knows how to render your types intelligently.

```rust
// 1. Render Components directly
@UserCard(user)

// 2. Render Strings (Automatically Escaped)
<p>{user_input}</p>

// 3. Render Integers/Floats
<span>"Count: " {count}</span>
```

### 🎨 CSS Variables & Scoping

Pass Rust data into your CSS effortlessly.

```rust
let progress = 75;
let color = "red";

html! {
    <style src="bar.css" />
    // Variables --width and --bg are available in bar.css!
    <div class="progress-bar" --width={format!("{}%", progress)} --bg={color}></div>
}
```

**In `bar.css`:**

```css
.progress-bar {
    width: var(--width); /* It just works */
    background: var(--bg);
}
```

### 🧩 Fragments

Return multiple elements without wrapper `<div>` soup.

```rust
// Automatic:
html! {
    <h1>"Hello"</h1>
    <p>"World"</p>
}

// Explicit (optional):
html! {
    <>
        <h1>"Hello"</h1>
        <p>"World"</p>
    </>
}
```

### 🔌 HTMX Integration

Azumi + HTMX is the perfect stack for modern SSR.

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
