# 🎓 Azumi Comprehensive Lesson Plan

## 🚀 Mastering Azumi: From Basics to Advanced Patterns

### 📚 Course Overview

This comprehensive lesson plan covers all aspects of Azumi, from basic syntax to advanced patterns, showcasing the full power of the framework's macro system while adhering to Azumi's strict validation rules.

## 🏗️ Foundational Concepts

### Lesson 1: Introduction to Azumi Components

```rust
// Basic component structure with proper syntax
#[azumi::component]
fn HelloWorld() -> impl azumi::Component {
    html! {
        <style>
            .greeting { color: "#1976d2"; font-size: "1.5rem"; }
        </style>
        <div class={greeting}>"Hello, Azumi!"</div>
    }
}
```

### Lesson 2: CSS Scoping & Validation Fundamentals

```rust
// Automatic CSS scoping demonstration
#[azumi::component]
fn ScopedComponent() -> impl azumi::Component {
    html! {
        <style>
            .container { padding: "1rem"; border: "1px solid #ddd"; }
            .title { color: "#2196f3"; }
        </style>
        <div class={container}>
            <h1 class={title}>"Automatically Scoped CSS"</h1>
            <p>"This CSS is scoped to this component only"</p>
        </div>
    }
}
```

### Lesson 3: Global vs Component CSS

```rust
// Understanding style scoping options
#[azumi::component]
fn StyleScopingExample() -> impl azumi::Component {
    html! {
        // Global styles - not scoped to component
        <style global>
            body { font-family: "Arial, sans-serif"; }
            .global_class { color: "purple"; }
        </style>

        // Component-scoped styles - automatically scoped
        <style>
            .component_class { background: "#f5f5f5"; }
            .local_class { color: "blue"; }
        </style>

        <div class={component_class}>
            <h2 class={local_class}>"Scoped Style"</h2>
            <p class="global_class">"Global Style"</p>
        </div>
    }
}
```

## 🎨 Core Features Deep Dive

### Lesson 4: Component Composition Patterns

```rust
// Building complex UIs from simple components
#[azumi::component]
fn Card(title: &str, content: &str) -> impl azumi::Component {
    html! {
        <style>
            .card { border: "1px solid #eee"; padding: "1rem"; margin: "0.5rem"; }
            .card_title { font-weight: "bold"; margin-bottom: "0.5rem"; }
        </style>
        <div class={card}>
            <h3 class={card_title}>{title}</h3>
            <p>{content}</p>
        </div>
    }
}

#[azumi::component]
fn Dashboard() -> impl azumi::Component {
    html! {
        <div>
            @Card(title="Welcome", content="Welcome to Azumi")
            @Card(title="Features", content="Type-safe components")
            @Card(title="Performance", content="Compile-time optimized")
        </div>
    }
}
```

### Lesson 5: Children Pattern

```rust
// Components with children parameter
#[azumi::component]
fn Container(children: impl azumi::Component) -> impl azumi::Component {
    html! {
        <style>
            .container { padding: "2rem"; border: "1px solid #ddd"; border-radius: "8px"; }
        </style>
        <div class={container}>
            {children}
        </div>
    }
}

#[azumi::component]
fn LayoutExample() -> impl azumi::Component {
    html! {
        @Container {
            <h1>"Container with Children"</h1>
            <p>"This content is passed as children"</p>
        }
    }
}
```

### Lesson 6: Control Flow Patterns

```rust
// @if, @else, @for, @match patterns
#[azumi::component]
fn ControlFlowExample(show_details: bool, items: Vec<&str>, status: &str) -> impl azumi::Component {
    html! {
        <style>
            .content { padding: "1rem"; }
            .item { margin: "0.5rem 0"; padding: "0.5rem"; background: "#f5f5f5"; }
            .status_active { color: "green"; }
            .status_inactive { color: "red"; }
        </style>
        <div class={content}>
            @if show_details {
                <h2>"Detailed View"</h2>
                @for item in &items {
                    <div class={item}>{item}</div>
                }
            }
            @if !show_details {
                <h2>"Summary View"</h2>
                <p>"Total items: " {items.len()}</p>
            }

            @match status {
                "active" => {
                    <p class={status_active}>"Status: Active"</p>
                }
                "inactive" => {
                    <p class={status_inactive}>"Status: Inactive"</p>
                }
                _ => {
                    <p>"Status: Unknown"</p>
                }
            }
        </div>
    }
}
```

