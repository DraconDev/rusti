use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 9: Local Variables with @let
pub fn lesson9() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 9: Local Variables with @let - Azumi"</title>
                    <style src="/static/pages/lessons.css" />
                </head>
                <body>
                    <div class="lesson-container">
                        <header class="lesson-header">
                            <a href="/" class="back-link">"← Back to Lessons"</a>
                            <div class="lesson-number">"Control Flow"</div>
                            <h1 class="lesson-title">"Local Variables with @let"</h1>
                            <p class="lesson-subtitle">"Create computed values and complex data transformations in templates"</p>
                        </header>

                        <section class="section">
                            <h2 class="section-title">"🎯 What You'll Learn"</h2>
                            <p>"Building on previous control flow lessons, you'll master:"</p>
                            <ul>
                                <li>"@let syntax and scoping rules"</li>
                                <li>"Creating computed values directly in templates"</li>
                                <li>"Complex data transformations and processing"</li>
                                <li>"When to use @let vs. external Rust code"</li>
                                <li>"Best practices for template logic organization"</li>
                            </ul>
                            <div class="prerequisite-box">
                                <strong>"📋 Prerequisites:"</strong>
                                <p>"Complete lessons on " <a href="/lesson-6">"@if"</a> ", " <a href="/lesson-7">"@for"</a> ", and " <a href="/lesson-8">"@match"</a> ". Understanding of Rust variables and expressions."</p>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"📝 Understanding @let"</h2>
                            <p>"The <code>@let</code> keyword creates temporary local variables within your template. This is powerful for:"</p>

                            <div class="feature-list">
                                <div class="feature-item">
                                    <h4>"🧮 Computed Values"</h4>
                                    <p>"Calculate values on-the-fly without leaving your template"</p>
                                </div>
                                <div class="feature-item">
                                    <h4>"🔄 Data Transformation"</h4>
                                    <p>"Convert and format data directly in the template context"</p>
                                </div>
                                <div class="feature-item">
                                    <h4>"📊 Complex Expressions"</h4>
                                    <p>"Break down complex logic into readable, manageable pieces"</p>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"💻 Basic @let Usage"</h2>
                            <p>"Start with simple variable assignments:"</p>

                            <div class="code-example">
                                <h4>"Simple Variable Assignment"</h4>
                                <pre class="code-block">{"pub fn user_greeting(user: &User) -> impl azumi::Component {"}
        {"    html! {"}
        {"        <div>"}
        {"            @let full_name = format!(\"{}\", user.first_name, user.last_name);"}
        {"            <p>{\"Welcome, \" full_name \"!\"}</p>"}
        {"        </div>"}
        {"    }"}
        {"}"}</pre>

                                <div class="code-explanation">
                                    <p>"<strong>Key points:</strong>"</p>
                                    <ul>
                                        <li>"Use <code>@let</code> followed by variable name and <code>=</code>"</li>
                                        <li>"Assign any valid Rust expression"</li>
                                        <li>"Variable is scoped to the current template level"</li>
                                        <li>"Can reference other variables and call functions"</li>
                                    </ul>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"🔄 Conditional Variable Assignment"</h2>
                            <p>"Create variables based on conditions:"</p>

                            <div class="code-example">
                                <h4>"Dynamic Display Names"</h4>
                                <pre class="code-block">{"pub fn user_display(user: &User) -> impl azumi::Component {"}
        {"    html! {"}
        {"        <div class=\"user-card\">"}
        {"            @let display_name = if user.nickname.is_empty() {"}
        {"                &user.full_name"}
        {"            } else {"}
        {"                &user.nickname"}
        {"            };"}
        {"            @let is_new_user = user.created_at.days_since_now() < 7;"}
        {"            "}
        {"            <h3>{display_name}</h3>"}
        {"            @if is_new_user {"}
        {"                <span class=\"new-user-badge\">\"New!\"</span>"}
        {"            }"}
        {"        </div>"}
        {"    }"}
        {"}"}</pre>

                                <div class="behavior-demo">
                                    <h4>"What this produces:"</h4>
                                    <div class="demo-output">
                                        <div class="user-card-demo">
                                            <h3>"John Doe"</h3>
                                            <span class="new-user-badge">"New!"</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"📊 Complex Data Transformations"</h2>
                            <p>"Process and transform data collections directly in templates:"</p>

                            <div class="code-example">
                                <h4>"Sorting and Filtering"</h4>
                                <pre class="code-block">{"pub fn recent_comments(comments: &[Comment]) -> impl azumi::Component {"}
        {"    html! {"}
        {"        <div class=\"comments-section\">"}
        {"            @let sorted_comments = {"}
        {"                let mut comments = comments.to_vec();"}
        {"                comments.sort_by(|a, b| b.created_at.cmp(&a.created_at));"}
        {"                comments.into_iter().take(5).collect::<Vec<_>>()"}
        {"            };"}
        {"            "}
        {"            @for comment in sorted_comments {"}
        {"                <div class=\"comment\">"}
        {"                    <p>{&comment.content}</p>"}
        {"                    <small>{\"by \" &comment.author} \" on \" {format_date(&comment.created_at)}</small>"}
        {"                </div>"}
        {"            }"}
        {"        </div>"}
        {"    }"}
        {"}"}</pre>

                                <div class="performance-note">
                                    <h4>"⚡ Performance Consideration"</h4>
                                    <p>"For complex transformations, consider moving logic to Rust functions for better performance, but @let is perfect for:"</p>
                                    <ul>
                                        <li>"Simple sorting and filtering"</li>
                                        <li>"Data formatting and formatting"</li>
                                        <li>"Conditional value selection"</li>
                                    </ul>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"🎨 Practical Examples"</h2>
                            <p>"See @let in action with real-world scenarios:"</p>

                            <div class="examples-grid">
                                <div class="example-card">
                                    <h4>"Shopping Cart Total"</h4>
                                    <pre class="code-block">{"@let total = items.iter()"}
        {".fold(0.0, |sum, item| sum + item.price);"}
        {"<p>{\"Total: $\" total.to_fixed(2)}</p>"}</pre>
                                </div>

                                <div class="example-card">
                                    <h4>"Date Formatting"</h4>
                                    <pre class="code-block">{"@let formatted_date = user.created_at"}
        {".format(\"%B %d, %Y\").to_string();"}
        {"<p>{\"Joined: \" formatted_date}</p>"}</pre>
                                </div>

                                <div class="example-card">
                                    <h4>"Array Length Display"</h4>
                                    <pre class="code-block">{"@let item_count = notifications.len();"}
        {"@let count_text = match item_count {"}
        {"    0 => \"No notifications\","}
        {"    1 => \"1 notification\","}
        {"    n => format!(\"{} notifications\", n),"}
        {"};"}
        {"<p>{count_text}</p>"}</pre>
                                </div>

                                <div class="example-card">
                                    <h4>"URL Slug Generation"</h4>
                                    <pre class="code-block">{"@let slug = title.to_lowercase()"}
        {".replace(\" \", \"-\")"}
        {".chars().filter(|c| c.is_alphanumeric() || *c == '-').collect::<String>();"}
        {"<a href={\"/posts/\" slug}>{title}</a>"}</pre>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"🔍 Variable Scope and Lifetime"</h2>
                            <p>"Understanding how @let variables work:"</p>

                            <div class="scope-examples">
                                <div class="scope-item">
                                    <h4>"Template-Level Scope"</h4>
                                    <pre class="code-block">{"html! {"}
        {"    <div>"}
        {"        @let message = \"Hello\";"}
        {"        @if condition {"}
        {"            {message} // ✓ Available here"}
        {"        }"}
        {"    </div>"}
        {"    {message} // ✗ Not available here!"}
    {"}"}</pre>
                                </div>

                                <div class="scope-item">
                                    <h4>"Block-Level Scope"</h4>
                                    <pre class="code-block">{"@if condition {"}
        {"    @let local_var = \"only in if block\";"}
        {"    {local_var} // ✓ Available"}
    {"}"}
        {"{local_var} // ✗ Not available outside!"}</pre>
                                </div>

                                <div class="scope-item">
                                    <h4>"Loop-Level Scope"</h4>
                                    <pre class="code-block">{"@for item in items {"}
        {"    @let index = item.index;"}
        {"    <p>{\"Item \" index \" of \" item.name}</p>"}
    {"}"}
        {"{index} // ✗ Not available outside loop!"}</pre>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"⚡ When to Use @let"</h2>
                            <p>"Guidelines for effective @let usage:"</p>

                            <div class="guidelines-grid">
                                <div class="guideline-item good">
                                    <h4>"✅ Use @let for:"</h4>
                                    <ul>
                                        <li>"Simple data formatting"</li>
                                        <li>"Conditional value selection"</li>
                                        <li>"Short calculations"</li>
                                        <li>"Template-specific transformations"</li>
                                        <li>"Improving readability"</li>
                                    </ul>
                                </div>

                                <div class="guideline-item bad">
                                    <h4>"❌ Don't use @let for:"</h4>
                                    <ul>
                                        <li>"Complex business logic"</li>
                                        <li>"Database queries"</li>
                                        <li>"Heavy computations"</li>
                                        <li>"Code that needs testing"</li>
                                        <li>"Reusable functionality"</li>
                                    </ul>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"📝 Practice Exercise"</h2>
                            <div class="exercise-box">
                                <h4>"🏋️ Build a User Dashboard Component"</h4>
                                <p>"Create a component that displays user statistics with computed values:"</p>

                                <div class="exercise-requirements">
                                    <h5>"Requirements:"</h5>
                                    <ul>
                                        <li>"Display user's full name (combine first + last)"</li>
                                        <li>"Show account age in days"</li>
                                        <li>"Calculate activity score based on posts and comments"</li>
                                        <li>"Display appropriate greeting based on time of day"</li>
                                        <li>"Show user tier (new, regular, premium) based on activity"</li>
                                    </ul>
                                </div>

                                <div class="starter-code">
                                    <h5>"Starter Code:"</h5>
                                    <pre class="code-block">{"pub fn user_dashboard(user: &User) -> impl azumi::Component {"}
        {"    html! {"}
        {"        <div class=\"dashboard\">"}
        {"            // Your @let statements and template code here"}
        {"        </div>"}
        {"    }"}
        {"}"}</pre>
                                </div>

                                <div class="exercise-hint">
                                    <details>
                                        <summary>"💡 Need hints?"</summary>
                                        <ul>
                                            <li>"Use <code>@let</code> for computed values like full name and activity score"</li>
                                            <li>"Use <code>@if</code> for conditional content like greetings"</li>
                                            <li>"Use <code>@match</code> for determining user tier"</li>
                                            <li>"Keep each @let statement focused on one concept"</li>
                                        </ul>
                                    </details>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"🎯 Key Takeaways"</h2>
                            <div class="takeaways-list">
                                <div class="takeaway-item">
                                    <div class="takeaway-icon">"🧮"</div>
                                    <div>
                                        <h4>"Computed Values"</h4>
                                        <p>"@let creates temporary variables for calculations and transformations"</p>
                                    </div>
                                </div>
                                <div class="takeaway-item">
                                    <div class="takeaway-icon">"📊"</div>
                                    <div>
                                        <h4>"Data Processing"</h4>
                                        <p>"Perfect for sorting, filtering, and formatting data in templates"</p>
                                    </div>
                                </div>
                                <div class="takeaway-item">
                                    <div class="takeaway-icon">"🔍"</div>
                                    <div>
                                        <h4>"Scope Awareness"</h4>
                                        <p>"Variables are scoped to their template level, preventing conflicts"</p>
                                    </div>
                                </div>
                                <div class="takeaway-item">
                                    <div class="takeaway-icon">"⚡"</div>
                                    <div>
                                        <h4>"Performance Balance"</h4>
                                        <p>"Use for simple operations; move complex logic to Rust functions"</p>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <nav class="nav-buttons">
                            <a href="/lesson-8" class="btn btn-secondary">"← Previous: Pattern Matching"</a>
                            <a href="/lesson-10" class="btn">"Next: Advanced Control Flow →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson9_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson9()))
}
