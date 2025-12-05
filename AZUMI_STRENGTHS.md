# Azumi: The Compile-Time Safe Web Framework

> Where type safety meets web development innovation

## 🎯 The Azumi Philosophy

**Azumi believes web development should be as safe and reliable as other programming domains.** Why should HTML templates and CSS be the wild west when your Rust code is bulletproof?

### **Core Principles**

-   **Bugs should be impossible** - Catch errors at compile time, not in production
-   **Type safety everywhere** - HTML, CSS, and Rust should all be validated
-   **Single source of truth** - Write logic once, get both server and client behavior
-   **Progressive enhancement** - Works without JavaScript, enhanced with it
-   **Developer confidence** - What compiles works, what works is correct

---

## 🏆 What Makes Azumi Revolutionary

### **1. CSS-HTML Co-Validation** (Industry First)

```rust
html! {
    <style>
        .btn { padding: "1rem"; background: "#4CAF50"; }
    </style>
    <button class={bton}>Click me</button>  // ❌ COMPILE ERROR!
}
```

**Why this matters**: Every other framework will silently ignore the typo or fail at runtime. Azumi catches it at compile time, saving you from broken production deployments.

### **2. Automatic CSS Scoping** (Zero Conflicts)

```rust
#[azumi::component]
pub fn Header() -> impl Component {
    html! {
        <style>.title { color: "blue"; }</style>
        <h1 class={title}>"Company Name"</h1>
    }
}

#[azumi::component]
pub fn Sidebar() -> impl Component {
    html! {
        <style>.title { color: "red"; }</style>  // No conflict!
        <h2 class={title}>"Navigation"</h2>
    }
}
```

**Why this matters**: No more BEM naming conventions, CSS modules, or utility classes. Azumi automatically scopes styles to each component.

### **3. Compiler-Generated Optimistic UI** (Zero JavaScript)

```rust
#[azumi::live]
pub struct Counter {
    pub count: i32,
}

impl Counter {
    pub fn increment(&mut self) {
        self.count += 1;  // Compiler generates: "count = count + 1"
    }
}

#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! {
        <button on:click={state.increment}>
            {"Count: "}{state.count}  // Updates instantly!
        </button>
    }
}
```

**Why this matters**: Write your business logic once in Rust. The compiler analyzes it and generates both the server handler AND the optimistic UI prediction. No JavaScript to write, maintain, or debug.

### **4. Built-in Accessibility Validation**

```rust
html! {
    <img src="photo.jpg" />  // ❌ COMPILE ERROR: Missing alt attribute
    <button>                // ❌ COMPILE ERROR: Must have content or aria-label
        "Click me"
    </button>
}
```

**Why this matters**: Accessibility isn't an afterthought. Azumi enforces WCAG guidelines at compile time, ensuring your applications are accessible from day one.

---

## 🚀 Revolutionary Features

### **Type-Safe Component System**

```rust
#[azumi::component]
pub fn UserCard<'a>(name: &'a str, age: i32, active: bool) -> impl Component + 'a {
    html! {
        <style>
            .card { border: "1px solid #ddd"; padding: "1rem"; }
            .active { background: "#e8f5e9"; }
        </style>
        <div class={card}>
            <h3 class={if active { "active" } else { "" }}>
                {name}
            </h3>
            <p>{"Age: "}{age}</p>
        </div>
    }
}
```

**Benefits:**

-   ✅ Type-safe props with lifetimes
-   ✅ Named arguments required (no positional bugs)
-   ✅ Automatic prop validation
-   ✅ No runtime prop checking needed

### **Progressive Enhancement Architecture**

```rust
#[azumi::component]
pub fn BlogPost(content: &str) -> impl Component {
    html! {
        <article>
            <h1>{"My Blog Post"}</h1>
            <p>{content}</p>

            // Works without JavaScript
            <a href="/comments">{"View Comments"}</a>

            // Enhanced with JavaScript
            <button on:click={load_comments}>
                {"Load Comments (AJAX)"}
            </button>
        </article>
    }
}
```

**Benefits:**

-   ✅ **SEO perfect** - Server-side rendered by default
-   ✅ **Accessible without JS** - Progressive enhancement
-   ✅ **Enhanced with JS** - Optional interactivity
-   ✅ **Fast first paint** - Minimal client-side JavaScript

### **Compile-Time Form Validation**

```rust
#[azumi::component]
pub fn ContactForm() -> impl Component {
    html! {
        <form>
            <div>
                <label for="email">"Email"</label>
                <input type="email" name="email" required />
                // ^ Validates email format at compile time
            </div>
            <div>
                <label for="age">"Age"</label>
                <input type="number" name="age" min="18" max="120" />
                // ^ Validates range at compile time
            </div>
            <button type="submit">{"Submit"}</button>
        </form>
    }
}
```

**Benefits:**

-   ✅ **No runtime validation errors** - Caught at compile time
-   ✅ **Accessible forms** - Proper labels and structure enforced
-   ✅ **Semantic HTML** - Valid document structure guaranteed

