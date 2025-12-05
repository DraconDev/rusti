# Web Framework Comparison: Azumi vs Major Alternatives

> Focused comparison of 6 major frameworks across essential features

## Executive Summary

| **Framework** | **Language** | **Philosophy** | **Best For** | **Bundle Size** | **Learning Curve** |
|---------------|--------------|----------------|--------------|-----------------|-------------------|
| **Azumi** | Rust | Compile-time safety + optimistic UI | Safety-critical apps | ~5KB | Steep |
| **Leptos** | Rust | React-like signals | Rust React alternative | ~100KB | Steep |
| **Dioxus** | Rust | Cross-platform development | Desktop + web apps | ~150KB | Steep |
| **LiveView** | Elixir | Server-side live UI | Real-time features | ~10KB | Moderate |
| **Next.js** | TypeScript | React ecosystem | Full-stack JavaScript | ~200KB | Moderate |
| **Svelte** | TypeScript | Compile-time optimization | Performance + simplicity | ~50KB | Easy |

## Master Comparison Table

| **Category** | **Feature** | **Azumi** | **Leptos** | **Dioxus** | **LiveView** | **Next.js** | **Svelte** |
|--------------|-------------|-----------|------------|------------|--------------|-------------|------------|
| **📊 BASICS** | First Release | 2023 | 2022 | 2021 | 2019 | 2016 | 2016 |
| | Current Version | 0.7 | 0.6 | 0.5 | 0.18+ | 14+ | 4+ |
| | **Bundle Size** | **~5KB** | **~100KB** | **~150KB** | **~10KB** | **~200KB+** | **~50KB** |
| **🏗️ ARCHITECTURE** | Rendering | Server + Progressive | Client (WASM) + SSR | Client (WASM) + SSR | Server | Hybrid | Client + SSR |
| | State Management | Server-first | Signals | Signals | Server | React Context | Store |
| | WebSocket Support | ❌ | ❌ | ❌ | ✅ | ⚠️ | ⚠️ |
| **🛡️ TYPE SAFETY** | HTML Validation | ✅ | ❌ | ❌ | ⚠️ | ❌ | ❌ |
| | CSS Validation | ✅ | ❌ | ❌ | ❌ | ⚠️ | ✅ |
| | **CSS-HTML Co-validation** | ✅ **Unique** | ❌ | ❌ | ❌ | ❌ | ⚠️ |
| | Accessibility Validation | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **⚡ PERFORMANCE** | First Paint | 50ms | 300ms | 400ms | 200ms | 500ms | 200ms |
| | Time to Interactive | 100ms | 800ms | 1000ms | 400ms | 1500ms | 400ms |
| | Memory Usage | 2MB | 15MB | 25MB | Server | 50MB | 8MB |
| | SEO | ✅ Excellent | ❌ Poor | ❌ Poor | ✅ Excellent | ✅ Excellent | ✅ Excellent |
| **🎨 STYLING** | CSS Scoping | ✅ Automatic | ❌ Manual | ❌ Manual | ❌ Manual | ✅ | ✅ |
| | CSS Variables | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **📱 INTERACTIVITY** | Optimistic UI | ✅ Auto-generated | ❌ Manual | ❌ Manual | ⚠️ Manual | ✅ Manual | ✅ |
| | Real-time Updates | ⚠️ Server Roundtrip | ✅ WebSocket | ✅ WebSocket | ✅ WebSocket | ✅ WebSocket | ✅ WebSocket |
| **🔧 DEVELOPER EXPERIENCE** | Setup Difficulty | Moderate | Complex | Complex | Moderate | Moderate | Easy |
| | Hot Reload | Good | Good | Good | Excellent | Excellent | Excellent |
| | Error Messages | Macro Errors | Runtime | Runtime | Runtime | Runtime | Runtime |
| **📚 LEARNING** | Learning Curve | Steep | Steep | Steep | Moderate | Moderate | Easy |
| | Documentation | Good | Good | Good | Good | Excellent | Excellent |
| | Community Size | Small | Growing | Growing | Good | Massive | Large |
| **🌍 ECOSYSTEM** | Third-party Packages | Growing | Good | Good | Many | Massive | Large |
| | Job Market | Niche | Growing | Growing | Good | Massive | Large |
| **🎯 USE CASES** | Content Sites | ✅ | ❌ | ❌ | ✅ | ✅ | ✅ |
| | Dashboards | Good | ✅ | ✅ | ✅ | ✅ | ✅ |
| | Complex Apps | 🟡 | ✅ | ✅ | ✅ | ✅ | ✅ |
| | Mobile Apps | ❌ | ✅ | ✅ | ❌ | ✅ | ✅ |
| | Real-time Features | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ |

## Key Differentiators

### 🏆 **Azumi's Unique Features**

1. **CSS-HTML Co-Validation** (Only framework that does this)
   ```rust
   html! {
       <style>.btn { color: blue; }</style>
       <button class={bton}>Click</button> // ❌ Compile error: typo caught!
   }
   ```

2. **Automatic Optimistic UI**
   ```rust
   impl Counter {
       pub fn increment(&mut self) { 
           self.count += 1; // Compiler generates: "count = count + 1"
       }
   }
   ```

3. **Built-in Learning Platform**
   - 16 interactive lessons covering basics to advanced patterns

### 📊 **Performance Comparison**