## 🔧 Advanced Patterns

### Lesson 7: Form Handling with Validation

```rust
// Form validation with compile-time checks
#[derive(Debug)]
struct UserForm {
    name: String,
    email: String,
    age: i32,
}

#[azumi::component]
fn UserFormComponent() -> impl azumi::Component {
    html! {
        <style>
            .form { display: "grid"; gap: "1rem"; max-width: "400px"; }
            .form_field { display: "grid"; gap: "0.5rem"; }
            .form_label { font-weight: "bold"; }
            .form_input { padding: "0.5rem"; border: "1px solid #ddd"; }
            .form_button { padding: "0.75rem"; background: "#2196f3"; color: "white"; border: "none"; cursor: "pointer"; }
        </style>
        <form class={form} bind={UserForm}>
            <div class={form_field}>
                <label class={form_label} for="name">"Name"</label>
                <input class={form_input} type="text" name="name" required />
            </div>
            <div class={form_field}>
                <label class={form_label} for="email">"Email"</label>
                <input class={form_input} type="email" name="email" required />
            </div>
            <div class={form_field}>
                <label class={form_label} for="age">"Age"</label>
                <input class={form_input} type="number" name="age" min="18" max="120" />
            </div>
            <button class={form_button} type="submit">"Submit"</button>
        </form>
    }
}
```

### Lesson 8: Action System Deep Dive

```rust
// Server-side interactivity patterns
#[derive(Serialize, Deserialize)]
struct CounterState {
    count: i32,
    last_updated: String,
}

#[azumi::component]
fn CounterDisplay(state: CounterState) -> impl azumi::Component {
    html! {
        <style>
            .counter { padding: "2rem"; text-align: "center"; border: "1px solid #eee"; }
            .count_display { font-size: "3rem"; margin: "1rem 0"; }
            .counter_button { padding: "1rem 2rem"; background: "#4caf50"; color: "white"; border: "none"; cursor: "pointer"; }
            .timestamp { font-size: "0.8rem"; color: "#666"; }
        </style>
        <div class={counter} az-scope={serde_json::to_string(&state).unwrap()}>
            <div class={count_display} az-bind:text="count">{state.count}</div>
            <button
                class={counter_button}
                az-on={click call increment_counter -> #counter}
            >
                "Increment"
            </button>
            <div class={timestamp}>"Last updated: " {state.last_updated}</div>
        </div>
    }
}

#[azumi::action]
async fn increment_counter(state: CounterState) -> impl azumi::Component {
    let new_state = CounterState {
        count: state.count + 1,
        last_updated: chrono::Local::now().format("%H:%M:%S").to_string(),
    };
    CounterDisplay(new_state)
}
```

### Lesson 9: Feature Composition

```rust
// Combining multiple Azumi features
#[azumi::component]
fn FeatureShowcase() -> impl azumi::Component {
    let items = vec!["Feature 1", "Feature 2", "Feature 3"];
    let active = true;

    html! {
        <style>
            .showcase { display: "grid"; gap: "2rem"; }
            .section { padding: "1rem"; border: "1px solid #eee"; }
            .feature_list { display: "grid"; gap: "0.5rem"; }
            .feature_item { padding: "0.5rem"; background: "#f0f0f0"; }
            .active_badge { background: "#4caf50"; color: "white"; padding: "0.25rem 0.5rem"; border-radius: "4px"; }
        </style>
        <div class={showcase}>
            <div class={section}>
                <h2>"Feature Composition"</h2>
                @if active {
                    <span class={active_badge}>"ACTIVE"</span>
                }
                <div class={feature_list}>
                    @for item in &items {
                        <div class={feature_item}>{item}</div>
                    }
                </div>
            </div>
        </div>
    }
}
```

## 🎯 Specialized Features

### Lesson 10: Accessibility Patterns