---

## 📚 The Learning Advantage

### **16 Interactive Lessons (Built-in)**

Azumi includes the most comprehensive web framework education platform:

1. **Lesson 0**: Component basics (`#[azumi::component]`, `html!` macro)
2. **Lesson 1**: CSS scoping and automatic conflict prevention
3. **Lesson 2**: Global vs scoped styles
4. **Lesson 3**: Component composition patterns
5. **Lesson 4**: Children pattern for layout components
6. **Lesson 5**: `@let` variables for local state
7. **Lesson 6**: Control flow (`@if`, `@for`, `@match`)
8. **Lesson 7**: Form handling with compile-time validation
9. **Lesson 8**: Server actions for interactivity
10. **Lesson 9**: Azumi Live introduction
11. **Lesson 10**: Auto-detecting live state
12. **Lesson 11**: Declarative event binding
13. **Lesson 12**: How optimistic UI works
14. **Lesson 13**: Advanced form patterns
15. **Lesson 14**: Component composition with live state
16. **Lesson 15**: Full application patterns

**Why this matters**: Most frameworks assume you already know web development. Azumi teaches it progressively with working examples you can modify and test.

---

## 💪 Real-World Benefits

### **For Developers**

```rust
// Before Azumi (typical web development):
// 1. Write HTML template
// 2. Write CSS stylesheet
// 3. Write JavaScript for interactivity
// 4. Debug why CSS classes don't match
// 5. Debug why form validation fails
// 6. Debug why accessibility is broken
// 7. Debug why optimistic UI doesn't work
// 8. Deploy and fix runtime errors

// With Azumi:
#[azumi::component]
pub fn TodoApp() -> impl Component {
    html! {
        <style>.error { color: "red"; }</style>
        <div>
            <input placeholder="Add todo..." />
            <button>{"Add"}</button>
            // If any part is wrong, the compiler tells you exactly what's wrong
        </div>
    }
}
```

**Benefits:**

-   ⚡ **Faster debugging** - Compile errors instead of runtime bugs
-   🛡️ **More confidence** - What compiles works
-   📚 **Better learning** - Built-in educational platform
-   🎯 **Less boilerplate** - Auto-generated type-safe APIs

### **For Teams**

-   **Knowledge sharing**: 16 lessons teach everyone the same concepts
-   **Code quality**: Compile-time validation prevents entire classes of bugs
-   **Maintainability**: Type-safe components are self-documenting
-   **Performance**: Server-first approach with progressive enhancement

### **For Businesses**

-   **Reliability**: Fewer production bugs due to compile-time safety
-   **Performance**: Excellent Core Web Vitals by default
-   **SEO**: Server-side rendering ensures perfect search engine indexing
-   **Accessibility**: Built-in WCAG compliance
-   **Maintenance**: Type safety reduces long-term maintenance costs

---

## 🔥 The Azumi Difference

### **Traditional Web Development**

```
Write code → Hope it works → Debug runtime errors → Fix in production
```

### **Azumi Development**

```
Write code → Compiler catches all errors → Deploy with confidence
```

---

## 🎯 Why Choose Azumi?

### **Perfect for:**

-   ✅ **Safety-critical applications** (finance, healthcare, enterprise)
-   ✅ **Content-heavy websites** that need excellent SEO
-   ✅ **Teams that value reliability** over rapid iteration
-   ✅ **Developers who want compile-time guarantees**
-   ✅ **Applications that must be accessible** from day one
-   ✅ **Projects where maintenance** is more important than speed

### **Not ideal for:**

-   ❌ **Rapid prototyping** where speed is more important than safety
-   ❌ **Teams without Rust experience** (learning curve is real)
-   ❌ **Games or highly interactive applications** (client-heavy)
-   ❌ **Projects with tight deadlines** (setup and learning time)

---

## 🚀 Getting Started

### **Try the Learning Platform**

```bash
git clone <repository>
cd azumi
cargo run -p demo
# Visit: http://localhost:8080
# 16 interactive lessons await!
```

### **Build Your First Component**

```rust
use azumi::html;

#[azumi::component]
pub fn Welcome() -> impl azumi::Component {
    html! {
        <style>
            .welcome { text-align: "center"; padding: "2rem"; }
        </style>
        <div class={welcome}>
            <h1>{"Welcome to Azumi!"}</h1>
            <p>{"Type-safe web development starts here."}</p>
        </div>
    }
}
```

---

## 💡 The Future of Web Development

**Azumi represents a fundamental shift** toward compile-time safety in web development. While other frameworks focus on runtime performance or developer experience improvements, **Azumi eliminates entire categories of bugs before they reach users**.

**We're not trying to be the most popular framework** - we're trying to be the most reliable.

**For developers who believe that bugs caught at compile time are worth the investment in learning Rust's powerful type system, Azumi offers something no other framework can provide: confidence that your web application is correct before it ever runs.**

---

_The only web framework that validates your HTML, scopes your CSS, and generates optimistic UI from Rust code—all at compile time._
