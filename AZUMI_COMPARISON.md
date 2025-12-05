# Azumi vs The World: Framework Comparison

> Where Azumi fits in the modern web framework landscape

## Executive Summary

| Dimension                | Azumi's Position                                 |
| ------------------------ | ------------------------------------------------ |
| **Compile-time Safety**  | 🏆 Industry-leading HTML/CSS/JS validation       |
| **Developer Experience** | 🌟 Type-safe components, minimal boilerplate     |
| **Performance**          | ⚡ Server-first with 0ms optimistic updates      |
| **Learning Curve**       | 📈 Steep (Rust + macro concepts) but rewarding   |
| **Ecosystem**            | 🌱 Growing, specialized for safety-critical apps |

---

## The Frameworks

### 🦀 Azumi - "Compile-Time Web Revolution"

-   **Language**: Rust
-   **Philosophy**: Type-safe HTML/CSS with compile-time validation
-   **Architecture**: Server-side rendering + progressive enhancement
-   **Unique**: Only framework that validates CSS classes exist in HTML at compile time

### ⚡ Leptos - "React for Rust"

-   **Language**: Rust
-   **Philosophy**: Runtime reactivity with signals
-   **Architecture**: Client-side (WASM) + optional SSR
-   **Target**: React developers wanting Rust performance

### 🏗️ Dioxus - "Cross-Platform Rust"

-   **Language**: Rust
-   **Philosophy**: React-like syntax across platforms (web/desktop/mobile)
-   **Architecture**: Virtual DOM with platform abstractions

### 🎭 Maud - "Template Excellence"

-   **Language**: Rust
-   **Philosophy**: Compile-time HTML templates with zero runtime
-   **Architecture**: Pure server-side rendering

### ⚛️ Next.js - "The JavaScript King"

-   **Language**: TypeScript/JavaScript
-   **Philosophy**: React-based full-stack with hybrid rendering
-   **Architecture**: SSR, SSG, and CSR combined

### 🌟 Svelte - "The Compiler Revolution"

-   **Language**: TypeScript/JavaScript
-   **Philosophy**: Compile-time optimization with minimal runtime
-   **Architecture**: Compile-to-Vanilla JS

### 🔥 HTMX - "HTML Over The Wire"

-   **Language**: Any (backend agnostic)
-   **Philosophy**: HTML-centric with minimal JavaScript
-   **Architecture**: Server-rendered HTML fragments

### 🐦 Phoenix LiveView - "Elixir's Live UI"

-   **Language**: Elixir
-   **Philosophy**: Server-rendered live views over WebSocket
-   **Architecture**: Stateful server connections

---

## Technical Comparison

### Type Safety

| Framework | HTML            | CSS             | Props         | Runtime Types |
| --------- | --------------- | --------------- | ------------- | ------------- |
| **Azumi** | ✅ Compile-time | ✅ Compile-time | ✅ Generated  | ✅ Rust       |
| Leptos    | ⚠️ JSX-like     | ❌ Manual       | ✅ Derive     | ✅ Rust       |
| Dioxus    | ⚠️ JSX-like     | ❌ Manual       | ✅ Props      | ✅ Rust       |
| Maud      | ✅ Templates    | ❌ Manual       | N/A           | ✅ Rust       |
| Next.js   | ❌ Runtime      | ⚠️ Modules      | ✅ TypeScript | ⚠️ TypeScript |
| Svelte    | ⚠️ Templates    | ✅ Scoped       | ⚠️ Props      | ⚠️ TypeScript |
| HTMX      | ❌ None         | ❌ None         | N/A           | ❌ None       |
| LiveView  | ⚠️ HEEx         | ❌ Manual       | ✅ Assigns    | ⚠️ Runtime    |

**Azumi's Edge**: Only framework validating CSS class usage at compile time.

### Performance Characteristics

| Framework | Initial JS | First Paint | Interactivity | Memory  |
| --------- | ---------- | ----------- | ------------- | ------- |
| **Azumi** | ~5KB       | Instant     | Progressive   | Minimal |
| Leptos    | ~100KB     | Moderate    | Full          | Medium  |
| Dioxus    | ~150KB     | Moderate    | Full          | High    |
| Maud      | 0KB        | Instant     | Manual        | Minimal |
| Next.js   | ~200KB+    | Moderate    | Full          | High    |
| Svelte    | ~50KB      | Fast        | Full          | Low     |
| HTMX      | ~14KB      | Instant     | Progressive   | Minimal |
| LiveView  | ~10KB      | Moderate    | Full          | Server  |

### Reactivity Model

| Framework | Model        | Optimistic UI | Flicker Prevention |
| --------- | ------------ | ------------- | ------------------ |
| **Azumi** | Server-first | ✅ Automatic  | ✅ Smart skip      |
| Leptos    | Signals      | ❌ Manual     | ⚠️ Virtual DOM     |
| Dioxus    | Virtual DOM  | ❌ Manual     | ⚠️ Diffing         |
| Maud      | None         | ❌ N/A        | ❌ Full render     |
| Next.js   | React        | ❌ Manual     | ⚠️ Suspense        |
| Svelte    | Compile-time | ❌ Manual     | ⚠️ Transitions     |
| HTMX      | Morph        | ❌ None       | ⚠️ Morph           |
| LiveView  | Server push  | ⚠️ Optimistic | ⚠️ Morph           |

**Azumi's Edge**: Automatic optimistic UI from Rust code analysis, plus smart morph skipping when prediction matches.