```rust
// Accessibility-validated components
#[azumi::component]
fn AccessibleCard(title: &str, description: &str, image_url: &str, alt_text: &str) -> impl azumi::Component {
    html! {
        <style>
            .card { max-width: "300px"; border: "1px solid #ddd"; border-radius: "8px"; overflow: "hidden"; }
            .card_image { width: "100%"; height: "200px"; object-fit: "cover"; }
            .card_content { padding: "1rem"; }
            .card_title { font-size: "1.2rem"; margin-bottom: "0.5rem"; }
            .card_description { color: "#666"; }
        </style>
        <article class={card} aria-labelledby="card-title" aria-describedby="card-desc">
            <img class={card_image} src={image_url} alt={alt_text} />
            <div class={card_content}>
                <h3 id="card-title" class={card_title}>{title}</h3>
                <p id="card-desc" class={card_description}>{description}</p>
            </div>
        </article>
    }
}
```

### Lesson 11: Advanced CSS Features

```rust
// CSS variables, pseudo-classes, and media queries
#[azumi::component]
fn AdvancedCSSExample() -> impl azumi::Component {
    html! {
        <style>
            :root {
                --primary-color: #2196f3;
                --secondary-color: #ff4081;
            }
            .advanced { padding: "2rem"; }
            .button {
                padding: "0.75rem 1.5rem";
                background: var(--primary-color);
                color: white;
                border: none;
                cursor: pointer;
                transition: background 0.3s;
            }
            .button:hover { background: var(--secondary-color); }
            .responsive { color: "blue"; }
            @media (max-width: 600px) {
                .responsive { color: "red"; }
            }
        </style>
        <div class={advanced}>
            <button class={button}>"Hover Me"</button>
            <p class={responsive}>"Responsive Text"</p>
        </div>
    }
}
```

## 🚀 Advanced Integration

### Lesson 12: Database Integration Patterns

```rust
// Database-connected components
#[azumi::component]
fn UserList(db: rusqlite::Connection) -> impl azumi::Component {
    // Fetch users from database
    let mut stmt = db.prepare("SELECT id, name, email FROM users LIMIT 10").unwrap();
    let users = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    }).unwrap().collect::<Result<Vec<User>, _>>().unwrap();

    html! {
        <style>
            .user_list { display: "grid"; gap: "1rem"; }
            .user_card { padding: "1rem"; border: "1px solid #eee"; border-radius: "4px"; }
            .user_name { font-weight: "bold"; }
            .user_email { color: "#666"; }
        </style>
        <div class={user_list}>
            @for user in &users {
                <div class={user_card}>
                    <div class={user_name}>{user.name}</div>
                    <div class={user_email}>{user.email}</div>
                </div>
            }
        </div>
    }
}
```

### Lesson 13: Real-time Features with External Scripts

```rust
// Real-time features using external scripts (no inline scripts)
#[azumi::component]
fn ChatInterface() -> impl azumi::Component {
    html! {
        <style>
            .chat_container { display: "grid"; gap: "1rem"; max-width: "600px"; }
            .messages { height: "400px"; overflow-y: "auto"; border: "1px solid #ddd"; padding: "1rem"; }
            .message { margin-bottom: "0.5rem"; padding: "0.5rem"; background: "#f0f0f0"; border-radius: "4px"; }
            .input_area { display: "grid"; gap: "0.5rem"; }
            .chat_input { padding: "0.5rem"; }
            .send_button { padding: "0.5rem"; background: "#2196f3"; color: "white"; border: "none"; }
        </style>
        <div class={chat_container}>
            <div class={messages} id="messages">
                <div class={message}>"Welcome to Azumi Chat!"</div>
            </div>
            <div class={input_area}>
                <input class={chat_input} type="text" id="chat-input" placeholder="Type your message..." />
                <button class={send_button} onclick="window.sendMessage()">"Send"</button>
            </div>
        </div>
        <!-- External script import (required by Azumi) -->
        <script src="/static/chat.js"></script>
    }
}
```

## 🎓 Mastery Projects

### Lesson 14: Complete Blog Application

