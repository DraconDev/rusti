# Azumi

**Type-Safe, Compile-Time Validated HTML/CSS Templates for Rust & Axum.**

Azumi is a **strict, compile-time HTML/CSS template system** for Rust that catches bugs before they reach production. It validates CSS classes, enforces accessibility standards, checks HTML structure, and provides component-scoped styling—all with **exact error locations** in your IDE.

```rust
use azumi::html;

#[azumi::component]
fn greeting<'a>(name: &'a str) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/greeting.css" />
        <div class="welcome-card">
            <h1>"Hello, " {name} "!"</h1>
            <p class="subtitle">"Welcome to Azumi"</p>
        </div>
    }
}

// Use with named arguments
@greeting(name="World")
```

**Zero runtime overhead.** Pure string generation at compile time.

---

## 🎯 What Makes Azumi Different?

### Compile-Time Everything

Unlike traditional template engines that catch errors at runtime, Azumi validates **everything** when you compile:

-   **CSS Classes**: Every `class="..."` is checked against your CSS files. Typo? **Compile error.** Unused class defined? **Warning with exact location.**
-   **Accessibility**: Missing `alt` on images? Invalid `aria-*` attribute? **Compile error** with helpful messages.
-   **HTML Structure**: Nested `<form>` tags? `<button>` containing another button? **Compile error** before it ships.
-   **Component Props**: Wrong argument type? **Compile error.** Missing required prop? **Compile error.**

### Location-Specific Errors

When something's wrong, Azumi tells you **exactly where**:

```
error: CSS class 'user-proifle' not defined
  --> src/components/user.rs:15:23
   |
15 |         <div class="user-proifle">
   |                     ^^^^^^^^^^^^
   |
help: Did you mean 'user-profile'?
```

Your IDE shows a red squiggle **on the exact class name**. No runtime debugging, no hunting through HTML.

### Auto-Scoped CSS

Every component's CSS is automatically scoped using hash-based attributes:

```rust
html! {
    <style src="/static/button.css" />
    <button class="primary">...</button>
}
```

Azumi transforms this to:

```html
<button class="primary" data-s3a7f9c>...</button>
```

And your CSS becomes:

```css
.primary[data-s3a7f9c] {
    /* Styles only affect this component */
}
```

**No global style leaks.** No naming conflicts. No `!important` hacks.

---

## 📦 Quick Start

### Installation

```toml
[dependencies]
azumi = { git = "https://github.com/yourorg/azumi" }  # or path/crates.io
```

### Your First Component

```rust
use azumi::html;

#[azumi::component]
fn hello_world() -> impl azumi::Component {
    html! {
        <div>
            <h1>"Hello, Azumi!"</h1>
            <p>"Type-safe templates at compile time."</p>
        </div>
    }
}

// In your Axum handler
use axumi::response::Html;

async fn handler() -> Html<String> {
    Html(azumi::render_to_string(&html! { @hello_world() }))
}
```

### Run the Demo

The best way to learn Azumi is through our **progressive lesson system**:

```bash
cd demo
cargo run
# Open http://localhost:8081
```

You'll find **18 complete lessons** covering:

-   Basic templates & interpolation
-   Control flow (`@if`, `@for`, `@match`)
-   Components with props
-   CSS scoping & variables
-   Form components
-   Advanced patterns
-   HTMX integration
-   And much more!

**Each lesson is fully interactive with live code examples.**

---

## 🚀 Core Features

### 1. Strict Quoted Syntax

Azumi requires quotes around all text and attributes. This eliminates lexer ambiguity:

```rust
// ✅ Correct
<h1 class="title">"Hello World"</h1>

// ❌ Won't compile
<h1 class=title>Hello World</h1>
```

**Why?** No confusion between Rust expressions and HTML text. Your editor can syntax highlight correctly.

### 2. Rust Interpolation

Seamlessly embed Rust expressions:

```rust
let name = "Alice";
let age = 30;

html! {
    <div>
        <p>"Name: " {name}</p>
        <p>"Age: " {age.to_string()}</p>
        <p>"Status: " {if age >= 18 { "Adult" } else { "Minor" }}</p>
    </div>
}
```

**Auto-concatenation**: Adjacent strings and expressions are automatically combined.

### 3. Control Flow

#### @if / else

```rust
@if user.is_admin {
    <button class="delete-btn">"Delete"</button>
} else {
    <span class="disabled">"Read Only"</span>
}
```

#### @for loops

```rust
@for item in &cart.items {
    <div class="cart-item">
        <span class="name">{&item.name}</span>
        <span class="price">"$" {item.price}</span>
    </div>
}
```

#### @match expressions

```rust
@match order.status {
    Pending => <span class="badge yellow">"Pending"</span>,
    Shipped => <span class="badge blue">"Shipped"</span>,
    Delivered => <span class="badge green">"Delivered"</span>,
    _ => <span class="badge gray">"Unknown"</span>,
}
```

#### @let bindings

```rust
@let total = cart.items.iter().map(|i| i.price).sum::<f64>();
@let formatted = format!("${:.2}", total);

<div class="total">
    <span>"Total: " {formatted}</span>
</div>
```

### 4. Fragments (Automatic)

**Azumi automatically handles multiple root elements** - you don't need explicit fragment syntax:

```rust
#[azumi::component]
fn user_stats<'a>(name: &'a str, posts: u32, followers: u32) -> impl azumi::Component + 'a {
    html! {
        // Multiple root elements work automatically
        <div class="stat">
            <span class="label">"Posts"</span>
            <span class="value">{posts}</span>
        </div>
        <div class="stat">
            <span class="label">"Followers"</span>
            <span class="value">{followers}</span>
        </div>
    }
}

// Both elements are inserted directly into the parent
<div class="user-card">
    <h3>{user_name}</h3>
    @user_stats(name=&user_name, posts=42, followers=1337)
</div>
```

**Optional explicit syntax:** You _can_ use `<></>` for clarity, but it's not required:

### 5. Components with #[component]

#### Basic Component

All components **must** use the `#[azumi::component]` macro:

```rust
#[azumi::component]
fn card<'a>(title: &'a str, content: &'a str) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/card.css" />
        <div class="card">
            <h3 class="card-title">{title}</h3>
            <p class="card-content">{content}</p>
        </div>
    }
}

// Call with named arguments using @ syntax
@card(title="Welcome", content="This is a card component")
```

#### Components with Children

```rust
#[azumi::component]
fn panel<'a>(title: &'a str, children: impl azumi::Component) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/panel.css" />
        <div class="panel">
            <div class="panel-header">
                <h2>{title}</h2>
            </div>
            <div class="panel-body">
                {children}
            </div>
        </div>
    }
}

// Use with children
@panel(title="Settings") {
    <p>"Panel content goes here"</p>
    <button class="save-btn">"Save"</button>
}
```

#### Complex Props

```rust
#[azumi::component]
fn user_badge<'a>(
    name: &'a str,
    role: &'a str,
    is_online: bool,
    avatar_url: &'a str
) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/badge.css" />
        <div class="user-badge">
            <img src={avatar_url} alt={format!("{} avatar", name)} class="avatar" />
            <div class="info">
                <span class="name">{name}</span>
                <span class="role">{role}</span>
                @if is_online {
                    <span class="status online">"● Online"</span>
                }
            </div>
        </div>
    }
}

// Call with all named arguments
@user_badge(
    name="Alice",
    role="Admin",
    is_online=true,
    avatar_url="/avatars/alice.jpg"
)
```

**Why named arguments?** Clear, self-documenting code. No parameter order bugs.

### 6. CSS Validation & Scoping

#### Automatic Validation

```rust
html! {
    <style src="/static/button.css" />
    <button class="btn-primary">"Click Me"</button>
}
```

Azumi reads `/static/button.css` at compile time:

