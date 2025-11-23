# Rusti vs Competitors: A Technical Comparison

## The Raw String Question: Does it sacrifice rigor?

**Short answer: No.** Raw strings (`r#""#`) are a **compiler feature**, not a workaround. Here's why Rusti's approach is rigorous:

### 1. **Compile-Time Validation is Preserved**
```rust
// ✅ Rusti - Compile-time error if variable doesn't exist
let user_name = "Alice";
rusti! {
    <div>Hello, {user_name}</div>  // Type-checked!
}

// The raw string is ONLY for CSS/JS content that Rust shouldn't parse
<style>{r#"body { margin: 2em; }"#}</style>  // Still validated at compile-time as a string
```

### 2. **No Runtime Template Parsing**
Unlike Tera, Handlebars, or Jinja2-style engines, Rusti templates are **pure Rust code** at runtime. There's zero template parsing overhead.

### 3. **Raw Strings Don't Reduce Safety**
CSS and JavaScript are **not Rust**. No Rust templating library can type-check them:
- Maud treats `<style>` as a string
- Leptos treats inline scripts as strings
- Askama treats them as opaque text blocks

**The difference:** Rusti is explicit about this boundary, while others hide it.

---

## Competitive Landscape Analysis

| Category | Feature | Rusti | Maud | Askama | Leptos/Dioxus | Tera |
|----------|---------|-------|------|--------|---------------|------|
| **Safety & Compilation** | Compile‑time safety | ✅ Full (all Rust expressions validated) | ✅ Full | ✅ Full | ✅ Full | ❌ Runtime checks only |
| | Type‑checked attributes | ✅ Yes (attributes are Rust strings) | ✅ Yes | ⚠️ Partial (only simple interpolations) | ✅ Yes | ❌ No |
| **Syntax & Learning** | Syntax style | HTML‑like (familiar to web devs) | Rust‑like (custom DSL) | Jinja2‑like (template files) | RSX (React‑like JSX) | Jinja2‑like |
| | Learning curve | Low (HTML) | Medium (custom macro) | Low (Jinja2) | Medium (React concepts) | Low (Jinja2) |
| **Styling & Scripting** | CSS handling | Raw strings, naked CSS, Tailwind | Rust strings (requires raw strings for complex CSS) | External files only | Raw strings | Template strings |
| | JS handling | Raw strings, external scripts | Rust strings (raw strings needed) | External files only | Raw strings | Template strings |
| | Inline style safety | ✅ Always safe (inside Rust string) | ⚠️ Fragile (needs quoting for `2em`) | ✅ Safe (treated as string) | ✅ Safe | ✅ Safe |
| **Component Model** | Component composition | ✅ Rust functions (full IDE support) | ✅ Rust functions | ❌ Templates only | ✅ Rust components (signals) | ❌ Templates only |
| | Reactivity / state | ❌ Not built‑in (use HTMX/Alpine) | ❌ No reactivity | ❌ No reactivity | ✅ Built‑in signals & reactivity | ❌ No reactivity |
| **Performance & Runtime** | Zero‑cost abstraction | ✅ Yes (no runtime parsing) | ✅ Yes | ✅ Yes | ⚠️ Virtual DOM overhead | ❌ Runtime template rendering |
| | Build‑time impact | Minimal (macro expands) | Minimal | Minimal (template compilation) | Higher (generated RSX code) | Higher (template compilation) |
| **Ecosystem** | Framework agnostic | ✅ Works with Axum, Actix, Rocket, etc. | ✅ Works with any framework | ✅ Works with any framework | ❌ Tied to Leptos/Dioxus | ✅ Works with any framework |
| | Community & docs | Growing, focused on hypermedia | Mature, smaller community | Mature, many examples | Emerging, active development | Mature, many plugins |

---

## Detailed Comparisons

### Rusti vs Maud

**Maud** uses a custom Rust-like syntax:

```rust
// Maud
html! {
    div.container {
        h1 { "Hello" }
        @if logged_in {
            a href="/logout" { "Logout" }
        }
    }
}

// Rusti
rusti! {
    <div class="container">
        <h1>Hello</h1>
        @if logged_in {
            <a href="/logout">Logout</a>
        }
    </div>
}
```

