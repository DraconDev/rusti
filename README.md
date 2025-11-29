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

## 🧠 Design Philosophy: Why Azumi Works This Way

### The Problem with Traditional Template Engines

Most template systems push errors to runtime:

-   **Typo in a CSS class?** You'll find out when the page loads (if you're lucky).
-   **Invalid HTML structure?** Silently renders broken markup.
-   **Missing accessibility attributes?** Ships to production, fails WCAG audits.
-   **Form field name doesn't match your struct?** Runtime deserialization error.

**Azumi's answer**: Catch everything at compile time. Your IDE shows the error before you even save the file.

### Why External CSS, Not Inline Styles?

Azumi **requires** external CSS files (`<style src="..." />`) and **blocks** inline `<style>` tags. Here's why:

#### 1. **Full IDE Support**

External CSS files give you:

-   ✅ Autocomplete for class names and properties
-   ✅ Syntax highlighting and error checking
-   ✅ CSS linting (Stylelint, etc.)
-   ✅ Jump-to-definition with extensions like CSS Peek Pro
-   ❌ None of this works with inline styles in Rust strings

#### 2. **Compile-Time Validation**

Azumi can only validate what it can read at compile time:

-   ✅ External files are parsed during macro expansion
-   ✅ Every class is checked, unused classes are warned
-   ❌ String literals with CSS can't be validated without a CSS parser in the macro
-   ❌ Dynamic inline styles bypass all safety checks

#### 3. **Automatic CSS Scoping**

Azumi's scoping system:

-   ✅ Reads your CSS file
-   ✅ Adds unique `data-*` attributes to your HTML
-   ✅ Rewrites CSS selectors to include those attributes
-   ❌ Can't scope inline styles (they're already in the HTML)

#### 4. **Hot Reloading**

With `include_bytes!()` tracking:

-   ✅ Change your CSS file → `cargo` detects it → recompiles automatically
-   ✅ Fast iteration on styles without touching Rust code
-   ❌ Inline CSS requires recompiling the component

#### 5. **Separation of Concerns**

-   **Rust**: Handles data, logic, component structure
-   **CSS**: Handles presentation, layout, theming
-   Mixing them makes both harder to reason about

### Why Quoted Text?

Azumi requires `"Hello"` instead of `Hello`. Traditional template engines allow unquoted text because they have custom lexers. Azumi reuses Rust's lexer for speed and simplicity.

**The alternative?** Build a full lexer that understands context:

```rust
<h1>Hello {name}</h1>  // Is "Hello 2e5" valid? What about "88user"?
```

These patterns confuse Rust's lexer. By requiring quotes, we get:

-   ✅ Zero ambiguity
-   ✅ Faster compile times (no custom parsing)
-   ✅ Better IDE support (Rust syntax highlighting works)
-   ✅ Clear distinction between text and expressions

### Why Component-Scoped CSS Only?

**Global CSS leads to:**

-   Name collisions (`.button` affects everything)
-   `!important` wars
-   Fear of changing styles (will it break something else?)
-   Massive single-file stylesheets (hard to split, hard to maintain)

**Azumi's approach:**

-   Each component gets its own CSS file
-   Styles are automatically scoped to that component
-   No conflicts, ever
-   You can have `.button` in 10 different components safely

**Exception:** `global.css` for truly global styles (resets, fonts, variables). This is intentionally limited to prevent misuse.

### Why Named Arguments for Components?

```rust
// ❌ Positional (fragile)
@user_badge("Alice", "Admin", true, "/avatar.jpg")

// ✅ Named (self-documenting)
@user_badge(
    name="Alice",
    role="Admin",
    is_online=true,
    avatar_url="/avatar.jpg"
)
```

**Benefits:**

-   Order doesn't matter
-   Clear what each value represents
-   Adding optional parameters doesn't break existing calls
-   Refactoring is safer (rename detection works)

### Why Type-Safe Form Binding?

```rust
#[derive(Deserialize)]
struct UserForm {
    username: String,
    email: String,
}

<form bind={UserForm}>
    <input name="usrname" />  // ❌ Compile error: no field 'usrname'
</form>
```

**Catches:**

-   Typos in field names
-   Deleted fields still referenced in HTML
-   Type mismatches (expecting `u32` but form sends `String`)

**Alternative?** You'd find out when deserializing fails at runtime—after users submit the form.

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