```rust
// Full-stack blog with all Azumi features
mod blog {
    use super::*;

    #[derive(Debug)]
    struct Post {
        id: i32,
        title: String,
        content: String,
        author: String,
        created_at: String,
    }

    #[azumi::component]
    fn PostCard(post: Post) -> impl azumi::Component {
        html! {
            <style>
                .post_card { border: "1px solid #eee"; padding: "1.5rem"; margin-bottom: "1rem"; border-radius: "8px"; }
                .post_title { font-size: "1.5rem"; margin-bottom: "0.5rem"; color: "#1976d2"; }
                .post_meta { display: "flex"; gap: "1rem"; color: "#666"; margin-bottom: "1rem"; font-size: "0.9rem"; }
                .post_content { line-height: "1.6"; }
                .post_author { font-weight: "bold"; }
            </style>
            <article class={post_card}>
                <h2 class={post_title}>{post.title}</h2>
                <div class={post_meta}>
                    <span class={post_author}>"By " {post.author}</span>
                    <span>"Published on " {post.created_at}</span>
                </div>
                <div class={post_content}>{post.content}</div>
            </article>
        }
    }

    #[azumi::component]
    fn BlogHomepage(posts: Vec<Post>) -> impl azumi::Component {
        html! {
            <style>
                .blog_container { max-width: "800px"; margin: "0 auto"; padding: "2rem"; }
                .blog_header { text-align: "center"; margin-bottom: "2rem"; }
                .blog_title { font-size: "2.5rem"; color: "#2196f3"; }
                .posts_list { display: "grid"; gap: "1.5rem"; }
            </style>
            <div class={blog_container}>
                <header class={blog_header}>
                    <h1 class={blog_title}>"Azumi Blog"</h1>
                    <p>"A showcase of Azumi's capabilities"</p>
                </header>
                <main class={posts_list}>
                    @for post in &posts {
                        @PostCard(post=post.clone())
                    }
                </main>
            </div>
        }
    }

    // Action for creating new posts
    #[azumi::action]
    async fn create_post(post: Post) -> impl azumi::Component {
        // Save to database
        // Return updated blog view
        BlogHomepage(vec![post]) // Simplified
    }
}
```

### Lesson 15: E-commerce Product Catalog

```rust
// Advanced e-commerce patterns
mod ecommerce {
    #[derive(Debug)]
    struct Product {
        id: i32,
        name: String,
        price: f64,
        description: String,
        in_stock: bool,
    }

    #[azumi::component]
    fn ProductCard(product: Product) -> impl azumi::Component {
        html! {
            <style>
                .product_card { border: "1px solid #eee"; padding: "1rem"; display: "grid"; gap: "0.5rem"; }
                .product_name { font-weight: "bold"; }
                .product_price { color: "#2196f3"; font-weight: "bold"; }
                .product_description { color: "#666"; font-size: "0.9rem"; }
                .add_button { padding: "0.5rem"; background: "#4caf50"; color: "white"; border: "none"; cursor: "pointer"; }
                .out_of_stock { opacity: "0.5"; }
            </style>
            <div class={if product.in_stock { "product_card" } else { "product_card out_of_stock" }}>
                <h3 class={product_name}>{product.name}</h3>
                <div class={product_price}>${product.price}</div>
                <p class={product_description}>{product.description}</p>
                @if product.in_stock {
                    <button class={add_button}>"Add to Cart"</button>
                }
                @if !product.in_stock {
                    <p>"Out of Stock"</p>
                }
            </div>
        }
    }

    #[azumi::component]
    fn ProductGrid(products: Vec<Product>) -> impl azumi::Component {
        html! {
            <style>
                .grid { display: "grid"; grid-template-columns: "repeat(auto-fill, minmax(250px, 1fr))"; gap: "1rem"; }
            </style>
            <div class={grid}>
                @for product in &products {
                    @ProductCard(product=product.clone())
                }
            </div>
        }
    }
}
```

## 🎯 Learning Path Summary

### Beginner Track (Lessons 1-6)

- Basic component syntax and CSS scoping
- Component composition and children patterns
- Control flow with @if, @for, @match
- Global vs component CSS management

### Intermediate Track (Lessons 7-11)

- Form handling and validation
- Action system for server-side interactivity
- Feature composition patterns
- Accessibility patterns and validation
- Advanced CSS features

### Advanced Track (Lessons 12-15)

- Database integration patterns
- Real-time features with external scripts
- Complete blog application
- E-commerce product catalog

## 🚀 Key Takeaways

1. **Compile-Time Safety**: Azumi catches errors before runtime
2. **Automatic Scoping**: No manual CSS management needed
3. **Type Safety**: Full Rust type system integration
4. **Progressive Enhancement**: Server-side rendering with optional interactivity
5. **Developer Experience**: Comprehensive validation and error messages
6. **Strict Validation**: No inline scripts, proper external imports

This expanded lesson plan provides a complete roadmap from basic concepts to advanced integration patterns, showcasing Azumi's full potential while adhering to all framework constraints and best practices.