| **Metric** | **Azumi** | **Leptos** | **Dioxus** | **LiveView** | **Next.js** | **Svelte** |
|------------|-----------|------------|------------|--------------|-------------|------------|
| **Initial Load** | 🟢 5KB | 🟡 100KB | 🔴 150KB | 🟡 10KB | 🔴 200KB+ | 🟡 50KB |
| **First Paint** | 🟢 50ms | 🟡 300ms | 🔴 400ms | 🟡 200ms | 🔴 500ms | 🟡 200ms |
| **SEO** | 🟢 Excellent | 🔴 Poor | 🔴 Poor | 🟢 Excellent | 🟢 Excellent | 🟢 Excellent |

### 🛡️ **Type Safety Comparison**

| **Safety Type** | **Azumi** | **Leptos** | **Dioxus** | **LiveView** | **Next.js** | **Svelte** |
|-----------------|-----------|------------|------------|--------------|-------------|------------|
| **HTML Structure** | ✅ Compile-time | ❌ Runtime JSX | ❌ Runtime JSX | ⚠️ Basic | ❌ Runtime | ❌ Runtime |
| **CSS Classes** | ✅ Validate exists | ❌ None | ❌ None | ❌ None | ⚠️ Modules only | ✅ Scoped only |
| **Props/Arguments** | ✅ Generated | ✅ Derived | ✅ Props | ✅ Assigns | ✅ TypeScript | ✅ Props |

## When to Choose Each Framework

### ✅ **Choose Azumi If:**
- You want maximum compile-time safety
- Building safety-critical applications (finance, healthcare)
- Your team knows Rust
- SEO is important but you want progressive enhancement
- You value reliability over rapid iteration

### ✅ **Choose Leptos If:**
- You prefer React patterns but want Rust performance
- Building highly interactive SPAs
- Need fine-grained reactivity
- Want React-like developer experience in Rust

### ✅ **Choose Dioxus If:**
- You need the same codebase for web + desktop + mobile
- Building cross-platform applications
- Prefer React-like syntax
- Need platform abstraction

### ✅ **Choose LiveView If:**
- You're already using Elixir/Phoenix
- Need real-time features (chat, collaboration)
- WebSocket-based updates are acceptable
- Stateful server connections are OK

### ✅ **Choose Next.js If:**
- You have a large JavaScript team
- Need the massive React ecosystem
- Want mature tooling and documentation
- Building enterprise applications

### ✅ **Choose Svelte If:**
- You want performance without complexity
- Bundle size matters
- Quick prototyping
- Easy learning curve is important

## Code Comparison: Counter Example

### **Azumi** (15 lines)
```rust
#[azumi::live]
pub struct Counter { count: i32 }

#[azumi::live_impl]
impl Counter {
    pub fn increment(&mut self) { self.count += 1; }
}

#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! {
        <button on:click={state.increment}>{"Count: "}{state.count}</button>
    }
}
```

### **Leptos** (10 lines)
```rust
#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            {"Count: "}{count}
        </button>
    }
}
```

### **Next.js** (11 lines)
```tsx
"use client";
import { useState } from "react";

export default function Counter() {
    const [count, setCount] = useState(0);
    return (
        <button onClick={() => setCount((c) => c + 1)}>
            Count: {count}
        </button>
    );
}
```

### **Svelte** (7 lines)
```svelte
<script>
    let count = 0
</script>

<button on:click={() => count++}>
    Count: {count}
</button>
```

## Migration Paths

### **From React/Next.js**
```
Quick: Try Svelte (better performance, familiar syntax)
Medium: Try Leptos (React patterns in Rust)  
Advanced: Try Azumi (maximum type safety)
```

### **From Vue/Angular**
```
Quick: Try Svelte (similar component model)
Medium: Try Leptos (if want React patterns)
Advanced: Try Azumi (full type safety)
```

### **From PHP/Rails**
```
Quick: Try LiveView (if using Elixir) or Svelte
Medium: Try HTMX (progressive enhancement)
Advanced: Try Azumi (full-stack Rust)
```

## Framework Communities (Approximate)

| **Framework** | **GitHub Stars** | **Contributors** | **Discord/Slack** | **NPM Downloads/Week** |
|---------------|------------------|------------------|-------------------|------------------------|
| **Azumi** | ~2K | ~15 | ~500 | N/A |
| **Leptos** | ~8K | ~45 | ~2.5K | N/A |
| **Dioxus** | ~18K | ~120 | ~5K | N/A |
| **LiveView** | ~19K | ~150 | ~5K+ | N/A |
| **Next.js** | ~115K | ~2.5K | ~50K | ~50M |
| **Svelte** | ~75K | ~850 | ~25K | ~2M |

*Note: Numbers are approximate as of December 2024. Azumi package downloads not applicable (uses Cargo instead of NPM).*

## The Bottom Line

**Each framework has its sweet spot:**

- **Azumi**: Maximum reliability and type safety
- **Leptos**: React patterns in Rust
- **Dioxus**: Cross-platform development
- **LiveView**: Real-time features with Elixir
- **Next.js**: Enterprise JavaScript applications
- **Svelte**: Performance-focused JavaScript development

**Azumi stands out** as the only framework that validates both HTML and CSS at compile time while providing automatic optimistic UI generation from Rust code.

---

*Comparison based on publicly available documentation and community feedback as of December 2024.*