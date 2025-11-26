use axum::response::{Html, IntoResponse};
use azumi::html;

/// Professional homepage with clear learning progression
pub fn homepage() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Azumi 2.0 - Learn by Examples"</title>
                <style src="/static/homepage.css" />
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" />
            </head>
            <body>
                <div class="page-container">
                    <!-- Hero Section -->
                    <header class="hero">
                        <div class="hero-content">
                            <h1>"⚡ Azumi 2.0"</h1>
                            <p class="tagline">"Learn Type-Safe HTML Templates for Rust"</p>
                            <p class="subtitle">"16 interactive examples • From beginner to advanced • 45 minutes total"</p>
                        </div>
                    </header>

                    <!-- Learning Path -->
                    <main class="learning-path">
                        <div class="path-intro">
                            <h2>"🎯 Learning Path"</h2>
                            <p>"Master Azumi step-by-step with focused, hands-on examples. Each lesson teaches one concept clearly."</p>
                        </div>

                        <!-- Level 1: Basics -->
                        <section class="level">
                            <div class="level-header">
                                <div class="level-badge level-1">"Level 1"</div>
                                <h3>"🟢 Basics"</h3>
                                <span class="duration">"15 minutes"</span>
                            </div>
                            <p class="level-description">"Essential syntax rules and fundamental concepts"</p>
                            
                            <div class="examples-grid">
                                <a href="/lesson-1" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-quote-left"></i>
                                        <h4>"1. Text & Quotes"</h4>
                                        <span class="time">"3 min"</span>
                                    </div>
                                    <p>"Learn mandatory quoting rules and basic text rendering"</p>
                                    <div class="skills">"Text, Quoting, Syntax"</div>
                                </a>

                                <a href="/lesson-2" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-tags"></i>
                                        <h4>"2. Attributes"</h4>
                                        <span class="time">"4 min"</span>
                                    </div>
                                    <p>"Master attribute syntax and HTML element structure"</p>
                                    <div class="skills">"Attributes, HTML Structure"</div>
                                </a>

                                <a href="/lesson-3" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-code"></i>
                                        <h4>"3. Expressions"</h4>
                                        <span class="time">"4 min"</span>
                                    </div>
                                    <p>"Insert Rust variables and expressions into templates"</p>
                                    <div class="skills">"Variables, Interpolation"</div>
                                </a>

                                <a href="/lesson-4" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-palette"></i>
                                        <h4>"4. Basic Styling"</h4>
                                        <span class="time">"4 min"</span>
                                    </div>
                                    <p>"Add external CSS and understand automatic scoping"</p>
                                    <div class="skills">"CSS, Styling, Scoping"</div>
                                </a>
                            </div>
                        </section>

                        <!-- Level 2: Components -->
                        <section class="level">
                            <div class="level-header">
                                <div class="level-badge level-2">"Level 2"</div>
                                <h3>"🧩 Components"</h3>
                                <span class="duration">"20 minutes"</span>
                            </div>
                            <p class="level-description">"Build reusable components with props and composition"</p>
                            
                            <div class="examples-grid">
                                <a href="/lesson-5" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-cube"></i>
                                        <h4>"5. Simple Components"</h4>
                                        <span class="time">"5 min"</span>
                                    </div>
                                    <p>"Create your first reusable component function"</p>
                                    <div class="skills">"Functions, Components"</div>
                                </a>

                                <a href="/lesson-6" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-cogs"></i>
                                        <h4>"6. Component Props"</h4>
                                        <span class="time">"8 min"</span>
                                    </div>
                                    <p>"Pass data to components with type-safe props"</p>
                                    <div class="skills">"Props, Type Safety"</div>
                                </a>

                                <a href="/lesson-7" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-magic"></i>
                                        <h4>"7. Default Props"</h4>
                                        <span class="time">"4 min"</span>
                                    </div>
                                    <p>"Make components flexible with optional parameters"</p>
                                    <div class="skills">"Optional Props, Defaults"</div>
                                </a>

                                <a href="/lesson-8" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-layer-group"></i>
                                        <h4>"8. Component Nesting"</h4>
                                        <span class="time">"3 min"</span>
                                    </div>
                                    <p>"Compose complex UIs by nesting components"</p>
                                    <div class="skills">"Composition, Nesting"</div>
                                </a>
                            </div>
                        </section>

                        <!-- Level 3: Control Flow -->
                        <section class="level">
                            <div class="level-header">
                                <div class="level-badge level-3">"Level 3"</div>
                                <h3>"🔀 Control Flow"</h3>
                                <span class="duration">"15 minutes"</span>
                            </div>
                            <p class="level-description">"Add logic to your templates with Rust control flow"</p>
                            
                            <div class="examples-grid">
                                <a href="/lesson-9" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-question-circle"></i>
                                        <h4>"9. If/Else"</h4>
                                        <span class="time">"4 min"</span>
                                    </div>
                                    <p>"Conditional rendering with @if and @else"</p>
                                    <div class="skills">"Conditionals, Logic"</div>
                                </a>

                                <a href="/lesson-10" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-list"></i>
                                        <h4>"10. For Loops"</h4>
                                        <span class="time">"5 min"</span>
                                    </div>
                                    <p>"Render lists and collections with @for"</p>
                                    <div class="skills">"Loops, Collections"</div>
                                </a>

                                <a href="/lesson-11" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-random"></i>
                                        <h4>"11. Match Expressions"</h4>
                                        <span class="time">"4 min"</span>
                                    </div>
                                    <p>"Pattern matching for complex conditional logic"</p>
                                    <div class="skills">"Pattern Matching"</div>
                                </a>

                                <a href="/lesson-12" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-variable"></i>
                                        <h4>"12. Variable Binding"</h4>
                                        <span class="time">"2 min"</span>
                                    </div>
                                    <p>"Local variables with @let statements"</p>
                                    <div class="skills">"Variables, Binding"</div>
                                </a>
                            </div>
                        </section>

                        <!-- Level 4: Advanced -->
                        <section class="level">
                            <div class="level-header">
                                <div class="level-badge level-4">"Level 4"</div>
                                <h3>"🚀 Advanced"</h3>
                                <span class="duration">"25 minutes"</span>
                            </div>
                            <p class="level-description">"Real-world patterns and interactive features"</p>
                            
                            <div class="examples-grid">
                                <a href="/lesson-13" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-sitemap"></i>
                                        <h4>"13. Layout Components"</h4>
                                        <span class="time">"8 min"</span>
                                    </div>
                                    <p>"Create consistent layouts with component composition"</p>
                                    <div class="skills">"Layouts, Architecture"</div>
                                </a>

                                <a href="/lesson-14" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-edit"></i>
                                        <h4>"14. Form Handling"</h4>
                                        <span class="time">"10 min"</span>
                                    </div>
                                    <p>"Build interactive forms with validation"</p>
                                    <div class="skills">"Forms, Validation"</div>
                                </a>

                                <a href="/lesson-15" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-bolt"></i>
                                        <h4>"15. HTMX Integration"</h4>
                                        <span class="time">"5 min"</span>
                                    </div>
                                    <p>"Add interactivity without JavaScript"</p>
                                    <div class="skills">"HTMX, Interactivity"</div>
                                </a>

                                <a href="/lesson-16" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-chart-bar"></i>
                                        <h4>"16. Dashboard Example"</h4>
                                        <span class="time">"2 min"</span>
                                    </div>
                                    <p>"Put it all together in a complete application"</p>
                                    <div class="skills">"Integration, Full App"</div>
                                </a>
                            </div>
                        </section>
                    </main>

                    <!-- Footer -->
                    <footer class="footer">
                        <div class="footer-content">
                            <h3>"🎓 What You'll Learn"</h3>
                            <div class="skills-grid">
                                <div class="skill-item">
                                    <i class="fas fa-shield-alt"></i>
                                    <span>"Type Safety"</span>
                                </div>
                                <div class="skill-item">
                                    <i class="fas fa-magic"></i>
                                    <span>"Compile-time Checks"</span>
                                </div>
                                <div class="skill-item">
                                    <i class="fas fa-paint-brush"></i>
                                    <span>"CSS Scoping"</span>
                                </div>
                                <div class="skill-item">
                                    <i class="fas fa-rocket"></i>
                                    <span>"Zero Runtime Overhead"</span>
                                </div>
                            </div>
                            <p class="footer-note">"Built with ❤️ for the Rust community"</p>
                        </div>
                    </footer>
                </div>
            </body>
        </html>
    }
}

pub async fn homepage_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&homepage()))
}