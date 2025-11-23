# Rusti Macro – Comprehensive Examples

Below is a collection of example snippets that demonstrate the full range of features supported by the `rusti!` macro. Feel free to copy‑paste these into your own projects or use them as a reference when building components.

---

## 1. Basic HTML
```rust
rusti! {
    <div class="p-4">
        <h1>Hello, world!</h1>
        <p>This is a plain paragraph.</p>
    </div>
}
```
---

## 2. Dynamic Content (Interpolation)
```rust
let name = "Alice";
let count = 3;
rusti! {
    <p>Welcome, { name }! You have { count } new messages.</p>
}
```
---

## 3. Conditional Rendering (`@if` / `@else`)
```rust
let logged_in = true;
let user = "Bob";
rusti! {
    <nav>
        @if logged_in {
            <span>Welcome, { user }!</span>
            <a href="/logout">Logout</a>
        } @else {
            <a href="/login">Login</a>
        }
    </nav>
}
```
---

## 4. Loops (`@for`)
```rust
let items = vec!["Apples", "Bananas", "Cherries"];
rusti! {
    <ul>
        @for item in items {
            <li>{ item }</li>
        }
    </ul>
}
```
---

## 5. Pattern Matching (`@match`)
```rust
enum Status { Active, Pending, Suspended }
let status = Status::Pending;
rusti! {
    @match status {
        Status::Active => { <span class="green">Active</span> }
        Status::Pending => { <span class="yellow">Pending</span> }
        Status::Suspended => { <span class="red">Suspended</span> }
    }
}
```
---

## 6. Component Composition (`@component_name`)
```rust
fn button(label: &str) -> impl rusti::Component + '_' {
    rusti! { <button class="btn">{ label }</button> }
}

fn page() -> impl rusti::Component {
    rusti! {
        <main>
            <h2>Dashboard</h2>
            @button("Save")
            @button("Cancel")
        </main>
    }
}
```
---

## 7. Dynamic Attributes
```rust
let disabled = true;
let img_src = "/avatars/me.png";
rusti! {
    <button disabled={disabled}>Submit</button>
    <img src={img_src} alt="Profile picture" />
}
```
---

## 8. Attributes with Spaces Around `=`
```rust
rusti! {
    <html lang = "en">
        <head>
            <meta charset = "UTF-8" />
        </head>
    </html>
}
```
---

## 9. Inline `<style>` and `<script>` (Raw Text)
```rust
rusti! {
    <style>
        body { background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%); }
        .card { backdrop-filter: blur(10px); border: 1px solid rgba(255,255,255,0.1); }
    </style>
    <script>
        console.log("Hello from inline script!");
    </script>
    <div class="card p-4">
        <h3>Styled Card</h3>
    </div>
}
```
---

## 10. Loose Styles / Scripts (No `<style>`/`<script>` wrapper)
You can embed raw CSS/JS directly as text nodes when you need them outside of a tag (e.g., when generating a `<style>` block programmatically). The parser treats any text that is not inside `<` … `>` as plain text, so braces are preserved.
```rust
let css = r#"body{background:linear-gradient(135deg,#0f172a 0%,#1e293b 100%)}"#;
let js = r#"console.log('inline');"#;
rusti! {
    <style>{ css }</style>
    <script>{ js }</script>
}
```
---

## 11. Self‑Closing Tags
```rust
rusti! {
    <img src="/logo.png" alt="Logo" />
    <br />
    <input type="text" name="username" />
}
```
---

## 12. Text with Braces (Escaping)
If you need literal `{` or `}` inside text (e.g., showing a code snippet), wrap the text in a raw string literal or escape it with double braces `{{` `}}` inside the macro:
```rust
rusti! {
    <pre>{ "{{ let x = 5; }}" }</pre>
}
```
---

## 13. Nested Components & Slots
```rust
fn card(title: &str, content: impl rusti::Component) -> impl rusti::Component + '_' {
    rusti! {
        <section class="card">
            <h2>{ title }</h2>
            @content
        </section>
    }
}

fn page() -> impl rusti::Component {
    rusti! {
        @card("Welcome", {
            <p>This is the inner slot content.</p>
        })
    }
}
```
---

## 14. HTML Comments (Not Supported – Workaround)
The parser does not currently support `<!-- comment -->`. Use Rust comments outside the macro or generate comments via a text node:
```rust
rusti! {
    <!-- This will cause a parse error -->
    // Instead, do:
    <p>{ "<!-- Safe comment -->" }</p>
}
```
---

## 15. Complex Example – Full Page
```rust
fn base_layout(title: &str, styles: &str, content: impl rusti::Component) -> impl rusti::Component + '_' {
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>{ title }</title>
                <style>{ styles }</style>
                <script src="https://cdn.tailwindcss.com"></script>
                <script src="https://unpkg.com/htmx.org@1.9.10"></script>
            </head>
            <body class="bg-gray-900 text-white min-h-screen">
                <nav class="glass-card rounded-2xl p-4 mb-8">
                    <a href="/" class="text-2xl font-bold">MyApp</a>
                    @if is_authenticated {
                        <a href="/profile">Profile</a>
                        <form action="/logout" method="post"><button type="submit">Logout</button></form>
                    } @else {
                        <a href="/login">Login</a>
                    }
                </nav>
                <main class="container mx-auto p-8">
                    @content
                </main>
                <footer class="text-center mt-8">© 2025 MyApp</footer>
            </body>
        </html>
    }
}
```

These examples should give you a solid foundation for using the `rusti!` macro in a variety of real‑world scenarios.
