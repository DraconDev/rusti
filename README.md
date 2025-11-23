# Rusti 🦀
**A Type-Safe, Rust-Native HTML Templating Library**

Rusti is a powerful, zero-cost, type-safe HTML templating engine for Rust. It allows you to write HTML-like syntax directly in your Rust code using the `rusti!` macro.

Inspired by Go's `templ` library, Rusti brings the component model to server-side Rust. Because it compiles to native Rust code at build time, it delivers blazing fast performance with full type safety.

## 🚀 Key Features

- **Type-Safe**: All variables and expressions are checked at compile time.
- **Zero-Cost Abstraction**: Compiles directly to `std::fmt::Write` calls. No runtime parsing, no virtual DOM.
- **Rust-Native Control Flow**: Use `@if`, `@for`, and `@match` directly in your templates.
- **Component Composition**: Build complex UIs from small, reusable Rust functions.
- **Automatic XSS Protection**: All dynamic content is HTML-escaped by default.
- **Framework Agnostic**: Works with Axum, Actix-web, Rocket, or any Rust program.
- **Multiple Styling Strategies**: Choose between Tailwind, inline styles, raw CSS, or syntax-highlighted CSS blocks.

---

## 📦 Installation

Add `rusti` to your `Cargo.toml`:

```toml
[dependencies]
rusti = { git = "https://github.com/DraconDev/rusti" }
```

> **📊 How does Rusti compare to Maud, Askama, or Leptos?**  
> See our [detailed competitive analysis](COMPETITIVE_ANALYSIS.md) for a technical breakdown of trade-offs, performance, and design philosophy.


---

## ⚠️ Syntax Rules: How to Appease the Rust Compiler

`rusti!` is a macro that runs **after** the Rust compiler tokenizes your code. This means you must write valid Rust tokens first, or the compiler will crash before your template even parses.

### TL;DR: The Golden Rules

1. **Use double quotes** for attributes: `class="foo"` (not `class='foo'`)
2. **Use Tailwind or inline styles** to avoid CSS headaches entirely
3. **If you see a compiler error with `2em`**, add quotes: `"2em"`
4. **For complex CSS/JS**, use raw strings: `r#"..."#`

---

## 1. The Law of Strings: "Double Quotes Only"

In Rust, **Single Quotes (`'`)** and **Double Quotes (`"`)** are completely different data types:
- `'a'` is a `char` (a single 4-byte number).
- `"a"` is a `&str` (a string of text).

**The Rule:** HTML attributes are text, so they must always use **Double Quotes**.

| Syntax | Status | Why? |
|--------|--------|------|
| `<div class="box">` | ✅ Valid | Standard String. |
| `<div class='box'>` | ❌ Crash | Rust thinks `'b` is a character literal, but it's too long. |
| `<div class=box>` | ❌ Crash | Rust keywords (like `type`, `for`) or symbols (`//`) will break parsing. |

### Examples

```rust
// ✅ CORRECT
rusti! {
    <div class="container">
        <a href="/home">Link</a>
    </div>
}

// ❌ WRONG - Single quotes crash the compiler
rusti! {
    <div class='container'>  // Error: character literal may only contain one codepoint
        <a href='/home'>Link</a>
    </div>
}
```

---


---

## 2. Emoji & Unicode in Text

**Emojis and Unicode characters work perfectly** - you just need to use Rust variables!

**The Issue:** Rust's tokenizer runs before the `rusti!` macro, so emojis directly in text confuse the lexer.

**The Solution:** Put emoji/Unicode text in Rust string variables:

```rust
// ❌ FAILS - Direct emoji in text
rusti! {
    <p>Status: ✅</p>  // Error: identifiers cannot contain emoji
}

// ✅ WORKS - Emoji in Rust variable
let status = "Status: ✅";  // Rust strings support full Unicode
rusti! {
    <p>{status}</p>  // Perfect! Emoji renders correctly
}

// ✅ WORKS - Multiple emojis
let greeting = "Hello 👋 Welcome! 🎉";
rusti! {
    <h1>{greeting}</h1>
}
```

