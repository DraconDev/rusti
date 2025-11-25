# Rusti Examples 📚

This document provides comprehensive, copy-paste examples demonstrating Rusti's features in real-world scenarios.

---

## Table of Contents

1. [Basic Components](#basic-components)
2. [Optional Props & Defaults](#optional-props--defaults)
3. [Typed Children & Layouts](#typed-children--layouts)
4. [HTMX Integration](#htmx-integration)
5. [Forms & Validation](#forms--validation)
6. [Styling Strategies](#styling-strategies)
7. [Advanced Patterns](#advanced-patterns)

---

## 💡 Quick Tip: Text and Quotes

Rusti **automatically strips outer quotes** from string literals in text content. This means both of these produce the same output:

```rust
<h1>"Welcome"</h1>  // Renders: Welcome
<h1>Welcome</h1>    // Renders: Welcome
```

**To show literal quotes**, use raw strings with expression syntax:
```rust
<p>{r#""This shows quotes""#}</p>  // Renders: "This shows quotes"
```

---

## Script Variable Injection

Rusti supports dynamic variable injection into client-side JavaScript using `@{ }` syntax.

### Basic Number Injection

```rust
use rusti::rusti;

fn counter_demo() -> impl rusti::Component {
    let count = 42;
    let max = 100;
    
    rusti! {
        <!DOCTYPE html>
        <html>
        <head><title>Counter Demo</title></head>
        <body>
            <div id="counter"></div>
            <script>
                const currentCount = @{ count };
                const maxCount = @{ max };
                
                document.getElementById("counter").textContent = 
                    `Count: ${currentCount} / ${maxCount}`;
            </script>
        </body>
        </html>
    }
}
```

### String Injection with @let

**Important**: Strings must be `String` type (not `&str`) for proper JavaScript output.

```rust
fn greeting_demo() -> impl rusti::Component {
    rusti! {
        <script>
            // ✅ Correct: Create String with @let
            @let user_name = "Alice".to_string();
            @let greeting = format!("Welcome, {}!", "Alice");
            
            const name = @{ user_name };
            const msg = @{ greeting };
            
            console.log(msg);  // "Welcome, Alice!"
        </script>
    }
}
```

### Arrays and Iteration

```rust
fn list_demo() -> impl rusti::Component {
    let fruits = vec!["🍎 Apple", "🍌 Banana", "🍊 Orange"];
    
    rusti! {
        <div>
            <ul id="fruit-list"></ul>
            <script>
                const list = document.getElementById("fruit-list");
                
                @for fruit in &fruits {
                    const item = document.createElement("li");
                    item.textContent = @{ fruit };
                    list.appendChild(item);
                }
            </script>
        </div>
    }
}
```

### Conditional Logic in Scripts

```rust
fn debug_demo() -> impl rusti::Component {
    let debug_mode = cfg!(debug_assertions);
    let items = vec![1, 2, 3, 4, 5];
    
    rusti! {
        <script>
            @if debug_mode {
                console.log("Debug mode enabled");
                
                @let count = items.len();
                console.log("Items:", @{ count });
            }
            
            const data = [];
            @for item in &items {
                data.push(@{ item });
            }
            
            console.log("Data:", data);
        </script>
    }
}
```

### Complex Data Serialization

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Config {
    api_url: String,
    timeout: u32,
}

fn config_demo() -> impl rusti::Component {
    let config = Config {
        api_url: "https://api.example.com".to_string(),
        timeout: 5000,
    };
    
    rusti! {
        <script>
            @let config_json = serde_json::to_string(&config).unwrap();
            const config = JSON.parse(@{ config_json });
            
            console.log("API URL:", config.api_url);
            console.log("Timeout:", config.timeout);
        </script>
    }
}
```

### Best Practice: Prefer HTMX

For most dynamic UIs, use HTMX instead of client-side JavaScript:

```rust
// ❌ Avoid: Complex client-side DOM manipulation
fn items_client_side() -> impl rusti::Component {
    let items = vec!["Item 1", "Item 2", "Item 3"];
    
    rusti! {
        <script>
            const container = document.getElementById("items");
            @for item in &items {
                const div = document.createElement("div");
                div.textContent = @{ item };
                container.appendChild(div);
            }
        </script>
    }
}

// ✅ Prefer: Server-side rendering with HTMX
fn items_htmx() -> impl rusti::Component {
    let items = vec!["Item 1", "Item 2", "Item 3"];
    
    rusti! {
        <div hx-get="/api/items" hx-trigger="load" hx-swap="innerHTML">
            @for item in &items {
                <div class="item">{item}</div>
            }
        </div>
    }
}
```

---

## Component Naming Patterns

Rusti provides three component patterns with different calling conventions.

### Pattern 1: PascalCase with Builder (Complex Components)

Use for components with 3+ props or optional props:

```rust
use rusti::component;

#[component]
fn ProfileCard(
    name: String,
    #[prop(default = "\"\".to_string())] avatar: String,
    #[prop(default = "\"gray\"".to_string())] badge_color: String,
    #[prop(default = "false")] verified: bool
) -> impl rusti::Component {
    rusti! {
        <div class="profile-card border rounded-lg p-6">
            @if !avatar.is_empty() {
                <img src={avatar} class="avatar rounded-full" />
            }
            <h3 class="text-xl font-bold">{name}</h3>
            @if verified {
                <span class="badge" style={format!("background: {}", badge_color)}>✓ Verified</span>
            }
        </div>
    }
}

// Usage: Named arguments
fn example() -> impl rusti::Component {
    rusti! {
        <div>
            {/* Minimal - uses defaults */}
            @ProfileCard(name = "Alice".to_string())
            
            {/* With avatar */}
            @ProfileCard(
                name = "Bob".to_string(),
                avatar = "/images/bob.jpg".to_string()
            )
            
            {/* Fully customized */}
            @ProfileCard(
                name = "Charlie".to_string(),
                avatar = "/images/charlie.jpg".to_string(),
                badge_color = "green".to_string(),
                verified = true
            )
        </div>
    }
}
```

**When to use:**
- 3+ properties
- Optional props with defaults
- Complex configuration

### Pattern 2: snake_case with Positional Args (Simple Components)

Use for simple components with 1-2 required props:

```rust
fn icon(name: &str, size: u32) -> impl rusti::Component + '_ {
    rusti! {
        <svg width={size.to_string()} height={size.to_string()} class="icon">
            <use href={format!("#icon-{}", name)} />
        </svg>
    }
}

fn badge(text: &str, color: &str) -> impl rusti::Component + '_ {
    rusti! {
        <span class="badge px-2 py-1 rounded" style={format!("background: {}", color)}>
            {text}
        </span>
    }
}

// Usage: Positional arguments (like normal functions)
fn example() -> impl rusti::Component {
    rusti! {
        <div>
            @icon("home", 24)
            @icon("user", 32)
            @badge("New", "red")
            @badge("Sale", "green")
        </div>
    }
}
```

**When to use:**
- 1-2 required props
- No optional props
- Simple, focused utilities

### Pattern 3: @component Variable

Use when components need conditional construction:

```rust
fn header(title: &str) -> impl rusti::Component + '_ {
    rusti! {
        <header class="header">
            <h1>{title}</h1>
        </header>
    }
}

fn admin_panel() -> impl rusti::Component {
    rusti! {
        <nav>Admin Controls</nav>
    }
}

fn page(user: &User) -> impl rusti::Component + '_ {
    // Build component conditionally
    let nav = if user.is_admin {
        admin_panel()
    } else {
        header("Welcome")
    };
    
    rusti! {
        <div>
            @nav  {/* Render the pre-built component */}
            <main>Page content here</main>
        </div>
    }
}
```

**When to use:**
- Conditional component selection
- Component needs complex setup
- Reusing same component multiple times

### Comparison Table

| Pattern | Example | Call Syntax | Use Case |
|---------|---------|-------------|----------|
| **PascalCase** | `ProfileCard` | `@ProfileCard(name = "...")` | 3+ props, optional props |
| **snake_case** | `button` | `@button("label", "class")` | 1-2 props, simple |
| **@variable** | `my_header` | `@my_header` | Conditional, pre-built |

---

## Basic Components

### Simple Button

```rust
use rusti::rusti;

fn button(label: &str, onclick: &str) -> impl rusti::Component + '_ {
    rusti! {
        <button 
            type="button"
            onclick={onclick}
            class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
            {label}
        </button>
    }
}
```

### Card Component

```rust
fn card(title: &str, description: &str) -> impl rusti::Component + '_ {
    rusti! {
        <div class="border rounded-lg shadow-md p-6 bg-white">
            <h3 class="text-xl font-bold mb-2">{title}</h3>
            <p class="text-gray-600">{description}</p>
        </div>
    }
}
```

---

## Optional Props & Defaults

### Alert Component with Variants

```rust
use rusti::component;

#[component]
fn alert(
    message: String,
    #[prop(default = "\"info\"".to_string())] variant: String,
    #[prop(default = "true")] dismissible: bool
) -> impl rusti::Component {
    let (bg_color, text_color, icon) = match variant.as_str() {
        "error" => ("bg-red-100", "text-red-800", "❌"),
        "warning" => ("bg-yellow-100", "text-yellow-800", "⚠️"),
        "success" => ("bg-green-100", "text-green-800", "✅"),
        _ => ("bg-blue-100", "text-blue-800", "ℹ️"),
    };
    
    rusti! {
        <div class={format!("p-4 rounded-lg {} {}", bg_color, text_color)}>
            <span class="mr-2">{icon}</span>
            {message}
            @if dismissible {
                <button class="ml-4 font-bold" onclick="this.parentElement.remove()">×</button>
            }
        </div>
    }
}

// Usage examples
fn demo() -> impl rusti::Component {
    rusti! {
        <div class="space-y-4">
            {/* Minimal usage - all defaults */}
            @alert(message = "This is an info message".to_string())
            
            {/* Error variant */}
            @alert(
                message = "Something went wrong!".to_string(),
                variant = "error".to_string()
            )
            
            {/* Success, non-dismissible */}
            @alert(
                message = "Operation completed!".to_string(),
                variant = "success".to_string(),
                dismissible = false
            )
        </div>
    }
}
```

### Data Table with Options

```rust
#[component]
fn data_table<T>(
    data: Vec<T>,
    #[prop(default = "true")] striped: bool,
    #[prop(default = "true")] bordered: bool,
    #[prop(default = "\"\"".to_string())] empty_message: String
) -> impl rusti::Component 
where
    T: std::fmt::Display + 'static
{
    let table_classes = format!(
        "w-full {} {}",
        if striped { "divide-y divide-gray-200" } else { "" },
        if bordered { "border border-gray-300" } else { "" }
    );
    
    rusti! {
        @if data.is_empty() {
            <p class="text-gray-500 italic">{empty_message}</p>
        } else {
            <table class={table_classes}>
                <tbody>
                    @for (idx, item) in data.iter().enumerate() {
                        <tr class={ if striped && idx % 2 == 0 { "bg-gray-50" } else { "" } }>
                            <td class="px-4 py-2">{item}</td>
                        </tr>
                    }
                </tbody>
            </table>
        }
    }
}
```

---

## Typed Children & Layouts

### Page Layout

```rust
#[component]
fn page_layout(title: String, children: impl rusti::Component) -> impl rusti::Component {
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>{title}</title>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="bg-gray-50 min-h-screen">
                <nav class="bg-white shadow-sm border-b border-gray-200 p-4">
                    <h1 class="text-2xl font-bold text-gray-800">{title}</h1>
                </nav>
                <main class="container mx-auto p-6">
                    @children
                </main>
                <footer class="bg-gray-800 text-white text-center p-4 mt-8">
                    <p>"© 2025 My App"</p>
                </footer>
            </body>
        </html>
    }
}

