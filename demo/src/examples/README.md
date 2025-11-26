# Azumi 2.0 Demo Examples

This directory contains **comprehensive examples** demonstrating azumi
2.0's features including the powerful `@let` variable binding and `@match` pattern matching syntax.

## Examples

### 1. Hello World (`hello.rs`)

**Route:** `/`
**Purpose:** Basic azumi 2.0 syntax demonstration

**Shows:**

-   Mandatory quoted text: `<h1>"Hello"</h1>`
-   Mandatory quoted attributes: `class="container"`
-   External CSS: `<style src="demo/static/hello.css" />` (scoped automatically)
-   External JS: `<script src="/static/hello.js" />`
-   Boolean attributes: `<button disabled>`

**Files:**

-   `hello.rs` - Component logic
-   `../static/hello.css` - Scoped styles
-   `../static/hello.js` - External JavaScript

---

### 2. Component Composition (`components.rs`)

**Route:** `/components`
**Purpose:** Demonstrate component patterns and composition

**Shows:**

-   Component functions with lifetime annotations
-   `@for` loops over collections
-   `@component` calls for composition
-   Dynamic content generation
-   Proper function naming (snake_case)

**Files:**

-   `components.rs` - Multiple component functions
-   `../static/components.css` - Component styles

---

### 3. HTMX Todo List (`htmx_todo.rs`)

**Route:** `/htmx-todo`
**Purpose:** Server-side rendering with HTMX

**Shows:**

-   HTMX attributes (`hx-post`, `hx-delete`, `hx-target`)
-   Server-side form handling
-   Dynamic content addition/removal
-   The recommended Rust approach (server-side vs client-side)
-   API endpoint integration

**API Endpoints:**

-   POST `/api/todos` - Add new todo
-   DELETE `/api/todos/:id` - Delete todo

**Files:**

-   `htmx_todo.rs` - Todo components and handlers
-   `../static/todo.css` - Dark theme styles

---

### 4. 🎯 @let & @match Mastery (`let_examples.rs`)

**Route:** `/let-examples`
**Purpose:** Comprehensive guide to Azumi's variable binding and pattern matching

**Shows:**

-   **Basic @let assignments:** `@let name = "value";`
-   **@match pattern matching:** Complex enum and data structure matching
-   **@let with @match combinations:** Using match results in let bindings
-   **Conditional @let assignments:** `@let status = if condition { "yes" } else { "no" };`
-   **Collection processing:** Filtering, mapping, and transforming data with @let
-   **Mathematical calculations:** Performing calculations and formatting results
-   **Dynamic CSS classes:** Computing class names based on application state
-   **Option and Result handling:** Working with Rust's error handling types
-   **Real-world utilities:** File size conversion, timestamp processing, email parsing
-   **Advanced pattern matching:** Nested matches, tuple destructuring, and complex conditions

**Key Features Demonstrated:**

-   Heavy use of `@match` patterns with enums, tuples, and ranges
-   `@let` variable binding within template contexts
-   Complex data transformation and processing
-   Integration with Rust's type system (Options, Results, enums)
-   Dynamic styling and class computation
-   Mathematical operations and unit conversions
-   Collection operations and aggregations

**Files:**

-   `let_examples.rs` - Complete @let & @match tutorial
-   `../static/let_examples.css` - Comprehensive styling examples

---

### 5. Control Flow (`control_flow.rs`)

**Route:** `/control-flow`
**Purpose:** Demonstrate @if, @for, and @match expressions

**Shows:**

-   `@if` conditional rendering (note: Azumi uses separate @if blocks, not @else)
-   `@for` loops over collections
-   `@match` pattern matching with ranges and literals
-   Integration of control flow with components

**Files:**

-   `control_flow.rs` - Control flow demonstrations
-   `../static/control_flow.css` - Control flow styles

---

### 6. Advanced Components (`advanced_components.rs`)

**Route:** `/advanced-components`
**Purpose:** Complex component patterns, forms, and validation

**Shows:**

-   Advanced component composition
-   Form handling and validation
-   Modal and popup patterns
-   Complex state management
-   Error handling and user feedback

**Files:**

-   `advanced_components.rs` - Advanced component examples
-   `../static/advanced_components.css` - Advanced component styles