**Why this works:** Rust correctly tokenizes strings, then we interpolate them into the template.

---

## 3. The Law of Backticks: "Use Raw Strings"

**Backticks (\`)** are illegal in Rust source code. They do not exist as tokens.

If you need to write JSON, inline JavaScript, or attributes containing quotes, use Rust's **Raw String** syntax: `r#" ... "#`.

| Use Case | Solution |
|----------|----------|
| JSON Attribute | `data-val=r#"{ "id": 1 }"#` |
| Inline JS | `<script>{r#" console.log("Hello") "#}</script>` |
| Quotes inside Quotes | `hx-vals=r#"{"count": 1}"#` |

### Examples

```rust
// ✅ CORRECT - Using raw strings for JSON
rusti! {
    <div hx-vals=r#"{"userId": 42, "action": "update"}"#>
        Update Profile
    </div>
}

// ✅ CORRECT - Using raw strings for inline JS
rusti! {
    <script>{r#"
        console.log("Rusti rocks!");
    "#}</script>
}

// ❌ WRONG - Backticks are illegal
rusti! {
    <script>{`console.log("nope")`}</script>  // Syntax error
}

### ⚠️ Special Case: Inline JavaScript

While simple scripts like `console.log("Hello")` might accidentally work, **always use raw strings** for `<script>` tags:

```rust
// ❌ FRAGILE - Works by accident, breaks easily
rusti! {
    <script>
        console.log("This works...");
        // But this breaks: let x = 2e10;  (scientific notation error)
        // And this breaks: const msg = 'hello';  (character literal error)
    </script>
}

// ✅ ROBUST - Use raw strings for all JavaScript
rusti! {
    <script>{r#"
        console.log("Safe!");
        const config = { timeout: 2e10 };  // Scientific notation OK
        const msg = 'hello';                // Single quotes OK
        document.querySelector('#app').textContent = "Rusti";
    "#}</script>
}

// ✅ BEST - Link external scripts (recommended for production)
rusti! {
    <script src="/static/app.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/htmx.org"></script>
}
```

> **💡 Does using raw strings reduce type safety?**  
> No! CSS and JavaScript are not Rust code, so no templating library can type-check them (Maud, Askama, Leptos all treat them as opaque strings). Raw strings make this boundary **explicit** rather than hiding it. Your Rust expressions (`{variables}`, `@if`, etc.) are still fully type-checked at compile-time.



---

## 3. The Law of CSS: "5 Ways to Style"

Rusti is flexible. Because it runs at compile-time, you can choose the styling strategy that fits your project. However, because you are writing inside Rust code, you must respect the Rust Tokenizer.

### Which CSS method should I use?

- 📱 **Modern project?** → **Option A: Tailwind** (zero issues, recommended)
- 🎨 **Writing CSS by hand?** → **Option D: Naked Syntax** (syntax highlighting)
- 📋 **Pasting large CSS blocks?** → **Option C: Raw Strings** (paste anything)
- ⚡ **Quick override?** → **Option B: Inline Styles** (always safe)
- 🛠️ **Design tokens/theming?** → **Option E: Variables** (type-safe theming)

---

### Option A: Tailwind CSS (The Happy Path) 🌊

**Best for:** Modern web development.

Since Tailwind classes are just strings, they fly through the Rust compiler without issues. You can even use arbitrary values like `[2em]` safely.

```rust
rusti! {
    <div class="p-[2em] m-4 bg-slate-900 text-white rounded-lg shadow-xl">
        <h1 class="text-4xl font-bold">Tailwind works perfectly.</h1>
        <p class="mt-2 text-gray-300">No compiler errors, ever.</p>
    </div>
}
```

**Why this works:** Tailwind classes are simple string literals. The Rust tokenizer never tries to parse them as code.

---

### Option B: Inline Styles (The Safe Bet) ✅

**Best for:** Dynamic values or quick overrides.

Inline styles are written as string literals. The Rust tokenizer does not look inside strings, so you can write **anything** here—even `2em`—and it will just work.

```rust
rusti! {
    // 2em is safe here because it's inside a string!
    <div style="margin: 2em; color: #ff0000; font-size: 16px;">
        Inline styles are bulletproof.
    </div>
}
```

**Why this works:** The entire `style` attribute is a Rust string literal. The compiler treats it as opaque text.

---

### Option C: Style Blocks with Raw Strings (The "Paste" Method) 📋

**Best for:** Pasting large CSS blocks or using AI-generated CSS.

If you don't care about syntax highlighting inside your Rust file, wrap your CSS in a **Raw String** (`r#""#`). This turns off the tokenizer, letting you write whatever you want (media queries, keyframes, `2em`).

```rust
rusti! {
    <style>{r#"
        /* The compiler treats this as one big text blob */
        @media (min-width: 700px) {
            body {
                margin: 2em;
                background: url('img.png');
                font-family: 'Inter', sans-serif;
            }
        }
        
        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }
    "#}</style>
}
```

**Why this works:** Raw strings (`r#""#`) tell Rust to ignore all special characters inside. No tokenization happens.

---

### Option D: "Naked" Rust Syntax (The IDE Method) 🎨

**Best for:** Writing CSS manually while keeping **Syntax Highlighting**.

Most CSS is actually valid Rust syntax! You can write standard CSS directly in the `rusti!` macro.

**The Catch:** Only one specific pattern breaks: `<digit>e<letter>` (e.g., `2em`, `1ex`).  
This is because Rust thinks you're writing scientific notation like `2e10` (2 × 10¹⁰).

| CSS Value | Rust Tokenizer Status | How to write it |
|-----------|----------------------|-----------------|
| `10px` | ✅ Valid (Integer + Suffix) | `padding: 10px;` |
| `#fff` | ✅ Valid (Punct + Ident) | `color: #fff;` |
| `100%` | ✅ Valid (Integer + Punct) | `width: 100%;` |
| `0.5em` | ✅ Valid (Float + Suffix) | `margin: 0.5em;` |
| `2rem` | ✅ Valid (Integer + Suffix) | `font-size: 2rem;` |
| `2em` | ❌ Invalid (Scientific Notation) | `margin: "2em";` (quote it!) |
| `1ex` | ❌ Invalid (Scientific Notation) | `line-height: "1ex";` (quote it!) |

```rust
rusti! {
    <style>
        .container {
            background-color: #f4f4f4;  /* ✅ Syntax highlighting works! */
            padding: 20px;              /* ✅ 20px is valid Rust */
            width: 100%;                /* ✅ 100% is valid Rust */
            font-size: 1.5rem;          /* ✅ Float + suffix works */
            
            /* ⚠️ The only rule: Quote 'em' if it's an integer */
            margin: "2em";              /* Must quote to avoid 2e crash */
            line-height: "1.5ex";       /* Must quote 'ex' units too */
        }
    </style>
}
```

**Why this works (mostly):** CSS syntax coincidentally overlaps with Rust tokens. The only exception is the scientific notation edge case.

**Note:** You may wrap **any** CSS value in quotes for consistency if you prefer, but it's only **required** for units starting with `e` (`em`, `ex`).

---

### Option E: Variables (The Theme Method) 🛠️

**Best for:** Theming and shared design tokens.

You can inject Rust variables directly into your CSS using curly braces `{}`.

```rust
let primary_color = "#3b82f6";
let spacing = "2em";  // Note: Use a string to avoid the 2em issue
let border_radius = "8px";

rusti! {
    <style>
        .btn {
            background-color: {primary_color};
            margin-top: {spacing};
            border-radius: {border_radius};
        }
    </style>
}
```

**Why this works:** Variables are injected as strings at runtime, bypassing the tokenizer entirely.

---

## 🧱 Quick Translation Guide (JS/HTML → Rust)

Coming from web development? Here's how to translate common patterns:

| Feature | JavaScript / HTML | Rusti (Rust) | Reason |
|---------|------------------|--------------|--------|
| **Quotes** | `<div id='app'>` | `<div id="app">` | Single quotes are for `char` only. |
| **Templates** | `` `Hello ${name}` `` | `r#"Hello {name}"#` | Backticks are illegal tokens. |
| **CSS Unit** | `margin: 2em` | `margin: "2em"` | `2e` looks like broken math. |
| **URL** | `href=http://...` | `href="http://..."` | `//` looks like a comment start. |
| **Variables** | `id="${id}"` | `id={id}` | Use curly braces for Rust variables. |
| **Control** | `{% if %}` / `{#if}` | `@if` | Distinct syntax for logic. |
| **Loops** | `{% for %}` / `{#each}` | `@for` | Rust-native iteration. |
| **JSON** | `data='{"x":1}'` | `data=r#"{"x":1}"#` | Raw strings for nested quotes. |
| **Inline Script** | `<script>alert('hi')</script>` | `<script>{r#"alert('hi')"#}</script>` | Wrap JS in raw strings. |

---

## 📖 Usage Guide

### 1. Basic Component

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

---

### 2. Dynamic Content & Attributes

Inject Rust variables using curly braces `{ }`.

```rust
fn greeting(name: &str, is_admin: bool) -> impl rusti::Component + '_ {
    let role_color = if is_admin { "text-red-500" } else { "text-blue-500" };
    
    rusti! {
        <div class="card">
            {/* Interpolate text */}
            <h1>Hello, { name }!</h1>
            
            {/* Interpolate attributes (no quotes needed for variables!) */}
            <span class={role_color}>User Role</span>
            
            {/* Complex logic in attributes */}
            <button disabled={!is_admin}>Delete</button>
        </div>
    }
}
```

---

### 3. Control Flow

Rusti provides first-class support for control flow using `@`.

```rust
fn user_list(users: Vec<&str>, role: Option<&str>) -> impl rusti::Component + '_ {
    rusti! {
        <div>
            {/* Pattern Matching */}
            @match role {
                Some("admin") => { <button>Delete User</button> }
                Some(_)       => { <span>Read Only</span> }
                None          => { <a href="/login">Log in</a> }
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
            } @else {
                <p>Found { users.len() } users.</p>
            }
        </div>
    }
}
```

---

### 4. Component Composition

Components are just Rust functions. Compose them using `@`:

```rust
fn button(label: &str, variant: &str) -> impl rusti::Component + '_ {
    let class = match variant {
        "primary" => "bg-blue-600 text-white",
        "danger" => "bg-red-600 text-white",
        _ => "bg-gray-200 text-black",
    };
    
    rusti! {
        <button class={class}>{ label }</button>
    }
}

fn page() -> impl rusti::Component {
    rusti! {
        <div>
            @button("Submit", "primary")
            @button("Cancel", "secondary")
            @button("Delete", "danger")
        </div>
    }
}
```

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
            </head>
            <body class="bg-gray-100 p-8">
                <h1 class="text-4xl font-bold text-indigo-600">
                    Hello from Axum + Rusti!
                </h1>
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

1. **Use Tailwind CSS** for styling whenever possible. It eliminates tokenizer issues entirely.
2. **Prefer inline styles** for dynamic or one-off overrides.
3. **Use raw strings** (`r#""#`) for complex CSS blocks or JavaScript.
4. **Always wrap `<script>` content in raw strings** - even simple scripts can break unexpectedly.
5. **Link external scripts** (`<script src="...">`) instead of inline JavaScript in production.
6. **Extract components early** to keep templates readable.
7. **Leverage Rust's type system** - pass typed structs instead of primitives.
8. **Test component rendering** with unit tests using `render_to_string()`.

---

## 🛠️ Project Structure

- **`src/`**: Runtime library (Component trait, escaping logic).
- **`macros/`**: The procedural macro implementation (Parser, Code Generator).
- **`demo/`**: A complete example application using Axum, HTMX, and Tailwind CSS.

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