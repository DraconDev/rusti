use azumi::html;

/// Reusable button component with consistent styling
pub struct Button {
    pub text: String,
    pub variant: ButtonVariant,
    pub href: Option<String>,
    pub onclick: Option<String>,
}

#[derive(Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            variant: ButtonVariant::Primary,
            href: None,
            onclick: None,
        }
    }

    pub fn secondary(mut self) -> Self {
        self.variant = ButtonVariant::Secondary;
        self
    }

    pub fn danger(mut self) -> Self {
        self.variant = ButtonVariant::Danger;
        self
    }

    pub fn with_href(mut self, href: &str) -> Self {
        self.href = Some(href.to_string());
        self
    }

    pub fn with_onclick(mut self, onclick: &str) -> Self {
        self.onclick = Some(onclick.to_string());
        self
    }
}

impl azumi::Component for Button {
    fn render(&self) -> impl azumi::Component {
        let variant_class = match self.variant {
            ButtonVariant::Primary => "btn",
            ButtonVariant::Secondary => "btn btn-secondary",
            ButtonVariant::Danger => "btn btn-danger",
        };

        let content = html! {
            {&self.text}
        };

        if let Some(href) = &self.href {
            html! {
                <a href={href} class={variant_class}>
                    {content}
                </a>
            }
        } else if let Some(onclick) = &self.onclick {
            html! {
                <button onclick={onclick} class={variant_class}>
                    {content}
                </button>
            }
        } else {
            html! {
                <button class={variant_class}>
                    {content}
                </button>
            }
        }
    }
}

/// Lesson card component for homepage
pub struct LessonCard {
    pub href: String,
    pub phase: String,
    pub title: String,
    pub description: String,
    pub lesson_type: LessonType,
}

#[derive(Clone)]
pub enum LessonType {
    Foundation,
    Core,
    Advanced,
    Mastery,
}

impl LessonType {
    fn css_class(&self) -> &'static str {
        match self {
            LessonType::Foundation => "foundation",
            LessonType::Core => "core", 
            LessonType::Advanced => "advanced",
            LessonType::Mastery => "mastery",
        }
    }
}

impl LessonCard {
    pub fn new(href: &str, phase: &str, title: &str, description: &str, lesson_type: LessonType) -> Self {
        Self {
            href: href.to_string(),
            phase: phase.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            lesson_type,
        }
    }
}

impl azumi::Component for LessonCard {
    fn render(&self) -> impl azumi::Component {
        let type_class = self.lesson_type.css_class();
        
        html! {
            <a href={&self.href} class={"lesson-card"}>
                <div class="lesson-card-number">{&self.phase}</div>
                <h3 class="lesson-card-title">{&self.title}</h3>
                <p class="lesson-card-desc">{&self.description}</p>
            </a>
        }
    }
}

/// Code block component with syntax highlighting
pub struct CodeBlock {
    pub content: String,
    pub language: Option<String>,
}

impl CodeBlock {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            language: None,
        }
    }

    pub fn with_language(mut self, language: &str) -> Self {
        self.language = Some(language.to_string());
        self
    }
}

impl azumi::Component for CodeBlock {
    fn render(&self) -> impl azumi::Component {
        html! {
            <pre class="code-block">
                <code class={format!("language-{}", self.language.as_ref().unwrap_or(&"text".to_string()))}>
                    {&self.content}
                </code>
            </pre>
        }
    }
}

/// Feature box component for showcasing capabilities
pub struct FeatureBox {
    pub icon: String,
    pub title: String,
    pub description: String,
}

impl FeatureBox {
    pub fn new(icon: &str, title: &str, description: &str) -> Self {
        Self {
            icon: icon.to_string(),
            title: title.to_string(),
            description: description.to_string(),
        }
    }
}

impl azumi::Component for FeatureBox {
    fn render(&self) -> impl azumi::Component {
        html! {
            <div class="feature-box">
                <div class="feature-icon">{&self.icon}</div>
                <h3 class="feature-title">{&self.title}</h3>
                <p class="feature-desc">{&self.description}</p>
            </div>
        }
    }
}

/// Navigation buttons component
pub struct NavButtons {
    pub previous: Option<NavButton>,
    pub next: Option<NavButton>,
}

pub struct NavButton {
    pub text: String,
    pub href: String,
}

impl NavButtons {
    pub fn new() -> Self {
        Self {
            previous: None,
            next: None,
        }
    }

    pub fn with_previous(mut self, text: &str, href: &str) -> Self {
        self.previous = Some(NavButton {
            text: text.to_string(),
            href: href.to_string(),
        });
        self
    }

    pub fn with_next(mut self, text: &str, href: &str) -> Self {
        self.next = Some(NavButton {
            text: text.to_string(),
            href: href.to_string(),
        });
        self
    }
}

impl azumi::Component for NavButtons {
    fn render(&self) -> impl azumi::Component {
        let previous_btn = if let Some(prev) = &self.previous {
            html! {
                <a href={&prev.href} class="btn btn-secondary">{&prev.text}</a>
            }
        } else {
            html! { <div></div> }
        };

        let next_btn = if let Some(next) = &self.next {
            html! {
                <a href={&next.href} class="btn">{&next.text}</a>
            }
        } else {
            html! { <div></div> }
        };

        html! {
            <nav class="nav-buttons">
                {previous_btn}
                {next_btn}
            </nav>
        }
    }
}