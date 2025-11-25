# azumi
 2.0 Demo Examples

This directory contains **3 clean examples** demonstrating azumi
 2.0's mandatory quoting rules and external file requirements.

## Examples

### 1. Hello World (`hello.rs`)
**Route:** `/`
**Purpose:** Basic azumi
 2.0 syntax demonstration

**Shows:**
- Mandatory quoted text: `<h1>"Hello"</h1>`
- Mandatory quoted attributes: `class="container"`
- External CSS: `<style src="demo/static/hello.css" />` (scoped automatically)
- External JS: `<script src="/static/hello.js" />`
- Boolean attributes: `<button disabled>`

**Files:**
- `hello.rs` - Component logic
- `../static/hello.css` - Scoped styles
- `../static/hello.js` - External JavaScript

---

### 2. Component Composition (`components.rs`)
**Route:** `/components`
**Purpose:** Demonstrate component patterns and composition

**Shows:**
- Component functions with lifetime annotations
- `@for` loops over collections
- `@component` calls for composition
- Dynamic content generation
- Proper function naming (snake_case)

**Files:**
- `components.rs` - Multiple component functions
- `../static/components.css` - Component styles

---

### 3. HTMX Todo List (`htmx_todo.rs`)
**Route:** `/htmx-todo`
**Purpose:** Server-side rendering with HTMX

**Shows:**
- HTMX attributes (`hx-post`, `hx-delete`, `hx-target`)
- Server-side form handling
- Dynamic content addition/removal
- The recommended Rust approach (server-side vs client-side)
- API endpoint integration

**API Endpoints:**
- POST `/api/todos` - Add new todo
- DELETE `/api/todos/:id` - Delete todo

**Files:**
- `htmx_todo.rs` - Todo components and handlers
- `../static/todo.css` - Dark theme styles

---

## Running the Demo

```bash
cd demo
cargo run
```

Visit http://localhost:8081 to see all examples.

## azumi
 2.0 Rules (Enforced)

All examples follow these mandatory rules:

### ✅ Must Do
1. **Quote all text:** `<h1>"text"</h1>`
2. **Quote all attribute values:** `class="value"`
3. **External CSS:** `<style src="file.css" />` (automatically scoped)
4. **External JS:** `<script src="/app.js" />`

### ❌ Not Allowed
1. Unquoted text: `<h1>text</h1>`
2. Unquoted attributes: `class=value`
3. Inline styles: `<style>.card { ... }</style>`
4. Inline scripts: `<script>const x = 42;</script>`

### ✓ Exceptions
- Boolean attributes: `disabled`, `checked`, etc. (no value needed)
- JSON data scripts: `<script type="application/json">{data}</script>`

---

## File Structure

```
examples/
├── mod.rs              # Module exports
├── hello.rs            # Hello World example
├── components.rs       # Component composition
├── htmx_todo.rs       # HTMX server-side rendering
└── README.md          # This file

../static/
├── hello.css          # Scoped hello world styles
├── hello.js           # Hello world interactivity  
├── components.css     # Component styles
└── todo.css           # HTMX todo styles
```

## Design Philosophy

These examples demonstrate **azumi
 2.0's opinionated approach:**

- **Rigor over convenience** - Mandatory quoting prevents edge cases
- **Tooling over inline code** - External files get IDE support
- **Type safety** - Catch errors at compile time, not runtime
- **Zero surprises** - Consistent rules, predictable behavior

No silent errors. No guesswork. Just Rust
.
