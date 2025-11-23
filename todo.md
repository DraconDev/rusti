# Rusti TODOs

## Known Limitations & Workarounds

### 1. Emoji Support in Text Content ✅ SOLVED
- **Issue**: Emojis directly in HTML text cause Rust lexer errors
- **Root Cause**: Rust's tokenizer runs before the `rusti!` macro, and emojis after whitespace are treated as invalid identifier tokens
- **✅ Solution**: Use Rust variables for text containing emojis

```rust
// ❌ FAILS - Emoji directly in text
rusti! {
    <a>Home ✅</a>  // Error: identifiers cannot contain emoji
}

// ✅ WORKS - Emoji in Rust variable
let text = "Home ✅";  // Rust strings support Unicode/emoji
rusti! {
    <a>{text}</a>  // Perfect!
}
```

**Why this works**: Rust strings are tokenized correctly, then interpolated into the template.

### 2. Datastar Integration ✅ FIXED
- Updated to modern `@sudodevnull/datastar` CDN
- Proper `data-store`, `data-text`, `data-on-click`, `data-model` attributes
- Raw strings for JSON in `data-store`
- Working examples: counter, input binding

## Future Parser Enhancements

### Potential Improvements
- [ ] Accept raw string literals as macro input: `rusti!(r#"..."#)` to bypass Rust lexer
- [ ] Better error messages for common mistakes
- [ ] Support for more complex JavaScript in attributes
- [ ] Macro diagnostics with spans pointing to exact error locations

### Documentation Additions
- [ ] Migration guides (Maud, Askama, Leptos, Tera → Rusti)
- [ ] Video tutorials showing DX
- [ ] Performance benchmarks vs other templating libraries
- [ ] Common patterns cookbook

### Example Demos
- [ ] HTMX advanced patterns (infinite scroll, lazy loading)
- [ ] Alpine.js integration
- [ ] Server-sent events (SSE) real-time updates
- [ ] WebSocket integration
- [ ] Form validation with error messages
- [ ] Authentication/authorization example

### Ecosystem
- [ ] LSP support for better IDE integration
- [ ] Syntax highlighting for `rusti!` blocks
- [ ] Hot reload development workflow
- [ ] Testing utilities for component rendering

## Pattern Reference

### ✅ Working Patterns

```rust
// Unicode/Emoji in variables
let greeting = "Hello 👋";
let status = "Done ✅";
rusti! { <p>{greeting} - {status}</p> }

// Complex JavaScript (use raw strings)
rusti! {
    <button onclick={r#"alert('Hello!')"#}>Click</button>
}

// JSON attributes (use raw strings)
rusti! {
    <div data-config={r#"{"theme": "dark"}"#}></div>
}

// CSS with em units (quote or use raw string)
rusti! {
    <style>
        .box { margin: "2em"; }  /* Quoted */
    </style>
}

// OR
rusti! {
    <style>{r#"
        .box { margin: 2em; }  /* Raw string */
    "#}</style>
}
```

### ❌ Patterns to Avoid

```rust
// Direct emoji in text (Rust lexer fails)
rusti! { <p>Hello ✅</p> }  // ❌ Fails

// Single quotes for attributes (Rust char literals)
rusti! { <div class='box'></div> }  // ❌ Fails

// Backticks (invalid Rust tokens)
rusti! { <script>{`code`}</script> }  // ❌ Fails

// Unquoted em units
rusti! { <style>.box { margin: 2em; }</style> }  // ❌ Fails (scientific notation)
```