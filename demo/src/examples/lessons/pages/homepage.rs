use axum::response::{Html, IntoResponse};
use azumi::html;

pub fn home_page() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Azumi Lessons - Learn Type-Safe HTML"</title>
                <style src="/static/pages/homepage.css" />
                <style src="/static/global.css" />
            </head>
            <body>
                <div class="page-container">
                    <header class="hero">
                        <h1 class="hero-title">"Azumi"</h1>
                        <p class="hero-subtitle">"Type-Safe, Compile-Time Validated HTML Templates for Rust"</p>
                    </header>

                    <section>
                        <h2 class="curriculum-title">"📚 Complete 20-Lesson Curriculum"</h2>
                        <p>"Master Azumi through 20 comprehensive lessons, featuring control flow, components, JavaScript integration, and production patterns:"</p>

                        <!-- Phase 1: Foundation Building -->
                        <div class="phase-header">
                            <h3 class="phase-title">"🏗️ Phase 1: Foundation Building (Lessons 1-5)"</h3>
                            <p class="phase-description">"Master the core concepts: getting started, quoting, CSS basics, scoping, and data binding."</p>
                        </div>
                        <div class="lessons-grid">
                            <a href="/lesson-1" class="lesson-card foundation">
                                <div class="lesson-card-number">"Foundation"</div>
                                <h3 class="lesson-card-title">"Getting Started with Azumi"</h3>
                                <p class="lesson-card-desc">"What is type-safe HTML? Your first template and why strict typing matters."</p>
                            </a>
                            <a href="/lesson-2" class="lesson-card foundation">
                                <div class="lesson-card-number">"Foundation"</div>
                                <h3 class="lesson-card-title">"The Quoting Fundamentals"</h3>
                                <p class="lesson-card-desc">"Master the core quoting rules that make Azumi type-safe and prevent bugs."</p>
                            </a>
                            <a href="/lesson-3" class="lesson-card foundation">
                                <div class="lesson-card-number">"Foundation"</div>
                                <h3 class="lesson-card-title">"CSS Integration Basics"</h3>
                                <p class="lesson-card-desc">"Add styles to templates and understand CSS validation concepts."</p>
                            </a>
                            <a href="/lesson-4" class="lesson-card foundation">
                                <div class="lesson-card-number">"Foundation"</div>
                                <h3 class="lesson-card-title">"Understanding Scoping"</h3>
                                <p class="lesson-card-desc">"Learn automatic CSS scoping and preventing style conflicts."</p>
                            </a>
                            <a href="/lesson-5" class="lesson-card foundation">
                                <div class="lesson-card-number">"Foundation"</div>
                                <h3 class="lesson-card-title">"Variables & Data Binding"</h3>
                                <p class="lesson-card-desc">"Pass data to templates and understand type safety in practice."</p>
                            </a>
                        </div>

                        <!-- Phase 2: Control Flow Mastery -->
                        <div class="phase-header">
                            <h3 class="phase-title">"⚡ Phase 2: Control Flow Mastery (Lessons 6-10)"</h3>
                            <p class="phase-description">"Master Azumi's control flow features: @if, @for, @match, @let, and advanced patterns."</p>
                        </div>
                        <div class="lessons-grid">
                            <a href="/lesson-6" class="lesson-card core">
                                <div class="lesson-card-number">"Control Flow"</div>
                                <h3 class="lesson-card-title">"Conditional Content with @if"</h3>
                                <p class="lesson-card-desc">"Build dynamic UIs with @if, else, and complex boolean expressions."</p>
                            </a>
                            <a href="/lesson-7" class="lesson-card core">
                                <div class="lesson-card-number">"Control Flow"</div>
                                <h3 class="lesson-card-title">"Loops & Iteration with @for"</h3>
                                <p class="lesson-card-desc">"Create dynamic lists and tables with @for loops and index tracking."</p>
                            </a>
                            <a href="/lesson-8" class="lesson-card core">
                                <div class="lesson-card-number">"Control Flow"</div>
                                <h3 class="lesson-card-title">"Pattern Matching with @match"</h3>
                                <p class="lesson-card-desc">"Powerful pattern matching with enums and complex conditional logic."</p>
                            </a>
                            <a href="/lesson-9" class="lesson-card core">
                                <div class="lesson-card-number">"Control Flow"</div>
                                <h3 class="lesson-card-title">"Local Variables with @let"</h3>
                                <p class="lesson-card-desc">"Create computed values and complex data transformations in templates."</p>
                            </a>
                            <a href="/lesson-10" class="lesson-card core">
                                <div class="lesson-card-number">"Control Flow"</div>
                                <h3 class="lesson-card-title">"Advanced Control Flow Patterns"</h3>
                                <p class="lesson-card-desc">"Combine @if, @for, @match, @let for sophisticated template logic."</p>
                            </a>
                        </div>

                        <!-- Phase 3: Component Architecture -->
                        <div class="phase-header">
                            <h3 class="phase-title">"🏗️ Phase 3: Component Architecture (Lessons 11-15)"</h3>
                            <p class="phase-description">"Build reusable components: props, composition, state management, and advanced patterns."</p>
                        </div>
                        <div class="lessons-grid">
                            <a href="/lesson-11" class="lesson-card advanced">
                                <div class="lesson-card-number">"Components"</div>
                                <h3 class="lesson-card-title">"Introduction to Components"</h3>
                                <p class="lesson-card-desc">"Learn component concepts and build your first reusable components."</p>
                            </a>
                            <a href="/lesson-12" class="lesson-card advanced">
                                <div class="lesson-card-number">"Components"</div>
                                <h3 class="lesson-card-title">"Component Props & Data Flow"</h3>
                                <p class="lesson-card-desc">"Type-safe props, interfaces, default values, and validation patterns."</p>
                            </a>
                            <a href="/lesson-13" class="lesson-card advanced">
                                <div class="lesson-card-number">"Components"</div>
                                <h3 class="lesson-card-title">"Component Composition"</h3>
                                <p class="lesson-card-desc">"Parent-child relationships, nesting, and composition patterns."</p>
                            </a>
                            <a href="/lesson-14" class="lesson-card advanced">
                                <div class="lesson-card-number">"Components"</div>
                                <h3 class="lesson-card-title">"Component State Management"</h3>
                                <p class="lesson-card-desc">"Internal state, updates, re-rendering, and lifecycle management."</p>
                            </a>
                            <a href="/lesson-15" class="lesson-card advanced">
                                <div class="lesson-card-number">"Components"</div>
                                <h3 class="lesson-card-title">"Advanced Component Patterns"</h3>
                                <p class="lesson-card-desc">"Higher-order components, render props, and composition techniques."</p>
                            </a>
                        </div>

                        <!-- Phase 4: JavaScript & Interactivity -->
                        <div class="phase-header">
                            <h3 class="phase-title">"🌐 Phase 4: JavaScript & Interactivity (Lessons 16-18)"</h3>
                            <p class="phase-description">"Integrate JavaScript safely and build interactive applications with HTMX."</p>
                        </div>
                        <div class="lessons-grid">
                            <a href="/lesson-16" class="lesson-card mastery">
                                <div class="lesson-card-number">"JavaScript"</div>
                                <h3 class="lesson-card-title">"JavaScript Integration"</h3>
                                <p class="lesson-card-desc">"Load external scripts safely and interact with DOM elements."</p>
                            </a>
                            <a href="/lesson-17" class="lesson-card mastery">
                                <div class="lesson-card-number">"JavaScript"</div>
                                <h3 class="lesson-card-title">"Interactive Components"</h3>
                                <p class="lesson-card-desc">"Build interactive UIs with state synchronization and form handling."</p>
                            </a>
                            <a href="/lesson-18" class="lesson-card mastery">
                                <div class="lesson-card-number">"JavaScript"</div>
                                <h3 class="lesson-card-title">"HTMX & Server Integration"</h3>
                                <p class="lesson-card-desc">"Progressive enhancement with HTMX and dynamic content loading."</p>
                            </a>
                        </div>

                        <!-- Phase 5: Production & Advanced -->
                        <div class="phase-header">
                            <h3 class="phase-title">"🚀 Phase 5: Production & Advanced (Lessons 19-20)"</h3>
                            <p class="phase-description">"Build production-ready applications with layout systems and deployment patterns."</p>
                        </div>
                        <div class="lessons-grid">
                            <a href="/lesson-19" class="lesson-card mastery">
                                <div class="lesson-card-number">"Production"</div>
                                <h3 class="lesson-card-title">"Layout Systems & Architecture"</h3>
                                <p class="lesson-card-desc">"Multi-page applications, navigation systems, and scalable architecture."</p>
                            </a>
                            <a href="/lesson-20" class="lesson-card mastery">
                                <div class="lesson-card-number">"Production"</div>
                                <h3 class="lesson-card-title">"Production Patterns & Deployment"</h3>
                                <p class="lesson-card-desc">"Error handling, performance optimization, testing, and deployment."</p>
                            </a>
                        </div>
                    </section>

                    <section class="marketing-section">
                        <h2 class="section-title">"✨ Why Azumi?"</h2>
                        <div class="features-grid">
                            <div class="feature-box">
                                <div class="feature-icon">"🔍"</div>
                                <h3 class="feature-title">"Compile-Time Validation"</h3>
                                <p class="feature-desc">"Every CSS class checked at compile time with exact error locations."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"🎨"</div>
                                <h3 class="feature-title">"Automatic Scoping"</h3>
                                <p class="feature-desc">"CSS is scoped to components automatically. No style leakage."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"⚡"</div>
                                <h3 class="feature-title">"Zero Runtime Cost"</h3>
                                <p class="feature-desc">"Everything happens at compile time. Pure Rust performance."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"🛡️"</div>
                                <h3 class="feature-title">"Strict by Design"</h3>
                                <p class="feature-desc">"Mandatory quoting prevents bugs. No dead CSS allowed."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"🏗️"</div>
                                <h3 class="feature-title">"Component Architecture"</h3>
                                <p class="feature-desc">"Reusable components with type-safe props and clean composition."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"🌐"</div>
                                <h3 class="feature-title">"Modern Web Standards"</h3>
                                <p class="feature-desc">"Works seamlessly with HTMX, Axum, and the Rust ecosystem."</p>
                            </div>
                        </div>
                    </section>

                    <section class="marketing-section">
                        <h2 class="section-title">"🎯 Learning Path"</h2>
                        <div class="learning-path">
                            <div class="path-step foundation">
                                <div class="step-number">"1-5"</div>
                                <div class="step-content">
                                    <h4>"Foundation Building"</h4>
                                    <p>"Master core concepts: getting started, quoting, CSS basics, scoping, and data binding."</p>
                                </div>
                            </div>
                            <div class="path-arrow">"→"</div>
                            <div class="path-step core">
                                <div class="step-number">"6-10"</div>
                                <div class="step-content">
                                    <h4>"Control Flow Mastery"</h4>
                                    <p>"Learn @if, @for, @match, @let, and advanced template logic patterns."</p>
                                </div>
                            </div>
                            <div class="path-arrow">"→"</div>
                            <div class="path-step advanced">
                                <div class="step-number">"11-15"</div>
                                <div class="step-content">
                                    <h4>"Component Architecture"</h4>
                                    <p>"Build reusable components with props, composition, and state management."</p>
                                </div>
                            </div>
                            <div class="path-arrow">"→"</div>
                            <div class="path-step mastery">
                                <div class="step-number">"16-20"</div>
                                <div class="step-content">
                                    <h4>"JavaScript & Production"</h4>
                                    <p>"Integrate JavaScript safely and build production-ready applications."</p>
                                </div>
                            </div>
                        </div>
                    </section>

                    <section class="marketing-section">
                        <h2 class="section-title">"💡 What Makes This Different"</h2>
                        <div class="features-grid">
                            <div class="feature-box">
                                <div class="feature-icon">"🧱"</div>
                                <h3 class="feature-title">"Progressive Complexity"</h3>
                                <p class="feature-desc">"20 carefully crafted lessons that build on each other systematically."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"🎮"</div>
                                <h3 class="feature-title">"Control Flow Focus"</h3>
                                <p class="feature-desc">"Deep coverage of @if, @for, @match, @let with practical examples."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"🧩"</div>
                                <h3 class="feature-title">"Component Mastery"</h3>
                                <p class="feature-desc">"5 dedicated lessons on building reusable, composable components."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"⚙️"</div>
                                <h3 class="feature-title">"JavaScript Integration"</h3>
                                <p class="feature-desc">"Learn how to safely combine Azumi with JavaScript and HTMX."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"💻"</div>
                                <h3 class="feature-title">"Hands-On Practice"</h3>
                                <p class="feature-desc">"Every concept includes interactive examples and coding exercises."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"🎯"</div>
                                <h3 class="feature-title">"Production Ready"</h3>
                                <p class="feature-desc">"Real-world patterns for building scalable web applications."</p>
                            </div>
                        </div>
                    </section>
                </div>
            </body>
        </html>
    }
}
// Handler for Axum
pub async fn homepage_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&home_page()))
}
