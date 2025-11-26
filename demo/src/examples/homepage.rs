use axum::response::{Html, IntoResponse};
use azumi::html;

/// Comprehensive homepage showcasing all Azumi examples and capabilities
pub fn homepage() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Azumi 2.0 - Complete Example Library"</title>
                <style src="/static/educational_homepage.css" />
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" />
            </head>
            <body>
                <div class="page-container">
                    <!-- Hero Section -->
                    <header class="hero">
                        <div class="hero-content">
                            <h1>"⚡ Azumi 2.0"</h1>
                            <p class="tagline">"Complete Example Library - From Basics to Real-World Apps"</p>
                            <p class="subtitle">"25+ interactive examples • 6 categories • Production-ready patterns"</p>
                        </div>
                    </header>

                    <!-- Quick Navigation -->
                    <nav class="quick-nav">
                        <div class="nav-categories">
                            <a href="#fundamentals" class="nav-category">
                                <i class="fas fa-graduation-cap"></i>
                                <span>"Fundamentals"</span>
                            </a>
                            <a href="#components" class="nav-category">
                                <i class="fas fa-cube"></i>
                                <span>"Components"</span>
                            </a>
                            <a href="#control-flow" class="nav-category">
                                <i class="fas fa-code-branch"></i>
                                <span>"Control Flow"</span>
                            </a>
                            <a href="#pattern-matching" class="nav-category">
                                <i class="fas fa-puzzle-piece"></i>
                                <span>"Pattern Matching"</span>
                            </a>
                            <a href="#data-processing" class="nav-category">
                                <i class="fas fa-database"></i>
                                <span>"Data Processing"</span>
                            </a>
                            <a href="#real-world" class="nav-category">
                                <i class="fas fa-rocket"></i>
                                <span>"Real-World Apps"</span>
                            </a>
                        </div>
                    </nav>

                    <main class="examples-container">
                        <!-- Section 1: Fundamentals -->
                        <section id="fundamentals" class="section">
                            <div class="section-header">
                                <div class="section-badge fundamentals">"Fundamentals"</div>
                                <h2>"🎓 Learn the Basics"</h2>
                                <p>"Start here: Essential syntax, rules, and core concepts"</p>
                                <span class="section-duration">"15 minutes"</span>
                            </div>

                            <div class="examples-grid">
                                <a href="/lesson-1" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-quote-left"></i>
                                        <h4>"Text & Quotes"</h4>
                                        <span class="time">"3 min"</span>
                                    </div>
                                    <p>"Learn mandatory quoting rules and basic text rendering"</p>
                                    <div class="skills">
                                        <span>"Text"</span>
                                        <span>"Quoting"</span>
                                        <span>"Syntax"</span>
                                    </div>
                                </a>

                                <a href="/lesson-2" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-tags"></i>
                                        <h4>"Attributes"</h4>
                                        <span class="time">"4 min"</span>
                                    </div>
                                    <p>"Master attribute syntax and HTML element structure"</p>
                                    <div class="skills">
                                        <span>"Attributes"</span>
                                        <span>"HTML Structure"</span>
                                    </div>
                                </a>

                                <a href="/lesson-3" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-code"></i>
                                        <h4>"Expressions"</h4>
                                        <span class="time">"4 min"</span>
                                    </div>
                                    <p>"Insert Rust variables and expressions into templates"</p>
                                    <div class="skills">
                                        <span>"Variables"</span>
                                        <span>"Interpolation"</span>
                                    </div>
                                </a>

                                <a href="/lesson-4" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-palette"></i>
                                        <h4>"Basic Styling"</h4>
                                        <span class="time">"4 min"</span>
                                    </div>
                                    <p>"Add external CSS and understand automatic scoping"</p>
                                    <div class="skills">
                                        <span>"CSS"</span>
                                        <span>"Styling"</span>
                                        <span>"Scoping"</span>
                                    </div>
                                </a>
                            </div>
                        </section>

                        <!-- Section 2: Components -->
                        <section id="components" class="section">
                            <div class="section-header">
                                <div class="section-badge components">"Components"</div>
                                <h2>"🧩 Build Reusable Components"</h2>
                                <p>"Create modular, type-safe UI components with props and composition"</p>
                                <span class="section-duration">"25 minutes"</span>
                            </div>

                            <div class="examples-grid">
                                <a href="/components" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-cube"></i>
                                        <h4>"Basic Components"</h4>
                                        <span class="time">"5 min"</span>
                                    </div>
                                    <p>"Create your first reusable component function"</p>
                                    <div class="skills">
                                        <span>"Functions"</span>
                                        <span>"Components"</span>
                                    </div>
                                </a>

                                <a href="/components" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-cogs"></i>
                                        <h4>"Component Props"</h4>
                                        <span class="time">"8 min"</span>
                                    </div>
                                    <p>"Pass data to components with type-safe props"</p>
                                    <div class="skills">
                                        <span>"Props"</span>
                                        <span>"Type Safety"</span>
                                    </div>
                                </a>

                                <a href="/hello" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-magic"></i>
                                        <h4>"Default Props"</h4>
                                        <span class="time">"4 min"</span>
                                    </div>
                                    <p>"Make components flexible with optional parameters"</p>
                                    <div class="skills">
                                        <span>"Optional Props"</span>
                                        <span>"Defaults"</span>
                                    </div>
                                </a>

                                <a href="/layouts" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-layer-group"></i>
                                        <h4>"Component Nesting"</h4>
                                        <span class="time">"3 min"</span>
                                    </div>
                                    <p>"Compose complex UIs by nesting components"</p>
                                    <div class="skills">
                                        <span>"Composition"</span>
                                        <span>"Nesting"</span>
                                    </div>
                                </a>

                                <a href="/advanced-components" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-star"></i>
                                        <h4>"Advanced Components"</h4>
                                        <span class="time">"5 min"</span>
                                    </div>
                                    <p>"Modals, forms with validation, and complex nesting"</p>
                                    <div class="skills">
                                        <span>"Modals"</span>
                                        <span>"Validation"</span>
                                        <span>"Advanced Patterns"</span>
                                    </div>
                                </a>

                                <a href="/ui-library" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-palette"></i>
                                        <h4>"UI Component Library"</h4>
                                        <span class="time">"8 min"</span>
                                    </div>
                                    <p>"Complete UI library with buttons, badges, progress bars"</p>
                                    <div class="skills">
                                        <span>"UI Components"</span>
                                        <span>"Design System"</span>
                                        <span>"Reusable Patterns"</span>
                                    </div>
                                </a>
                            </div>
                        </section>

                        <!-- Section 3: Control Flow -->
                        <section id="control-flow" class="section">
                            <div class="section-header">
                                <div class="section-badge control-flow">"Control Flow"</div>
                                <h2>"🔀 Add Logic to Templates"</h2>
                                <p>"Conditional rendering, loops, and dynamic content"</p>
                                <span class="section-duration">"20 minutes"</span>
                            </div>

                            <div class="examples-grid">
                                <a href="/control-flow" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-question-circle"></i>
                                        <h4>"If/Else Conditions"</h4>
                                        <span class="time">"4 min"</span>
                                    </div>
                                    <p>"Conditional rendering with @if and @else"</p>
                                    <div class="skills">
                                        <span>"Conditionals"</span>
                                        <span>"Logic"</span>
                                    </div>
                                </a>

                                <a href="/control-flow" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-list"></i>
                                        <h4>"For Loops"</h4>
                                        <span class="time">"5 min"</span>
                                    </div>
                                    <p>"Render lists and collections with @for"</p>
                                    <div class="skills">
                                        <span>"Loops"</span>
                                        <span>"Collections"</span>
                                    </div>
                                </a>

                                <a href="/control-flow" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-random"></i>
                                        <h4>"Match Expressions"</h4>
                                        <span class="time">"4 min"</span>
                                    </div>
                                    <p>"Pattern matching for complex conditional logic"</p>
                                    <div class="skills">
                                        <span>"Pattern Matching"</span>
                                        <span>"Enum Handling"</span>
                                    </div>
                                </a>

                                <a href="/control-flow" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-variable"></i>
                                        <h4>"Variable Binding"</h4>
                                        <span class="time">"4 min"</span>
                                    </div>
                                    <p>"Local variables with @let statements"</p>
                                    <div class="skills">
                                        <span>"Variables"</span>
                                        <span>"Binding"</span>
                                        <span>"Scope"</span>
                                    </div>
                                </a>

                                <a href="/forms" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-edit"></i>
                                        <h4>"Form Handling"</h4>
                                        <span class="time">"3 min"</span>
                                    </div>
                                    <p>"Build interactive forms with validation"</p>
                                    <div class="skills">
                                        <span>"Forms"</span>
                                        <span>"Validation"</span>
                                        <span>"Input Handling"</span>
                                    </div>
                                </a>
                            </div>
                        </section>

                        <!-- Section 4: Pattern Matching -->
                        <section id="pattern-matching" class="section">
                            <div class="section-header">
                                <div class="section-badge pattern-matching">"Pattern Matching"</div>
                                <h2>"🎭 Advanced Pattern Matching"</h2>
                                <p>"Master Rust's powerful pattern matching in templates"</p>
                                <span class="section-duration">"30 minutes"</span>
                            </div>

                            <div class="examples-grid">
                                <a href="/let-examples" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-target"></i>
                                        <h4>"@let + @match Mastery"</h4>
                                        <span class="time">"12 min"</span>
                                    </div>
                                    <p>"Comprehensive @let examples with heavy @match usage"</p>
                                    <div class="skills">
                                        <span>"@let"</span>
                                        <span>"@match"</span>
                                        <span>"Variable Binding"</span>
                                        <span>"Pattern Matching"</span>
                                    </div>
                                </a>

                                <a href="/advanced-patterns" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-puzzle-piece"></i>
                                        <h4>"Complex Patterns"</h4>
                                        <span class="time">"10 min"</span>
                                    </div>
                                    <p>"Struct & enum matching, guards, collection patterns"</p>
                                    <div class="skills">
                                        <span>"Complex Patterns"</span>
                                        <span>"Guard Conditions"</span>
                                        <span>"Destructuring"</span>
                                        <span>"Advanced Matching"</span>
                                    </div>
                                </a>

                                <a href="/control-flow" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-layer-group"></i>
                                        <h4>"Nested Matching"</h4>
                                        <span class="time">"5 min"</span>
                                    </div>
                                    <p>"Multi-level pattern matching with complex conditions"</p>
                                    <div class="skills">
                                        <span>"Nested Patterns"</span>
                                        <span>"Complex Logic"</span>
                                    </div>
                                </a>

                                <a href="/advanced-patterns" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-check-circle"></i>
                                        <h4>"Result & Option Handling"</h4>
                                        <span class="time">"3 min"</span>
                                    </div>
                                    <p>"Safe handling of Result and Option types"</p>
                                    <div class="skills">
                                        <span>"Result Types"</span>
                                        <span>"Option Types"</span>
                                        <span>"Error Handling"</span>
                                    </div>
                                </a>
                            </div>
                        </section>

                        <!-- Section 5: Data Processing -->
                        <section id="data-processing" class="section">
                            <div class="section-header">
                                <div class="section-badge data-processing">"Data Processing"</div>
                                <h2>"📊 Handle Real Data"</h2>
                                <p>"Transform, filter, and analyze data with Azumi"</p>
                                <span class="section-duration">"25 minutes"</span>
                            </div>

                            <div class="examples-grid">
                                <a href="/data-processing" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-chart-line"></i>
                                        <h4>"Sales Analysis"</h4>
                                        <span class="time">"8 min"</span>
                                    </div>
                                    <p>"Real-world data processing with statistics and charts"</p>
                                    <div class="skills">
                                        <span>"Data Analysis"</span>
                                        <span>"Statistics"</span>
                                        <span>"Charts"</span>
                                    </div>
                                </a>

                                <a href="/data-processing" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-search"></i>
                                        <h4>"Search & Filter Engine"</h4>
                                        <span class="time">"7 min"</span>
                                    </div>
                                    <p>"Advanced filtering with multiple criteria and search"</p>
                                    <div class="skills">
                                        <span>"Search"</span>
                                        <span>"Filtering"</span>
                                        <span>"Collections"</span>
                                    </div>
                                </a>

                                <a href="/data-processing" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-cogs"></i>
                                        <h4>"Data Pipeline"</h4>
                                        <span class="time">"6 min"</span>
                                    </div>
                                    <p>"Multi-step data transformation and validation"</p>
                                    <div class="skills">
                                        <span>"Data Pipeline"</span>
                                        <span>"Transformation"</span>
                                        <span>"Validation"</span>
                                    </div>
                                </a>

                                <a href="/data-processing" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-tachometer-alt"></i>
                                        <h4>"Live Statistics"</h4>
                                        <span class="time">"4 min"</span>
                                    </div>
                                    <p>"Real-time metrics with dynamic updates and charts"</p>
                                    <div class="skills">
                                        <span>"Real-time Data"</span>
                                        <span>"Metrics"</span>
                                        <span>"Dynamic Updates"</span>
                                    </div>
                                </a>
                            </div>
                        </section>

                        <!-- Section 6: Real-World Applications -->
                        <section id="real-world" class="section">
                            <div class="section-header">
                                <div class="section-badge real-world">"Real-World Apps"</div>
                                <h2>"🚀 Production-Ready Applications"</h2>
                                <p>"Complete applications showcasing real-world patterns"</p>
                                <span class="section-duration">"35 minutes"</span>
                            </div>

                            <div class="examples-grid">
                                <a href="/real-world-apps" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-shopping-cart"></i>
                                        <h4>"E-commerce Catalog"</h4>
                                        <span class="time">"12 min"</span>
                                    </div>
                                    <p>"Full product catalog with search, filters, and cart"</p>
                                    <div class="skills">
                                        <span>"E-commerce"</span>
                                        <span>"Product Catalog"</span>
                                        <span>"Shopping Cart"</span>
                                        <span>"Search & Filter"</span>
                                    </div>
                                </a>

                                <a href="/real-world-apps" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-chart-bar"></i>
                                        <h4>"Analytics Dashboard"</h4>
                                        <span class="time">"10 min"</span>
                                    </div>
                                    <p>"Real-time analytics with charts, KPIs, and live data"</p>
                                    <div class="skills">
                                        <span>"Analytics"</span>
                                        <span>"Charts"</span>
                                        <span>"KPIs"</span>
                                        <span>"Live Data"</span>
                                    </div>
                                </a>

                                <a href="/real-world-apps" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-share-alt"></i>
                                        <h4>"Social Media Feed"</h4>
                                        <span class="time">"8 min"</span>
                                    </div>
                                    <p>"Interactive social feed with likes, comments, and sharing"</p>
                                    <div class="skills">
                                        <span>"Social Media"</span>
                                        <span>"User Interaction"</span>
                                        <span>"Real-time Updates"</span>
                                    </div>
                                </a>

                                <a href="/real-world-apps" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-tasks"></i>
                                        <h4>"Project Management Board"</h4>
                                        <span class="time">"5 min"</span>
                                    </div>
                                    <p>"Kanban-style project management with drag-and-drop"</p>
                                    <div class="skills">
                                        <span>"Project Management"</span>
                                        <span>"Kanban"</span>
                                        <span>"Task Management"</span>
                                    </div>
                                </a>

                                <a href="/dashboard" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-chart-pie"></i>
                                        <h4>"Business Dashboard"</h4>
                                        <span class="time">"5 min"</span>
                                    </div>
                                    <p>"Complete business dashboard with multiple data views"</p>
                                    <div class="skills">
                                        <span>"Dashboard"</span>
                                        <span>"Business Intelligence"</span>
                                        <span>"Data Visualization"</span>
                                    </div>
                                </a>

                                <a href="/htmx-todo" class="example-card">
                                    <div class="example-header">
                                        <i class="fas fa-bolt"></i>
                                        <h4>"HTMX Todo App"</h4>
                                        <span class="time">"5 min"</span>
                                    </div>
                                    <p>"Interactive todo app with HTMX and server-side rendering"</p>
                                    <div class="skills">
                                        <span>"HTMX"</span>
                                        <span>"Todo App"</span>
                                        <span>"Interactivity"</span>
                                        <span>"Server-side"</span>
                                    </div>
                                </a>
                            </div>
                        </section>
                    </main>

                    <!-- Summary Section -->
                    <section class="summary-section">
                        <h2>"🎯 What You'll Master"</h2>
                        <div class="capabilities-grid">
                            <div class="capability">
                                <i class="fas fa-shield-alt"></i>
                                <h4>"Type Safety"</h4>
                                <p>"Compile-time checks for all templates"</p>
                            </div>
                            <div class="capability">
                                <i class="fas fa-magic"></i>
                                <h4>"Pattern Matching"</h4>
                                <p>"Rust's powerful matching in HTML"</p>
                            </div>
                            <div class="capability">
                                <i class="fas fa-cube"></i>
                                <h4>"Component System"</h4>
                                <p>"Reusable, composable UI components"</p>
                            </div>
                            <div class="capability">
                                <i class="fas fa-database"></i>
                                <h4>"Data Processing"</h4>
                                <p>"Transform and display complex data"</p>
                            </div>
                            <div class="capability">
                                <i class="fas fa-rocket"></i>
                                <h4>"Real Applications"</h4>
                                <p>"Production-ready patterns and examples"</p>
                            </div>
                            <div class="capability">
                                <i class="fas fa-paint-brush"></i>
                                <h4>"CSS Scoping"</h4>
                                <p>"Automatic CSS scoping and isolation"</p>
                            </div>
                        </div>
                    </section>

                    <!-- Footer -->
                    <footer class="footer">
                        <div class="footer-content">
                            <h3>"🚀 Ready to Build?"</h3>
                            <p>"25+ examples covering everything from basics to real-world applications"</p>
                            <div class="footer-stats">
                                <span>"📚 25 Examples"</span>
                                <span>"⚡ Type Safe"</span>
                                <span>"🎯 Production Ready"</span>
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