**Trade-offs:**
- **Maud Pro:** No HTML parser needed, pure Rust syntax
- **Maud Con:** Learning curve (not HTML), verbose for large templates
- **Rusti Pro:** Looks like actual HTML, familiar to web devs
- **Rusti Con:** Requires a parser (but it's compile-time!)

**CSS/JS Handling:**
```rust
// Maud - Must escape everything or use raw strings
html! {
    style { (PreEscaped(r#"body { margin: 2em; }"#)) }
}

// Rusti - Same approach, cleaner syntax
rusti! {
    <style>{r#"body { margin: 2em; }"#}</style>
}
```

**Verdict:** Both are equally rigorous. Rusti has better DX for HTML-heavy applications.

---

### Rusti vs Askama

**Askama** uses external template files (Jinja2-style):

```html
<!-- templates/profile.html -->
<div class="profile">
    <h1>{{ user.name }}</h1>
    {% if user.is_admin %}
        <button>Delete</button>
    {% endif %}
</div>
```

```rust
// Rust code
#[derive(Template)]
#[template(path = "profile.html")]
struct ProfileTemplate {
    user: User,
}
```

**Trade-offs:**
- **Askama Pro:** Separation of concerns, designer-friendly
- **Askama Con:** No inline components, harder to refactor
- **Rusti Pro:** Components are Rust functions, full IDE support
- **Rusti Con:** Mixing HTML and Rust (some see as a pro!)

**CSS/JS Handling:**
```html
<!-- Askama - External file, opaque to Rust -->
<style>
    body { margin: 2em; }  <!-- Works fine, but no validation -->
</style>
```

**Verdict:** Askama is great for traditional web apps with designers. Rusti is better for component-driven architectures.

---

### Rusti vs Leptos/Dioxus

**Leptos/Dioxus** use RSX (React-style syntax):

```rust
// Leptos
view! {
    <div class="container">
        <h1>"Hello"</h1>
        {move || if logged_in() {
            view! { <a href="/logout">"Logout"</a> }
        } else {
            view! { <a href="/login">"Login"</a> }
        }}
    </div>
}

// Rusti
rusti! {
    <div class="container">
        <h1>Hello</h1>
        @if logged_in {
            <a href="/logout">Logout</a>
        } @else {
            <a href="/login">Login</a>
        }
    </div>
}
```

**Trade-offs:**
- **Leptos Pro:** Full-stack reactivity, Signals, WASM support
- **Leptos Con:** Tied to Leptos framework, heavier runtime
- **Rusti Pro:** Framework-agnostic, zero runtime, simpler
- **Rusti Con:** No built-in reactivity (use HTMX/Alpine)

**CSS/JS Handling:**
```rust
// Leptos - Same raw string approach
view! {
    <style>{r#"body { margin: 2em; }"#}</style>
}

// Rusti - Identical
rusti! {
    <style>{r#"body { margin: 2em; }"#}</style>
}
```

**Verdict:** Leptos is for SPAs with complex state. Rusti is for server-side rendering and hypermedia-driven apps.

---

## The "Rigor" Question: Where Does Safety Come From?

### ✅ What Rusti DOES Type-Check (Rigorous)
1. **Variable existence**: `{user_name}` fails at compile-time if undefined
2. **Component signatures**: `@button("text", wrong_type)` fails at compile-time
3. **Rust expressions**: `{user.email}` fails if `user` doesn't have `email`
4. **Control flow**: `@if` checks that condition is `bool`
5. **Iteration**: `@for` checks that iterable implements `Iterator`

### ⚠️ What Rusti CANNOT Type-Check (No template engine can)
1. **CSS validity**: `color: rgb(999, 999, 999)` won't error
2. **JavaScript syntax**: `consol.log()` won't error (typo in `console`)
3. **HTML structure**: `<div><span></div></span>` won't error (mismatched tags)
4. **Attribute values**: `href="htp://broken"` won't error (typo in `http`)

**This is the same for ALL Rust templating libraries.** The difference is:
- **Maud/Rusti**: Explicit about the boundary (raw strings)
- **Askama/Tera**: Hides it (external files)
- **Leptos/Dioxus**: Same explicit approach (raw strings)

---

## The Verdict: Is Rusti "Rigorous"?

**Yes, absolutely.** Here's why:

### 1. **Compile-Time Guarantees**
Every Rust expression is validated at compile-time. If it compiles, your variables/types are correct.

### 2. **No Runtime Template Parsing**
Unlike Tera/Handlebars, there's zero overhead. The template IS Rust code.

### 3. **Explicit Boundaries**
Raw strings make it clear: "This is CSS/JS, not Rust." This is MORE honest than hiding it.

### 4. **Progressive Enhancement**
You can start with raw strings and move to:
- **Tailwind** (eliminates CSS issues)
- **External scripts** (eliminates JS issues)
- **Type-safe builders** (if you need CSS-in-Rust)

### 5. **Industry Standard**
React uses JSX strings, Vue uses template strings, Svelte uses strings. **Every framework treats non-code as strings.**

---

## Recommendations by Use Case

| If you're building... | Use this | Why |
|----------------------|----------|-----|
| **Server-side rendered apps** | Rusti or Askama | Compile-time safety, no runtime overhead |
| **Component libraries** | Rusti or Maud | Rust functions as components |
| **SPAs with reactivity** | Leptos or Dioxus | Built-in signals and state management |
| **Designer-friendly templates** | Askama or Tera | Separation of concerns |
| **HTMX/hypermedia apps** | **Rusti** (best fit) | HTML-first, zero JS overhead |
| **Rapid prototyping** | Rusti | Lowest learning curve for web devs |

---

## Conclusion: Raw Strings Are Not a Weakness

**Raw strings are a strength:**
1. They're explicit about the Rust/non-Rust boundary
2. They prevent fragile "accidental" parsing
3. They're the same approach used by Leptos, React, Vue, Svelte
4. They compile to zero-cost code

**The alternative (parsing CSS/JS as Rust) is worse:**
- Fragile (breaks on `2em`, single quotes, etc.)
- Misleading (looks like it's validated, but it's not)
- Limits expressiveness (can't use all CSS/JS features)

**Rusti's philosophy:** Be explicit, be fast, be HTML-first. Raw strings serve all three goals.