---

## Code Comparison: Counter Component

### Azumi

```rust
#[azumi::live]
pub struct Counter { pub count: i32 }

#[azumi::live_impl(component = "counter_view")]
impl Counter {
    pub fn increment(&mut self) { self.count += 1; }
}

#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! {
        <style>.btn { padding: "0.5rem"; }</style>
        <button class={btn} on:click={state.increment}>
            "Count: " {state.count}
        </button>
    }
}
```

**Lines**: 15 | **Boilerplate**: Minimal | **Type Safety**: Full

### Leptos

```rust
#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "Count: " {count}
        </button>
    }
}
```

**Lines**: 10 | **Boilerplate**: Moderate | **Type Safety**: Partial (no CSS)

### Next.js

```tsx
"use client";
import { useState } from "react";

export default function Counter() {
    const [count, setCount] = useState(0);
    return (
        <button onClick={() => setCount((c) => c + 1)}>Count: {count}</button>
    );
}
```

**Lines**: 11 | **Boilerplate**: Moderate | **Type Safety**: TypeScript only

### Svelte

```svelte
<script>
    let count = 0
</script>

<button on:click={() => count++}>
    Count: {count}
</button>
```

**Lines**: 7 | **Boilerplate**: Minimal | **Type Safety**: Partial

### HTMX

```html
<div hx-get="/counter" hx-trigger="click">Count: 0</div>
```

**Lines**: 3 | **Boilerplate**: Minimal | **Type Safety**: None

---

## When to Use Each

### Choose Azumi When:

-   ✅ Type safety is paramount (financial, healthcare, enterprise)
-   ✅ Server-side rendering with progressive enhancement
-   ✅ You want compile-time CSS validation
-   ✅ Your team knows Rust
-   ✅ Performance and reliability matter more than iteration speed

### Choose Leptos/Dioxus When:

-   ✅ Building highly interactive SPAs
-   ✅ Need client-side Rust (WASM)
-   ✅ Cross-platform (Dioxus for desktop/mobile)
-   ✅ React-like patterns preferred

### Choose Next.js When:

-   ✅ Large JavaScript ecosystem needs
-   ✅ Team expertise in React
-   ✅ Need mature tooling and documentation
-   ✅ SEO-critical with complex interactivity

### Choose Svelte When:

-   ✅ Performance matters, bundle size critical
-   ✅ Simple syntax preferred
-   ✅ Quick prototyping
-   ✅ Small to medium projects

### Choose HTMX When:

-   ✅ Minimal JavaScript philosophy
-   ✅ Any backend language
-   ✅ Simple interactions
-   ✅ Progressive enhancement focus

### Choose LiveView When:

-   ✅ Already using Elixir/Phoenix
-   ✅ Real-time features (chat, collaboration)
-   ✅ Stateful server connections acceptable
-   ✅ WebSocket-based updates preferred

---

## Azumi's Unique Features

### 1. CSS-HTML Co-Validation

```rust
html! {
    <style>
        .my_class { color: "blue"; }
    </style>
    <div class={my_clas}>"Hello"</div> // ❌ COMPILE ERROR: typo caught!
}
```

No other framework does this.

### 2. Automatic Optimistic UI

```rust
impl Counter {
    pub fn increment(&mut self) {
        self.count += 1; // Compiler generates prediction automatically
    }
}
```

No manual client code needed.

### 3. Smart Morph Optimization

When prediction matches server response, DOM updates are **skipped entirely**:

```
✅ Prediction matched server - skipping morph
```

Zero flicker for predicted updates.

### 4. Double-Quoted CSS Enforcement

```rust
// ✅ Correct
.btn { padding: "1rem"; background: "#4CAF50"; }

// ❌ Compile error - prevents lexer issues
.btn { padding: 1rem; background: #4CAF50; }
```

Consistent, parseable CSS at compile time.

---

## Limitations

### Azumi is NOT ideal for:

-   ❌ Rapid prototyping (Rust learning curve)
-   ❌ Heavy client-side interactivity (games, drawing apps)
-   ❌ Real-time collaboration (WebSocket-native solutions better)
-   ❌ Teams without Rust experience
-   ❌ Mobile app development

### Trade-offs:

-   Learning curve is steep
-   Ecosystem is smaller than React/Vue
-   Macro errors can be cryptic
-   Server roundtrip for complex logic

---

## Migration Paths

### From React/Next.js:

1. Start with Maud for templates
2. Add Leptos for interactivity
3. Graduate to Azumi for full type safety

### From HTMX:

1. Azumi's `az-on` syntax is similar
2. Add `#[azumi::live]` for automatic predictions
3. Gain compile-time validation

### From Phoenix LiveView:

1. Similar server-first philosophy
2. Azumi uses HTTP (stateless) vs WebSocket (stateful)
3. Compile-time vs runtime validation

---

## Conclusion

**Azumi occupies a unique position**: maximum compile-time safety with progressive enhancement. It's not trying to be the most popular framework—it's trying to be the most reliable.

| If You Value    | Choose           |
| --------------- | ---------------- |
| Maximum safety  | **Azumi**        |
| Rapid iteration | Next.js / Svelte |
| Rich ecosystem  | Next.js          |
| Cross-platform  | Dioxus           |
| Real-time       | LiveView         |
| Simplicity      | HTMX / Svelte    |

**Azumi is for teams who believe bugs caught at compile time are worth the investment in Rust expertise.**