---

### 7. Layouts (`layouts.rs`)

**Route:** `/layouts`
**Purpose:** Layout composition patterns and reusable structures

**Shows:**

-   Layout function composition
-   Nested component structures
-   Responsive design patterns
-   Component hierarchy management

**Files:**

-   `layouts.rs` - Layout composition examples
-   `../static/layouts.css` - Layout styles

---

### 8. Forms (`forms.rs`)

**Route:** `/forms`
**Purpose:** Form handling and input management

**Shows:**

-   Form input handling
-   Form validation patterns
-   Input state management
-   Form submission handling

**Files:**

-   `forms.rs` - Form examples
-   `../static/forms.css` - Form styles

---

### 9. Dashboard (`dashboard.rs`)

**Route:** `/dashboard`
**Purpose:** Complex layout and data visualization

**Shows:**

-   Complex dashboard layouts
-   Data visualization components
-   Real-time data patterns
-   Multi-component coordination

**Files:**

-   `dashboard.rs` - Dashboard example
-   `../static/dashboard.css` - Dashboard styles

---

### 10. Tailwind CSS (`tailwind.rs`)

**Route:** `/tailwind`
**Purpose:** Utility-first styling with Tailwind CSS

**Shows:**

-   Integration with Tailwind CSS
-   Utility class composition
-   Responsive design with utilities
-   CDN integration patterns

**Files:**

-   `tailwind.rs` - Tailwind integration examples

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

-   Boolean attributes: `disabled`, `checked`, etc. (no value needed)
-   JSON data scripts: `<script type="application/json">{data}</script>`

---

## File Structure

```

examples/
├── mod.rs              # Module exports
├── hello.rs            # Hello World example
├── components.rs       # Component composition
├── control_flow.rs     # @if, @for, @match expressions
├── let_examples.rs     # 🎯 @let & @match mastery
├── advanced_components.rs # Complex components & forms
├── layouts.rs          # Layout composition
├── forms.rs            # Form handling
├── dashboard.rs        # Complex dashboard layouts
├── htmx_todo.rs       # HTMX server-side rendering
├── tailwind.rs         # Tailwind CSS integration
└── README.md          # This file

../static/
├── hello.css          # Scoped hello world styles
├── hello.js           # Hello world interactivity
├── components.css     # Component styles
├── control_flow.css   # Control flow styles
├── let_examples.css   # 🎯 @let examples styling
├── advanced_components.css # Advanced component styles
├── layouts.css        # Layout styles
├── forms.css          # Form styles
├── dashboard.css      # Dashboard styles
├── todo.css           # HTMX todo styles
└── tailwind.css       # Tailwind integration styles
```

## Design Philosophy

These examples demonstrate **azumi 2.0's opinionated approach:**

-   **Rigor over convenience** - Mandatory quoting prevents edge cases
-   **Tooling over inline code** - External files get IDE support
-   **Type safety** - Catch errors at compile time, not runtime
-   **Zero surprises** - Consistent rules, predictable behavior
-   **Powerful pattern matching** - Leverage Rust's @match and @let for complex logic
-   **Functional programming** - Transform data with @let and collections

### Key @let Features Demonstrated

The **@let & @match mastery** example showcases:

1. **Variable Binding:** Create local variables within templates
2. **Pattern Matching:** Use @match for complex conditional logic
3. **Data Transformation:** Process and format data dynamically
4. **Collection Operations:** Filter, map, and aggregate collections
5. **Type Safety:** Work seamlessly with Rust's Option, Result, and enums
6. **Dynamic Styling:** Compute CSS classes based on application state
7. **Real-world Utilities:** File processing, date handling, validation

### @let Syntax Patterns

```rust
// Basic assignment
@let name = "value";

// Pattern matching
@let display = match status {
    Status::Active => "✅ Active",
    Status::Pending => "⏳ Pending",
    _ => "❓ Unknown"
};

// Conditional assignment
@let message = if count > 10 { "Many" } else { "Few" };

// Collection processing
@let filtered_items: Vec<_> = items.iter()
    .filter(|item| item.active)
    .collect();
```

No silent errors. No guesswork. Just Rust with powerful template variables.