-   ✅ `btn-primary` defined? All good.
-   ❌ `btn-primry` (typo)? **Compile error** with suggestion.
-   ⚠️ `btn-secondary` defined but never used? **Warning** with location.

#### Auto-Scoping

```css
/* /static/button.css - before */
.btn-primary {
    background: blue;
}

/* After Azumi processing */
.btn-primary[data-s8f3a1] {
    background: blue;
}
```

**Scoping is automatic.** Your components never affect each other.

#### Global CSS Exception

Need global styles? Use `global.css`:

```rust
<style src="/static/global.css" />  // Not scoped, not validated
```

#### CSS Variables

Pass Rust values to CSS:

```rust
let width_percent = 75;
let theme_color = "#3498db";

html! {
    <style src="/static/progress.css" />
    <div class="progress-bar" --width={width_percent} --color={theme_color}>
        <span class="fill"></span>
    </div>
}
```

```css
/* /static/progress.css */
.progress-bar {
    background: #eee;
}

.fill {
    width: var(--width);
    background: var(--color);
}
```

**Type-safe CSS theming.** No JavaScript required.

### 7. Accessibility Enforcement

Azumi enforces WCAG guidelines at compile time:

#### Images Require Alt

```rust
// ❌ Error: Missing alt attribute
<img src="/logo.png" />

// ✅ Valid (descriptive alt)
<img src="/logo.png" alt="Company Logo" />

// ✅ Valid (decorative image)
<img src="/divider.png" alt="" />
```

#### Valid Input Types

```rust
// ❌ Error: Invalid input type 'txt'
// Help: Did you mean 'text'?
<input type="txt" />

// ✅ Valid
<input type="text" />
<input type="email" />
<input type="password" />
```

#### Buttons Need Labels

```rust
// ❌ Error: Button without accessible label
<button class="icon-btn"></button>

// ✅ Valid (text content)
<button class="save-btn">"Save"</button>

// ✅ Valid (aria-label)
<button class="icon-btn" aria-label="Close dialog">
    <span class="icon-close"></span>
</button>
```

#### Valid ARIA Roles

```rust
// ❌ Error: Invalid ARIA role 'menus'
// Help: Did you mean 'menu'?
<div role="menus">...</div>

// ✅ Valid
<div role="menu">...</div>
<nav role="navigation">...</nav>
```

### 8. Type-Safe Forms (Form Binding)

Azumi validates form input names against Rust structs at compile time:

```rust
#[derive(Deserialize)]
struct UserForm {
    username: String,
    email: String,
}

html! {
    // Validates that all inputs have names matching UserForm fields
    <form bind={UserForm}>
        <input name="username" /> // ✅ Valid
        <input name="usrname" />  // ❌ Compile Error: Field not found
    </form>
}
```

### 9. HTML Structure Validation

#### No Nested Forms

```rust
// ❌ Error: Forms cannot be nested
<form>
    <form>...</form>
</form>
```

#### Proper Table Structure

```rust
// ❌ Error: <tr> must be inside <thead>, <tbody>, or <tfoot>
<table>
    <tr>...</tr>
</table>

// ✅ Valid
<table>
    <tbody>
        <tr><td>"Data"</td></tr>
    </tbody>
</table>
```

#### Lists Contain Only List Items

```rust
// ❌ Error: <ul> can only contain <li>
<ul>
    <div>...</div>
</ul>

// ✅ Valid
<ul>
    <li>"Item 1"</li>
    <li>"Item 2"</li>
</ul>
```

#### No Interactive Elements in Buttons

```rust
// ❌ Error: Buttons cannot contain interactive elements
<button>
    <a href="/link">"Click"</a>
</button>

// ✅ Valid
<button>"Click"</button>
```

---

## 🎨 Advanced Features

### HTMX Integration

Azumi works seamlessly with HTMX for interactive UIs without JavaScript:

