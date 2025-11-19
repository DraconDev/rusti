# Rusti

A type-safe, Rust-native HTML templating library inspired by Go's `templ`.

## Features

- **Rust-Native**: Write templates directly in your `.rs` files using the `rusti!` macro.
- **Type-Safe**: Components are just Rust functions. Arguments are type-checked.
- **Zero-Cost Abstraction**: Templates are compiled to efficient `std::fmt::Write` calls. No runtime parsing or interpretation.
- **Component Composition**: Nest components easily using `@Component(args)` syntax.

## Installation

Add `rusti` to your `Cargo.toml`:

```toml
[dependencies]
rusti = { path = "rusti" } # Replace with actual version when published
```

## Usage

### Basic Component

Define a component as a function that returns `impl rusti::Component`. Use the `rusti!` macro to define the HTML structure.

```rust
use rusti::rusti;

fn hello(name: &str) -> impl rusti::Component + '_ {
    rusti! {
        <div>
            <h1>Hello, { name }!</h1>
            <p>Welcome to Rusti.</p>
        </div>
    }
}

fn main() {
    let component = hello("World");
    println!("{}", rusti::render_to_string(&component));
}
```

### Component Composition

You can call other components using the `@` syntax.

```rust
fn header(title: &str) -> impl rusti::Component + '_ {
    rusti! {
        <header><h1>{ title }</h1></header>
    }
}

fn page(title: &str) -> impl rusti::Component + '_ {
    rusti! {
        <div>
            @header(title)
            <main>Content</main>
        </div>
    }
}
```

### Control Flow

Currently, you can use Rust expressions `{ ... }` to embed dynamic content. Complex control flow (if/for) inside the macro is planned but can be achieved using Rust logic outside or inside expressions.

## License

MIT
