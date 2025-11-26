use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 1: Getting Started with Azumi
pub fn lesson1() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 1: Getting Started with Azumi"</title>
                    <style src="/static/page/lessons.css" />
                    <style src="/static/page/lesson1.css" />
                </head>
                <body>
                    <div class="lesson-container">
                        <header class="lesson-header">
                            <a href="/" class="back-link">"← Back to Lessons"</a>
                            <div class="lesson-number">"Foundation"</div>
                            <h1 class="lesson-title">"Getting Started with Azumi"</h1>
                            <p class="lesson-subtitle">"Learn what type-safe HTML means and create your first template"</p>
                        </header>

                        <section class="section">
                            <h2 class="section-title">"🎯 What You'll Learn"</h2>
                            <p>"By the end of this lesson, you'll understand:"</p>
                            <ul>
                                <li>"What makes HTML 'type-safe' and why it matters"</li>
                                <li>"How Azumi prevents common web development bugs"</li>
                                <li>"The difference between traditional templating and type-safe templates"</li>
                                <li>"How to create your very first Azumi template"</li>
                            </ul>
                            <div class="prerequisite-box">
                                <strong>"📋 Prerequisites:"</strong>
                                <p>"Basic understanding of Rust syntax. No web development experience required!"</p>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"🤔 What is Type-Safe HTML?"</h2>
                            <p>"Traditional web templates have a fundamental problem: they can fail at runtime. Let me show you what this means:"</p>

                            <div class="comparison-grid">
                                <div class="comparison-side">
                                    <h3>"❌ Traditional Templates (Runtime Errors)"</h3>
                                    <div class="code-demo">
                                        <pre class="code-block">{"<!-- Traditional HTML template -->"}
    {"<div class=\"user-name\">"}
    {"  Hello {{ user_name }}!"}
    {"</div>"}
    {"<style>"}
    {".user-profile { color: blue; }"}
    {"</style>"}</pre>
                                    </div>
                                    <div class="error-demo">
                                        <h4>"🚨 What can go wrong:"</h4>
                                        <ul>
                                            <li>"Misspelled class names show no error"</li>
                                            <li>"Missing CSS files break styling"</li>
                                            <li>"HTML structure errors appear in browser"</li>
                                            <li>"No compile-time feedback"</li>
                                        </ul>
                                    </div>
                                </div>

                                <div class="comparison-side">
                                    <h3>"✅ Azumi Templates (Compile-Time Safety)"</h3>
                                    <div class="code-demo">
                                        <pre class="code-block">{"// Azumi template with type safety"}
    {"html! {"}
    {"  <div class=\"user-name\">"}
    {"    \"Hello \" user_name \"!\""}
    {"  </div>"}
    {"}"}</pre>
                                    </div>
                                    <div class="safety-demo">
                                        <h4>"🛡️ What Azumi guarantees:"</h4>
                                        <ul>
                                            <li>"All CSS classes must exist"</li>
                                            <li>"All HTML attributes are validated"</li>
                                            <li>"Missing templates fail at compile time"</li>
                                            <li>"100% type safety"</li>
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"🏗️ How Azumi Works"</h2>
                            <p>"Azumi sits in the compilation pipeline and validates everything before your code runs:"</p>

                            <div class="pipeline-demo">
                                <div class="pipeline-step">
                                    <div class="step-number">"1"</div>
                                    <div class="step-content">
                                        <h4>"You Write Template"</h4>
                                        <p>"HTML-like syntax with Rust expressions"</p>
                                    </div>
                                </div>
                                <div class="pipeline-arrow">"→"</div>
                                <div class="pipeline-step">
                                    <div class="step-number">"2"</div>
                                    <div class="step-content">
                                        <h4>"Azumi Parses & Validates"</h4>
                                        <p>"Checks CSS classes, HTML structure, and Rust code"</p>
                                    </div>
                                </div>
                                <div class="pipeline-arrow">"→"</div>
                                <div class="pipeline-step">
                                    <div class="step-number">"3"</div>
                                    <div class="step-content">
                                        <h4>"Compiler Generates Code"</h4>
                                        <p>"Creates optimized Rust code with safety guarantees"</p>
                                    </div>
                                </div>
                                <div class="pipeline-arrow">"→"</div>
                                <div class="pipeline-step">
                                    <div class="step-number">"4"</div>
                                    <div class="step-content">
                                        <h4>"Safe Runtime Execution"</h4>
                                        <p>"Your app runs with zero template-related bugs"</p>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"💻 Your First Azumi Template"</h2>
                            <p>"Let's create a simple template step by step. Don't worry if you don't understand everything yet - we'll build on this:"</p>

                            <div class="step-by-step">
                                <div class="step-item">
                                    <div class="step-label">"Step 1"</div>
                                    <h4>"Basic Template Structure"</h4>
                                    <p>"Every Azumi template starts with the <code>html!</code> macro:"</p>
                                    <div class="code-demo">
                                        <pre class="code-block">{"use azumi::html;"}
    {""}
    {"pub fn my_template() -> impl azumi::Component {"}
    {"    html! {"}
    {"        // Your template goes here"}
    {"    }"}
    {"}"}</pre>
                                    </div>
                                </div>

                                <div class="step-item">
                                    <div class="step-label">"Step 2"</div>
                                    <h4>"Add Some HTML"</h4>
                                    <p>"Now add HTML-like content. Notice how everything is quoted:"</p>
                                    <div class="code-demo">
                                        <pre class="code-block">{"pub fn my_template() -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div class=\"container\">"}
    {"            <h1>\"Welcome to Azumi!\"</h1>"}
    {"            <p>\"This is your first template.\"</p>"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                                    </div>
                                </div>

                                <div class="step-item">
                                    <div class="step-label">"Step 3"</div>
                                    <h4>"Add Dynamic Content"</h4>
                                    <p>"You can embed Rust variables directly:"</p>
                                    <div class="code-demo">
                                        <pre class="code-block">{"pub fn greeting_template(name: &str) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div class=\"container\">"}
    {"            <h1>\"Welcome to Azumi!\"</h1>"}
    {"            <p>\"Hello \" name \"!\"</p>"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                                    </div>
                                </div>
                            </div>

                            <div class="live-demo">
                                <h4>"🎭 Live Example"</h4>
                                <p>"This is what your template will render:"</p>
                                <div class="demo-output">
                                    <div class="container-demo">
                                        <h1>"Welcome to Azumi!"</h1>
                                        <p>"Hello Student! This is your first template rendered in a browser."</p>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"🔍 Why Strict Quoting?"</h2>
                            <p>"You might be wondering why Azumi requires quotes around everything. Here's the technical reason:"</p>

                            <div class="technical-explanation">
                                <h4>"The Lexer Challenge"</h4>
                                <p>"When Rust's lexer reads template code, it needs to distinguish between:"</p>
                                <ul>
                                    <li>"<code><</code> - HTML tag start vs Rust's less-than operator"</li>
                                    <li>"<code>></code> - HTML tag end vs Rust's greater-than operator"</li>
                                    <li>"<code>{</code> - Template expression vs Rust code block"</li>
                                    <li>"<code>}</code> - Template expression end vs Rust code block end"</li>
                                </ul>
                                <div class="warning-box">
                                    <strong>"⚠️ Without quotes:"</strong>
                                    <p>"The lexer gets confused and can't parse your template correctly, leading to cryptic compilation errors."</p>
                                </div>
                                <div class="solution-box">
                                    <strong>"✅ With quotes:"</strong>
                                    <p>"Everything is crystal clear. The lexer knows exactly what each character represents."</p>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"🎯 Key Takeaways"</h2>
                            <div class="takeaways-grid">
                                <div class="takeaway-item">
                                    <div class="takeaway-icon">"🛡️"</div>
                                    <h4>"Compile-Time Safety"</h4>
                                    <p>"Errors are caught before your code ever runs"</p>
                                </div>
                                <div class="takeaway-item">
                                    <div class="takeaway-icon">"⚡"</div>
                                    <h4>"Zero Runtime Overhead"</h4>
                                    <p>"All validation happens during compilation"</p>
                                </div>
                                <div class="takeaway-item">
                                    <div class="takeaway-icon">"🎯"</div>
                                    <h4>"Precise Error Messages"</h4>
                                    <p>"Exact locations for any issues found"</p>
                                </div>
                                <div class="takeaway-item">
                                    <div class="takeaway-icon">"🚀"</div>
                                    <h4>"Better Developer Experience"</h4>
                                    <p>"Catch bugs in your IDE, not in production"</p>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"📝 Practice Exercise"</h2>
                            <div class="exercise-box">
                                <h4>"🏋️ Your Turn: Create a Profile Template"</h4>
                                <p>"Try to create a template that displays user information. Here's what to include:"</p>
                                <ul>
                                    <li>"User's name as a heading"</li>
                                    <li>"User's email as a paragraph"</li>
                                    <li>"User's bio as another paragraph"</li>
                                    <li>"A welcome message that includes the user's name"</li>
                                </ul>
                                <div class="starter-code">
                                    <h5>"Starter Code:"</h5>
                                    <pre class="code-block">{"pub fn user_profile(name: &str, email: &str, bio: &str) -> impl azumi::Component {"}
    {"    html! {"}
    {"        // Your code here"}
    {"    }"}
    {"}"}</pre>
                                </div>
                                <div class="solution-hint">
                                    <details>
                                        <summary>"💡 Need a hint?"</summary>
                                        <p>"Remember: All text content must be in quotes. You'll need to use the <code>html!</code> macro and create HTML elements with classes."</p>
                                    </details>
                                </div>
                            </div>
                        </section>

                        <nav class="nav-buttons">
                            <a href="/" class="btn btn-secondary">"← Home"</a>
                            <a href="/lesson-2" class="btn">"Next: The Quoting Fundamentals →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson1_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson1()))
}
