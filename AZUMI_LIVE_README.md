# Azumi Live: Compiler-Driven Reactive UI

Azumi Live enables **optimistic UI updates** by analyzing Rust code at compile time and generating predictable mutations as JavaScript commands. Write your logic once in Rust - the compiler handles the rest.

## Quick Start

```rust
use azumi::prelude::*;

// 1. Define state with #[azumi::live]
#[azumi::live]
pub struct Counter {
    pub count: i32,
    pub active: bool,
}

// 2. Define mutations with #[azumi::live_impl]
#[azumi::live_impl]
impl Counter {
    pub fn increment(&mut self) {
        self.count += 1;  // → Auto-prediction: "count += 1"
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;  // → Auto-prediction: "active = !active"
    }
}

// 3. Create component with #[azumi::component]
#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! {
        <div>
            <p>"Count: " {state.count}</p>
            <button on:click={state.increment}>"Add 1"</button>
            <button on:click={state.toggle}>"Toggle"</button>
        </div>
    }
}
```

---

## Macro Reference

### `#[azumi::live]`

Marks a struct as "live" (reactive) state. Automatically:

-   Adds `Serialize`/`Deserialize` derives
-   Implements `LiveState` trait
-   Generates `to_scope()` for JSON serialization

```rust
#[azumi::live]
pub struct MyState {
    pub field1: i32,
    pub field2: bool,
}
```

---

### `#[azumi::live_impl]`

Marks an impl block for mutation analysis. Automatically:

-   Analyzes each method for predictable patterns
-   Generates prediction DSL strings
-   Registers Axum action handlers

```rust
#[azumi::live_impl]
impl MyState {
    pub fn my_action(&mut self) {
        self.field1 += 1;        // → Prediction: "field1 += 1"
        self.field2 = !self.field2;  // → Prediction: "field2 = !field2"
    }
}
```

**Supported Predictions:**

| Pattern     | Rust Code          | Prediction DSL |
| ----------- | ------------------ | -------------- |
| Toggle      | `self.x = !self.x` | `x = !x`       |
| Set literal | `self.x = true`    | `x = true`     |
| Increment   | `self.x += 1`      | `x += 1`       |
| Decrement   | `self.x -= 1`      | `x -= 1`       |

---

### `#[azumi::component]`

Creates a type-safe component with auto-generated Props.

**Live state detection:** If first parameter is `state: &T` (where T is a user-defined type), the component automatically:

-   Wraps output in `<div az-scope="...">` with serialized state
-   Returns `impl Component`

```rust
#[azumi::component]
pub fn my_view<'a>(state: &'a MyState) -> impl Component + 'a {
    html! { /* ... */ }
}
```

---

## HTML Macro Syntaxes

### Event Binding: `on:event={state.method}`

**New declarative syntax** for binding events to state methods:

```html
<button on:click="{state.increment}">"Click me"</button>
```

Auto-generates:

```html
<button az-on="click call increment" data-predict="count += 1">Click me</button>
```

---

### Manual Event DSL: `az-on={...}`

For more control, use the manual DSL:

```html
<button az-on="{click" call my_action ->#target_scope}> "Do something"</button>
```

**DSL Format:** `{event} {command} {args} -> {target}`

| Part      | Description    | Examples                             |
| --------- | -------------- | ------------------------------------ |
| `event`   | DOM event      | `click`, `submit`, `input`, `change` |
| `command` | Action type    | `call`, `set`, `toggle`              |
| `args`    | Command args   | `my_method`, `field = value`         |
| `target`  | Scope selector | `#id`, `.class`, `this` (optional)   |

---

### State Scope: `az-scope={...}`

Defines reactive state boundaries:

```html
<div az-scope="{state.to_scope()}">
    <!-- State is available in this subtree -->
</div>
```

---

### Predictions: `data-predict="..."`

Hint for optimistic UI (auto-generated with `on:event`):

```html
<button data-predict="count += 1">...</button>
```

---

## Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│  User clicks button with on:click={state.increment}             │
├──────────────────────────────────────────────────────────────────┤
│  1. OPTIMISTIC: Execute data-predict="count += 1" immediately   │
│  2. SERVER: POST /_azumi/action/increment with state JSON       │
│  3. MORPH: Server returns new HTML, morphed into DOM            │
│  4. RECONCILE: If prediction wrong, server HTML wins            │
└──────────────────────────────────────────────────────────────────┘
```

**Key benefits:**

-   ⚡ **0ms perceived latency** (optimistic updates)
-   🔒 **Server authoritative** (Rust is source of truth)
-   🎯 **Single source** (no JS/Rust duplication)
-   📦 **~5KB runtime** (vs 100KB+ for React)

---

## Examples

### Like Button

```rust
#[azumi::live]
pub struct LikeButton {
    pub liked: bool,
    pub count: i32,
}

#[azumi::live_impl]
impl LikeButton {
    pub fn toggle(&mut self) {
        self.liked = !self.liked;
        self.count += if self.liked { 1 } else { -1 };
    }
}

#[azumi::component]
pub fn like_view<'a>(state: &'a LikeButton) -> impl Component + 'a {
    html! {
        <button on:click={state.toggle}>
            {if state.liked { "❤️" } else { "🤍" }}
            " " {state.count}
        </button>
    }
}
```

### Toggle Switch

```rust
#[azumi::live]
pub struct Toggle { pub enabled: bool }

#[azumi::live_impl]
impl Toggle {
    pub fn flip(&mut self) { self.enabled = !self.enabled; }
}

#[azumi::component]
pub fn toggle_view<'a>(state: &'a Toggle) -> impl Component + 'a {
    html! {
        <button on:click={state.flip}>
            {if state.enabled { "ON ✓" } else { "OFF ✗" }}
        </button>
    }
}
```

---

## Files

| File                      | Description                                       |
| ------------------------- | ------------------------------------------------- |
| `macros/src/live.rs`      | `#[azumi::live]` and `#[azumi::live_impl]` macros |
| `macros/src/component.rs` | `#[azumi::component]` with live detection         |
| `macros/src/lib.rs`       | `html!` macro with `on:event` parsing             |
| `src/lib.rs`              | `LiveState` trait definition                      |
| `client/azumi.js`         | Runtime for events, morphing, predictions         |

---

## Comparison

| Feature       | Azumi Live   | React/Next.js | Phoenix LiveView |
| ------------- | ------------ | ------------- | ---------------- |
| Bundle size   | ~5KB         | 100KB+        | ~10KB            |
| Optimistic UI | Automatic    | Manual        | Manual           |
| Type safety   | Compile-time | Runtime       | Runtime          |
| Transport     | HTTP         | HTTP          | WebSocket        |
| Scaling       | Stateless    | Stateless     | Stateful         |
