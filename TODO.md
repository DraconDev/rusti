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
