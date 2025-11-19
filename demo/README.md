# Rusti Examples

This directory contains standalone examples demonstrating different features of Rusti.

## Running Examples

Each example is a separate binary that can be run independently:

```bash
# Hello World - Basic template usage
cargo run --bin 01_hello

# HTML Attributes - Tailwind CSS integration
cargo run --bin 02_attributes

# Component Composition - Reusable components
cargo run --bin 03_composition

# Conditional Rendering - If/else patterns
cargo run --bin 04_conditionals

# Match Expressions - Pattern matching
cargo run --bin 05_match

# XSS Protection - Automatic HTML escaping
cargo run --bin 06_xss_protection

# Full Web Server - Axum integration (in main.rs)
cargo run
```

## Example Overview

| Example | File | Features Demonstrated |
|---------|------|----------------------|
| **Hello World** | `01_hello.rs` | Basic template syntax, string interpolation |
| **Attributes** | `02_attributes.rs` | HTML attributes, Tailwind CSS classes |
| **Composition** | `03_composition.rs` | Component composition with `@` syntax |
| **Conditionals** | `04_conditionals.rs` | If/else rendering, dynamic content |
| **Match** | `05_match.rs` | Match expressions, pattern matching |
| **XSS Protection** | `06_xss_protection.rs` | Automatic HTML escaping for security |
| **Web Server** | `main.rs` | Full Axum web server with multiple routes |

## Learning Path

1. Start with **01_hello** to understand basic syntax
2. Move to **02_attributes** to see HTML attributes
3. Learn **03_composition** for building complex UIs
4. Explore **04_conditionals** and **05_match** for dynamic rendering
5. Review **06_xss_protection** to understand security
6. Finally, check **main.rs** for a complete web application

Each example is self-contained and demonstrates a specific feature clearly.
