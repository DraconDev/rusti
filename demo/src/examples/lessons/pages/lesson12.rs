use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 12: Component Props & Data Flow
pub fn lesson12() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 12: Component Props & Data Flow - Azumi"</title>
                <style src="/static/pages/lessons.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-number">"Components"</div>
                        <h1 class="lesson-title">"Component Props & Data Flow"</h1>
                        <p class="lesson-subtitle">"Learn to define interfaces, pass data safely, and build reusable component APIs"</p>
                    </header>

                    <section class="section">
                        <h2 class="section-title">"🎯 What You'll Learn"</h2>
                        <p>"This lesson dives deep into component data flow and interfaces:"</p>
                        <ul>
                            <li>"Defining component interfaces with props"</li>
                            <li>"Type-safe prop passing and validation"</li>
                            <li>"Default values and optional properties"</li>
                            <li>"Complex prop patterns and composition"</li>
                            <li>"Best practices for component APIs"</li>
                        </ul>
                        <div class="prerequisite-box">
                            <strong>"📋 Prerequisites:"</strong>
                            <p>"Complete " <a href="/lesson-11">"Introduction to Components" lesson. Understanding of Rust structs, enums, and basic Azumi templates."</p>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🏗️ Component Props System"</h2>
                        <p>"Props are the way components receive data from their parent. Think of them as function parameters for components."</p>

                        <div class="props-overview">
                            <div class="props-benefit">
                                <h4>"🔒 Type Safety"</h4>
                                <p>"All props are validated at compile time"</p>
                            </div>
                            <div class="props-benefit">
                                <h4>"📝 Clear Interfaces"</h4>
                                <p>"Components have explicit, documented APIs"</p>
                            </div>
                            <div class="props-benefit">
                                <h4>"🎯 Reusability"</h4>
                                <p>"Same component, different data"</p>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"📝 Defining Component Interfaces"</h2>
                        <p>"Start by defining the data your component needs:"</p>

                        <div class="code-example">
                            <h4>"Basic Props Struct"</h4>
                            <pre class="code-block">{"// Define what data this component accepts"}
    {"#[derive(Clone)]"}
    {"pub struct UserCardProps {"}
    {"    pub name: String,"}
    {"    pub email: String,"}
    {"    pub avatar_url: Option<String>,"}
    {"    pub is_online: bool,"}
    {"}"}
    {""}
    {"// Component function that uses these props"}
    {"pub fn user_card(props: UserCardProps) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div class=\"user-card\">"}
    {"            @if let Some(avatar) = &props.avatar_url {"}
    {"                <img src={avatar} alt={format!(\"{}'s avatar\", props.name)} class=\"avatar\" />"}
    {"            } else {"}
    {"                <div class=\"avatar-placeholder\">"}
    {"                    {&props.name.chars().next().unwrap_or('?').to_uppercase().to_string()}"}
    {"                </div>"}
    {"            }"}
    {""}
    {"            <div class=\"user-info\">"}
    {"                <h3>{&props.name}</h3>"}
    {"                <p>{&props.email}</p>"}
    {"                @if props.is_online {"}
    {"                    <span class=\"online-indicator\">\"🟢 Online\"</span>"}
    {"                } else {"}
    {"                    <span class=\"offline-indicator\">\"🔴 Offline\"</span>"}
    {"                }"}
    {"            </div>"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"💻 Using Components with Props"</h2>
                        <p>"Pass data to components when using them:"</p>

                        <div class="code-example">
                            <h4>"Component Usage Patterns"</h4>
                            <pre class="code-block">{"// In a parent component"}
    {"html! {"}
    {"    <div class=\"user-list\">"}
    {"        @for user in users {"}
    {"            {user_card(UserCardProps {"}
    {"                name: user.name.clone(),"}
    {"                email: user.email.clone(),"}
    {"                avatar_url: user.avatar_url.clone(),"}
    {"                is_online: user.is_online,"}
    {"            })}"}
    {"        }"}
    {"    </div>"}
    {"}"}
    {""}
    {"// With optional props"}
    {"html! {"}
    {"    <div>"}
    {"        {user_card(UserCardProps {"}
    {"            name: \"Alice\".to_string(),"}
    {"            email: \"alice@example.com\".to_string(),"}
    {"            avatar_url: None, // Optional field"}
    {"            is_online: true,"}
    {"        })}"}
    {"    </div>"}
    {"}"}</pre>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎨 Default Values & Validation"</h2>
                        <p>"Make components easier to use with smart defaults:"</p>

                        <div class="code-example">
                            <h4>"Button Component with Defaults"</h4>
                            <pre class="code-block">{"#[derive(Clone)]"}
    {"pub struct ButtonProps {"}
    {"    pub text: String,"}
    {"    pub variant: ButtonVariant,"}
    {"    pub disabled: bool,"}
    {"    pub size: ButtonSize,"}
    {"}"}
    {""}
    {"#[derive(Clone)]"}
    {"pub enum ButtonVariant {"}
    {"    Primary,"}
    {"    Secondary,"}
    {"    Danger,"}
    {"}"}
    {""}
    {"#[derive(Clone)]"}
    {"pub enum ButtonSize {"}
    {"    Small,"}
    {"    Medium,"}
    {"    Large,"}
    {"}"}
    {""}
    {"// Provide convenient constructors"}
    {"impl Default for ButtonProps {"}
    {"    fn default() -> Self {"}
    {"        ButtonProps {"}
    {"            text: \"Click me\".to_string(),"}
    {"            variant: ButtonVariant::Primary,"}
    {"            disabled: false,"}
    {"            size: ButtonSize::Medium,"}
    {"        }"}
    {"    }"}
    {"}"}
    {""}
    {"impl ButtonProps {"}
    {"    pub fn primary(text: &str) -> Self {"}
    {"        Self {"}
    {"            text: text.to_string(),"}
    {"            variant: ButtonVariant::Primary,"}
    {"            disabled: false,"}
    {"            size: ButtonSize::Medium,"}
    {"        }"}
    {"    }"}
    {""}
    {"    pub fn danger(text: &str) -> Self {"}
    {"        Self {"}
    {"            text: text.to_string(),"}
    {"            variant: ButtonVariant::Danger,"}
    {"            disabled: false,"}
    {"            size: ButtonSize::Medium,"}
    {"        }"}
    {"    }"}
    {""}
    {"    pub fn disabled(mut self) -> Self {"}
    {"        self.disabled = true;"}
    {"        self"}
    {"    }"}
    {"}"}</pre>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎯 Complex Prop Patterns"</h2>
                        <p>"Handle advanced data structures and relationships:"</p>

                        <div class="patterns-grid">
                            <div class="pattern-card">
                                <h4>"🎨 Variant Props"</h4>
                                <p>"Use enums for mutually exclusive options"</p>
                                <pre class="code-block">{"pub enum AlertType {"}
    {"    Success,"}
    {"    Error,"}
    {"    Warning,"}
    {"    Info,"}
    {"}"}</pre>
                            </div>

                            <div class="pattern-card">
                                <h4>"📝 Children Props"</h4>
                                <p>"Pass content as props"</p>
                                <pre class="code-block">{"pub struct ModalProps {"}
    {"    pub title: String,"}
    {"    pub content: impl azumi::Component,"}
    {"    pub on_close: Option<Box<dyn Fn()>>,"}
    {"}"}</pre>
                            </div>

                            <div class="pattern-card">
                                <h4>"🔗 Event Props"</h4>
                                <p>"Handle user interactions"</p>
                                <pre class="code-block">{"pub struct FormProps {"}
    {"    pub on_submit: Box<dyn Fn(FormData)>,"}
    {"    pub on_change: Option<Box<dyn Fn(String)>>,"}
    {"}"}</pre>
                            </div>

                            <div class="pattern-card">
                                <h4>"📊 Data Props"</h4>
                                <p>"Handle collections and complex data"</p>
                                <pre class="code-block">{"pub struct TableProps<T> {"}
    {"    pub data: Vec<T>,"}
    {"    pub columns: Vec<ColumnDef<T>>,"}
    {"}"}</pre>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🔄 Data Flow Patterns"</h2>
                        <p>"Understand how data flows through component hierarchies:"</p>

                        <div class="flow-patterns">
                            <div class="flow-item">
                                <h4>"⬇️ Top-Down Data Flow"</h4>
                                <p>"Data flows from parent to child components"</p>
                                <div class="flow-diagram">
                                    <div class="flow-node parent">"Parent Component"</div>
                                    <div class="flow-arrow">"↓ Props"</div>
                                    <div class="flow-node child">"Child Component"</div>
                                </div>
                            </div>

                            <div class="flow-item">
                                <h4>"⬆️ Event Upward"</h4>
                                <p>"Events bubble up from child to parent"</p>
                                <div class="flow-diagram">
                                    <div class="flow-node child">"Child Component"</div>
                                    <div class="flow-arrow">"↑ Events"</div>
                                    <div class="flow-node parent">"Parent Component"</div>
                                </div>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"⚠️ Props Validation"</h2>
                        <p>"Add runtime validation for important invariants:"</p>

                        <div class="code-example">
                            <h4>"Smart Constructor Pattern"</h4>
                            <pre class="code-block">{"pub struct ProgressBarProps {"}
    {"    value: u32,"}
    {"    max: u32,"}
    {"}"}
    {""}
    {"impl ProgressBarProps {"}
    {"    pub fn new(value: u32, max: u32) -> Result<Self, &'static str> {"}
    {"        if max == 0 {"}
    {"            return Err(\"Max value cannot be 0\");"}
    {"        }"}
    {"        if value > max {"}
    {"            return Err(\"Value cannot exceed max\");"}
    {"        }"}
    {""}
    {"        Ok(ProgressBarProps { value, max })"}
    {"    }"}
    {""}
    {"    pub fn new_clamped(value: u32, max: u32) -> Self {"}
    {"        Self::new(value.min(max), max.max(1)).unwrap_or(Self {"}
    {"            value: 0,"}
    {"            max: 1,"}
    {"        })"}
    {"    }"}
    {"}"}</pre>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"📋 Props Best Practices"</h2>
                        <p>"Guidelines for creating maintainable component APIs:"</p>

                        <div class="best-practices">
                            <div class="practice-item good">
                                <h4>"✅ DO"</h4>
                                <ul>
                                    <li>"Use descriptive prop names"</li>
                                    <li>"Group related props in structs"</li>
                                    <li>"Provide default values when sensible"</li>
                                    <li>"Validate important invariants"</li>
                                    <li>"Document complex prop relationships"</li>
                                </ul>
                            </div>

                            <div class="practice-item bad">
                                <h4>"❌ DON'T"</h4>
                                <ul>
                                    <li>"Make props optional without good reason"</li>
                                    <li>"Mix concerns in a single prop struct"</li>
                                    <li>"Ignore type safety for convenience"</li>
                                    <li>"Create prop soup components"</li>
                                    <li>"Hide important behavior in prop defaults"</li>
                                </ul>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"📝 Practice Exercise"</h2>
                        <div class="exercise-box">
                            <h4>"🏋️ Build a Data Table Component"</h4>
                            <p>"Create a flexible table component with advanced prop patterns:"</p>
                            
                            <div class="exercise-requirements">
                                <h5>"Requirements:"</h5>
                                <ul>
                                    <li>"Accept generic data with column definitions"</li>
                                    <li>"Support sorting, filtering, and pagination"</li>
                                    <li>"Handle loading and empty states"</li>
                                    <li>"Provide callbacks for user actions"</li>
                                    <li>"Include responsive design options"</li>
                                </ul>
                            </div>

                            <div class="starter-code">
                                <h5>"Starter Template:"</h5>
                                <pre class="code-block">{"#[derive(Clone)]"}
    {"pub struct TableProps<T> {"}
    {"    // Define your props here"}
    {"}"}
    {""}
    {"pub fn data_table<T>("}
    {"    props: TableProps<T>")
    {") -> impl azumi::Component {"}
    {"    // Your implementation here"}
    {"}"}</pre>
                            </div>

                            <div class="exercise-hint">
                                <details>
                                    <summary>"💡 Need hints?"</summary>
                                    <ul>
                                        <li>"Use generic types for flexible data handling"</li>
                                        <li>"Separate display concerns from data management"</li>
                                        <li>"Provide sensible defaults for styling props"</li>
                                        <li>"Think about event handling patterns"</li>
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
                                    <h4>"Clear Interfaces"</h4>
                                    <p>"Props define explicit contracts between components"</p>
                                </div>
                            </div>
                            <div class="takeaway-item">
                                <div class="takeaway-icon">"🔒"</div>
                                <div>
                                    <h4>"Type Safety"</h4>
                                    <p>"Compile-time validation prevents runtime errors"</p>
                                </div>
                            </div>
                            <div class="takeaway-item">
                                <div class="takeaway-icon">"🔄"</div>
                                <div>
                                    <h4>"Data Flow"</h4>
                                    <p>"Data flows down, events bubble up"</p>
                                </div>
                            </div>
                            <div class="takeaway-item">
                                <div class="takeaway-icon">"🎨"</div>
                                <div>
                                    <h4>"Flexible APIs"</h4>
                                    <p>"Default values and builders make components easy to use"</p>
                                </div>
                            </div>
                        </div>
                    </section>

                    <nav class="nav-buttons">
                        <a href="/lesson-11" class="btn btn-secondary">"← Previous: Introduction to Components"</a>
                        <a href="/lesson-13" class="btn">"Next: Component Composition →"</a>
                    </nav>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson12_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson12()))
}