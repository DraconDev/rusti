# Code-Focused 20-Lesson Azumi Curriculum

_Learning through Working Examples - Code First, Documentation Separate_

## Philosophy: Code as Teacher

Each lesson is a complete, working code example that demonstrates a specific concept. The code itself teaches - no verbose explanations needed. Separate README provides comprehensive documentation.

## Lesson Structure (Code-Focused)

### Phase 1: Foundation Building (Lessons 1-5)

#### Lesson 1: "Hello World Template"

```rust
// First working Azumi template
pub fn hello_world() -> impl azumi::Component {
    html! {
        <div>
            <h1>"Hello, World!"</h1>
        </div>
    }
}
```

#### Lesson 2: "Data Binding Basics"

```rust
// Passing data to templates
pub fn user_greeting(user: &User) -> impl azumi::Component {
    html! {
        <div>
            <h1>{"Hello " user.name "!"}</h1>
        </div>
    }
}
```

#### Lesson 3: "Conditional Rendering"

```rust
// Using @if for conditional content
pub fn user_status(user: &User) -> impl azumi::Component {
    html! {
        <div>
            @if user.is_logged_in {
                <p>"Welcome back!"</p>
            } else {
                <p>"Please log in"</p>
            }
        </div>
    }
}
```

#### Lesson 4: "Loops and Iteration"

```rust
// Using @for to render lists
pub fn todo_list(todos: &[Todo]) -> impl azumi::Component {
    html! {
        <ul>
            @for todo in todos {
                <li>{&todo.text}</li>
            }
        </ul>
    }
}
```

#### Lesson 5: "CSS Integration"

```rust
// Adding styles to templates
pub fn styled_button() -> impl azumi::Component {
    html! {
        <style src="/static/button.css" />
        <button class="btn-primary">"Click Me"</button>
    }
}
```

### Phase 2: Control Flow Mastery (Lessons 6-10)

#### Lesson 6: "Pattern Matching"

```rust
// Using @match for complex conditions
pub fn user_role_display(user: &User) -> impl azumi::Component {
    html! {
        <div>
            @match user.role {
                Role::Admin => <span class="admin-badge">"Admin"</span>,
                Role::User => <span class="user-badge">"User"</span>,
                Role::Guest => <span class="guest-badge">"Guest"</span>,
            }
        </div>
    }
}
```

#### Lesson 7: "Local Variables"

```rust
// Using @let for computed values
pub fn formatted_price(product: &Product) -> impl azumi::Component {
    html! {
        <div>
            @let final_price = product.price * (1.0 - product.discount);
            <p>{"Price: $" final_price.to_string()}</p>
        </div>
    }
}
```

#### Lesson 8: "Nested Control Flow"

```rust
// Combining @if, @for, @match
pub fn dashboard(user: &User) -> impl azumi::Component {
    html! {
        <div>
            @if user.is_admin {
                <h2>"Admin Dashboard"</h2>
                @for widget in user.admin_widgets {
                    @match widget {
                        Widget::Chart(data) => render_chart(data),
                        Widget::Table(data) => render_table(data),
                    }
                }
            } else {
                <h2>"User Dashboard"</h2>
            }
        </div>
    }
}
```

#### Lesson 9: "Advanced List Processing"

```rust
// Complex data transformations
pub fn filtered_search(items: &[Item], query: &str) -> impl azumi::Component {
    html! {
        <div>
            @let filtered = items.iter()
                .filter(|item| item.name.contains(query))
                .collect::<Vec<_>>();

            @if filtered.is_empty() {
                <p>"No results found"</p>
            } else {
                <ul>
                    @for item in filtered {
                        <li>{&item.name}</li>
                    }
                </ul>
            }
        </div>
    }
}
```

#### Lesson 10: "Error States and Loading"

```rust
// Handling different data states
pub fn data_view(data: &Result<Vec<Item>, String>) -> impl azumi::Component {
    html! {
        <div>
            @match data {
                Ok(items) => {
                    <ul>
                        @for item in items {
                            <li>{&item.name}</li>
                        }
                    </ul>
                },
                Err(error) => <p class="error">{"Error: " error}</p>,
            }
        </div>
    }
}
```

### Phase 3: Component Architecture (Lessons 11-15)

#### Lesson 11: "Simple Component"

```rust
// Creating reusable components
#[derive(Clone)]
pub struct ButtonProps {
    pub text: String,
    pub variant: ButtonVariant,
}

pub fn button(props: ButtonProps) -> impl azumi::Component {
    html! {
        <button class={format!("btn btn-{}", match props.variant {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Secondary => "secondary",
        })}>
            {&props.text}
        </button>
    }
}
```

#### Lesson 12: "Component with Children"

```rust
// Passing content to components
#[derive(Clone)]
pub struct CardProps {
    pub title: String,
    pub children: impl azumi::Component,
}

pub fn card(props: CardProps) -> impl azumi::Component {
    html! {
        <div class="card">
            <h3>{&props.title}</h3>
            {props.children}
        </div>
    }
}
```

#### Lesson 13: "Component Composition"

```rust
// Building complex UIs from simple components
pub fn user_profile(user: &User) -> impl azumi::Component {
    html! {
        <div>
            {card(CardProps {
                title: "Profile".to_string(),
                children: html! {
                    <div>
                        <p>{"Name: " user.name}</p>
                        <p>{"Email: " user.email}</p>
                    </div>
                }
            })}

            {card(CardProps {
                title: "Recent Activity".to_string(),
                children: html! {
                    <ul>
                        @for activity in &user.activities {
                            <li>{&activity.description}</li>
                        }
                    </ul>
                }
            })}
        </div>
    }
}
```

