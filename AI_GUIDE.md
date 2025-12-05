# AI Guide to Writing Azumi Code

> A reference guide for AI assistants helping developers write Azumi applications.

## Critical Rules

### 1. CSS Values MUST Be Double-Quoted

```rust
// ✅ CORRECT - all values quoted
.btn {
    padding: "1rem";
    background: "#4CAF50";
    color: "white";
    border-radius: "8px";
}

// ❌ WRONG - will cause compile errors or lexer issues
.btn {
    padding: 1rem;      // Lexer can't parse "1rem" as token
    background: #4CAF50; // # causes parsing issues
    color: white;        // Unquoted identifier
}
```

### 2. CSS Classes Become Rust Variables

```rust
html! {
    <style>
        .my_button { padding: "1rem"; }  // Defines variable `my_button`
    </style>
    <button class={my_button}>"Click"</button>  // Uses variable
}
```

**Naming rules:**

-   Use `snake_case` for class names (becomes Rust identifier)
-   Avoid hyphens in class names (`my-button` → use `my_button`)

### 3. Live State Requires Component Link

```rust
// State definition
#[azumi::live]
pub struct Counter { pub count: i32 }

// Implementation MUST specify component
#[azumi::live_impl(component = "counter_view")]  // ← Required!
impl Counter {
    pub fn increment(&mut self) { self.count += 1; }
}

// Component MUST match the name
#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! { /* ... */ }
}
```

### 4. Event Binding Syntax

```rust
// ✅ CORRECT - use on:event={state.method}
<button on:click={state.increment}>"Click"</button>

// ❌ WRONG - don't use closures or function calls
<button on:click={|| state.increment()}>"Click"</button>
<button on:click={state.increment()}>"Click"</button>
```

---

## Common Patterns

### Basic Component

```rust
#[azumi::component]
pub fn MyComponent(title: &str, count: i32) -> impl Component {
    html! {
        <style>
            .container { padding: "1rem"; }
            .title { font-size: "1.5rem"; color: "#333"; }
        </style>
        <div class={container}>
            <h1 class={title}>{title}</h1>
            <p>"Count: " {count}</p>
        </div>
    }
}
```

### Live Interactive Component

```rust
#[azumi::live]
pub struct TodoApp {
    pub items: Vec<String>,
    pub input: String,
}

#[azumi::live_impl(component = "todo_view")]
impl TodoApp {
    pub fn add(&mut self) {
        if !self.input.is_empty() {
            self.items.push(self.input.clone());
            self.input.clear();
        }
    }
}

#[azumi::component]
pub fn todo_view<'a>(state: &'a TodoApp) -> impl Component + 'a {
    html! {
        <style>
            .app { max-width: "400px"; margin: "0 auto"; }
            .input { padding: "0.5rem"; width: "70%"; }
            .btn { padding: "0.5rem 1rem"; background: "#4CAF50"; color: "white"; }
            .item { padding: "0.5rem"; border-bottom: "1px solid #eee"; }
        </style>
        <div class={app}>
            <input class={input} value={state.input} placeholder="Add item..." />
            <button class={btn} on:click={state.add}>"Add"</button>

            @for item in &state.items {
                <div class={item}>{item}</div>
            }
        </div>
    }
}
```

### Conditional Rendering

```rust
html! {
    // If-else expression
    {if state.active {
        "Active ✓"
    } else {
        "Inactive ✗"
    }}

    // Conditional class
    <div class={if state.active { active_class } else { inactive_class }}>
        "Content"
    </div>
}
```

### Loops

```rust
html! {
    // For loop with @for
    @for item in &state.items {
        <div>{item}</div>
    }

    // For loop with index
    @for (i, item) in state.items.iter().enumerate() {
        <div>{i + 1}". " {item}</div>
    }
}
```

### Dynamic CSS Variables

