# 🎯 Azumi Component Composition Guide

## The Azumi Way: Explicit Over Implicit

Instead of React's implicit `{children}`, Azumi promotes **explicit component composition** for better type safety, maintainability, and developer experience.

## ❌ React Pattern (Not Supported)

```rust
// This does NOT work in Azumi:
#[azumi::component]
fn Card(title: &str, children: impl Component) -> impl Component {
    html! {
        <div class="card">
            <h2>{title}</h2>
            {children}  // ❌ No implicit children prop
        </div>
    }
}

// Usage (React-style):
@Card(title="My Card") {
    <p>"Some content here"</p>
}
```

## ✅ Azumi Patterns (Recommended)

### 1. **Functional Composition** (Recommended)

```rust
#[azumi::component]
fn Card(title: &'static str, content: &'static str, footer: &'static str) -> impl Component {
    html! {
        <div class="card">
            <h2>{title}</h2>
            <p>{content}</p>
            <div class="card-footer">
                <span>{footer}</span>
            </div>
        </div>
    }
}

// Usage:
@Card(
    title="Welcome Card",
    content="This is the main content of the card.",
    footer="Card footer content"
)
```

### 2. **Component Composition**

```rust
#[azumi::component]
fn CardHeader(title: &'static str) -> impl Component {
    html! {
        <div class="card-header">
            <h2>{title}</h2>
        </div>
    }
}

#[azumi::component]
fn CardBody(content: &'static str) -> impl Component {
    html! {
        <div class="card-body">
            <p>{content}</p>
        </div>
    }
}

#[azumi::component]
fn CardFooter(footer: &'static str) -> impl Component {
    html! {
        <div class="card-footer">
            <span>{footer}</span>
        </div>
    }
}

#[azumi::component]
fn CompleteCard(title: &'static str, content: &'static str, footer: &'static str) -> impl Component {
    html! {
        <div class="card">
            @CardHeader(title)
            @CardBody(content)
            @CardFooter(footer)
        </div>
    }
}
```

### 3. **Builder Pattern**

```rust
struct CardBuilder {
    title: Option<String>,
    content: Option<String>,
    footer: Option<String>,
    variant: String,
}

impl CardBuilder {
    fn new() -> Self {
        CardBuilder {
            title: None,
            content: None,
            footer: None,
            variant: "default".to_string(),
        }
    }

    fn title(mut self, title: &'static str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    fn content(mut self, content: &'static str) -> Self {
        self.content = Some(content.to_string());
        self
    }

    fn footer(mut self, footer: &'static str) -> Self {
        self.footer = Some(footer.to_string());
        self
    }

    fn variant(mut self, variant: &'static str) -> Self {
        self.variant = variant.to_string();
        self
    }

    fn build(self) -> impl azumi::Component {
        html! {
            <div class={format!("card {}", self.variant)}>
                @if let Some(title) = self.title {
                    <div class="card-header">
                        <h2>{title}</h2>
                    </div>
                }
                @if let Some(content) = self.content {
                    <div class="card-body">
                        <p>{content}</p>
                    </div>
                }
                @if let Some(footer) = self.footer {
                    <div class="card-footer">
                        <span>{footer}</span>
                    </div>
                }
            </div>
        }
    }
}

// Usage:
@CardBuilder::new()
    .title("Dynamic Card")
    .content("Flexible content based on props")
    .footer("Footer text")
    .variant("featured")
    .build()
```

### 4. **Configuration-Driven Components**

```rust
#[derive(Clone)]
struct CardConfig {
    title: String,
    content: String,
    footer: Option<String>,
    show_header: bool,
    show_footer: bool,
    variant: CardVariant,
}

#[derive(Clone)]
enum CardVariant {
    Default,
    Featured,
    Compact,
}

#[azumi::component]
fn ConfigurableCard(config: CardConfig) -> impl Component {
    html! {
        <div class={format!("card {}", match config.variant {
            CardVariant::Default => "default",
            CardVariant::Featured => "featured",
            CardVariant::Compact => "compact",
        })}>
            @if config.show_header {
                <div class="card-header">
                    <h2>{config.title}</h2>
                </div>
            }
            <div class="card-body">
                <p>{config.content}</p>
            </div>
            @if config.show_footer {
                <div class="card-footer">
                    <span>{config.footer}</span>
                </div>
            }
        </div>
    }
}

// Usage:
@ConfigurableCard(CardConfig {
    title: "User Profile".to_string(),
    content: "Profile information and stats".to_string(),
    footer: Some("View Details".to_string()),
    show_header: true,
    show_footer: true,
    variant: CardVariant::Featured,
})
```

### 5. **Slot Pattern with @let**

```rust
#[azumi::component]
fn FlexibleCard(
    title: &'static str,
    header_content: Option<&'static str>,
    body_content: &'static str,
    footer_content: Option<&'static str>
) -> impl Component {
    html! {
        <div class="card">
            <div class="card-header">
                <h2>{title}</h2>
                @if let Some(header) = header_content {
                    <div class="card-header-extra">{header}</div>
                }
            </div>
            <div class="card-body">
                <p>{body_content}</p>
            </div>
            @if let Some(footer) = footer_content {
                <div class="card-footer">
                    <span>{footer}</span>
                </div>
            }
        </div>
    }
}

// Usage with conditional content:
@let show_special_badge = true;
@let badge_text = if show_special_badge { Some("NEW") } else { None };

@FlexibleCard(
    title="Product Card",
    header_content=badge_text,
    body_content="Product description and features",
    footer_content=Some("Add to Cart")
)
```

## 🎯 Why Azumi's Approach is Better

### **1. Type Safety**

-   All props are explicitly typed and validated at compile time
-   No runtime surprises with missing or incorrect props
-   IDE autocomplete and refactoring support

### **2. Maintainability**

-   Component interface is clear and self-documenting
-   No implicit dependencies or hidden prop flows
-   Easy to test and debug

### **3. Performance**

-   No runtime prop drilling or reconciliation
-   Compile-time optimization opportunities
-   Zero-cost abstractions

### **4. Developer Experience**

-   Clear API boundaries
-   Excellent error messages
-   Predictable behavior

## 🔧 Advanced Patterns

### **Conditional Slot Rendering**

```rust
#[azumi::component]
fn SmartCard(
    title: &'static str,
    show_status: bool,
    status_text: Option<&'static str>,
    content: &'static str,
    actions: Vec<Action>
) -> impl Component {
    html! {
        <div class="card">
            <div class="card-header">
                <h2>{title}</h2>
                @if show_status {
                    <span class="status-badge">{status_text}</span>
                }
            </div>
            <div class="card-body">
                <p>{content}</p>
            </div>
            @if !actions.is_empty() {
                <div class="card-actions">
                    @for action in actions {
                        <button onclick={action.handler}>{action.text}</button>
                    }
                </div>
            }
        </div>
    }
}
```

### **Dynamic Composition**

```rust
#[azumi::component]
fn ComposeCard(
    components: Vec<impl azumi::Component>
) -> impl Component {
    html! {
        <div class="composed-card">
            @for component in components {
                {component}
            }
        </div>
    }
}
```

## 🎉 Summary

**Azumi's explicit composition patterns provide:**

-   ✅ **Better Type Safety** - No runtime prop errors
-   ✅ **Cleaner Code** - Explicit interfaces
-   ✅ **Better Tooling** - Full IDE support
-   ✅ **Performance** - Compile-time optimization
-   ✅ **Maintainability** - Easy to understand and modify

**Stop thinking React, start thinking Functional Rust!**