### SEO with the head! Macro

Azumi provides a `head!` macro for generating SEO-friendly meta tags:

```rust
use azumi::head;

#[azumi::component]
fn product_page<'a>(product: &'a Product) -> impl azumi::Component + 'a {
    html! {
        <html>
            <head>
                {head! {
                    title: product.name.clone(),
                    description: product.description.clone(),
                    image: product.image_url.clone(),
                    type: "product"
                }}
            </head>
            <body>
                // ... product content
            </body>
        </html>
    }
}
```

**Generated output:**

```html
<title>Product Name</title>
<meta property="og:title" content="Product Name" />
<meta property="og:description" content="Product description..." />
<meta property="og:image" content="https://example.com/image.jpg" />
<meta property="og:type" content="product" />
<meta name="twitter:card" content="summary_large_image" />
<meta name="twitter:title" content="Product Name" />
<meta name="twitter:description" content="Product description..." />
<meta name="twitter:image" content="https://example.com/image.jpg" />
```

**Why this way?** Consistent meta tags across pages, fewer bugs, better social media previews.

### Schema.org Structured Data

The `#[derive(Schema)]` macro generates JSON-LD structured data for search engines:

```rust
use azumi::Schema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Schema)]
#[schema(type = "Person")]
struct Author {
    #[schema(name = "name")]
    full_name: String,

    #[schema(name = "jobTitle")]
    role: String,

    #[schema(name = "url")]
    website: String,

    #[schema(skip)]
    internal_id: u32,  // Not included in schema
}

#[azumi::component]
fn author_page<'a>(author: &'a Author) -> impl azumi::Component + 'a {
    html! {
        <html>
            <head>
                // Automatically generates proper JSON-LD
                <script type="application/ld+json">
                    {author.to_schema_script()}
                </script>
            </head>
            <body>
                <h1>{&author.full_name}</h1>
                <p>{&author.role}</p>
            </body>
        </html>
    }
}
```

**Generated JSON-LD:**

```json
{
    "@context": "https://schema.org",
    "@type": "Person",
    "name": "Alice Johnson",
    "jobTitle": "Software Engineer",
    "url": "https://alice.dev"
}
```

**Attributes:**

-   `#[schema(type = "...")]`: Sets the `@type` field (Person, Product, Organization, etc.)
-   `#[schema(name = "...")]`: Maps Rust field name to Schema.org property
-   `#[schema(skip)]`: Excludes field from schema output

**Why this matters:** Better SEO, rich search results, knowledge graph inclusion.

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

### Core Requirements

#### 1. **Quote All Text and Attributes**

```rust
<h1 class="title">"Text content"</h1>  // ✅ Correct
<h1 class=title>Text content</h1>       // ❌ Won't compile
```

**Why?** Eliminates lexer ambiguity. Rust's lexer treats unquoted text as identifiers, which causes conflicts with patterns like `Hello2e5` or `88user`.

#### 2. **Use External CSS Files Only**

```rust
// ✅ Correct - External file
<style src="/static/component.css" />

// ❌ Error - Inline styles blocked
<style>.class { color: red; }</style>

// ❌ Error - Local stylesheet link blocked
<link rel="stylesheet" href="/local.css" />
```

**Why?**

-   Enables compile-time validation
-   Allows automatic scoping
-   Provides IDE support (autocomplete, linting)
-   Enables hot-reloading via `include_bytes!()`

**Exception:** CDN stylesheets are allowed:

```rust
<link rel="stylesheet" href="https://cdn.example.com/styles.css" />  // ✅ OK
```

#### 3. **All CSS Classes Must Be Defined**

Every `class="..."` must exist in the referenced CSS file:

```rust
<style src="/static/button.css" />
<button class="btn-primary">"Click"</button>  // ✅ If defined in button.css
<button class="btn-primery">"Click"</button>  // ❌ Compile error + suggestion
```

**Unused classes trigger warnings:**

```css
/* button.css */
.btn-primary {
    ...;
} /* Used */
.btn-secondary {
    ...;
} /* ⚠️ Warning: defined but never used */
```

#### 4. **Components Must Use #[azumi::component]**

```rust
// ✅ Correct
#[azumi::component]
fn button<'a>(text: &'a str) -> impl azumi::Component + 'a {
    html! { <button>{text}</button> }
}

// ❌ Won't work with @ syntax
fn button<'a>(text: &'a str) -> impl azumi::Component + 'a {
    html! { <button>{text}</button> }
}
```