```rust
html! {
    <style>
        .progress {
            width: "calc(var(--pct) * 100%)";
            background: "#4CAF50";
            height: "20px";
        }
    </style>
    // Pass dynamic value via CSS variable
    <div class={progress} style="--pct: {completion}"></div>
}
```

---

## Component Props

### Required Props

```rust
#[azumi::component]
pub fn Card(title: &str, content: &str) -> impl Component {
    // title and content are required
}

// Usage:
Card(title = "Hello", content = "World")
```

### Optional Props with Defaults

```rust
#[azumi::component]
pub fn Button(
    label: &str,
    #[prop(default = "primary")] variant: &str,
    #[prop(default = false)] disabled: bool,
) -> impl Component {
    // ...
}

// Usage:
Button(label = "Click me")  // Uses defaults
Button(label = "Submit", variant = "secondary", disabled = true)
```

### Children Props

```rust
#[azumi::component]
pub fn Container(children: impl Component) -> impl Component {
    html! {
        <style>
            .container { padding: "1rem"; border: "1px solid #ddd"; }
        </style>
        <div class={container}>
            {children}
        </div>
    }
}

// Usage:
@Container {
    <p>"Content inside container"</p>
}
```

---

## Handler Registration

For Axum integration, register action handlers:

```rust
use axum::Router;

fn main() {
    let app = Router::new()
        .merge(azumi::action::register_actions());  // Registers all #[azumi::live_impl] handlers

    // ... serve app
}
```

---

## Common Mistakes

### ❌ Missing Component Link

```rust
// WRONG - missing component link
#[azumi::live_impl]
impl Counter { /* ... */ }

// CORRECT
#[azumi::live_impl(component = "counter_view")]
impl Counter { /* ... */ }
```

### ❌ Unquoted CSS Values

```rust
// WRONG - causes compile errors
.btn { padding: 1rem; }

// CORRECT
.btn { padding: "1rem"; }
```

### ❌ Using Hyphens in Class Names

```rust
// WRONG - can't be a Rust identifier
.my-button { }
<div class={my-button}></div>  // Error!

// CORRECT
.my_button { }
<div class={my_button}></div>
```

### ❌ Closure Event Handlers

```rust
// WRONG - don't use closures
<button on:click={|| state.increment()}>

// CORRECT - direct method reference
<button on:click={state.increment}>
```

### ❌ Missing State Reference in Component

```rust
// WRONG - state must be first param for live components
#[azumi::component]
pub fn counter_view() -> impl Component { }

// CORRECT
#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a { }
```

---

## File Structure

Typical Azumi project:

```
src/
├── main.rs           # Axum router setup
├── components/       # Reusable components
│   ├── mod.rs
│   └── button.rs
├── pages/            # Page components
│   ├── mod.rs
│   └── home.rs
└── state/            # Live state definitions
    ├── mod.rs
    └── counter.rs
```

---

## Quick Reference

| Feature       | Syntax                                    |
| ------------- | ----------------------------------------- |
| Component     | `#[azumi::component]`                     |
| Live State    | `#[azumi::live]`                          |
| State Methods | `#[azumi::live_impl(component = "name")]` |
| Event         | `on:click={state.method}`                 |
| Class         | `class={class_name}`                      |
| Conditional   | `{if cond { a } else { b }}`              |
| Loop          | `@for item in items { }`                  |
| CSS Variable  | `style="--var: {value}"`                  |
| Children      | `@ComponentName { children }`             |

---

## Prediction Patterns

These Rust patterns generate automatic client predictions:

| Rust Code          | Generated Prediction |
| ------------------ | -------------------- |
| `self.x = !self.x` | `x = !x`             |
| `self.x = true`    | `x = true`           |
| `self.x = 42`      | `x = 42`             |
| `self.x += 1`      | `x = x + 1`          |
| `self.x -= 1`      | `x = x - 1`          |

Complex logic (async, database, etc.) runs server-only with no prediction.