```rust
#[azumi::component]
fn todo_item<'a>(id: u32, text: &'a str, done: bool) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/todo.css" />
        <li class="todo-item" id={format!("todo-{}", id)}>
            <input
                type="checkbox"
                checked={done}
                hx-post={format!("/api/todos/{}/toggle", id)}
                hx-swap="outerHTML"
                hx-target={format!("#todo-{}", id)}
            />
            <span class="text">{text}</span>
            <button
                class="delete-btn"
                hx-delete={format!("/api/todos/{}", id)}
                hx-swap="outerHTML"
                hx-target={format!("#todo-{}", id)}
            >"×"</button>
        </li>
    }
}
```

**Server-side rendering + HTMX = Full interactivity, zero JavaScript bundle.**

### External Scripts

```rust
html! {
    <div id="chart-container"></div>

    // CDN library
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>

    // Your script
    <script src="/static/chart-init.js"></script>
}
```

### JSON-LD Structured Data

```rust
let product_json = serde_json::to_string(&product).unwrap();

html! {
    <script type="application/ld+json">
        {product_json}
    </script>
}
```

---

## 🛠️ IDE Integration

### CSS Peek Pro Extension (VS Code)

Install **CSS Peek Pro** to jump to CSS definitions:

1. Install extension
2. Hover over `src="/static/file.css"`
3. Click to jump to definition
4. Edit CSS with full autocomplete

**Full IDE support for external CSS files.**

### Error Display

Azumi errors appear directly in your IDE with exact locations:

```
error: CSS class 'button-primery' not defined
  --> src/components/button.rs:8:23
   |
 8 |         <button class="button-primery">
   |                       ^^^^^^^^^^^^^^^
   |
   = note: Did you mean 'button-primary'?
   = note: Defined in /static/button.css
```

Red squiggles, hover tooltips, quick fixes—all work out of the box.

---

## 📚 Learn More

### Explore the Demo

The `demo/` directory contains a complete Axum application with **34 progressive lessons**:

| Lesson | Topic                  | Key Concepts                         |
| ------ | ---------------------- | ------------------------------------ |
| 0      | HTML Fragments         | Multiple root elements, `<>child</>` |
| 1      | Hello World            | Basic templates, interpolation       |
| 2      | Unquoted Variables     | String interpolation                 |
| 3      | CSS Integration        | External stylesheets                 |
| 4      | CSS Validation         | Compile-time class checking          |
| 5      | Inline Interpolation   | Complex expressions                  |
| 6      | Pattern Matching       | `@match` directive                   |
| 7      | @let Directive         | Computed values                      |
| 8      | Nested Control Flow    | Combined `@if/@for/@match`           |
| 9      | List Processing        | Filtered data                        |
| 10     | Result Handling        | `@match` with `Ok/Err`               |
| 11     | Simple Components      | Reusable button components           |
| 12     | Component Children     | Passing content to components        |
| 13     | Composition            | Building UIs from simple blocks      |
| 14     | Component Variants     | Using Enums for styles               |
| 15     | Reusable Inputs        | Creating generic form fields         |
| 16     | JavaScript Integration | External libraries                   |
| 17     | HTMX Integration       | Interactive apps without JS          |
| 18     | CSS Variables          | Dynamic theming, `--custom-props`    |
| 19     | Accessibility          | Skip links, ARIA, landmarks          |
| 20     | Conditional Classes    | Dynamic class names                  |
| 21     | CSS Scoping            | Automatic scoping demo               |
| 22     | Data Tables            | Type-safe table iteration            |
| 23     | Global CSS & CDN       | `global.css`, external links         |
| 24     | Boolean Attributes     | `disabled`, `required`, `checked`    |
| 25     | Schema.org JSON-LD     | `#[derive(Schema)]` macro            |
| 26     | Multiple CSS Files     | Loading multiple stylesheets         |
| 27     | SEO Meta Tags          | `head!` macro usage                  |
| 28     | Error Handling         | 404/500 page components              |
| 29     | Advanced Composition   | Slots/Render Props pattern           |
| 30     | Loading States         | Skeleton screens                     |
| 31     | Type-Safe Forms        | Form binding validation              |
| 32     | String Optimization    | `{\"prefix\" expr}` pattern          |
| 33     | Strict Validation      | HTML structure rules                 |
| 34     | Capstone               | Social Profile (Heavier Use)         |

