# Azumi

**Type-Safe, Compile-Time Validated HTML/CSS Templates for Rust & Axum.**

Azumi is a **strict** HTML/CSS template system for Rust. It validates **CSS classes**, **accessibility**, and **HTML structure** at compile time with **location-specific errors**, enforces **component-scoped styling**, and ensures **type safety**. Every class must be defined; unused CSS is detected. Zero runtime overhead.

---

## 🎯 What is Azumi?

Azumi is a **compile-time HTML/CSS macro** that:

- ✅ **Validates CSS classes** at compile time - missing classes & dead CSS with **exact locations**
- ✅ **Enforces accessibility** - `img` alt, valid inputs/buttons/ARIA, button labels
- ✅ **Validates HTML structure** - tables/lists children, no nested forms, no interactive buttons
- ✅ **Supports CSS variables** - `--var={rust_value}` syntax
- ✅ **Automatic CSS scoping** - hash-based `[data-s{hash}]` isolation
- ✅ **Strict quoted syntax** - no lexer ambiguity
- ✅ **Rust control flow** - `@if/@for/@match/@let`
- ✅ **Fragments** - `<></>` for multiple roots
- ✅ **Components** - `#[azumi::component]` with props/defaults
- ✅ **Axum/HTMX** seamless integration
- ✅ **Full IDE support** via `<style src>`
- ✅ **Zero runtime** - pure string generation

---

## ❌ What Azumi is NOT

- ❌ **No JS Framework** - Server-side + HTMX/Alpine
- ❌ **No HTML-in-Rust** - Quoted macro, not parser
- ❌ **No CSS Framework** - Validates **your** CSS (no Tailwind)
- ❌ **No Inline Styles/Scripts** - External files only
- ❌ **No Leniency** - Breaks rules? Won't compile

---

## 🧭 Design Philosophy

**Strict = Maintainable.** Prevents technical debt:

| Problem | Azumi Fix |
|---------|-----------|
| CSS typos/dead code | Compile-time validation + locations |
| Global style leaks | Auto-scoping |
| Inline mess | External CSS/JS + IDE |
| A11y/HTML bugs | Enforced rules |
| Lexer issues | Strict quoting |

**Syntax:** `@` = Rust (`@if`), `<>` = HTML.

---

## 📊 Comparison

| Feature | Azumi | Maud | Askama | Leptos | React SSR |
|---------|-------|------|--------|--------|-----------|
| CSS Validation | ✅ Exact | ❌ | ❌ | ❌ | ❌ |
| CSS Scoping | ✅ Auto | ❌ | ❌ | ❌ | CSS-in-JS |
| A11y/HTML Checks | ✅ Full | ❌ | ❌ | ❌ | ❌ |
| Zero Runtime | ✅ | ✅ | ✅ | Signals | VDOM |
| Strictness | 🔒 | ⚠️ | ⚠️ | ✅ | ⚠️ |

**Azumi unique:** Compile-time CSS/A11y/HTML validation + scoping.

---

## ⚡ Rules (Strict - Follow or Fail)

### ✅ Must

1. **Quote text/attrs:** `<h1>\"Hello\" class=\"box\">`
2. **External CSS:** `<style src=\"file.css\" />`
3. **Define all classes** - or compile error @ exact spot
4. **Images:** `<img alt=\"\" />`
5. **Valid types:** input/button types checked w/ suggestions
6. **ARIA roles:** Valid only
7. **Buttons:** Text or `aria-label`/title

### ❌ Blocked

1. Inline `<style>`/`<script>`
2. Local `<link href=\"/local.css\">` - use `<style src>`
3. Unquoted text/attrs
4. Undefined classes

### ⚠️ Exceptions

1. **Boolean attrs:** `<input disabled />`
2. **Global CSS:** `global.css` - no scope/validate
3. **CDN:** `<link href=\"https://cdn...\">`
4. **JSON:** `<script type=\"application/json\">{data}</script>`

---

## 🚀 Features & Examples

### 1. Templates

```rust
use azumi::html;

html! {
    <div class=\"container\">
        <h1>\"Hello \" {name} \"!\"</h1>
    </div>
}
```

**Interpolation:** Auto-concat `{\"$\" price}`.

### 2. Fragments

```rust
<>
    <h1>\"Title\"</h1>
    <p>\"Para\"</p>
</>
```

### 3. Control Flow

**@if/@for/@match/@let:**

```rust
@let date = format_date(now);
@if logged_in {
    @for item in items {
        @match item.status {
            Active => <span class=\"green\">\"OK\"</span>,
            _ => <span class=\"red\">\"Fail\"</span>,
        }
    }
} else {
    <p>\"Login\"</p>
}
```

**Match single expr:** No braces needed.

### 4. Components

```rust
#[azumi::component]
fn Button(text: &str, #[prop(default = \"primary\")] variant: &str) {
    html! {
        <style src=\"/static/button.css\" />
        <button class={format!(\"btn-{}\", variant)}>{text}</button>
    }
}

// Use: @Button(text=\"Click\", variant=\"secondary\")
```

### 5. CSS Features

**Scoping:** Auto `class[data-sabc123] {}`

**Variables:**

HTML: `<div class=\"bar\" --width={progress} --color={color}>`

CSS: `.bar { width: var(--width); background: var(--color); }`

**Validation:** Missing/dead CSS → errors.

### 6. Validations (Compile-Time)

- **CSS:** All classes used/defined, dead CSS
- **A11y:**
  - `img` alt req'd (empty OK for decor)
  - Input/button types valid (+ suggestions: \"text\" → \"text\")
  - ARIA roles valid
  - Buttons: text/aria-label/title req'd
- **HTML:**
  - Table: proper children (tr in tbody etc.)
  - Lists: li only
  - No nested `<form>`
  - No interactive in `<button>` (links/inputs)

Errors: **Exact line/col** in editor.

### 7. HTMX

```rust
<button hx-post=\"/api\" hx-swap=\"outerHTML\" class=\"btn\">\"Click\"</button>
```

---

## 📦 Install

```toml
[dependencies]
azumi = { path = \"./\" }  # or git/crates.io
azumi-macros = { path = \"./macros\" }
```

For Axum: See demo.

## 🛠️ Demo

```bash
cd demo
cargo run
# http://localhost:8081 - lessons + tests
```

**20 Lessons:** Progressive examples (hello → full app).

---

## 🔧 IDE

VS Code **CSS Peek** ext for `<style src>` jump-to-def.

---

## 🏗️ Structure

```
azumi/
├── src/     # Core
├── macros/  # html!/component macros + validators
└── demo/    # Axum app + 20 lessons/tests/CSS
```

## 📜 License

MIT
