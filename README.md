# Rusti

A type-safe, Rust-native HTML templating library inspired by Go's `templ`.

## Features

- **Rust-Native**: Write templates directly in your `.rs` files using the `rusti!` macro.
- **Type-Safe**: Components are just Rust functions. Arguments are type-checked.
- **Zero-Cost Abstraction**: Templates are compiled to efficient `std::fmt::Write` calls. No runtime parsing or interpretation.
- **Component Composition**: Nest components easily using `@Component(args)` syntax.
- ✅ **Type-Safe**: All templates checked at compile time
- ✅ **Zero-Cost**: Compiles to efficient `fmt::Write` calls
- ✅ **XSS Protected**: Automatic HTML escaping for all dynamic content
- ✅ **Component Composition**: Nest components infinitely
- ✅ **Tailwind Support**: Full HTML attribute parsing
- ✅ **Rust Integration**: Use `if/else`, `match`, and any Rust expression

## Quick Start

Add Rusti to your `Cargo.toml`:
```toml
[dependencies]
rusti = { path = "path/to/rusti" }
```

Write type-safe templates:
```rust
use rusti::rusti;

fn greeting(name: &str) -> impl rusti::Component + '_ {
    rusti! {
        <div class="greeting">
            <h1>Hello, { name }!</h1>
        </div>
    }
}

fn main() {
    println!("{}", greeting("World").render_to_string());
}
```

## Web Framework Integration

### Axum
```rust
use axum::{response::Html, routing::get, Router};

async fn index() -> Html<String> {
    let page = rusti! {
        <html>
            <head><title>My App</title></head>
            <body><h1>Welcome!</h1></body>
        </html>
    };
    Html(rusti::Component::render_to_string(&page))
}
```

## Examples

Run the demo web server:
```bash
cargo run -p rusti-demo
```

Visit `http://127.0.0.1:3000` to see:
- Component composition
- Conditional rendering
- Match expressions
- Tailwind CSS styling

## Documentation

- **[USAGE.md](USAGE.md)** - Comprehensive usage guide
- **[LIMITATIONS.md](LIMITATIONS.md)** - Current parser limitations
- **[KNOWN_ISSUES.md](KNOWN_ISSUES.md)** - IDE/tooling quirks

## Project Structure

```
rusti/
├── src/                # Runtime library (Component trait, Escaped type)
├── macros/             # Procedural macro implementation
├── demo/               # Demo web server with Axum
└── Cargo.toml          # Main library + workspace config
```

## How It Works

The `rusti!` macro parses HTML-like syntax and generates Rust code:

```rust
rusti! { <h1>{ name }</h1> }
```

Compiles to:
```rust
move |__rusti_writer: &mut dyn std::fmt::Write| {
    write!(__rusti_writer, "<h1>")?;
    write!(__rusti_writer, "{}", rusti::Escaped(name))?;
    write!(__rusti_writer, "</h1>")?;
    Ok(())
}
```

## License

MIT