// Usage
fn home_page() -> impl rusti::Component {
    rusti! {
        @page_layout(title = "Home".to_string()) {
            <h2 class="text-3xl font-bold mb-4">"Welcome!"</h2>
            <p>"This is the home page content."</p>
        }
    }
}
```

### Card with Actions

```rust
#[component]
fn action_card(
    title: String,
    #[prop(default = "\"\"".to_string())] subtitle: String,
    children: impl rusti::Component
) -> impl rusti::Component {
    rusti! {
        <div class="bg-white rounded-lg shadow-md overflow-hidden">
            <div class="bg-gradient-to-r from-blue-500 to-purple-600 text-white p-6">
                <h2 class="text-2xl font-bold">{title}</h2>
                @if !subtitle.is_empty() {
                    <p class="text-blue-100 mt-1">{subtitle}</p>
                }
            </div>
            <div class="p-6">
                @children
            </div>
        </div>
    }
}

// Usage
fn dashboard() -> impl rusti::Component {
    rusti! {
        @action_card(
            title = "User Settings".to_string(),
            subtitle = "Manage your account".to_string()
        ) {
            <form>
                <label class="block mb-2">
                    "Email:"
                    <input type="email" class="border rounded px-2 py-1" />
                </label>
                <button type="submit" class="mt-4 px-4 py-2 bg-blue-500 text-white rounded">
                    "Save Changes"
                </button>
            </form>
        }
    }
}
```

### Collapsible Section

```rust
#[component]
fn collapsible(
    title: String,
    #[prop(default = "false")] initially_open: bool,
    children: impl rusti::Component
) -> impl rusti::Component {
    let display = if initially_open { "block" } else { "none" };
    
    rusti! {
        <div class="border rounded-lg mb-2">
            <button 
                class="w-full text-left px-4 py-3 bg-gray-100 hover:bg-gray-200 font-semibold"
                onclick="this.nextElementSibling.style.display = this.nextElementSibling.style.display === 'none' ? 'block' : 'none'"
            >
                {title}
                <span class="float-right">"▼"</span>
            </button>
            <div style={format!("display: {}", display)} class="p-4">
                @children
            </div>
        </div>
    }
}
```

---

## HTMX Integration

### Live Search

```rust
fn live_search() -> impl rusti::Component {
    rusti! {
        <div class="max-w-2xl mx-auto">
            <div class="relative">
                <input 
                    type="search"
                    name="q"
                    placeholder="Search products..."
                    class="w-full px-4 py-2 border rounded-lg"
                    hx-get="/api/search"
                    hx-trigger="keyup changed delay:300ms"
                    hx-target="#search-results"
                    hx-indicator="#search-spinner"
                />
                <div id="search-spinner" class="htmx-indicator absolute right-3 top-3">
                    <svg class="animate-spin h-5 w-5 text-gray-500" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                </div>
            </div>
            <div id="search-results" class="mt-4"></div>
        </div>
    }
}
```

### Infinite Scroll

```rust
fn product_list(products: Vec<Product>, page: usize) -> impl rusti::Component {
    rusti! {
        <div class="grid grid-cols-3 gap-4">
            @for product in products.iter() {
                <div class="border rounded p-4">
                    <h3 class="font-bold">{&product.name}</h3>
                    <p class="text-gray-600">${product.price}</p>
                </div>
            }
        </div>
        
        {/* Load More Trigger */}
        <div 
            hx-get={format!("/api/products?page={}", page + 1)}
            hx-trigger="revealed"
            hx-swap="afterend"
            class="text-center py-4"
        >
            <span class="htmx-indicator">"Loading more..."</span>
        </div>
    }
}
```

### Form with Optimistic UI

```rust
fn todo_item(id: i32, text: &str, completed: bool) -> impl rusti::Component {
    rusti! {
        <li 
            id={format!("todo-{}", id)} 
            class="flex items-center gap-2 p-2 border-b"
        >
            <input 
                type="checkbox"
                checked={completed}
                hx-post={format!("/api/todos/{}/toggle", id)}
                hx-target={format!("#todo-{}", id)}
                hx-swap="outerHTML"
            />
            <span class={ if completed { "line-through text-gray-400" } else { "" } }>
                {text}
            </span>
            <button 
                hx-delete={format!("/api/todos/{}", id)}
                hx-target={format!("#todo-{}", id)}
                hx-swap="outerHTML swap:1s"
                hx-confirm="Delete this todo?"
                class="ml-auto text-red-500 hover:text-red-700"
            >
                "Delete"
            </button>
        </li>
    }
}
```

### Notification with Auto-dismiss

```rust
fn notification(message: &str, duration_ms: u32) -> impl rusti::Component {
    rusti! {
        <div 
            class="fixed top-4 right-4 bg-green-500 text-white px-6 py-3 rounded shadow-lg"
            hx-on:htmx:after-settle={format!("setTimeout(() => this.remove(), {})", duration_ms)}
        >
            {message}
        </div>
    }
}
```

---

## Forms & Validation

### Login Form

```rust
fn login_form(error: Option<&str>) -> impl rusti::Component {
    rusti! {
        <form 
            hx-post="/auth/login"
            hx-target="#login-error"
            class="max-w-md mx-auto bg-white p-8 rounded-lg shadow-md"
        >
            <h2 class="text-2xl font-bold mb-6">"Sign In"</h2>
            
            @if let Some(err) = error {
                <div id="login-error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                    {err}
                </div>
            } else {
                <div id="login-error"></div>
            }
            
            <div class="mb-4">
                <label class="block text-gray-700 text-sm font-bold mb-2" for="email">
                    "Email"
                </label>
                <input 
                    type="email" 
                    id="email"
                    name="email"
                    required
                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                />
            </div>
            
            <div class="mb-6">
                <label class="block text-gray-700 text-sm font-bold mb-2" for="password">
                    "Password"
                </label>
                <input 
                    type="password" 
                    id="password"
                    name="password"
                    required
                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                />
            </div>
            
            <button 
                type="submit"
                class="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            >
                "Sign In"
            </button>
        </form>
    }
}
```

---

## Styling Strategies

### External CSS File

```rust
// main.rs
fn app() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <style src="styles/main.css" />
            </head>
            <body>
                <div class="custom-container">
                    <h1 class="custom-heading">"Styled with External CSS"</h1>
                </div>
            </body>
        </html>
    }
}
```

### Dynamic Theming

```rust
#[derive(Debug)]
struct Theme {
    primary_color: String,
    secondary_color: String,
    background_color: String,
}

