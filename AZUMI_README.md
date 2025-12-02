# 🚀 Azumi: Type-Safe, Compile-Time HTML for Rust

[![Crates.io](https://img.shields.io/crates/v/azumi.svg)](https://crates.io/crates/azumi)
[![Docs.rs](https://docs.rs/azumi/badge.svg)](https://docs.rs/azumi)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Azumi** is a revolutionary web framework for Rust that brings compile-time safety to HTML templating, CSS styling, and component architecture.

## 🌟 Key Features

### ✅ Compile-Time Validation

- **CSS-HTML Co-Validation**: Catches undefined classes, typos, and mismatches before runtime
- **Accessibility Validation**: Enforces alt text, ARIA roles, and proper HTML structure
- **Strict HTML Validation**: Validates tag nesting, attribute names, and document structure

### ✅ Automatic CSS Scoping

- **Cryptographic Scoping**: Each component gets a unique scope ID automatically
- **No Manual CSS Modules**: Forget about BEM or CSS-in-JS naming conventions
- **Style Isolation**: Components are automatically isolated from each other

### ✅ Hybrid Architecture

- **Server-Side Rendering**: Fast first paint and excellent SEO
- **Progressive Enhancement**: Optional client-side interactivity
- **Action System**: Simplified server-side interactivity pattern

### ✅ Developer Experience

- **Comprehensive Error Messages**: Clear, actionable compile-time errors
- **Named Arguments**: Enforced named arguments prevent positional argument bugs
- **Built-in Learning Platform**: Interactive lessons and examples

## 🚀 Quick Start

### 1. Add to your project

```toml
[dependencies]
azumi = "0.7"
```

### 2. Create your first component

```rust
use azumi::prelude::*;

#[azumi::component]
fn Greeting(name: &str) -> impl Component {
    html! {
        <style>
            .greeting { color: "#1976d2"; font-size: "1.2rem"; }
        </style>
        <div class="greeting">
            "Hello, " <strong>{name}</strong> "!"
        </div>
    }
}
```

### 3. Use in your Axum application

```rust
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async {
            azumi::render_to_string(&Greeting(name="World"))
        }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

## 📚 Core Concepts

### HTML Macro

```rust
html! {
    <div class="container">
        <h1>"Welcome to Azumi"</h1>
        <p>"This HTML is validated at compile time!"</p>
    </div>
}
```

### Component System

```rust
#[azumi::component]
fn UserCard(name: &str, age: i32, is_active: bool) -> impl Component {
    html! {
        <style>
            .card { padding: "1rem"; border: "1px solid #ddd"; }
            .active { background: "#e8f5e9"; }
        </style>
        <div class={if is_active { "card active" } else { "card" }}>
            <h3>{name}</h3>
            <p>"Age: " {age}</p>
        </div>
    }
}
```

### Action System

```rust
#[derive(Serialize, Deserialize)]
struct LikeState {
    liked: bool,
    count: i32,
}

#[azumi::action]
async fn toggle_like(state: LikeState) -> impl Component {
    let new_state = LikeState {
        liked: !state.liked,
        count: state.count + 1,
    };
    like_button(new_state)
}
```

## 🎓 Learning Resources

### Built-in Lessons

Azumi includes a comprehensive learning platform with interactive lessons:

- **Lesson 1**: Basic HTML generation
- **Lesson 6**: Component composition
- **Lesson 7**: Props and parameters
- **Lesson 10**: Advanced interactivity

### Example Projects

- **Azumi+ Demo**: Hybrid client/server interactivity
- **Todo App**: Complete CRUD application
- **Accessibility Tests**: Validated accessible components

## 🔧 Advanced Features

### CSS Validation

```rust
// This will fail at compile time:
html! {
    <div class="non-existent">  // ❌ Error: CSS class not defined
        "Content"
    </div>
}

// This passes validation:
html! {
    <style>
        .valid-class { color: red; }
    </style>
    <div class="valid-class">  // ✅ Valid
        "Content"
    </div>
}
```

### Accessibility Enforcement

```rust
// This fails accessibility validation:
html! {
    <img src="photo.jpg" />  // ❌ Error: Missing alt attribute
}

// This passes:
html! {
    <img src="photo.jpg" alt="User profile photo" />  // ✅ Valid
}
```

### Action Integration

```rust
// Simple counter with server-side state
#[azumi::action]
async fn increment_counter(count: i32) -> impl Component {
    counter_display(count + 1)
}
```

## 📈 Performance Characteristics

- **⚡ Fast First Paint**: Server-side rendering ensures instant content
- **📦 Small Bundle Size**: Minimal client-side JavaScript
- **🔒 Type Safety**: Full Rust type system integration
- **🛡️ Runtime Safety**: Compile-time validation prevents runtime errors

## 🤝 Community & Support

- **Documentation**: Comprehensive API docs and examples
- **Discord**: Join our community for help and discussion
- **GitHub**: Issues, feature requests, and contributions welcome

## 🚀 Roadmap

- **v0.8**: Enhanced action system with more extractors
- **v0.9**: WebSocket support for real-time features
- **v1.0**: Stable API and production-ready features

## 📝 License

Azumi is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

**Experience the future of Rust web development with Azumi - where compile-time safety meets web innovation!**
