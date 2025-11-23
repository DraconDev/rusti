# Rusti Examples & Patterns

This document provides copy-paste examples for common Rusti patterns.

## Table of Contents
- [Emoji & Unicode](#emoji--unicode)
- [CSS Styling](#css-styling)
- [JavaScript](#javascript)
- [JSON in Attributes](#json-in-attributes)
- [Component Patterns](#component-patterns)
- [HTMX Integration](#htmx-integration)
- [Datastar Integration](#datastar-integration)

---

## Emoji & Unicode

### ✅ Working Pattern
```rust
use rusti::rusti;

fn emoji_page() -> impl rusti::Component {
    let title = "Welcome! 🎉";
    let status = "Done ✅";
    let greeting = "Hello 👋 How are you?";
    
    rusti! {
        <html>
            <head><title>{title}</title></head>
            <body>
                <h1>{title}</h1>
                <p>{greeting}</p>
                <div class="status">{status}</div>
            </body>
        </html>
    }
}
```

### ❌ What Doesn't Work
```rust
// This fails - emoji directly in text
rusti! {
    <p>Status: ✅</p>  // Error: identifiers cannot contain emoji
}
```

---

## CSS Styling

### Option 1: Tailwind (Recommended)
```rust
use rusti::rusti;

fn tailwind_example() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="bg-gray-900 text-white p-8">
                <div class="max-w-4xl mx-auto space-y-4">
                    <h1 class="text-4xl font-bold">Tailwind Works!</h1>
                    <p class="text-gray-300">No CSS issues at all.</p>
                </div>
            </body>
        </html>
    }
}
```

### Option 2: Inline Styles (Always Safe)
```rust
use rusti::rusti;

fn inline_styles() -> impl rusti::Component {
    rusti! {
        <div style="margin: 2em; padding: 20px; background: #f4f4f4;">
            Inline styles can use any unit without quotes!
        </div>
    }
}
```

### Option 3: Style Block with Raw Strings
```rust
use rusti::rusti;

fn raw_string_css() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <style>{r#"
                    body {
                        margin: 2em;        /* No quotes needed in raw strings */
                        padding: 20px;
                        font-family: 'Inter', sans-serif;
                    }
                    
                    .container {
                        max-width: 1200px;
                        margin: 0 auto;
                    }
                "#}</style>
            </head>
            <body>
                <div class="container">Content</div>
            </body>
        </html>
    }
}
```

### Option 4: "Naked" CSS (Quote em Units)
```rust
use rusti::rusti;

fn naked_css() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <style>
                    body {
                        margin: "2em";              /* Quote em units */
                        background-color: #f4f4f4;  /* Hex colors work fine */
                        padding: 20px;              /* px, rem, % work without quotes */
                        font-size: 1.5rem;          /* Floats work fine */
                    }
                </style>
            </head>
            <body>Content</body>
        </html>
    }
}
```

### Option 5: CSS Variables
```rust
use rusti::rusti;

fn css_variables() -> impl rusti::Component {
    let primary_color = "#3b82f6";
    let spacing = "2em";  // Use strings to avoid lexer issues
    
    rusti! {
        <html>
            <head>
                <style>
                    .button {
                        background-color: {primary_color};
                        margin: {spacing};
                    }
                </style>
            </head>
            <body>
                <button class="button">Click me</button>
            </body>
        </html>
    }
}
```

---

## JavaScript

### Inline Scripts (Always Use Raw Strings)
```rust
use rusti::rusti;

fn inline_script() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <script>{r#"
                    console.log("Hello from Rusti!");
                    
                    // Single quotes work inside raw strings
                    const msg = 'This works!';
                    
                    // Complex JavaScript is fine
                    function greet(name) {
                        alert(`Hello, ${name}!`);
                    }
                "#}</script>
            </head>
            <body>
                <button onclick={r#"greet('World')"#}>Greet</button>
            </body>
        </html>
    }
}
```

### External Scripts (Best Practice for Production)
```rust
use rusti::rusti;

fn external_scripts() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <script src="/static/app.js"></script>
                <script src="https://unpkg.com/htmx.org"></script>
            </head>
            <body>Content</body>
        </html>
    }
}
```

---

## JSON in Attributes

### HTMX, Alpine.js, Datastar
```rust
use rusti::rusti;

fn json_attributes() -> impl rusti::Component {
    rusti! {
        <html>
            <body>
                <!-- HTMX with JSON -->
                <div hx-vals={r#"{"userId": 42, "action": "update"}"#}>
                    Update Profile
                </div>
                
                <!-- Alpine.js -->
                <div x-data={r#"{ count: 0, name: 'Alice' }"#}>
                    <p x-text="name"></p>
                </div>
                
                <!-- Datastar -->
                <div data-store={r#"{ items: [], loading: false }"#}>
                    <button data-on-click="$loading = true">Load</button>
                </div>
            </body>
        </html>
    }
}
```

---

## Component Patterns

### Simple Component
```rust
use rusti::rusti;

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
            @button("Delete", "danger")
        </div>
    }
}
```

### Component with Children
```rust
use rusti::rusti;

fn card<'a>(title: &'a str, children: impl rusti::Component + 'a) -> impl rusti::Component + 'a {
    rusti! {
        <div class="card">
            <h2>{title}</h2>
            <div class="card-body">
                {children}
            </div>
        </div>
    }
}

fn page() -> impl rusti::Component {
    rusti! {
        @card("Profile", rusti! {
            <p>User details go here</p>
        })
    }
}
```

### Conditional Rendering
```rust
use rusti::rusti;

fn user_badge(logged_in: bool, name: &str) -> impl rusti::Component + '_ {
    rusti! {
        <div class="nav">
            @if logged_in {
                <span>Welcome, {name}</span>
                <a href="/logout">Logout</a>
            } @else {
                <a href="/login">Login</a>
            }
        </div>
    }
}
```

### Loops
```rust
use rusti::rusti;

fn todo_list(items: &[&str]) -> impl rusti::Component + '_ {
    rusti! {
        <ul>
            @for item in items.iter() {
                <li>{item}</li>
            }
        </ul>
    }
}
```

### Pattern Matching
```rust
use rusti::rusti;

enum Status {
    Active,
    Pending,
    Suspended,
}

fn status_badge(status: Status) -> impl rusti::Component {
    rusti! {
        @match status {
            Status::Active => { <span class="badge-green">Active</span> }
            Status::Pending => { <span class="badge-yellow">Pending</span> }
            Status::Suspended => { <span class="badge-red">Suspended</span> }
        }
    }
}
```

---

## HTMX Integration

### Basic Example
```rust
use rusti::rusti;

fn htmx_page() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <script src="https://unpkg.com/htmx.org@1.9.10"></script>
            </head>
            <body>
                <button hx-get="/api/data" 
                        hx-target="#result"
                        hx-swap="innerHTML"
                        class="btn">
                    Load Data
                </button>
                <div id="result"></div>
            </body>
        </html>
    }
}
```

### HTMX with JSON POST
```rust
use rusti::rusti;

fn htmx_form() -> impl rusti::Component {
    rusti! {
        <form hx-post="/api/submit"
              hx-vals={r#"{"source": "rusti-form"}"#}
              hx-target="#response">
            <input type="text" name="name" />
            <button type="submit">Submit</button>
        </form>
        <div id="response"></div>
    }
}
```

---

## Datastar Integration

### Counter Example
```rust
use rusti::rusti;

fn datastar_counter() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <script type="module" src="https://cdn.jsdelivr.net/npm/@sudodevnull/datastar"></script>
            </head>
            <body>
                <div data-store={r#"{ count: 0 }"#}>
                    <p>Count: <span data-text="$count"></span></p>
                    <button data-on-click="$count++">Increment</button>
                    <button data-on-click="$count--">Decrement</button>
                </div>
            </body>
        </html>
    }
}
```

### Input Binding
```rust
use rusti::rusti;

fn datastar_input() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <script type="module" src="https://cdn.jsdelivr.net/npm/@sudodevnull/datastar"></script>
            </head>
            <body>
                <div data-store={r#"{ name: 'World' }"#}>
                    <input type="text" data-model="name" />
                    <p>Hello, <span data-text="$name"></span>!</p>
                </div>
            </body>
        </html>
    }
}
```

---

## Quick Reference: What Works & What Doesn't

### ✅ Works Great
- Tailwind CSS classes
- Inline styles (any unit)
- Raw strings for CSS/JS
- Quoted `"2em"` units
- Unquoted `2 em` (with space) - parser fixes it!
- Emojis in Rust variables
- JSON in raw string attributes
- HTMX/Datastar/Alpine.js
- Named arguments in components (`@alert(msg="Hi")`)

### ❌ Doesn't Work
- Single quotes for attributes: `class='foo'`
- Backticks: `` `code` ``
- Direct emoji in text: `<p>✅</p>`
- Unquoted `2em` (without space) - crashes Rust lexer
- JavaScript without raw strings

---

## Component Macro (Named Arguments)

Use the `#[component]` attribute to create components with named arguments:

```rust
use rusti::component;

#[component]
fn alert_box(message: String, is_error: bool) -> impl rusti::Component {
    rusti! {
        <div class={ if is_error { "bg-red-500" } else { "bg-blue-500" } }>
            {message}
        </div>
    }
}

fn page() -> impl rusti::Component {
    rusti! {
        <div>
            <!-- Named arguments are type-checked! -->
            @alert_box(
                message = "Operation Successful".to_string(),
                is_error = false
            )
        </div>
    }
}
```

---

## Tips for Success

1. **Use Tailwind** for styling whenever possible - zero tokenizer issues
2. **Use raw strings** (`r#""#`) for any CSS/JavaScript blocks
3. **Put emojis in variables** - `let text = "Hello ✅"; rusti! { <p>{text}</p> }`
4. **Always double-quote attributes** - `class="box"`, not `class='box'`
5. **Test incrementally** - add features one at a time
6. **Check `cargo check`** often - errors are caught at compile time

---

## Need Help?

- Check the [README](README.md) for full documentation
- See [Competitive Analysis](COMPETITIVE_ANALYSIS.md) for comparisons
- Review [TODO](todo.md) for known limitations and roadmap