fn themed_page(theme: &Theme) -> impl rusti::Component + '_ {
    rusti! {
        <html>
            <head>
                <style>
                    :root {{
                        "--primary": {&theme.primary_color};
                        "--secondary": {&theme.secondary_color};
                        "--background": {&theme.background_color};
                    }}
                    
                    body {{
                        background-color: var(--background);
                    }}
                    
                    .btn-primary {{
                        background-color: var(--primary);
                    }}
                </style>
            </head>
            <body>
                <button class="btn-primary px-4 py-2 text-white rounded">
                    "Themed Button"
                </button>
            </body>
        </html>
    }
}
```

---

## Advanced Patterns

### Higher-Order Component (Wrapper)

```rust
#[component]
fn with_auth<F>(
    is_authenticated: bool,
    render_content: F
) -> impl rusti::Component 
where
    F: Fn() -> Box<dyn rusti::Component>
{
    rusti! {
        @if is_authenticated {
            @(render_content())
        } else {
            <div class="text-center p-8">
                <p class="text-xl mb-4">"Please log in to continue"</p>
                <a href="/login" class="px-4 py-2 bg-blue-500 text-white rounded">
                    "Log In"
                </a>
            </div>
        }
    }
}
```

### Conditional Rendering with Match

```rust
#[derive(Debug)]
enum LoadingState<T> {
    Loading,
    Success(T),
    Error(String),
}

