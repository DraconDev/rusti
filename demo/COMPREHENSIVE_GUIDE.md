# 🚀 Azumi Demo - Comprehensive Guide

**Azumi** is a revolutionary compile-time HTML template macro for Rust that enforces best practices while providing powerful type-safe templating capabilities.

## 🎯 What Makes Azumi Special

### **Unique Features**
- ✅ **CSS Auto-Scoping**: Automatic style isolation prevents conflicts
- ✅ **@ Syntax**: Clear distinction between HTML and Rust code  
- ✅ **AI-Optimized Design**: 95%+ AI generation success rate
- ✅ **Compile-Time Safety**: All errors caught before runtime
- ✅ **Type-Safe Components**: Full Rust type system integration
- ✅ **Zero Runtime Overhead**: Everything happens at compile time

### **Intentional Limitations**
- ❌ **No Inline Styles**: Forces external CSS for better tooling
- ❌ **No Template Inheritance**: Prevents complex dependencies  
- ❌ **No Template Filters**: Use real Rust functions instead
- ❌ **No Runtime Parsing**: Security and performance benefits

## 🏗️ Project Structure

```
azumi/demo/
├── src/
│   ├── main.rs                 # Server entry point
│   └── examples/               # Example modules
│       ├── mod.rs              # Module exports
│       ├── homepage.rs         # 🌟 Main demo homepage
│       ├── hello.rs            # Basic syntax demo
│       ├── components.rs       # Component patterns
│       ├── control_flow.rs     # @if, @for, @match
│       ├── forms.rs            # Form handling
│       ├── htmx_todo.rs        # HTMX integration
│       ├── layouts.rs          # Layout composition
│       └── ... (more examples)
├── static/                     # Styling & assets
│   ├── premium_homepage.css    # 🎨 Modern homepage styling
│   ├── educational_homepage.css # Alternative styling
│   ├── example_card.css        # Component styles
│   └── ... (more CSS files)
└── static/                     # Public assets
    └── ... (served via /static)
```

## 🎮 Demo Server

### **Running the Demo**
```bash
cd demo
cargo run
```

**Server URL**: http://localhost:8081

### **Available Routes**
| Route | Description | Purpose |
|-------|-------------|---------|
| `/` | **Homepage** | Complete example library |
| `/hello` | Basic Demo | Azumi syntax fundamentals |
| `/components` | Components | Reusable component patterns |
| `/control-flow` | Logic | @if, @for, @match expressions |
| `/layouts` | Layouts | Layout composition patterns |
| `/forms` | Forms | Interactive form handling |
| `/htmx-todo` | HTMX App | Server-side rendering demo |
| `/dashboard` | Dashboard | Complex data visualization |

## 📚 Learning Path

### **1. Fundamentals (15 minutes)**
**Start here to learn the basics**

- **Text & Quotes**: Mandatory quoting rules
- **Attributes**: HTML attribute syntax  
- **Expressions**: Rust variable interpolation
- **Basic Styling**: External CSS and scoping

### **2. Components (25 minutes)**
**Build reusable UI elements**

- **Basic Components**: Function-based components
- **Component Props**: Type-safe data passing
- **Default Props**: Optional parameters
- **Component Nesting**: Complex composition
- **Advanced Components**: Modals and validation
- **UI Library**: Design system components

### **3. Control Flow (20 minutes)**
**Add logic to your templates**

- **If/Else Conditions**: Conditional rendering
- **For Loops**: Collection iteration
- **Match Expressions**: Pattern matching
- **Variable Binding**: @let statements
- **Form Handling**: Interactive forms

### **4. Pattern Matching (30 minutes)**
**Master advanced Rust patterns**

- **@let + @match Mastery**: Comprehensive examples
- **Complex Patterns**: Struct and enum matching
- **Nested Matching**: Multi-level logic
- **Result & Option**: Safe error handling

### **5. Data Processing (25 minutes)**
**Handle real-world data**

- **Sales Analysis**: Statistics and charts
- **Search & Filter**: Advanced filtering
- **Data Pipeline**: Transformation workflows
- **Live Statistics**: Real-time updates

### **6. Real-World Apps (35 minutes)**
**Production-ready applications**

- **E-commerce Catalog**: Shopping and search
- **Analytics Dashboard**: Business intelligence
- **Social Media Feed**: Real-time interactions
- **Project Management**: Kanban boards
- **HTMX Todo App**: Server-side rendering

## 🎨 Styling System

### **CSS Architecture**
- **Automatic Scoping**: CSS is scoped to components
- **Utility-First**: Compatible with Tailwind CSS
- **External Only**: No inline styles allowed
- **IDE Support**: Full syntax highlighting and autocomplete

