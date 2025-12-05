# Azumi Live: Compiler-Driven Reactive UI

> Write Rust. Get instant UI. Zero JavaScript required.

Azumi Live is a **compile-time reactive UI system** that analyzes your Rust mutations and generates optimistic client predictions automatically. The server remains authoritative while users experience zero-latency interactions.

## The Problem We Solve: "The Two-Brain Problem"

In traditional web development, you write logic **twice**:

1. **Server Logic (Rust):** `self.count += 1`
2. **Client Logic (JS):** `setCount(count + 1)` for optimistic UI

This creates:

-   **Synchronization bugs** - logic can drift apart
-   **Cognitive load** - constantly deciding "is this client or server?"
-   **Duplication** - same mutation written in two languages

**Azumi Live's Solution:** Write the logic **once** in Rust. The compiler analyzes it and generates both server handlers AND client predictions automatically.

---

## The Core Idea

```rust
#[azumi::live]
pub struct Counter { pub count: i32 }

#[azumi::live_impl(component = "counter_view")]
impl Counter {
    pub fn increment(&mut self) {
        self.count += 1;  // Compiler generates: "count = count + 1"
    }
}

#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! {
        <style>
            .btn { padding: "0.5rem 1rem"; background: "#4CAF50"; color: "white"; }
        </style>
        <button class={btn} on:click={state.increment}>
            "Count: " {state.count}
        </button>
    }
}
```

**What happens when the user clicks:**

1. ⚡ **Instant** (0ms): Prediction `count = count + 1` updates UI immediately
2. 🔄 **Background**: Server executes real Rust logic, returns new HTML
3. ✅ **Reconcile**: If prediction matches server, skip DOM update (no flicker!)
4. 🔧 **Fallback**: If mismatch, server HTML takes over (safety net)

---

## Quick Start

### 1. Define State

```rust
#[azumi::live]
pub struct MyState {
    pub value: i32,
    pub active: bool,
}
```

The `#[azumi::live]` macro:

-   Adds `Serialize`/`Deserialize` for JSON transport
-   Implements `LiveState` trait
-   Generates `to_scope()` for embedding state in HTML

### 2. Define Mutations with Component Link

```rust
#[azumi::live_impl(component = "my_view")]
impl MyState {
    pub fn increment(&mut self) { self.value += 1; }
    pub fn toggle(&mut self) { self.active = !self.active; }
}
```

The `#[azumi::live_impl(component = "...")]` macro:

-   Analyzes each method for predictable patterns
-   Generates prediction DSL strings (e.g., `value = value + 1`)
-   Links to the specified component for server-side re-rendering
-   Registers Axum action handlers at `/_azumi/action/{State}/{method}`

### 3. Create Component

```rust
#[azumi::component]
pub fn my_view<'a>(state: &'a MyState) -> impl Component + 'a {
    html! {
        <style>
            .container { padding: "1rem"; }
            .value { font-size: "2rem"; font-weight: "bold"; }
        </style>
        <div class={container}>
            <div class={value}>{state.value}</div>
            <button on:click={state.increment}>"Add 1"</button>
            <button on:click={state.toggle}>
                {if state.active { "Active ✓" } else { "Inactive ✗" }}
            </button>
        </div>
    }
}
```

---

## CSS Integration

Azumi validates CSS at compile time. **All values must be double-quoted strings:**

```rust
html! {
    <style>
        .card {
            background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)";
            padding: "1rem";
            border-radius: "8px";
            box-shadow: "0 4px 6px rgba(0, 0, 0, 0.1)";
        }
        .title { color: "#333"; font-weight: "bold"; }
    </style>
    <div class={card}>
        <h3 class={title}>"Hello World"</h3>
    </div>
}
```

**Benefits:**

-   ✅ CSS classes become Rust variables (type-checked!)
-   ✅ Typo in class name? **Compile error**
-   ✅ Automatic CSS scoping (no collisions)
-   ✅ CSS and HTML validated together

**Dynamic CSS Variables:**

```rust
html! {
    <style>
        .progress { width: "calc(var(--pct) * 100%)"; }
    </style>
    <div class={progress} style="--pct: {completion}"></div>
}
```

---

## Prediction Patterns

The compiler analyzes your Rust code and generates predictions:

| Rust Pattern       | Generated Prediction |
| ------------------ | -------------------- |
| `self.x = !self.x` | `x = !x`             |
| `self.x = true`    | `x = true`           |
| `self.x = 42`      | `x = 42`             |
| `self.x += 1`      | `x = x + 1`          |
| `self.x -= 1`      | `x = x - 1`          |