fn render_data<T: std::fmt::Display>(state: &LoadingState<T>) -> impl rusti::Component + '_ {
    rusti! {
        <div class="p-4">
            @match state {
                LoadingState::Loading => {
                    <div class="flex items-center gap-2">
                        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
                        <span>"Loading..."</span>
                    </div>
                }
                LoadingState::Success(data) => {
                    <div class="bg-green-50 p-4 rounded">
                        <p class="text-green-800">{data}</p>
                    </div>
                }
                LoadingState::Error(err) => {
                    <div class="bg-red-50 p-4 rounded">
                        <p class="text-red-800">"Error: "{err}</p>
                    </div>
                }
            }
        </div>
    }
}
```

---

## Complete Example: Task Manager

```rust
use rusti::{rusti, component};

#[derive(Debug, Clone)]
struct Task {
    id: i32,
    title: String,
    completed: bool,
}

#[component]
fn task_list(
    tasks: Vec<Task>,
    children: impl rusti::Component
) -> impl rusti::Component {
    rusti! {
        <div class="max-w-2xl mx-auto p-6">
            <h1 class="text-3xl font-bold mb-6">"Task Manager"</h1>
            
            @children
            
            <div class="space-y-2 mt-4">
                @for task in tasks.iter() {
                    @task_item(task.clone())
                }
            </div>
            
            @if tasks.is_empty() {
                <p class="text-gray-500 text-center py-8 italic">
                    "No tasks yet. Add one to get started!"
                </p>
            }
        </div>
    }
}

