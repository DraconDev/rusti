# Rusti Roadmap & TODO

## 🚀 Future Improvements

1. make a todo app example using htmx as much as possible and compare it to the todo app example in the rusti-demo repo

2. support for emojis in html

3. add fragment support

4. lets update the readme and examples with all the script related features we did and learned

5. add 
This is a very practical, grounded approach. You are essentially implementing Server-Side Data Injection, which is a requirement for 99% of SSR apps.
Before we talk about compiling Rust to JS, let's perfect this specific pattern because it is the foundation of the "Bridge" between your server and client.
Here is my analysis of your snippet, why it is likely buggy, and the one change that will make it bulletproof.
The Good
Scope Access: You can access my_number from the outer Rust scope. This is seamless.
Shadowing/Local Vars: You can define @let inside the script tag (in Rust land) to prepare data for the JS land.
Syntax: @{ ... } is a very clear delimiter. It tells the parser "Stop reading JS tokens, read a Rust expression here."
The Problem (The "Buggy" Part)
You are manually formatting strings:
code
Rust
// Your code
const str = @{ format!("\"{}\"", my_string) };
Why this is dangerous/buggy:
Quote Escaping: If my_string contains a double quote (Hello "World"), your JS breaks: const str = "Hello "World"". Syntax Error.
XSS Injection: If my_string contains </script>, it closes the tag early and injects HTML.
Type Fatigue: You have to manually remember to wrap strings in quotes but leave numbers alone.
The Fix: Automatic JSON Serialization
Instead of treating the output as a simple string substitution, you should require (or automatically use) serde_json.
If your macro implementation changes to use serde_json::to_string(&value), your usage becomes cleaner and safer.
The Improved Syntax:
code
Rust
rusti! {
    <script>
        // No manual formatting!
        // The macro calls json!(my_number) internally
        const num = @{ my_number }; 
        
        // The macro automatically adds quotes and escapes special chars
        const str = @{ my_string }; 
        
        // Works for complex structs too!
        const user = @{ user_struct };
    </script>
}


7. <!-- bug: < > are not parsed correctly they are interpreted as html tags !!! -->

8. f string solution for ?
hx-post={format!("/api/tasks/{}/toggle", task.id)}
hx-target={format!("#task-{}", task.id)}
Current: id={format!("task-{}", task.id)}
Dream: id=f"task-{task.id}" (If Rust eventually stabilizes f-strings, you are ready).

9. scoped css

10. optional props with default values

11. props spreading 

12. explain in readme the @input vs input Input component decision

13. update the error reporting to point to the exact location of the error in the source code


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



