# Rusti Roadmap & TODO

## 🚀 Future Improvements

bug: < > are not parsed correctly they are interpreted as html tags !!!

add fragment support


### 1. Surgical Error Reporting (High Priority)
- **Goal**: Improve compiler error messages by pointing to the exact location of the error in the source code.
- **Status**: Completed.


### 2. Optional Props & Defaults
- **Goal**: Allow optional arguments in `#[component]` macros.
- **Implementation**:
    - Support `#[prop(default = ...)]` attribute.
    - Generate builder pattern or use `Default` trait for `Props`.

### 3. Typed Children
- **Goal**: Better support for passing components as children.
- **Status**: Completed.

### 4. Namespaced Attributes
- **Goal**: Support attributes with colons (e.g., `xml:lang`, `v-bind:class`, `hx-on:click`).
- **Status**: Completed.