#[component]
fn task_item(task: Task) -> impl rusti::Component {
    rusti! {
        <div 
            id={format!("task-{}", task.id)}
            class="flex items-center gap-3 p-3 bg-white border rounded shadow-sm"
        >
            <input 
                type="checkbox"
                checked={task.completed}
                hx-post={format!("/api/tasks/{}/toggle", task.id)}
                hx-target={format!("#task-{}", task.id)}
                hx-swap="outerHTML"
                class="w-5 h-5"
            />
            <span class={ if task.completed { "flex-1 line-through text-gray-400" } else { "flex-1" } }>
                {task.title}
            </span>
            <button 
                hx-delete={format!("/api/tasks/{}", task.id)}
                hx-target={format!("#task-{}", task.id)}
                hx-swap="outerHTML swap:500ms"
                hx-confirm="Delete this task?"
                class="text-red-500 hover:text-red-700 px-2"
            >
                "×"
            </button>
        </div>
    }
}

#[component]
fn add_task_form() -> impl rusti::Component {
    rusti! {
        <form 
            hx-post="/api/tasks"
            hx-target="#task-list"
            hx-swap="beforeend"
            hx-on:htmx:after-request="this.reset()"
            class="flex gap-2"
        >
            <input 
                type="text"
                name="title"
                placeholder="Add a new task..."
                required
                class="flex-1 px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <button 
                type="submit"
                class="px-6 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600"
            >
                "Add"
            </button>
        </form>
    }
}

