# Rusti Roadmap & TODO

## 🚀 Future Improvements



7. <!-- bug: < > are not parsed correctly they are interpreted as html tags !!! -->

8. scoped css, and get rid of style sheets if we do this?

9. optional props with default values

10. props spreading 

11. explain in readme the @input vs input Input component decision

12. update the error reporting to point to the exact location of the error in the source code


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



