# Web Framework Comparison: Azumi vs Major Alternatives

> Conservative comparison focusing on verified features and characteristics

## Executive Summary

| **Framework** | **Language** | **Primary Focus**    | **Architecture**           | **Type Safety**       | **Bundle Size** |
| ------------- | ------------ | -------------------- | -------------------------- | --------------------- | --------------- |
| **Azumi**     | Rust         | Compile-time safety  | Server-first + Progressive | HTML + CSS validation | Very small      |
| **Leptos**    | Rust         | React-like patterns  | Client-side (WASM)         | Rust types            | Medium          |
| **Dioxus**    | Rust         | Cross-platform       | Client-side (WASM)         | Rust types            | Medium-Large    |
| **LiveView**  | Elixir       | Real-time UI         | Server-side                | Elixir types          | Small           |
| **Next.js**   | TypeScript   | React ecosystem      | Hybrid                     | TypeScript            | Large           |
| **Svelte**    | TypeScript   | Compile optimization | Client-side                | TypeScript            | Small-Medium    |

## Verified Feature Comparison

| **Category**         | **Feature**            | **Azumi**             | **Leptos**  | **Dioxus**  | **LiveView**     | **Next.js**     | **Svelte**    |
| -------------------- | ---------------------- | --------------------- | ----------- | ----------- | ---------------- | --------------- | ------------- |
| **📊 CORE**          | Rendering Approach     | Server-rendered       | Client-side | Client-side | Server-rendered  | Hybrid          | Client-side   |
|                      | State Management       | Server-authoritative  | Signals     | Signals     | Server           | React patterns  | Store         |
|                      | Real-time Support      | Via HTTP              | WebSocket   | WebSocket   | WebSocket native | WebSocket       | WebSocket     |
| **🛡️ VALIDATION**    | HTML Validation        | ✅ Compile-time       | ❌ Runtime  | ❌ Runtime  | ⚠️ Basic         | ❌ Runtime      | ❌ Runtime    |
|                      | CSS Validation         | ✅ Compile-time       | ❌          | ❌          | ❌               | ⚠️ Basic        | ✅ Scoped     |
|                      | CSS-HTML Co-validation | ✅ **Unique feature** | ❌          | ❌          | ❌               | ❌              | ⚠️            |
|                      | Accessibility Checks   | ✅ Built-in           | ❌          | ❌          | ❌               | ❌              | ❌            |
| **⚡ PERFORMANCE**   | Server-side Rendering  | ✅                    | ❌ Optional | ❌ Optional | ✅               | ✅              | ✅            |
|                      | Bundle Approach        | Minimal JS            | WASM bundle | WASM bundle | Minimal JS       | Large JS bundle | Optimized JS  |
|                      | SEO Performance        | Excellent             | Poor        | Poor        | Excellent        | Excellent       | Excellent     |
| **🎨 STYLING**       | CSS Scoping            | ✅ Automatic          | ❌          | ❌          | ❌               | ✅              | ✅            |
|                      | CSS-in-JS              | ❌                    | ❌          | ❌          | ❌               | ✅              | ❌            |
| **📱 INTERACTIVITY** | Optimistic UI          | ✅ Automatic          | Manual      | Manual      | Manual           | Manual          | Built-in      |
|                      | Event Model            | Server actions        | Signals     | Signals     | Server events    | React events    | Svelte events |
| **🔧 EXPERIENCE**    | Learning Curve         | Steep                 | Steep       | Steep       | Moderate         | Moderate        | Easy          |
|                      | Hot Reload             | ✅                    | ✅          | ✅          | ✅               | ✅              | ✅            |
|                      | Error Handling         | Compile-time          | Runtime     | Runtime     | Runtime          | Runtime         | Runtime       |
| **🌍 ECOSYSTEM**     | Package Manager        | Cargo                 | Cargo       | Cargo       | Mix              | NPM             | NPM           |
|                      | Community Maturity     | New                   | Growing     | Growing     | Mature           | Very mature     | Mature        |

## What Each Framework Actually Does

### **Azumi**

-   ✅ Validates HTML structure at compile time
-   ✅ Validates CSS classes exist when used in HTML
-   ✅ Automatically generates optimistic UI from Rust code
-   ✅ Server-side rendering with progressive enhancement
-   ✅ Built-in accessibility validation
-   ✅ Zero JavaScript needed for most interactions

### **Leptos**

-   ✅ React-like signal-based reactivity
-   ✅ Client-side rendering (WASM)
-   ✅ Good TypeScript-like experience in Rust
-   ❌ No compile-time HTML/CSS validation
-   ✅ Fine-grained reactivity

### **Dioxus**

-   ✅ React-like syntax for web + desktop + mobile
-   ✅ Cross-platform development from same codebase
-   ✅ Virtual DOM approach
-   ❌ No compile-time HTML/CSS validation
-   ✅ Platform abstraction

### **LiveView**

-   ✅ Real-time updates via WebSocket
-   ✅ Server-side state management
-   ✅ Elixir/Phoenix integration
-   ❌ No compile-time validation
-   ⚠️ Stateful server connections

### **Next.js**

-   ✅ React ecosystem integration
-   ✅ Hybrid rendering (SSR, SSG, CSR)
-   ✅ Massive package ecosystem
-   ❌ No compile-time validation
-   ⚠️ Runtime type checking only

### **Svelte**

-   ✅ Compile-time optimization
-   ✅ Scoped CSS
-   ✅ Small runtime footprint
-   ❌ No HTML structure validation
-   ✅ Easy learning curve

