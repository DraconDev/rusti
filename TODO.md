# Rusti Roadmap & TODO

## 🚀 Future Improvements

### 1. Surgical Error Reporting (High Priority)
- **Goal**: Improve compiler error messages by pointing to the exact location of the error in the source code.
- **Current State**: Errors often point to the macro invocation site.
- **Implementation**: Refactor parser to use `syn` to parse `TokenStream` directly instead of converting to string. Use `syn::spanned::Spanned` to attach span information to nodes.


### 2. Optional Props & Defaults
- **Goal**: Allow optional arguments in `#[component]` macros.
- **Implementation**:
    - Support `#[prop(default = ...)]` attribute.
    - Generate builder pattern or use `Default` trait for `Props`.

### 3. Typed Children
- **Goal**: Better support for passing components as children.
- **Implementation**:
    - Support `children: impl Component` prop automatically?
    - Slot mechanism?

### 4. Namespaced Attributes
- [/] Support attributes with colons (e.g., `xml:lang`, `v-bind:class`, `hx-on:click`).
- **Implementation**: Update `parse_html_name` to handle `:`.

### 5. IDE Support
- **Goal**: Improve developer experience.
- **Implementation**: VS Code extension or LSP for `rusti!` macro content.

### 6. Better Text Parsing
- **Goal**: Improve whitespace handling in text nodes.
- **Current State**: `TokenStream` conversion loses some whitespace information.
- **Implementation**: Use `syn::spanned::Spanned` or custom lexer to preserve whitespace.