**Smart Morph Optimization:** When prediction matches server response, DOM updates are **skipped entirely** - eliminating visual flickering.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│  User clicks <button on:click={state.increment}>                │
├─────────────────────────────────────────────────────────────────┤
│  1. OPTIMISTIC: Execute prediction immediately (0ms latency)   │
│  2. CAPTURE: Save original state before prediction             │
│  3. SERVER: POST /_azumi/action/{State}/{method} with state    │
│  4. COMPARE: Extract az-scope from response, compare states    │
│  5. RECONCILE:                                                  │
│     - Match? Skip morph, update az-scope attribute only ✅      │
│     - Mismatch? Morph DOM with server HTML (safety net) ⚠️      │
│  6. ERROR: Rollback to original state if request fails 🔄       │
└─────────────────────────────────────────────────────────────────┘
```

**Key Properties:**

-   ⚡ **0ms perceived latency** via optimistic updates
-   🔒 **Server authoritative** - Rust logic is the source of truth
-   🎯 **Single source** - no JS/Rust logic duplication
-   📦 **~5KB runtime** (Idiomorph + Azumi client)
-   🚫 **No flicker** - smart morph skips unnecessary DOM updates

---

## Event Binding Syntax

### Declarative: `on:event={state.method}`

```rust
<button on:click={state.increment}>"Click me"</button>
```

Generates:

```html
<button az-on="click call Counter/increment" data-predict="count = count + 1">
    Click me
</button>
```

### Manual DSL: `az-on="..."`

For advanced control:

```rust
<button az-on="click call my_action -> #target">"Do it"</button>
```

Format: `{event} call {action} -> {target_selector}`

---

## File Structure

| File                      | Description                                       |
| ------------------------- | ------------------------------------------------- |
| `macros/src/live.rs`      | `#[azumi::live]` and `#[azumi::live_impl]` macros |
| `macros/src/component.rs` | `#[azumi::component]` with live state detection   |
| `macros/src/lib.rs`       | `html!` macro with `on:event` parsing             |
| `macros/src/style.rs`     | CSS parsing, validation, and scoping              |
| `src/lib.rs`              | `LiveState` trait, rendering utilities            |
| `client/azumi.js`         | Event delegation, predictions, smart morphing     |
| `client/idiomorph.js`     | Efficient DOM morphing library                    |

---

## Examples

### Like Button

```rust
#[azumi::live]
pub struct LikeButton { pub liked: bool, pub count: i32 }

#[azumi::live_impl(component = "like_button_view")]
impl LikeButton {
    pub fn toggle(&mut self) {
        self.liked = !self.liked;
        self.count += if self.liked { 1 } else { -1 };
    }
}

#[azumi::component]
pub fn like_button_view<'a>(state: &'a LikeButton) -> impl Component + 'a {
    html! {
        <style>
            .like_btn { font-size: "1.5rem"; cursor: "pointer"; }
        </style>
        <button class={like_btn} on:click={state.toggle}>
            {if state.liked { "❤️" } else { "🤍" }} " " {state.count}
        </button>
    }
}
```

### Tab Navigation

```rust
#[azumi::live]
pub struct TabState { pub active_tab: String }

#[azumi::live_impl(component = "tabs_view")]
impl TabState {
    pub fn select_overview(&mut self) { self.active_tab = "overview".to_string(); }
    pub fn select_features(&mut self) { self.active_tab = "features".to_string(); }
}

#[azumi::component]
pub fn tabs_view<'a>(state: &'a TabState) -> impl Component + 'a {
    html! {
        <style>
            .tab { padding: "0.5rem 1rem"; cursor: "pointer"; }
            .active { background: "#4CAF50"; color: "white"; }
        </style>
        <div>
            <button class="tab" on:click={state.select_overview}>"Overview"</button>
            <button class="tab" on:click={state.select_features}>"Features"</button>
        </div>
        <div>
            {if state.active_tab == "overview" {
                "Overview content..."
            } else {
                "Features content..."
            }}
        </div>
    }
}
```

---

## Comparison

| Feature        | Azumi Live       | React/Next.js | Phoenix LiveView | HTMX      |
| -------------- | ---------------- | ------------- | ---------------- | --------- |
| Bundle size    | ~5KB             | 100KB+        | ~10KB            | ~14KB     |
| Optimistic UI  | **Automatic**    | Manual        | Manual           | None      |
| Type safety    | **Compile-time** | Runtime       | Runtime          | None      |
| CSS validation | **Compile-time** | None          | None             | None      |
| Transport      | HTTP             | HTTP          | WebSocket        | HTTP      |
| Server state   | Stateless        | Stateless     | Stateful         | Stateless |
| Flicker-free   | **Smart skip**   | Virtual DOM   | Morph            | Morph     |

---

## Requirements

-   Rust 1.70+
-   Axum web framework
-   Modern browser (ES6+)