fn task_page(tasks: Vec<Task>) -> impl rusti::Component {
    rusti! {
        @task_list(tasks = tasks) {
            @add_task_form()
        }
    }
}
```

---

## Tips & Tricks

### 1. Component Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_button_renders() {
        let html = rusti::render_to_string(&button("Click me", "handleClick()"));
        assert!(html.contains("Click me"));
        assert!(html.contains("handleClick()"));
    }
}
```

### 2. Conditional Classes

```rust
fn dynamic_button(is_primary: bool, is_loading: bool) -> impl rusti::Component {
    let classes = format!(
        "px-4 py-2 rounded {}{}",
        if is_primary { "bg-blue-500 text-white" } else { "bg-gray-200" },
        if is_loading { " opacity-50 cursor-not-allowed" } else { "" }
    );
    
    rusti! {
        <button class={classes} disabled={is_loading}>
            @if is_loading {
                <span>"Loading..."</span>
            } else {
                <span>"Submit"</span>
            }
        </button>
    }
}
```

### 3. Reusable Icons

```rust
fn icon(name: &str, size: u32) -> impl rusti::Component {
    match name {
        "check" => rusti! {
            <svg width={size.to_string()} height={size.to_string()} viewBox="0 0 24 24" fill="currentColor">
                <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z"/>
            </svg>
        },
        "close" => rusti! {
            <svg width={size.to_string()} height={size.to_string()} viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12 19 6.41z"/>
            </svg>
        },
        _ => rusti! { <span>"?"</span> }
    }
}
```

---

For more examples, check out the [demo application](../demo/) which includes fully working implementations of these patterns!
