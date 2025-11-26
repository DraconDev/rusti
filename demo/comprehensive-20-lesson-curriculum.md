# Complete 20-Lesson Azumi Curriculum
*Featuring Control Flow, Components, JavaScript Integration & Advanced Patterns*

## Phase 1: Foundation Building (Lessons 1-5)

### Lesson 1: "Getting Started with Azumi"
- What is type-safe HTML
- Why strict typing matters
- Your first template
- Visual comparisons with traditional templates

### Lesson 2: "The Quoting Fundamentals"
- Rule 1: Quote all text content
- Rule 2: Quote all attributes
- Nested quotes and escaping
- Common mistakes and solutions

### Lesson 3: "CSS Integration Basics"
- Adding styles to templates
- CSS validation concepts
- First styling exercise
- Understanding CSS vs Azumi CSS

### Lesson 4: "Understanding Scoping"
- What is CSS scoping
- Local vs global styles
- Preventing style conflicts
- Simple scoping exercise

### Lesson 5: "Variables & Data Binding"
- Passing data to templates
- Type safety in practice
- Simple variable usage
- Data flow concepts

## Phase 2: Control Flow Mastery (Lessons 6-10)

### Lesson 6: "Conditional Content with @if"
- Basic @if syntax and usage
- Boolean expressions in templates
- @else and @else if chains
- Practical conditional rendering examples

### Lesson 7: "Loops & Iteration with @for"
- @for loops and syntax
- Iterating over arrays and ranges
- Index tracking in loops
- Building dynamic lists and tables

### Lesson 8: "Pattern Matching with @match"
- @match syntax and power
- Enum pattern matching
- Complex conditional logic
- Practical pattern matching examples

### Lesson 9: "Local Variables with @let"
- @let syntax and scoping
- Computed values in templates
- Temporary variable usage
- Complex data transformations

### Lesson 10: "Advanced Control Flow Patterns"
- Combining @if, @for, @match, @let
- Nested control structures
- Performance considerations
- Real-world control flow patterns

## Phase 3: Component Architecture (Lessons 11-15)

### Lesson 11: "Introduction to Components"
- Component concept and benefits
- When to create components
- Basic component structure
- Simple component examples

### Lesson 12: "Component Props & Data Flow"
- Defining component interfaces
- Type-safe prop passing
- Default values and validation
- Complex prop patterns

### Lesson 13: "Component Composition"
- Parent-child relationships
- Component nesting
- Shared state management
- Composition patterns

### Lesson 14: "Component State Management"
- Internal component state
- State updates and re-rendering
- Controlled vs uncontrolled components
- State lifecycle management

### Lesson 15: "Advanced Component Patterns"
- Higher-order components
- Render props pattern
- Component composition techniques
- Performance optimization

## Phase 4: JavaScript & Interactivity (Lessons 16-18)

### Lesson 16: "JavaScript Integration"
- Loading external JavaScript
- Script tag usage and safety
- Interacting with DOM elements
- Event handling integration

### Lesson 17: "Interactive Components"
- Building interactive UIs
- State synchronization with JS
- Form handling and validation
- Real-time updates

### Lesson 18: "HTMX & Server Integration"
- HTMX integration with Azumi
- Progressive enhancement
- Dynamic content loading
- Form submissions and responses

## Phase 5: Production & Advanced (Lessons 19-20)

### Lesson 19: "Layout Systems & Architecture"
- Layout component patterns
- Multi-page application structure
- Navigation systems
- Application architecture

### Lesson 20: "Production Patterns & Deployment"
- Error handling strategies
- Performance optimization
- Testing components
- Deployment best practices

---

## Detailed Lesson Outlines with Code Examples

### Lesson 6: "Conditional Content with @if" - Detailed

**Learning Objectives:**
- Master @if syntax and basic usage
- Understand boolean expressions in templates
- Learn @else and @else if chains
- Build practical conditional rendering

**Key Code Examples:**
```rust
// Basic @if usage
html! {
    <div>
        @if user.is_logged_in {
            <p>"Welcome back!"</p>
        }
    </div>
}

// @else and @else if
html! {
    <div>
        @if user.role == "admin" {
            <p>"Admin Dashboard"</p>
        } else if user.role == "user" {
            <p>"User Portal"</p>
        } else {
            <p>"Guest Access"</p>
        }
    </div>
}

// Complex boolean expressions
html! {
    <div>
        @if user.is_premium && user.has_notifications {
            <div class="premium-notifications">
                <p>"You have new premium features!"</p>
            </div>
        }
    </div>
}
```

### Lesson 8: "Pattern Matching with @match" - Detailed

**Learning Objectives:**
- Master @match syntax and power
- Understand enum pattern matching
- Learn complex conditional logic
- Build practical pattern matching examples

**Key Code Examples:**
```rust
// Basic @match usage
#[derive(Debug)]
enum UserStatus {
    Active,
    Inactive,
    Banned,
    Pending,
}

html! {
    <div class="user-status">
        @match user.status {
            UserStatus::Active => {
                <span class="status-active">"Active"</span>
            }
            UserStatus::Inactive => {
                <span class="status-inactive">"Inactive"</span>
            }
            UserStatus::Banned => {
                <span class="status-banned">"Banned"</span>
            }
            UserStatus::Pending => {
                <span class="status-pending">"Pending Approval"</span>
            }
        }
    </div>
}

// Complex pattern matching
html! {
    <div>
        @match user.profile {
            Some(profile) => {
                <div class="profile-info">
                    <h2>{&profile.name}</h2>
                    <p>{&profile.bio}</p>
                </div>
            }
            None => {
                <div class="no-profile">
                    <p>"No profile information available"</p>
                </div>
            }
        }
    </div>
}
```