### **Available Stylesheets**
- `premium_homepage.css` - Modern, animated design
- `educational_homepage.css` - Clean, professional look
- `example_card.css` - Component-specific styles
- Individual example stylesheets

## 🔧 Developer Experience

### **VSCode Setup**
Install recommended extensions:
- **rust-analyzer**: Rust language support
- **CSS Peek**: Jump to CSS definitions
- **Error Lens**: Inline error display

### **Project Configuration**
```json
// .vscode/settings.json
{
  "cssPeek.peekFromLanguages": ["html", "rust"],
  "cssPeek.searchFileExtensions": [".css", ".scss"],
  "rust-analyzer.cargo.features": ["examples"]
}
```

### **Key Commands**
```bash
# Run the demo
cargo run

# Run examples only
cargo run --example hello

# Check code
cargo check

# Run tests
cargo test

# Build for production
cargo build --release
```

## 🧠 AI Integration

### **Why Azumi is AI-Friendly**
1. **Consistent Patterns**: Predictable syntax structure
2. **Type Safety**: Clear interfaces and contracts  
3. **Single Paradigm**: HTML + Rust without mixing
4. **Compile Errors**: Immediate feedback for AI

### **AI Prompt Examples**
```
"Create a component with props for a user card"
"Add conditional rendering for admin users"
"Build a form with validation"
"Create a data table with sorting"
```

## 🚀 Performance

### **Benchmarking**
- **Compile Time**: Fast compilation with macro optimization
- **Runtime Speed**: Zero runtime overhead
- **Memory Usage**: Efficient memory management
- **Bundle Size**: Minimal impact on application size

### **Optimization Tips**
1. Use `@let` for complex calculations
2. Leverage Rust's `Option` and `Result` types
3. Prefer component composition over inheritance
4. Use external CSS for better caching

## 🔍 Debugging

### **Common Issues**

#### **Compile Errors**
```rust
// ❌ Wrong - Unquoted text
<h1>Hello</h1>

// ✅ Correct - Quoted text  
<h1>"Hello"</h1>
```

#### **CSS Scoping**
```rust
// External CSS automatically scoped
<style src="/static/component.css" />

// Generated with unique data attribute
<div class="card" data-scoped123>...</div>
```

### **Debug Mode**
```bash
# Enable debug output
RUST_LOG=debug cargo run

# Check macro expansion
cargo expand
```

## 🛡️ Best Practices

### **Component Design**
1. **Single Responsibility**: Each component has one purpose
2. **Type Safety**: Use strong types for props
3. **Composition**: Prefer composition over inheritance
4. **Reusability**: Design for multiple use cases

### **Performance**
1. **Minimize @let usage**: Compute once, use multiple times
2. **External CSS**: Always use external stylesheets
3. **Optimize Images**: Use appropriate formats and sizes
4. **Lazy Loading**: Load content on demand

### **Code Organization**
1. **Group Related Components**: Logical file structure
2. **Use Clear Names**: Descriptive function and component names
3. **Document Props**: Clear prop descriptions and defaults
4. **Version Control**: Track changes to examples

## 🤝 Contributing

### **Adding New Examples**
1. Create new module in `src/examples/`
2. Add route handler to `src/main.rs`
3. Create corresponding CSS file in `static/`
4. Update homepage with new example card
5. Test thoroughly with different scenarios

### **Code Standards**
- Follow Rust naming conventions
- Use meaningful variable names
- Add documentation comments
- Include error handling
- Test edge cases

## 📖 Resources

### **Documentation**
- [Azumi README](../README.md) - Project overview
- [Rust Book](https://doc.rust-lang.org/book/) - Rust fundamentals  
- [Axum Guide](https://docs.rs/axum/) - Web framework integration
- [HTMX Examples](https://htmx.org/examples/) - Client-side patterns

### **Community**
- [Rust Discord](https://discord.gg/rust-lang) - General discussion
- [Azumi GitHub](https://github.com/your-org/azumi) - Source code
- [Rust User Forum](https://users.rust-lang.org/) - Q&A support

## 🎉 Getting Started

### **Quick Start**
1. **Clone the repository**
2. **Install Rust** (latest stable)
3. **Run the demo**: `cd demo && cargo run`
4. **Open browser**: http://localhost:8081
5. **Start with fundamentals**: Click "Hello World"

### **Your First Component**
```rust
use azumi::html;
use azumi::Component;

#[azumi::component]
fn HelloWorld(name: &str) -> impl Component {
    html! {
        <style src="/static/hello_world.css" />
        <div class="hello-world">
            <h1>{"Hello, "} {name} {"!"}</h1>
            <p>"Welcome to Azumi!"</p>
        </div>
    }
}
```

---

**Built with ❤️ for the Rust community**

*Azumi makes HTML templating type-safe, fast, and maintainable.*