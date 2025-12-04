# Azumi Live: Compiler-Driven Optimistic UI

> **"Phoenix LiveView for Rust, without the scaling cost of WebSockets, with automatic Optimistic UI."**

## Executive Summary

Azumi Live is the architectural evolution of the Azumi framework that moves the complexity of optimistic UI from the developer to the compiler. Instead of manually writing client-side JavaScript for instant UI updates, the `#[azumi::live]` macro analyzes Rust code at compile time and automatically generates predictable mutations as JavaScript commands.

---

## The Evolution Path

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                           THE AZUMI EVOLUTION                                    │
├─────────────────┬─────────────────────┬─────────────────────────────────────────┤
│  Pure Azumi     │  Azumi+ (Current)   │  Azumi Live (Goal)                      │
│  (SSR Only)     │  az-on + actions    │  #[azumi::live] Compiler Magic          │
├─────────────────┼─────────────────────┼─────────────────────────────────────────┤
│  Full page      │  AJAX + DOM Morph   │  Compiler-Generated Optimistic UI       │
│  reload on      │  Manual az-on DSL   │  Single Source of Truth                 │
│  every action   │  Developer chooses  │  Auto-predict simple mutations          │
│                 │  client vs server   │  Fallback for complex logic             │
├─────────────────┼─────────────────────┼─────────────────────────────────────────┤
│  UX: 🐢 Slow    │  UX: ⚡ Instant     │  UX: ⚡ Instant                          │
│  DX: ✅ Simple  │  DX: ⚠️ Complex     │  DX: ✅ Simple                           │
└─────────────────┴─────────────────────┴─────────────────────────────────────────┘
```

---

## The Problem Being Solved

### The "Two-Brain" Problem

In traditional web development, you write logic **twice**:

1. **Server Logic (Rust):** `db.count += 1`
2. **Client Logic (JS):** `setCount(count + 1)` for optimistic UI

This creates:

-   **Synchronization bugs** - logic can drift apart
-   **Cognitive load** - constantly deciding "is this client or server?"
-   **Duplication** - same mutation written in two languages

### Azumi Live's Solution

Write the logic **once** in Rust. The compiler analyzes it:

```rust
#[azumi::live]
pub struct Counter { count: i32 }

impl Counter {
    pub fn increment(&mut self) {
        self.count += 1;  // Compiler: "Pure math! I'll predict this."
    }
}
```

The macro generates:

-   ✅ Server-side action handler (the real logic)
-   ✅ Client-side prediction (optimistic UI)
-   ✅ Automatic synchronization

---

## Architecture Comparison

### vs Pure Azumi (Full Page Reload)

| Aspect             | Pure Azumi       | Azumi Live            |
| ------------------ | ---------------- | --------------------- |
| Click interaction  | Full page reload | 0ms optimistic update |
| Focus/scroll state | Lost             | Preserved (morphing)  |
| User perception    | "Web 1.0"        | "Native app"          |

### vs Azumi+ (Manual Hybrid)

| Aspect                   | Azumi+                          | Azumi Live                 |
| ------------------------ | ------------------------------- | -------------------------- |
| Client logic             | Manually written `az-on`        | Auto-generated             |
| Server logic             | `#[azumi::action]`              | Same Rust method           |
| Sync bugs                | Possible (two sources of truth) | Impossible (single source) |
| Developer cognitive load | High                            | Low                        |

### vs Next.js (React SPA)

| Aspect              | Next.js              | Azumi Live      |
| ------------------- | -------------------- | --------------- |
| Initial load        | 🐢 Heavy (100KB+ JS) | ⚡ Light (~5KB) |
| Bundle size         | Megabytes            | Kilobytes       |
| Time to Interactive | 2-5 seconds          | Instant         |
| Hydration           | Yes (complex)        | No              |
| Low-end devices     | Struggles            | Works great     |

### vs Phoenix LiveView (Elixir)

| Aspect        | LiveView              | Azumi Live            |
| ------------- | --------------------- | --------------------- |
| Transport     | WebSockets (stateful) | HTTP (stateless)      |
| Scaling       | Connection per user   | Standard HTTP scaling |
| Optimistic UI | Manual                | Automatic (compiler)  |
| Type safety   | Runtime               | Compile-time          |

---

## Technical Implementation

### The `#[azumi::live]` Macro

The macro performs **static analysis** on method bodies to classify each line:

```
┌─────────────────────────────────────────────────────────────────┐
│                    STATIC ANALYSIS                               │
├─────────────────────────────────────────────────────────────────┤
│  self.count += 1     →  PREDICTABLE  →  Generate JS: count += 1 │
│  self.open = !self   →  PREDICTABLE  →  Generate JS: open = !   │
│  self.name = "foo"   →  PREDICTABLE  →  Generate JS: name = ... │
│  db.save().await     →  UNPREDICTABLE → Server roundtrip only   │
│  external_fn()       →  UNPREDICTABLE → Server roundtrip only   │
└─────────────────────────────────────────────────────────────────┘
```

