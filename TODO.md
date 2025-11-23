# Rusti Roadmap & TODO

## 🚀 Future Improvements

### 1. Surgical Error Reporting (High Priority)
- **Goal**: Improve compiler error messages by pointing to the exact location of the error in the source code.
- **Current State**: Errors often point to the macro invocation site.
- **Implementation**: Refactor parser to use `syn` to parse `TokenStream` directly instead of converting to string. Use `syn::spanned::Spanned` to attach span information to nodes.


### 3. Optional Props & Defaults
- **Goal**: Allow optional arguments in `#[component]` macros.
- **Implementation**:
    - Support `#[prop(default = ...)]` attribute.
    - Generate builder pattern or use `Default` trait for `Props`.

### 4. Typed Children
- **Goal**: Better support for passing components as children.
- **Implementation**:
    - Support `children: impl Component` prop automatically?
    - Slot mechanism?

### 5. Fix Compilation Errors (Immediate)
- **Goal**: Fix `expected one of ...` errors in `base_layout_demo.rs` and `extreme.rs`.
- **Status**: Completed.

### 6. Fix CSS Loading
- **Goal**: Support `<style src="...">` to embed external CSS files.
- **Status**: Completed.

### 7. Namespaced Attributes
- **Goal**: Support attributes with colons (e.g., `xml:lang`, `v-bind:class`).
- **Implementation**: Update `parse_html_name` to handle `:`.

### 8. HTMX Integration Helpers
- **Goal**: Type-safe macros or helpers for HTMX attributes.
- **Implementation**: `hx!` macro or specific attribute builders.

### 9. IDE Support
- **Goal**: Improve developer experience.
- **Implementation**: VS Code extension or LSP for `rusti!` macro content.

### 10. Better Text Parsing
- **Goal**: Improve whitespace handling in text nodes.
- **Current State**: `TokenStream` conversion loses some whitespace information.
- **Implementation**: Use `syn::spanned::Spanned` or custom lexer to preserve whitespace.
