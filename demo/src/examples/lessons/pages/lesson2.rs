use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 2: The Quoting Fundamentals
pub fn lesson2() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 2: The Quoting Fundamentals - Azumi"</title>
                    <style src="/static/lessons.css" />
                    <style src="/static/lesson2.css" />
                </head>
                <body>
                    <div class="lesson-container">
                        <header class="lesson-header">
                            <a href="/" class="back-link">"← Back to Lessons"</a>
                            <div class="lesson-number">"Foundation"</div>
                            <h1 class="lesson-title">"The Quoting Fundamentals"</h1>
                            <p class="lesson-subtitle">"Master the core quoting rules that make Azumi type-safe and prevent bugs"</p>
                        </header>

                        <section class="section">
                            <h2 class="section-title">"🎯 What You'll Learn"</h2>
                            <p>"Building on Lesson 1, you'll master:"</p>
                            <ul>
                                <li>"Why ALL text content must be quoted in Azumi"</li>
                                <li>"How to quote text content correctly (the 3 rules)"</li>
                                <li>"How to quote HTML attributes properly"</li>
                                <li>"Common quoting mistakes and how to fix them"</li>
                                <li>"Nested quotes and escaping"</li>
                            </ul>
                            <div class="prerequisite-box">
                                <strong>"📋 Prerequisites:"</strong>
                                <p>"Complete <a href=\"/lesson-1\">Lesson 1: Getting Started</a>. You should understand basic Azumi template structure."</p>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"🤔 Why Does Azumi Require Quotes?"</h2>
                            <p>"Let's understand the fundamental reason behind Azumi's strict quoting requirements:"</p>

                            <div class="problem-explanation">
                                <h4>"⚠️ The Lexer's Challenge"</h4>
                                <p>"When Rust's compiler reads your template code, it uses a 'lexer' to understand what each character represents. The lexer faces a critical ambiguity:"</p>

                                <div class="character-confusion">
                                    <div class="confusion-item">
                                        <code>"<"</code>
                                        <span>"HTML tag start OR Rust's less-than operator?"</span>
                                    </div>
                                    <div class="confusion-item">
                                        <code>">"</code>
                                        <span>"HTML tag end OR Rust's greater-than operator?"</span>
                                    </div>
                                    <div class="confusion-item">
                                        <code>"{"</code>
                                        <span>"Template expression OR Rust code block?"</span>
                                    </div>
                                    <div class="confusion-item">
                                        <code>"}"</code>
                                        <span>"Template expression end OR Rust code block end?"</span>
                                    </div>
                                </div>

                                <div class="solution-highlight">
                                    <strong>"✅ Azumi's Solution: Strict Quoting"</strong>
                                    <p>"By requiring quotes around all text, Azumi makes the lexer's job crystal clear. The lexer knows EXACTLY what each character represents."</p>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"📝 Rule 1: Quote All Text Content"</h2>
                            <p>"Every piece of text that appears in your template must be wrapped in quotes. Here are the three types:"</p>

                            <div class="rule-examples">
                                <div class="rule-item">
                                    <div class="rule-header">
                                        <div class="rule-number">"1"</div>
                                        <h3>"Simple Text"</h3>
                                    </div>
                                    <div class="rule-comparison">
                                        <div class="wrong-example">
                                            <div class="example-label incorrect">"❌ Wrong (Won't Compile)"</div>
                                            <pre class="code-block">{"<h1>Welcome to Azumi</h1>"}</pre>
                                        </div>
                                        <div class="correct-example">
                                            <div class="example-label correct">"✅ Correct"</div>
                                            <pre class="code-block">{"<h1>\"Welcome to Azumi\"</h1>"}</pre>
                                        </div>
                                    </div>
                                </div>

                                <div class="rule-item">
                                    <div class="rule-header">
                                        <div class="rule-number">"2"</div>
                                        <h3>"Text with Variables"</h3>
                                    </div>
                                    <div class="rule-comparison">
                                        <div class="wrong-example">
                                            <div class="example-label incorrect">"❌ Wrong"</div>
                                            <pre class="code-block">{"<p>Hello user.name</p>"}</pre>
                                        </div>
                                        <div class="correct-example">
                                            <div class="example-label correct">"✅ Correct"</div>
                                            <pre class="code-block">{"<p>\"Hello \" user.name</p>"}</pre>
                                        </div>
                                    </div>
                                </div>

                                <div class="rule-item">
                                    <div class="rule-header">
                                        <div class="rule-number">"3"</div>
                                        <h3>"Mixed Text and Variables"</h3>
                                    </div>
                                    <div class="rule-comparison">
                                        <div class="wrong-example">
                                            <div class="example-label incorrect">"❌ Wrong"</div>
                                            <pre class="code-block">{"<p>Welcome user.name, you have count notifications</p>"}</pre>
                                        </div>
                                        <div class="correct-example">
                                            <div class="example-label correct">"✅ Correct"</div>
                                            <pre class="code-block">{"<p>\"Welcome \" user.name \", you have \" count \" notifications\"</p>"}</pre>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"🏷️ Rule 2: Quote All Attributes"</h2>
                            <p>"HTML attributes must also be quoted. This includes class names, IDs, data attributes, and more:"</p>

                            <div class="attribute-examples">
                                <div class="attribute-grid">
                                    <div class="attribute-example">
                                        <h4>"Class Names"</h4>
                                        <div class="comparison">
                                            <div class="wrong">
                                                <code>"<div class=container>"</code>
                                            </div>
                                            <div class="correct">
                                                <code>"<div class=\"container\">"</code>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="attribute-example">
                                        <h4>"IDs"</h4>
                                        <div class="comparison">
                                            <div class="wrong">
                                                <code>"<input id=userInput>"</code>
                                            </div>
                                            <div class="correct">
                                                <code>"<input id=\"userInput\">"</code>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="attribute-example">
                                        <h4>"Data Attributes"</h4>
                                        <div class="comparison">
                                            <div class="wrong">
                                                <code>"<div data-userid=123>"</code>
                                            </div>
                                            <div class="correct">
                                                <code>"<div data-userid=\"123\">"</code>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="attribute-example">
                                        <h4>"href Links"</h4>
                                        <div class="comparison">
                                            <div class="wrong">
                                                <code>"<a href=/dashboard>"</code>
                                            </div>
                                            <div class="correct">
                                                <code>"<a href=\"/dashboard\">"</code>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"🧩 Nested Quotes & Escaping"</h2>
                            <p>"Sometimes you need quotes inside quoted text. Here's how to handle them:"</p>

                            <div class="nested-examples">
                                <div class="nested-item">
                                    <h4>"Quoting Inside Text Content"</h4>
                                    <p>"Use single quotes inside double-quoted text:"</p>
                                    <div class="code-demo">
                                        <pre class="code-block">{"// This works fine"}
    {"<p>\"He said 'Hello World' to everyone\"</p>"}
    {""}
    {"// Or use escaped double quotes"}
    {"<p>\"He said \\\"Hello World\\\" to everyone\"</p>"}</pre>
                                    </div>
                                </div>

                                <div class="nested-item">
                                    <h4>"Quoting Inside Attributes"</h4>
                                    <p>"For attributes, escape internal quotes:"</p>
                                    <div class="code-demo">
                                        <pre class="code-block">{"// Using single quotes for attribute"}
    {"<input placeholder='Say \"Hello\"' />"}
    {""}
    {"// Or escaping double quotes"}
    {"<input placeholder=\"Say \\\"Hello\\\"\" />"}</pre>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"🎭 Live Examples"</h2>
                            <p>"See these quoting rules in action. These examples are rendered using proper Azumi quoting:"</p>

                            <div class="live-examples">
                                <div class="example-output">
                                    <h4>"Simple Text Output"</h4>
                                    <div class="rendered-example">
                                        <h1>"Welcome to Azumi Fundamentals!"</h1>
                                        <p>"Learning proper quoting is essential for type-safe templates."</p>
                                    </div>
                                </div>

                                <div class="example-output">
                                    <h4>"Dynamic Content Output"</h4>
                                    <div class="rendered-example">
                                        <div class="user-greeting">
                                            <p>"Welcome back, John Doe!"</p>
                                            <p>"You have 5 new notifications waiting for you."</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"🔧 Common Mistakes & Solutions"</h2>
                            <p>"Here are the most common quoting mistakes and how to fix them:"</p>

                            <div class="mistakes-grid">
                                <div class="mistake-item">
                                    <div class="mistake-header">
                                        <span class="mistake-icon">"❌"</span>
                                        <h4>"Forgot to Quote Text"</h4>
                                    </div>
                                    <div class="mistake-example">
                                        <pre class="code-block">{"<h1>Welcome User</h1>"}</pre>
                                    </div>
                                    <div class="mistake-solution">
                                        <span class="solution-icon">"✅"</span>
                                        <pre class="code-block">{"<h1>\"Welcome User\"</h1>"}</pre>
                                    </div>
                                </div>

                                <div class="mistake-item">
                                    <div class="mistake-header">
                                        <span class="mistake-icon">"❌"</span>
                                        <h4>"Unquoted Attributes"</h4>
                                    </div>
                                    <div class="mistake-example">
                                        <pre class="code-block">{"<div class=container id=main></div>"}</pre>
                                    </div>
                                    <div class="mistake-solution">
                                        <span class="solution-icon">"✅"</span>
                                        <pre class="code-block">{"<div class=\"container\" id=\"main\"></div>"}</pre>
                                    </div>
                                </div>

                                <div class="mistake-item">
                                    <div class="mistake-header">
                                        <span class="mistake-icon">"❌"</span>
                                        <h4>"Mixed Quote Types Wrong"</h4>
                                    </div>
                                    <div class="mistake-example">
                                        <pre class="code-block">{"<p>\"He said 'Hello'</p>"}</pre>
                                    </div>
                                    <div class="mistake-solution">
                                        <span class="solution-icon">"✅"</span>
                                        <pre class="code-block">{"<p>\"He said 'Hello'</p>"}</pre>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"📋 Quick Reference Guide"</h2>
                            <div class="reference-grid">
                                <div class="reference-card">
                                    <h4>"Text Content"</h4>
                                    <ul>
                                        <li>"Always use double quotes"</li>
                                        <li>"Wrap every text string"</li>
                                        <li>"Mix text + variables with +"</li>
                                    </ul>
                                </div>

                                <div class="reference-card">
                                    <h4>"Attributes"</h4>
                                    <ul>
                                        <li>"Quote all attribute values"</li>
                                        <li>"Use double or single quotes"</li>
                                        <li>"Escape internal quotes"</li>
                                    </ul>
                                </div>

                                <div class="reference-card">
                                    <h4>"Variables"</h4>
                                    <ul>
                                        <li>"Variables don't need quotes"</li>
                                        <li>"Can mix with quoted text"</li>
                                        <li>"Keep simple expressions outside quotes"</li>
                                    </ul>
                                </div>
                            </div>
                        </section>

                        <section class="section">
                            <h2 class="section-title">"📝 Practice Exercise"</h2>
                            <div class="exercise-box">
                                <h4>"🏋️ Quote This Template"</h4>
                                <p>"Take this template and add all the necessary quotes:"</p>

                                <div class="exercise-template">
                                    <h5>"Your Task:"</h5>
                                    <pre class="code-block">{"pub fn user_card(name: &str, email: &str, role: &str) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div class=user-card>"}
    {"            <h2>name</h2>"}
    {"            <p>email</p>"}
    {"            <span class=role-badge>role</span>"}
    {"            <a href=/users/name/edit>Edit Profile</a>"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                                </div>

                                <div class="exercise-instructions">
                                    <h5>"Instructions:"</h5>
                                    <ul>
                                        <li>"Quote all text content"</li>
                                        <li>"Quote all HTML attributes"</li>
                                        <li>"Make sure variable references work correctly"</li>
                                    </ul>
                                </div>

                                <div class="exercise-hint">
                                    <details>
                                        <summary>"💡 Need a hint?"</summary>
                                        <p>"Remember: Text goes in quotes, attributes go in quotes, variables don't. Think about each line individually."</p>
                                    </details>
                                </div>
                            </div>
                        </section>

                        <nav class="nav-buttons">
                            <a href="/lesson-1" class="btn btn-secondary">"← Previous: Getting Started"</a>
                            <a href="/lesson-3" class="btn">"Next: CSS Integration Basics →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson2_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson2()))
}