**Start at Lesson 0 and work your way up.** Each builds on the previous.

```bash
cd demo
cargo run
# Navigate to http://localhost:8081/lessons
```

---

## ⚡ Rules Reference

### Must Follow

1. **Quote all text and attributes**

    ```rust
    <h1 class="title">"Text"</h1>  // ✅
    <h1 class=title>Text</h1>       // ❌
    ```

2. **Use external CSS files**

    ```rust
    <style src="/static/file.css" />  // ✅
    <style>.class {}</style>           // ❌
    ```

3. **Define all CSS classes**

    - Every class must exist in the referenced CSS
    - Unused classes trigger warnings

4. **Use `#[azumi::component]` for components**

    ```rust
    #[azumi::component]            // ✅
    fn my_component() { ... }

    fn my_component() { ... }      // ❌ Won't work with @ syntax
    ```

5. **Call components with named args**

    ```rust
    @button(text="Click", variant="primary")  // ✅
    button("Click", "primary")                 // ❌
    ```

6. **Images need alt attributes**

    ```rust
    <img src="..." alt="Description" />  // ✅
    <img src="..." />                     // ❌
    ```

7. **Use valid input/button types**

    - Azumi validates against HTML spec
    - Provides suggestions for typos

8. **Buttons need accessible labels**
    - Text content, or
    - `aria-label` attribute, or
    - `title` attribute

### Exceptions

-   **Boolean attributes**: `<input disabled />` (no value needed)
-   **Global CSS**: `global.css` files bypass scoping/validation
-   **CDN links**: `<link href="https://..." />` allowed
-   **JSON scripts**: `<script type="application/ld+json">` allowed

---

## 🏗️ Architecture

```
azumi/
├── src/              # Core runtime library
│   └── lib.rs        # Component trait, render functions
├── macros/           # Procedural macros
│   ├── src/
│   │   ├── lib.rs         # html! macro entry
│   │   ├── component.rs   # #[component] macro
│   │   ├── css.rs         # CSS parser & validator
│   │   ├── validation/    # HTML/A11y validators
│   │   └── token_parser.rs
└── demo/             # Complete Axum application
    ├── src/
    │   ├── main.rs
    │   └── examples/
    │       └── lessons/   # 18 progressive lessons
    └── static/
        └── pages/         # Per-lesson CSS
```

---

## 🎯 When to Use Azumi

### Perfect For

-   **Server-side rendered apps** with Axum
-   **HTMX-based interactive UIs**
-   **Type-safe templates** for emails, PDFs, reports
-   **Component libraries** with strict guarantees
-   **Accessibility-critical applications**
-   Teams that want **compile-time safety**

### Not Ideal For

-   **Client-side SPAs** (use Leptos/Dioxus)
-   **Existing JSX codebases** (different syntax paradigm)
-   **Rapid prototyping** where you need flexibility over safety
-   Projects **requiring inline styles** (Azumi enforces external CSS)

---

## 🤝 Contributing

Contributions welcome! Check out:

-   The demo lessons for examples
-   `macros/src/validation/` for adding new checks
-   `macros/src/css.rs` for CSS features

---

## 📜 License

MIT

---

## 🚦 Getting Started Checklist

-   [ ] Clone the repository
-   [ ] Run `cd demo && cargo run`
-   [ ] Open http://localhost:8081
-   [ ] Start with Lesson 0
-   [ ] Work through all 18 lessons
-   [ ] Build your first component
-   [ ] Integrate with your Axum app
-   [ ] Enjoy compile-time safety!

**Questions?** Check the demo lessons—they answer 95% of common questions.

**Found a bug?** Open an issue with a minimal example.

**Want to contribute?** Start by adding a new lesson to the demo!