**Why?** The macro generates:

-   Proper `Component` trait implementation
-   Named argument support for `@button(text="Click")`
-   Display implementation for rendering

#### 5. **Call Components with Named Arguments**

```rust
// ✅ Correct - Self-documenting
@card(title="Welcome", content="Hello world")

// ❌ Positional arguments not supported
card("Welcome", "Hello world")
```

**Why?**

-   Order-independent (add optional params without breaking calls)
-   Self-documenting code
-   Safer refactoring

#### 6. **Images Require Alt Attributes**

```rust
// ✅ Descriptive alt text
<img src="/logo.png" alt="Company Logo" />

// ✅ Decorative image (empty alt)
<img src="/divider.png" alt="" />

// ❌ Compile error: Missing alt
<img src="/photo.jpg" />
```

**Why?** WCAG 2.1 Level A compliance. Screen readers need descriptions.

#### 7. **Valid Input/Button Types**

```rust
// ✅ Valid HTML input types
<input type="text" />
<input type="email" />
<input type="password" />
<input type="number" />

// ❌ Compile error: Invalid type 'txt'
// Help: Did you mean 'text'?
<input type="txt" />
```

**Azumi validates against HTML spec** and provides helpful suggestions for typos.

#### 8. **Buttons Need Accessible Labels**

```rust
// ✅ Text content provides label
<button class="save-btn">"Save Changes"</button>

// ✅ aria-label for icon buttons
<button class="icon-btn" aria-label="Close dialog">
    <span class="icon-x"></span>
</button>

// ✅ title attribute works too
<button class="icon-btn" title="Delete item">
    <span class="icon-trash"></span>
</button>

// ❌ Compile error: No accessible label
<button class="icon-btn"><span class="icon"></span></button>
```

**Why?** Screen reader users need to know what buttons do.

#### 9. **Strict HTML Structure Rules**

Azumi enforces semantic HTML:

```rust
// ❌ Forms cannot be nested
<form>
    <form>...</form>  // Compile error
</form>

// ❌ Tables need proper structure
<table>
    <tr>...</tr>  // Error: <tr> must be in <thead>, <tbody>, or <tfoot>
</table>

// ✅ Correct table structure
<table>
    <tbody>
        <tr><td>"Data"</td></tr>
    </tbody>
</table>

// ❌ Lists can only contain list items
<ul>
    <div>...</div>  // Compile error
</ul>

// ✅ Correct list structure
<ul>
    <li>"Item 1"</li>
    <li>"Item 2"</li>
</ul>

// ❌ Interactive elements can't be nested
<button>
    <a href="/link">"Click"</a>  // Compile error
</button>

// ❌ Paragraphs can't contain block elements
<p>
    <div>"Block"</div>  // Compile error: browsers auto-close <p>
</p>
```

**Why?** Browsers have implicit rules that silently fix invalid HTML, causing unexpected rendering. Azumi catches these at compile time.

### Special Cases & Exceptions

#### Boolean Attributes

No value needed for boolean attributes:

```rust
<input type="checkbox" disabled checked />  // ✅ Correct
<input type="text" required autofocus />    // ✅ Correct

<input disabled="true" />  // ⚠️ Works but unnecessary
```

#### Global CSS

`global.css` bypasses scoping and validation:

```rust
<style src="/static/global.css" />  // ✅ Not scoped, not validated
```

Use for: CSS resets, global fonts, CSS custom properties. **Do not use for component styles.**

#### CDN Resources

External resources from CDN are allowed:

```rust
// ✅ CDN stylesheet
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" />

// ✅ CDN script
<script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
```

#### JSON Scripts

JSON-LD scripts are allowed:

```rust
<script type="application/ld+json">
    {product_json}
</script>
```

**Why?** Structured data for SEO, not executable JavaScript.

#### External JavaScript

Must be external files or CDN links:

```rust
// ✅ External script
<script src="/static/app.js"></script>

// ✅ CDN library
<script src="https://unpkg.com/htmx.org@1.9.10"></script>

// ❌ Inline JavaScript blocked
<script>const x = 42;</script>
```

**Why?** Same reasons as CSS—IDE support, security auditing, Content Security Policy compatibility.

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
