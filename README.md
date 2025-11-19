# Rusti

A type-safe, Rust-native HTML templating library inspired by Go's `templ`.

`rusti` allows you to write HTML-like syntax directly within your Rust code using procedural macros. It compiles down to efficient Rust code that writes directly to a formatter, ensuring high performance and type safety.

## Features

- **Rust-Native**: Write templates directly in your `.rs` files using the `rusti!` macro.
- **Type-Safe**: Components are compiled to Rust functions. Arguments are type-checked.
- **Zero-Cost Abstraction**: Templates are compiled to efficient `std::fmt::Write` calls. No runtime parsing or interpretation.
- **Component Composition**: Easily nest and compose components.

## Installation

Add `rusti` to your `Cargo.toml`:

```toml
[dependencies]
rusti = { path = "path/to/rusti" } # Replace with actual version/path
```

## Usage

### Defining a Component

Use the `rusti!` macro to define a component. A component is simply a Rust function that returns `impl rusti::Component`.

```rust
use rusti::rusti;

rusti! {
    fn hello(name: &str) {
        <div>
            <h1>Hello, { name }!</h1>
            <p>Welcome to Rusti.</p>
        </div>
    }
}
```

### Using a Component

Components are just functions. You can call them to get a renderable object.

```rust
fn main() {
    let component = hello("World");
    
    // Render to string (using a helper or custom wrapper)
    // Note: We currently recommend wrapping in a struct that implements Display
    // for easy printing, or using the component directly if you have a helper.
}
```

### Syntax

#### Elements
Standard HTML tags are supported.

```rust
<div><span>Content</span></div>
```

#### Text
Plain text is rendered as-is.

```rust
<p>Hello World</p>
```

#### Expressions
Use `{ ... }` to embed Rust expressions. The expression must implement `std::fmt::Display`.

```rust
<div>Value: { 1 + 2 }</div>
```

#### Component Calls
Use `@` to call other components.

```rust
rusti! {
    fn layout(title: &str) {
        <html>
            <head><title>{ title }</title></head>
            <body>
                @header()
                <main>Content</main>
            </body>
        </html>
    }
}
```

## Project Structure

- `rusti`: The runtime library.
- `rusti-macros`: The procedural macro implementation.
- `example`: A demo application.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
