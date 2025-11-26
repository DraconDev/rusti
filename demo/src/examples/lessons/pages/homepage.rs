use axum::response::{Html, IntoResponse};
use azumi::html;

pub fn homepage() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Azumi Lessons - Learn Type-Safe HTML"</title>
                <style src="/static/page/lessons.css" />
                <style src="/static/homepage.css" />
                <style src="/static/global.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="hero">
                        <h1 class="hero-title">"Azumi"</h1>
                        <p class="hero-subtitle">"Type-Safe, Compile-Time Validated HTML Templates for Rust"</p>
                    </header>

                    <section>
                        <h2 class="section-title">"📚 Complete Learning Path"</h2>
                        <p>"Master Azumi through 12 comprehensive lessons, building from fundamentals to advanced production patterns:"</p>

                        <div class="lessons-grid">
                            <a href="/lesson-1" class="lesson-card">
                                <div class="lesson-card-number">"Foundation"</div>
                                <h3 class="lesson-card-title">"Getting Started with Azumi"</h3>
                                <p class="lesson-card-desc">"What is type-safe HTML? Your first template and why strict typing matters."</p>
                            </a>

                            <a href="/lesson-2" class="lesson-card">
                                <div class="lesson-card-number">"Foundation"</div>
                                <h3 class="lesson-card-title">"The Quoting Fundamentals"</h3>
                                <p class="lesson-card-desc">"Master the core quoting rules that make Azumi type-safe and prevent bugs."</p>
                            </a>

                            <a href="/lesson-3" class="lesson-card">
                                <div class="lesson-card-number">"Foundation"</div>
                                <h3 class="lesson-card-title">"CSS Integration Basics"</h3>
                                <p class="lesson-card-desc">"Add styles to templates and understand CSS validation concepts."</p>
                            </a>

                            <a href="/lesson-4" class="lesson-card">
                                <div class="lesson-card-number">"Foundation"</div>
                                <h3 class="lesson-card-title">"Understanding Scoping"</h3>
                                <p class="lesson-card-desc">"Learn automatic CSS scoping and preventing style conflicts."</p>
                            </a>

                            <a href="/lesson-5" class="lesson-card">
                                <div class="lesson-card-number">"Core"</div>
                                <h3 class="lesson-card-title">"Design Tokens & Global Styles"</h3>
                                <p class="lesson-card-desc">"Create consistent design systems with CSS custom properties."</p>
                            </a>

                            <a href="/lesson-6" class="lesson-card">
                                <div class="lesson-card-number">"Core"</div>
                                <h3 class="lesson-card-title">"Dynamic Content with Control Flow"</h3>
                                <p class="lesson-card-desc">"Use Rust control flow directly in your templates for dynamic content."</p>
                            </a>

                            <a href="/lesson-7" class="lesson-card">
                                <div class="lesson-card-number">"Core"</div>
                                <h3 class="lesson-card-title">"Reusable Components"</h3>
                                <p class="lesson-card-desc">"Build maintainable components with props and composition."</p>
                            </a>

                            <a href="/lesson-8" class="lesson-card">
                                <div class="lesson-card-number">"Advanced"</div>
                                <h3 class="lesson-card-title">"Interactive Frontends with HTMX"</h3>
                                <p class="lesson-card-desc">"Build dynamic web apps with server-side rendering and HTMX."</p>
                            </a>

                            <a href="/lesson-9" class="lesson-card">
                                <div class="lesson-card-number">"Advanced"</div>
                                <h3 class="lesson-card-title">"Layout Systems & Architecture"</h3>
                                <p class="lesson-card-desc">"Create scalable layouts and multi-page application structures."</p>
                            </a>

                            <a href="/lesson-10" class="lesson-card">
                                <div class="lesson-card-number">"Advanced"</div>
                                <h3 class="lesson-card-title">"Production Patterns & Real Apps"</h3>
                                <p class="lesson-card-desc">"Deploy real applications with best practices and performance optimization."</p>
                            </a>

                            <a href="/lesson-11" class="lesson-card">
                                <div class="lesson-card-number">"Mastery"</div>
                                <h3 class="lesson-card-title">"Advanced Component Patterns"</h3>
                                <p class="lesson-card-desc">"Higher-order components, state management, and testing strategies."</p>
                            </a>

                            <a href="/lesson-12" class="lesson-card">
                                <div class="lesson-card-number">"Mastery"</div>
                                <h3 class="lesson-card-title">"Deployment & Scaling"</h3>
                                <p class="lesson-card-desc">"Production deployment, optimization, and community resources."</p>
                            </a>
                        </div>
                    </section>

                    <section style="margin-top: 4rem;">
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

                    <section style="margin-top: 4rem;">
                        <h2 class="section-title">"🎯 Learning Path"</h2>
                        <div class="learning-path">
                            <div class="path-step">
                                <div class="step-number">"1-4"</div>
                                <div class="step-content">
                                    <h4>"Foundation Building"</h4>
                                    <p>"Master core concepts: getting started, quoting, CSS basics, and scoping."</p>
                                </div>
                            </div>
                            <div class="path-arrow">"→"</div>
                            <div class="path-step">
                                <div class="step-number">"5-7"</div>
                                <div class="step-content">
                                    <h4>"Core Mastery"</h4>
                                    <p>"Learn design systems, control flow, and reusable components."</p>
                                </div>
                            </div>
                            <div class="path-arrow">"→"</div>
                            <div class="path-step">
                                <div class="step-number">"8-10"</div>
                                <div class="step-content">
                                    <h4>"Advanced Application"</h4>
                                    <p>"Build real applications with HTMX, layouts, and production patterns."</p>
                                </div>
                            </div>
                            <div class="path-arrow">"→"</div>
                            <div class="path-step">
                                <div class="step-number">"11-12"</div>
                                <div class="step-content">
                                    <h4>"Mastery & Scaling"</h4>
                                    <p>"Advanced patterns, testing, deployment, and community resources."</p>
                                </div>
                            </div>
                        </div>
                    </section>

                    <section style="margin-top: 4rem;">
                        <h2 class="section-title">"💡 Learning Approach"</h2>
                        <div class="features-grid">
                            <div class="feature-box">
                                <div class="feature-icon">"🧱"</div>
                                <h3 class="feature-title">"Build Gradually"</h3>
                                <p class="feature-desc">"Each lesson builds on the previous one. No overwhelming jumps."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"💻"</div>
                                <h3 class="feature-title">"Hands-On Practice"</h3>
                                <p class="feature-desc">"Every concept includes practical exercises and real code examples."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"🎯"</div>
                                <h3 class="feature-title">"Clear Prerequisites"</h3>
                                <p class="feature-desc">"Know exactly what you need before starting each lesson."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"🏆"</div>
                                <h3 class="feature-title">"Progressive Challenges"</h3>
                                <p class="feature-desc">"Optional advanced exercises for those who want to go deeper."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"🔄"</div>
                                <h3 class="feature-title">"Spaced Repetition"</h3>
                                <p class="feature-desc">"Key concepts reinforced throughout the learning path."</p>
                            </div>

                            <div class="feature-box">
                                <div class="feature-icon">"🌟"</div>
                                <h3 class="feature-title">"Real-World Focus"</h3>
                                <p class="feature-desc">"Build actual applications, not just toy examples."</p>
                            </div>
                        </div>
                    </section>
                </div>
            </body>
        </html>
    }
}

pub async fn homepage_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&homepage()))
}