#### Lesson 14: "Component with State"

```rust
// Managing component state
pub fn counter(initial: u32) -> impl azumi::Component {
    let mut count = initial;

    html! {
        <div>
            <p>{"Count: " count.to_string()}</p>
            <button onclick={move || count += 1}>"Increment"</button>
        </div>
    }
}
```

#### Lesson 15: "Reusable Form Component"

```rust
// Building flexible form components
#[derive(Clone)]
pub struct FormFieldProps {
    pub label: String,
    pub value: String,
    pub on_change: Box<dyn Fn(String)>,
}

pub fn form_field(props: FormFieldProps) -> impl azumi::Component {
    html! {
        <div class="form-field">
            <label>{&props.label}</label>
            <input
                type="text"
                value={&props.value}
                oninput={move |e| props.on_change(e.value())}
            />
        </div>
    }
}
```

### Phase 4: JavaScript & Interactivity (Lessons 16-18)

#### Lesson 16: "JavaScript Integration"

```rust
// Loading and using external JavaScript
pub fn interactive_chart() -> impl azumi::Component {
    html! {
        <html>
            <head>
                <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
            </head>
            <body>
                <canvas id="myChart" width="400" height="200"></canvas>
                <script>
                    "const ctx = document.getElementById('myChart').getContext('2d');
                    new Chart(ctx, {
                        type: 'bar',
                        data: {
                            labels: ['Red', 'Blue', 'Yellow', 'Green', 'Purple', 'Orange'],
                            datasets: [{
                                label: 'My First Dataset',
                                data: [12, 19, 3, 5, 2, 3],
                                backgroundColor: [
                                    'rgba(255, 99, 132, 0.2)',
                                    'rgba(54, 162, 235, 0.2)',
                                    'rgba(255, 205, 86, 0.2)',
                                    'rgba(75, 192, 192, 0.2)',
                                    'rgba(153, 102, 255, 0.2)',
                                    'rgba(255, 159, 64, 0.2)'
                                ]
                            }]
                        }
                    });"
                </script>
            </body>
        </html>
    }
}
```

#### Lesson 17: "HTMX Integration"

```rust
// Building interactive UIs without JavaScript frameworks
pub fn todo_with_htmx() -> impl azumi::Component {
    html! {
        <div>
            <h1>"Todos"</h1>

            <form hx-post="/todos" hx-swap="beforeend">
                <input name="text" type="text" placeholder="Add a todo..." />
                <button type="submit">"Add"</button>
            </form>

            <div id="todo-list">
                <div class="todo">
                    <span>"Learn Azumi"</span>
                    <button hx-delete="/todos/1" hx-swap="remove">"×"</button>
                </div>
            </div>
        </div>
    }
}
```

#### Lesson 18: "Real-time Updates"

```rust
// Combining HTMX with server-sent events
pub fn live_chat() -> impl azumi::Component {
    html! {
        <div>
            <div id="chat-messages" hx-sse="connect:/chat/stream">
                <!-- Messages will appear here -->
            </div>

            <form hx-post="/chat/message" hx-swap="none">
                <input name="message" type="text" placeholder="Type a message..." />
                <button type="submit">"Send"</button>
            </form>
        </div>
    }
}
```

### Phase 5: Production & Advanced (Lessons 19-20)

#### Lesson 19: "Layout System"

```rust
// Building reusable layout patterns
pub fn app_layout(title: &str, content: impl azumi::Component) -> impl azumi::Component {
    html! {
        <html>
            <head>
                <title>{title}</title>
                <style src="/static/layout.css" />
            </head>
            <body>
                <header class="app-header">
                    <nav>
                        <a href="/">"Home"</a>
                        <a href="/about">"About"</a>
                    </nav>
                </header>

                <main class="app-main">
                    {content}
                </main>

                <footer class="app-footer">
                    <p>"© 2024 My App"</p>
                </footer>
            </body>
        </html>
    }
}
```

#### Lesson 20: "Full Application Example"

```rust
// Complete CRUD application
pub fn blog_post_editor(post: &Option<Post>) -> impl azumi::Component {
    html! {
        <div>
            <h1>{if post.is_some() { "Edit Post" } else { "New Post" }}</h1>

            <form hx-post="/blog/save" hx-swap="none">
                @if let Some(existing_post) = post {
                    <input type="hidden" name="id" value={existing_post.id.to_string()} />
                }

                <div>
                    <label>"Title"</label>
                    <input name="title" type="text" value={post.as_ref().map(|p| &p.title).unwrap_or("")} />
                </div>

                <div>
                    <label>"Content"</label>
                    <textarea name="content">{post.as_ref().map(|p| &p.content).unwrap_or("")}</textarea>
                </div>

                <div>
                    <button type="submit">"Save"</button>
                    <a href="/blog">"Cancel"</a>
                </div>
            </form>
        </div>
    }
}
```

## Implementation Benefits

This code-focused approach provides:

1. **Immediate Value**: Each lesson is a working example
2. **Copy-Paste Ready**: Students can use examples directly
3. **Progressive Complexity**: Each lesson builds on previous ones
4. **Real-World Patterns**: Examples mirror production use cases
5. **Minimal Cognitive Load**: No verbose explanations to parse

## Supporting Documentation

**Separate README.md** provides:

- Comprehensive feature overview
- Installation instructions
- Advanced concepts
- API reference
- Best practices guide
- Troubleshooting

## Next Steps

1. **Create 20 focused code examples** following this structure
2. **Build comprehensive README** for documentation
3. **Test all examples** work correctly
4. **Add interactive playground** for experimentation
5. **Create quick reference guide** for common patterns

This approach transforms learning from reading documentation to running and modifying real code examples.
