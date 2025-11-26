use azumi::html;

/// Main lesson container with fade-in animation
pub struct LessonContainer {
    pub content: String,
}

impl azumi::Component for LessonContainer {
    fn render(&self) -> impl azumi::Component {
        let style = r#"
            <style>
            .lesson-container {
                max-width: 1000px;
                margin: 0 auto;
                padding: var(--spacing-lg);
                animation: fadeInUp 0.6s ease-out;
                --azumi-primary: #4f46e5;
                --azumi-secondary: #10b981;
                --azumi-danger: #ef4444;
                --azumi-bg: #0f172a;
                --azumi-surface: #1e293b;
                --azumi-text: #f1f5f9;
                --azumi-text-dim: #94a3b8;
                --spacing-xs: 0.5rem;
                --spacing-sm: 1rem;
                --spacing-md: 1.5rem;
                --spacing-lg: 2rem;
                --spacing-xl: 3rem;
                --radius-sm: 0.375rem;
                --radius-md: 0.5rem;
            }
            
            @keyframes fadeInUp {
                from {
                    opacity: 0;
                    transform: translateY(20px);
                }
                to {
                    opacity: 1;
                    transform: translateY(0);
                }
            }
            
            @media (max-width: 768px) {
                .lesson-container {
                    padding: var(--spacing-md);
                }
            }
            </style>
        "#;
        
        html! {
            <div class="lesson-container">
                {&self.content}
            </div>
        }
    }
}

impl LessonContainer {
    pub fn new(content: &str) -> Self {
        Self { content: content.to_string() }
    }
}

/// Lesson header with border and animated gradient title
pub struct LessonHeader {
    pub back_link: String,
    pub lesson_number: String,
    pub title: String,
    pub subtitle: String,
}

impl azumi::Component for LessonHeader {
    fn render(&self) -> impl azumi::Component {
        let style = r#"
            <style>
            .lesson-header {
                margin-bottom: var(--spacing-xl);
                padding-bottom: var(--spacing-lg);
                border-bottom: 2px solid var(--azumi-surface);
                position: relative;
                --azumi-primary: #4f46e5;
                --azumi-secondary: #10b981;
                --azumi-bg: #0f172a;
                --azumi-surface: #1e293b;
                --azumi-text-dim: #94a3b8;
                --spacing-sm: 1rem;
                --spacing-md: 1.5rem;
                --spacing-lg: 2rem;
                --spacing-xl: 3rem;
            }
            
            .lesson-header::after {
                content: '';
                position: absolute;
                bottom: -2px;
                left: 0;
                width: 100px;
                height: 2px;
                background: linear-gradient(90deg, var(--azumi-primary), var(--azumi-secondary));
                border-radius: 1px;
            }
            
            .lesson-title {
                font-size: clamp(2rem, 5vw, 3rem);
                font-weight: 700;
                margin: var(--spacing-md) 0;
                background: linear-gradient(135deg, var(--azumi-primary), var(--azumi-secondary), var(--azumi-primary));
                background-size: 200% 200%;
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                background-clip: text;
                animation: gradientShift 3s ease-in-out infinite;
                line-height: 1.2;
            }
            
            @keyframes gradientShift {
                0%, 100% { background-position: 0% 50%; }
                50% { background-position: 100% 50%; }
            }
            
            .lesson-subtitle {
                color: var(--azumi-text-dim);
                font-size: 1.25rem;
                line-height: 1.6;
                max-width: 600px;
                font-weight: 400;
            }
            
            .lesson-number {
                color: var(--azumi-primary);
                font-size: 0.875rem;
                font-weight: 600;
                text-transform: uppercase;
                letter-spacing: 0.1em;
                background: rgba(79, 70, 229, 0.1);
                padding: 0.375rem 0.75rem;
                border-radius: 1rem;
                display: inline-block;
                margin-bottom: var(--spacing-sm);
                border: 1px solid rgba(79, 70, 229, 0.2);
            }
            </style>
        "#;
        
        html! {
            <header class="lesson-header">
                <a href="/" class="back-link">"← Back to Lessons"</a>
                <div class="lesson-number">{&self.lesson_number}</div>
                <h1 class="lesson-title">{&self.title}</h1>
                <p class="lesson-subtitle">{&self.subtitle}</p>
            </header>
        }
    }
}

impl LessonHeader {
    pub fn new(back_link: &str, lesson_number: &str, title: &str, subtitle: &str) -> Self {
        Self {
            back_link: back_link.to_string(),
            lesson_number: lesson_number.to_string(),
            title: title.to_string(),
            subtitle: subtitle.to_string(),
        }
    }
}

/// Section wrapper with consistent spacing
pub struct Section {
    pub title: String,
    pub content: String,
}

impl azumi::Component for Section {
    fn render(&self) -> impl azumi::Component {
        let style = r#"
            <style>
            .section {
                margin-bottom: var(--spacing-xl);
                animation: fadeInUp 0.6s ease-out 0.2s both;
                --spacing-xl: 3rem;
            }
            
            .section-title {
                font-size: 1.75rem;
                font-weight: 600;
                margin-bottom: var(--spacing-md);
                color: var(--azumi-text);
                position: relative;
                display: flex;
                align-items: center;
                gap: 0.75rem;
                --azumi-text: #f1f5f9;
                --spacing-md: 1.5rem;
            }
            
            .section-title::before {
                content: '';
                width: 4px;
                height: 1.5rem;
                background: linear-gradient(180deg, var(--azumi-primary), var(--azumi-secondary));
                border-radius: 2px;
                --azumi-primary: #4f46e5;
                --azumi-secondary: #10b981;
            }
            
            @keyframes fadeInUp {
                from {
                    opacity: 0;
                    transform: translateY(20px);
                }
                to {
                    opacity: 1;
                    transform: translateY(0);
                }
            }
            </style>
        "#;
        
        html! {
            <section class="section">
                <h2 class="section-title">{&self.title}</h2>
                <div class="section-content">
                    {&self.content}
                </div>
            </section>
        }
    }
}

impl Section {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}