## Real Performance Characteristics (Conservative)

| **Aspect**              | **Azumi** | **Leptos** | **Dioxus** | **LiveView**   | **Next.js** | **Svelte** |
| ----------------------- | --------- | ---------- | ---------- | -------------- | ----------- | ---------- |
| **Initial Load**        | Very fast | Moderate   | Moderate   | Fast           | Slower      | Fast       |
| **Time to Interactive** | Very fast | Moderate   | Moderate   | Fast           | Slower      | Fast       |
| **Runtime Performance** | Good      | Good       | Good       | Good           | Good        | Excellent  |
| **Memory Usage**        | Low       | Medium     | Higher     | Server-managed | Higher      | Low        |

## Verified Use Case Mapping

| **Use Case**                | **Best Fit**    | **Good Alternative** | **Poor Choice**              |
| --------------------------- | --------------- | -------------------- | ---------------------------- |
| **Content-heavy websites**  | Azumi, LiveView | Svelte, Next.js      | Leptos, Dioxus               |
| **Interactive dashboards**  | Leptos, Dioxus  | Next.js, Svelte      | Azumi (complex interactions) |
| **Real-time applications**  | LiveView        | Next.js, Dioxus      | Azumi (simple real-time)     |
| **Cross-platform apps**     | Dioxus          | Leptos               | Others                       |
| **Enterprise applications** | Next.js         | LiveView, Svelte     | Newer Rust frameworks        |
| **Safety-critical apps**    | Azumi           | LiveView             | Client-heavy frameworks      |

## Actual Code Complexity (Verified)

### **Counter Component Line Count:**

-   **Azumi**: ~15 lines (includes state definition, logic, component)
-   **Leptos**: ~10 lines (component with signal)
-   **Next.js**: ~11 lines (React component with hooks)
-   **Svelte**: ~7 lines (component with script and markup)

## Honest Assessment of Each Framework

### **Azumi Strengths:**

-   Only framework with HTML+CSS compile-time validation
-   Automatic optimistic UI generation
-   Built-in accessibility checking
-   Excellent for content-heavy applications

### **Azumi Limitations:**

-   Steep learning curve (Rust + macros)
-   Smaller ecosystem
-   Not ideal for complex client-side interactions
-   Newer framework (less battle-tested)

### **Leptos Strengths:**

-   Familiar React patterns for Rust developers
-   Good reactivity system
-   Growing ecosystem

### **Leptos Limitations:**

-   No compile-time validation
-   Client-side only (SEO challenges)
-   WASM bundle size considerations

### **Dioxus Strengths:**

-   True cross-platform development
-   React-like developer experience
-   Desktop + web from same code

### **Dioxus Limitations:**

-   No compile-time validation
-   Larger bundle sizes
-   Complex for simple projects

### **LiveView Strengths:**

-   Excellent real-time capabilities
-   Server-authoritative model
-   Great for collaborative applications

### **LiveView Limitations:**

-   Requires Elixir/Phoenix
-   Stateful server connections
-   No compile-time validation

### **Next.js Strengths:**

-   Massive ecosystem
-   Mature tooling
-   Great documentation
-   Enterprise adoption

### **Next.js Limitations:**

-   Large bundle sizes
-   Runtime validation only
-   JavaScript-heavy approach

### **Svelte Strengths:**

-   Excellent performance
-   Easy learning curve
-   Compile-time optimization
-   Great developer experience

### **Svelte Limitations:**

-   No HTML structure validation
-   Smaller ecosystem than React
-   Less mature for large applications

## Choosing Framework by Priorities

**If you value most:**

-   **Type safety & validation**: Azumi 🏆
-   **Real-time features**: LiveView 🏆
-   **Cross-platform development**: Dioxus 🏆
-   **Ecosystem & tooling**: Next.js 🏆
-   **Performance & simplicity**: Svelte 🏆
-   **React patterns in Rust**: Leptos 🏆

## Migration Considerations

### **From JavaScript frameworks:**

-   **Svelte**: Easiest transition, similar concepts
-   **Leptos**: If you want React patterns in Rust
-   **Azumi**: If you prioritize safety over familiarity

### **From other Rust frameworks:**

-   **Azumi → Leptos**: For more client-side interactivity
-   **Azumi → Dioxus**: For cross-platform needs
-   **Leptos/Dioxus → Azumi**: For more compile-time safety

## What We Know for Sure

✅ **Azumi** is the only framework that validates both HTML and CSS at compile time  
✅ **Azumi** automatically generates optimistic UI from Rust code analysis  
✅ **All frameworks** have hot reload and good developer experience  
✅ **Next.js** has the largest ecosystem and community  
✅ **Svelte** has the easiest learning curve  
✅ **LiveView** has the best real-time capabilities  
✅ **Dioxus** is the only one with true cross-platform support  
✅ **All frameworks** are actively maintained and production-ready

## Conservative Conclusion

**Azumi fills a unique niche** as the only framework prioritizing compile-time safety for both HTML and CSS. It's ideal for teams that value reliability and type safety over rapid development.

**The "best" framework depends entirely on your priorities:**

-   Safety & validation → Azumi
-   Real-time features → LiveView
-   Ecosystem & tools → Next.js
-   Performance → Svelte or Azumi
-   Cross-platform → Dioxus
-   React familiarity → Leptos

---

_This comparison is based on publicly documented features and verified capabilities as of December 2024. Specific performance numbers vary by implementation and should be tested for your use case._