### Supported Predictions (Strict Subset)

Start with only these predictable patterns:

| Pattern       | Rust               | Generated JS         |
| ------------- | ------------------ | -------------------- |
| Direct assign | `self.x = true`    | `state.x = true`     |
| Toggle        | `self.x = !self.x` | `state.x = !state.x` |
| Increment     | `self.x += 1`      | `state.x += 1`       |
| Decrement     | `self.x -= 1`      | `state.x -= 1`       |

Anything not in this list → **no prediction, full server roundtrip**.

### The Runtime (`azumi.js`)

The existing `azumi.js` runtime (~4KB) already supports:

1. **Event delegation** - Global listeners for `click`, `submit`, etc.
2. **State engine** - `Proxy` + `WeakMap` for reactive local state
3. **Morphing** - `idiomorph` for intelligent DOM diffing
4. **Action calls** - `fetch()` to `/_azumi/action/{name}`

Azumi Live adds:

-   **Prediction execution** - Run optimistic mutations before server response
-   **Rollback** - If server response differs, reconcile

---

## Example: The Like Button

### Azumi+ (Current - Manual)

```rust
// MANUAL: Developer writes both client and server logic

#[azumi::component]
fn like_button(count: i32, liked: bool) -> impl Component {
    html! {
        <div az-scope={json!({"liked": liked, "count": count})}>
            <button
                // 1. Manually write optimistic update
                az-on="click set liked = !liked; click set count = liked ? count + 1 : count - 1"
                // 2. Manually write server action
                az-on-server="click call toggle_like -> this"
            >
                "Like"
            </button>
        </div>
    }
}
```

### Azumi Live (Goal - Automatic)

```rust
// AUTOMATIC: Write logic once, compiler handles the rest

#[azumi::live]
pub struct LikeButton {
    liked: bool,
    count: i32
}

impl LikeButton {
    pub async fn toggle(&mut self) {
        // Compiler analyzes these lines:
        self.liked = !self.liked;      // → PREDICT
        self.count += if self.liked { 1 } else { -1 }; // → PREDICT

        // db::save_like(self.liked).await; // → SERVER ONLY
    }
}

#[azumi::template]
fn like_button_view(state: &LikeButton) -> impl Component {
    html! {
        <button on:click={|_| state.toggle()}>
            {if state.liked { "Unlike" } else { "Like" }}
            " (" {state.count} ")"
        </button>
    }
}
```

---

## Implementation Roadmap

### Phase 1: Foundation (Current State ✅)

-   [x] `azumi.js` with event delegation and morphing
-   [x] `#[azumi::action]` for server-side handlers
-   [x] `az-on` DSL parsing in `html!` macro
-   [x] `idiomorph` integration

### Phase 2: `#[azumi::live]` Macro

-   [ ] Parse struct fields with `syn`
-   [ ] Analyze method bodies for predictable patterns
-   [ ] Generate `data-predict` attributes
-   [ ] Generate server action handler

### Phase 3: Runtime Evolution

-   [ ] Add prediction execution to `azumi.js`
-   [ ] Implement rollback on server mismatch
-   [ ] Add transition states (optimistic → confirmed)

### Phase 4: Polish

-   [ ] Error handling for prediction failures
-   [ ] DevTools integration
-   [ ] Documentation and examples

---

## Why This Is The "Theoretical Limit"

Within the **Server-Authoritative** paradigm, Azumi Live represents the ceiling:

1. **Maximum Simplicity** - You cannot write less code than "State + Method"
2. **Maximum Performance** - 0ms latency (optimistic UI)
3. **Maximum Safety** - Compile-time type checking

The only architecture "beyond" this is **Local-First (CRDTs)**, which:

-   Requires downloading the entire database to the browser
-   Adds extreme complexity (conflict resolution)
-   Only makes sense for offline-first apps (Figma, Linear)

For 99% of web applications, **Azumi Live is the optimal architecture**.

---

## Key Quotes

> "You are building Phoenix LiveView for Rust, but without the scaling cost of WebSockets, and with the Optimistic UI capabilities of Svelte."

> "The genius of this architecture is that it moves the complexity of Optimistic UI from the Developer to the Compiler."

> "Azumi Live treats the browser like a Television - it just shows what the server sends - while the heavy lifting stays in the data center."

---

## References

-   [Lesson 8: Action System](./demo/src/examples/lessons/pages/lesson8.rs) - Current Azumi+ implementation
-   [azumi.js](./client/azumi.js) - The client runtime
-   [Phoenix LiveView](https://hexdocs.pm/phoenix_live_view) - Inspiration (Elixir)
-   [idiomorph](https://github.com/bigskysoftware/idiomorph) - DOM morphing library
