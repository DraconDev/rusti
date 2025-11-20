# Current Limitations

## 1. Inline `<style>` and `<script>` Tags

**Status**: Not supported

**Issue**: The parser treats content inside `<style>` and `<script>` tags as potential Rust code, causing parse errors with CSS and JavaScript syntax (colons, semicolons, etc. conflict with Rust syntax).

**Workaround**: 
- Use external CSS/JS files
- Use inline `style` attributes (limited)
- Link tags for CSS: `<link rel="stylesheet" href="/styles.css">`
- Script tags with `src` attribute: `<script src="/app.js"></script>`

**Example**:
```rust
rusti! {
    <html>
        <head>
            <link></link>  // External CSS
        </head>
        <body>
            <h1>Hello</h1>
            <script></script>  // External JS
        </body>
    </html>
}
```

## 2. Conditional Rendering Type Issues

**Status**: Partial support

**Issue**: Each `rusti!` macro invocation creates a unique closure type, so you can't directly use if/else to choose between different rusti! calls and assign to the same variable.

**Workaround**: Create helper functions that return boxed components or use `dyn Component` trait objects.

**What works**:
```rust
// Helper function approach
fn badge(is_admin: bool) -> Box<dyn rusti::Component> {
    if is_admin {
        Box::new(rusti! { <span>Admin</span> })
    } else {
        Box::new(rusti! { <span>User</span> })
    }
}
```

**What doesn't work**:
```rust
// This fails - different closure types
let badge = if is_admin {
    rusti! { <span>Admin</span> }  // Type A
} else {
    rusti! { <span>User</span> }   // Type B - incompatible!
};
```

## 3. Inline `<style>` and `<script>` Tags

**Status**: Partial support

**Issue**: The parser treats content inside `<style>` and `<script>` tags as potential Rust code.

**Workaround**:
- Use external CSS/JS files (Recommended)
- Use spaced closing tags for scripts: `< / script >` (Parser now supports this)

## Future Enhancements

These features could be added in future versions:

1. **Raw content support**: Special syntax to include CSS/JS without parsing (e.g., `@raw { ... }`)
2. **Fragment rendering**: Support for rendering component collections without a wrapper element
3. **More complex pattern matching**: Enhanced `@match` capabilities

## What Works Great

✅ Basic HTML structure  
✅ Component composition with `@component(args)`  
✅ Dynamic content with `{ expr }`  
✅ XSS protection (automatic HTML escaping)  
✅ Type-safe components  
✅ Nested elements  
✅ External CSS and JavaScript files