### Lesson 9: "Local Variables with @let" - Detailed

**Learning Objectives:**
- Master @let syntax and scoping
- Understand computed values
- Learn temporary variable usage
- Build complex data transformations

**Key Code Examples:**
```rust
// Basic @let usage
html! {
    <div>
        @let full_name = format!("{} {}", user.first_name, user.last_name);
        <p>{"Hello " full_name "!"}</p>
    </div>
}

// Computed values
html! {
    <div class="user-card">
        @let display_name = if user.nickname.is_empty() {
            &user.full_name
        } else {
            &user.nickname
        };
        @let is_new_user = user.created_at.days_since_now() < 7;
        
        <h3>{display_name}</h3>
        @if is_new_user {
            <span class="new-user-badge">"New!"</span>
        }
    </div>
}

// Complex transformations
html! {
    <div>
        @let sorted_comments = {
            let mut comments = user.comments.clone();
            comments.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            comments
        };
        
        @for comment in sorted_comments {
            <div class="comment">
                <p>{&comment.content}</p>
                <small>{&comment.author}</small>
            </div>
        }
    </div>
}
```

### Lesson 16: "JavaScript Integration" - Detailed

**Learning Objectives:**
- Learn to load external JavaScript safely
- Understand script tag usage
- Interact with DOM elements
- Handle events integration

**Key Code Examples:**
```rust
// Loading external JavaScript
html! {
    <html>
        <head>
            <script src="https://cdn.example.com/library.js"></script>
            <script>
                "console.log('Page loaded!');"
            </script>
        </head>
        <body>
            <div id="app">
                <h1>"Interactive Azumi App"</h1>
                <button id="counter-btn" onclick="incrementCounter()">
                    "Click me: 0"
                </button>
            </div>
            
            <script>
                r#"
                let counter = 0;
                function incrementCounter() {
                    counter++;
                    document.getElementById('counter-btn').textContent = 'Click me: ' + counter;
                }
                "#
            </script>
        </body>
    </html>
}

// Dynamic JavaScript with template variables
html! {
    <body>
        <div id="user-dashboard">
            <h1>{"Welcome " user.name "!"}</h1>
            <div id="user-stats" data-user-id={user.id.to_string()}>
                <p>{"Points: " user.points}</p>
            </div>
        </div>
        
        <script>
            {format!(r#"
                const userData = {{
                    id: {},
                    name: "{}",
                    points: {}
                }};
                
                function initDashboard() {{
                    console.log('Dashboard initialized for:', userData.name);
                    updateUserInterface(userData);
                }}
                
                document.addEventListener('DOMContentLoaded', initDashboard);
            "#, user.id, user.name, user.points)}
        </script>
    </body>
}
```

### Lesson 12: "Component Props & Data Flow" - Detailed

**Learning Objectives:**
- Define component interfaces
- Type-safe prop passing
- Default values and validation
- Complex prop patterns

**Key Code Examples:**
```rust
// Basic component with props
#[derive(Clone)]
struct UserCardProps {
    name: String,
    email: String,
    avatar_url: Option<String>,
    is_online: bool,
}

fn user_card(props: UserCardProps) -> impl azumi::Component {
    html! {
        <div class="user-card">
            @if let Some(avatar) = &props.avatar_url {
                <img src={avatar} alt={format!("{}'s avatar", props.name)} class="avatar" />
            } @else {
                <div class="avatar-placeholder">
                    {&props.name.chars().next().unwrap_or('?').to_uppercase().to_string()}
                </div>
            }
            
            <div class="user-info">
                <h3>{&props.name}</h3>
                <p>{&props.email}</p>
                @if props.is_online {
                    <span class="online-indicator">"🟢 Online"</span>
                } @else {
                    <span class="offline-indicator">"🔴 Offline"</span>
                }
            </div>
        </div>
    }
}

// Using the component
html! {
    <div class="user-list">
        @for user in users {
            {user_card(UserCardProps {
                name: user.name.clone(),
                email: user.email.clone(),
                avatar_url: user.avatar_url.clone(),
                is_online: user.is_online,
            })}
        }
    </div>
}

// Component with default values
#[derive(Clone)]
struct ButtonProps {
    text: String,
    variant: ButtonVariant,
    disabled: bool,
}

#[derive(Clone)]
enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

impl Default for ButtonProps {
    fn default() -> Self {
        ButtonProps {
            text: "Click me".to_string(),
            variant: ButtonVariant::Primary,
            disabled: false,
        }
    }
}

fn button(props: ButtonProps) -> impl azumi::Component {
    let variant_class = match props.variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary", 
        ButtonVariant::Danger => "btn-danger",
    };
    
    html! {
        <button 
            class={format!("btn {}", variant_class)}
            disabled={props.disabled}
        >
            {&props.text}
        </button>
    }
}
```

## Implementation Benefits

This 20-lesson structure provides:

1. **Gradual Progression**: Each lesson builds on previous knowledge
2. **Comprehensive Coverage**: All major Azumi features covered
3. **Practical Focus**: Real-world examples and use cases
4. **Control Flow Emphasis**: @let, @if, @match prominently featured
5. **Component Architecture**: Deep dive into reusable components
6. **JavaScript Integration**: Shows how to combine with JS safely
7. **Production Readiness**: Advanced patterns and deployment

## Next Steps

1. **Implement Lessons 3-5** to complete Foundation phase
2. **Create comprehensive CSS** for all new lessons
3. **Build interactive examples** for each control flow feature
4. **Add hands-on exercises** throughout all lessons
5. **Test the complete learning path** from beginner to advanced

This structure transforms Azumi learning from a reference documentation style into a true comprehensive course that guides users through mastery of all features.