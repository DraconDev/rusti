use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 11: Introduction to Components
pub fn lesson11() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 11: Introduction to Components - Azumi"</title>
                <style src="/static/pages/lessons.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-number">"Components"</div>
                        <h1 class="lesson-title">"Introduction to Components"</h1>
                        <p class="lesson-subtitle">"Learn component concepts and build your first reusable components"</p>
                    </header>

                    <section class="section">
                        <h2 class="section-title">"🎯 What You'll Learn"</h2>
                        <p>"This lesson introduces the core concept of components in Azumi:"</p>
                        <ul>
                            <li>"What components are and why they matter"</li>
                            <li>"When to create components vs. functions"</li>
                            <li>"Basic component structure and syntax"</li>
                            <li>"Simple component examples and patterns"</li>
                            <li>"Benefits of component-based architecture"</li>
                        </ul>
                        <div class="prerequisite-box">
                            <strong>"📋 Prerequisites:"</strong>
                            <p>"Complete the <a href="/lesson-10">\"Advanced Control Flow\"</a> lesson. Understanding of Rust functions and basic Azumi templates."</p>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🤔 What are Components?"</h2>
                        <p>"Components are reusable, self-contained pieces of UI that can be composed together to build complex interfaces. Think of them as \"custom HTML tags\" that you define yourself."</p>

                        <div class="concept-comparison">
                            <div class="comparison-item">
                                <h4>"🔧 Functions vs Components"</h4>
                                <div class="comparison-grid">
                                    <div class="comparison-side">
                                        <h5>"Regular Functions"</h5>
                                        <pre class="code-block">{"pub fn greeting_component("}
    {"    name: &str,"}
    {"    age: u32,"}
    {") -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div>"}
    {"            <h3>{\"Hello \" name}</h3>"}
    {"            <p>{\"Age: \" age.to_string()}</p>"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                                    </div>
                                    <div class="comparison-side">
                                        <h5>"Component Functions"</h5>
                                        <pre class="code-block">{"pub fn user_profile("}
    {"    props: UserProfileProps,"}
    {") -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div class=\"profile-card\">"}
    {"            <img src={&props.avatar_url} />"}
    {"            <h3>{&props.name}</h3>"}
    {"            <p>{&props.bio}</p>"}
    {"            <span class=\"status\">{&props.status}</span>"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🏗️ When to Create Components"</h2>
                        <p>"Use components when you need reusable UI patterns. Here are the key indicators:"</p>

                        <div class="guidelines-list">
                            <div class="guideline-item">
                                <h4>"✅ Create Components When:"</h4>
                                <ul>
                                    <li>"The same UI pattern appears in multiple places"</li>
                                    <li>"You want to encapsulate complex styling and behavior"</li>
                                    <li>"The UI piece has clear inputs (props) and outputs (HTML)"</li>
                                    <li>"You want to make testing easier by isolating functionality"</li>
                                    <li>"The component will be used by multiple developers"</li>
                                </ul>
                            </div>
                            <div class="guideline-item">
                                <h4>"❌ Don't Create Components When:"</h4>
                                <ul>
                                    <li>"It's used only once and won't be reused"</li>
                                    <li>"It's just a simple function call with minimal logic"</li>
                                    <li>"The component would be more complex than the original code"</li>
                                    <li>"You're prematurely optimizing (YAGNI principle)"</li>
                                </ul>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"📝 Basic Component Structure"</h2>
                        <p>"Components follow a consistent structure. Here's the anatomy of a component:"</p>

                        <div class="structure-breakdown">
                            <div class="structure-part">
                                <h4>"1. Props Struct"</h4>
                                <p>"Define the input data structure:"</p>
                                <pre class="code-block">{"#[derive(Clone)]"}
    {"pub struct ButtonProps {"}
    {"    pub text: String,"}
    {"    pub variant: ButtonVariant,"}
    {"    pub disabled: bool,"}
    {"}"}
    {""}
    {"#[derive(Clone)]"}
    {"pub enum ButtonVariant {"}
    {"    Primary,"}
    {"    Secondary,"}
    {"}"}</pre>
                            </div>

                            <div class="structure-part">
                                <h4>"2. Component Function"</h4>
                                <p>"Implement the rendering logic:"</p>
                                <pre class="code-block">{"pub fn button_component("}
    {"    props: ButtonProps,"}
    {") -> impl azumi::Component {"}
    {"    let variant_class = match props.variant {"}
    {"        ButtonVariant::Primary => \"btn-primary\","}
    {"        ButtonVariant::Secondary => \"btn-secondary\","}
    {"    };"}
    {""}
    {"    html! {"}
    {"        <button"}
    {"            class={format!(\"btn {}\", variant_class)}"}
    {"            disabled={props.disabled}"}
    {"        >"}
    {"            {&props.text}"}
    {"        </button>"}
    {"    }"}
    {"}"}</pre>
                            </div>

                            <div class="structure-part">
                                <h4>"3. Props Builder (Optional)"</h4>
                                <p>"Add convenient constructor methods:"</p>
                                <pre class="code-block">{"impl ButtonProps {"}
    {"    pub fn new(text: &str) -> Self {"}
    {"        Self {"}
    {"            text: text.to_string(),"}
    {"            variant: ButtonVariant::Primary,"}
    {"            disabled: false,"}
    {"        }"}
    {"    }"}
    {""}
    {"    pub fn secondary(mut self) -> Self {"}
    {"        self.variant = ButtonVariant::Secondary;"}
    {"        self"}
    {"    }"}
    {"}"}</pre>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎨 Simple Component Examples"</h2>
                        <p>"Let's build some practical components step by step:"</p>

                        <div class="example-walkthrough">
                            <div class="example-step">
                                <h4>"Example 1: Alert Component"</h4>
                                <p>"A component for displaying different types of alerts:"</p>
                                
                                <pre class="code-block">{"#[derive(Clone)]"}
    {"pub struct AlertProps {"}
    {"    pub message: String,"}
    {"    pub alert_type: AlertType,"}
    {"}"}
    {""}
    {"#[derive(Clone)]"}
    {"pub enum AlertType {"}
    {"    Info,"}
    {"    Success,"}
    {"    Warning,"}
    {"    Error,"}
    {"}"}
    {""}
    {"pub fn alert(props: AlertProps) -> impl azumi::Component {"}
    {"    let alert_class = match props.alert_type {"}
    {"        AlertType::Info => \"alert-info\","}
    {"        AlertType::Success => \"alert-success\","}
    {"        AlertType::Warning => \"alert-warning\","}
    {"        AlertType::Error => \"alert-error\","}
    {"    };"}
    {""}
    {"    html! {"}
    {"        <div class={format!(\"alert {}\", alert_class)}>"}
    {"            <p>{&props.message}</p>"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>

                                <div class="demo-output">
                                    <div class="alert alert-info">
                                        <p>"This is an informational message"</p>
                                    </div>
                                </div>
                            </div>

                            <div class="example-step">
                                <h4>"Example 2: Card Component"</h4>
                                <p>"A reusable card layout component:"</p>
                                
                                <pre class="code-block">{"#[derive(Clone)]"}
    {"pub struct CardProps {"}
    {"    pub title: String,"}
    {"    pub content: String,"}
    {"    pub footer: Option<String>,"}
    {"}"}
    {""}
    {"pub fn card(props: CardProps) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div class=\"card\">"}
    {"            <div class=\"card-header\">"}
    {"                <h3>{&props.title}</h3>"}
    {"            </div>"}
    {"            <div class=\"card-body\">"}
    {"                <p>{&props.content}</p>"}
    {"            </div>"}
    {"            @if let Some(footer) = &props.footer {"}
    {"                <div class=\"card-footer\">"}
    {"                    <small>{footer}</small>"}
    {"                </div>"}
    {"            }"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>

                                <div class="demo-output">
                                    <div class="card-demo">
                                        <div class="card-header">
                                            <h3>"Card Title"</h3>
                                        </div>
                                        <div class="card-body">
                                            <p>"This is the card content area where you can display information."</p>
                                        </div>
                                        <div class="card-footer">
                                            <small>"Card footer content"</small>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🔄 Component Composition"</h2>
                        <p>"Components can be composed together to build complex interfaces:"</p>

                        <div class="composition-example">
                            <h4>"Building a Dashboard from Components"</h4>
                            <pre class="code-block">{"pub fn dashboard(user: &User) -> impl azumi::Component {"}
    {"    let alert_props = AlertProps {"}
    {"        message: format!(\"Welcome back, {}!\", user.name),"}
    {"        alert_type: AlertType::Success,"}
    {"    };"}
    {""}
    {"    let profile_props = UserProfileProps {"}
    {"        name: user.name.clone(),"}
    {"        bio: user.bio.clone(),"}
    {"        avatar_url: user.avatar_url.clone(),"}
    {"        status: user.status.clone(),"}
    {"    };"}
    {""}
    {"    html! {"}
    {"        <div class=\"dashboard\">"}
    {"            {alert(alert_props)}"}
    {"            {user_profile(profile_props)}"}
    {"            {card(CardProps {"}
    {"                title: \"Recent Activity\".to_string(),"}
    {"                content: \"Your recent activities will appear here.\".to_string(),"}
    {"                footer: Some(\"Updated just now\".to_string()),"}
    {"            })}"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎯 Component Benefits"</h2>
                        <p>"Components provide several key advantages:"</p>

                        <div class="benefits-grid">
                            <div class="benefit-item">
                                <div class="benefit-icon">"♻️"</div>
                                <h4>"Reusability"</h4>
                                <p>"Write once, use everywhere. Components can be reused across different parts of your application."</p>
                            </div>
                            <div class="benefit-item">
                                <div class="benefit-icon">"🧪"</div>
                                <h4>"Testability"</h4>
                                <p>"Isolated components are easier to test independently. Each component has clear inputs and outputs."</p>
                            </div>
                            <div class="benefit-item">
                                <div class="benefit-icon">"🔧"</div>
                                <h4>"Maintainability"</h4>
                                <p>"Changes to a component automatically propagate to all places where it's used."</p>
                            </div>
                            <div class="benefit-item">
                                <div class="benefit-icon">"🎨"</div>
                                <h4>"Consistency"</h4>
                                <p>"Components enforce consistent styling and behavior across your application."</p>
                            </div>
                            <div class="benefit-item">
                                <div class="benefit-icon">"👥"</div>
                                <h4>"Team Collaboration"</h4>
                                <p>"Components provide a shared vocabulary for teams, making collaboration easier."</p>
                            </div>
                            <div class="benefit-item">
                                <div class="benefit-icon">"📈"</div>
                                <h4>"Scalability"</h4>
                                <p>"Large applications become manageable by breaking them into reusable pieces."</p>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"⚠️ Common Component Patterns"</h2>
                        <p>"Here are some patterns to follow and avoid:"</p>

                        <div class="patterns-list">
                            <div class="pattern-item good">
                                <h4>"✅ Good Patterns"</h4>
                                <ul>
                                    <li>"Keep components focused on a single responsibility"</li>
                                    <li>"Use clear, descriptive prop names"</li>
                                    <li>"Provide default values for optional props"</li>
                                    <li>"Include helpful error messages for invalid props"</li>
                                    <li>"Document complex component logic"</li>
                                </ul>
                            </div>
                            <div class="pattern-item bad">
                                <h4>"❌ Anti-Patterns"</h4>
                                <ul>
                                    <li>"Avoid god objects that do too many things"</li>
                                    <li>"Don't make components too opinionated about styling"</li>
                                    <li>"Avoid complex prop validation in the render method"</li>
                                    <li>"Don't create components for trivial HTML fragments"</li>
                                    <li>"Avoid deeply nested component hierarchies"</li>
                                </ul>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"📝 Practice Exercise"</h2>
                        <div class="exercise-box">
                            <h4>"🏋️ Build a Component Library"</h4>
                            <p>"Create a small set of reusable components for a blog application:"</p>
                            
                            <div class="exercise-requirements">
                                <h5>"Components to Build:"</h5>
                                <ul>
                                    <li>"<strong>ArticleCard</strong> - Display article preview with title, excerpt, and author"</li>
                                    <li>"<strong>AuthorBadge</strong> - Show author information with avatar and name"</li>
                                    <li>"<strong>TagList</strong> - Display a list of tags with proper styling"</li>
                                    <li>"<strong>ReadTime</strong> - Calculate and display estimated reading time"</li>
                                </ul>
                            </div>

                            <div class="exercise-instructions">
                                <h5>"Success Criteria:"</h5>
                                <ul>
                                    <li>"Each component has a clear props struct"</li>
                                    <li>"Components follow consistent naming conventions"</li>
                                    <li>"Include at least one conditional rendering example"</li>
                                    <li>"Components are composable (can be used together)"</li>
                                    <li>"Add helpful comments and documentation"</li>
                                </ul>
                            </div>

                            <div class="starter-code">
                                <h5>"Starter Template:"</h5>
                                <pre class="code-block">{"// ArticleCard component"}
    {"pub struct ArticleCardProps {"}
    {"    pub title: String,"}
    {"    pub excerpt: String,"}
    {"    pub author: String,"}
    {"    pub publish_date: String,"}
    {"    pub tags: Vec<String>,"}
    {"    pub read_time_minutes: u32,"}
    {"}"}
    {""}
    {"pub fn article_card(props: ArticleCardProps) -> impl azumi::Component {"}
    {"    // Your implementation here"}
    {"}"}</pre>
                            </div>

                            <div class="exercise-hint">
                                <details>
                                    <summary>"💡 Need a hint?"</summary>
                                    <ul>
                                        <li>"Use @if for conditional rendering like empty tag lists"</li>
                                        <li>"Use @for to iterate over tag arrays"</li>
                                        <li>"Combine multiple components in ArticleCard"</li>
                                        <li>"Consider edge cases like long titles or empty excerpts"</li>
                                    </ul>
                                </details>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎯 Key Takeaways"</h2>
                        <div class="takeaways-list">
                            <div class="takeaway-item">
                                <div class="takeaway-icon">"🏗️"</div>
                                <div>
                                    <h4>"Components = Reusable UI"</h4>
                                    <p>"Components are self-contained UI pieces with clear inputs and outputs"</p>
                                </div>
                            </div>
                            <div class="takeaway-item">
                                <div class="takeaway-icon">"🎯"</div>
                                <div>
                                    <h4>"Single Responsibility"</h4>
                                    <p>"Each component should have one clear purpose and do it well"</p>
                                </div>
                            </div>
                            <div class="takeaway-item">
                                <div class="takeaway-icon">"🔄"</div>
                                <div>
                                    <h4>"Composition Power"</h4>
                                    <p>"Complex interfaces emerge from composing simple components"</p>
                                </div>
                            </div>
                            <div class="takeaway-item">
                                <div class="takeaway-icon">"📈"</div>
                                <div>
                                    <h4>"Scalability Benefits"</h4>
                                    <p>"Components make large applications manageable and maintainable"</p>
                                </div>
                            </div>
                        </div>
                    </section>

                    <nav class="nav-buttons">
                        <a href="/lesson-10" class="btn btn-secondary">"← Previous: Advanced Control Flow"</a>
                        <a href="/lesson-12" class="btn">"Next: Component Props & Data Flow →"</a>
                    </nav>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson11_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson11()))
}