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
                <style src="lessons.css" />
                <style src="homepage.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="hero">
                        <h1 class="hero-title">"Azumi"</h1>
                        <p class="hero-subtitle">"Type-Safe, Compile-Time Validated HTML Templates for Rust"</p>
                    </header>

                    <section>
                        <h2 class="section-title">"📚 Complete Learning Path"</h2>
                        <p>"Master Azumi through 8 comprehensive lessons, from basics to real-world applications:"</p>

                        <div class="lessons-grid">
                            <a href="/lesson-1" class="lesson-card">
                                <div class="lesson-card-number">"Lesson 1"</div>
                                <h3 class="lesson-card-title">"Hello World"</h3>
                                <p class="lesson-card-desc">"Learn the fundamental quoting rules that make Azumi type-safe."</p>
                            </a>

                            <a href="/lesson-2" class="lesson-card">
                                <div class="lesson-card-number">"Lesson 2"</div>
                                <h3 class="lesson-card-title">"CSS Validation"</h3>
                                <p class="lesson-card-desc">"Catch CSS typos at compile time with location-specific errors."</p>
                            </a>

                            <a href="/lesson-3" class="lesson-card">
                                <div class="lesson-card-number">"Lesson 3"</div>
                                <h3 class="lesson-card-title">"Global Styles"</h3>
                                <p class="lesson-card-desc">"Share design tokens across components with global.css."</p>
                            </a>

                            <a href="/lesson-4" class="lesson-card">
                                <div class="lesson-card-number">"Lesson 4"</div>
                                <h3 class="lesson-card-title">"Control Flow"</h3>
                                <p class="lesson-card-desc">"Use Rust's native control flow directly in templates."</p>
                            </a>

                            <a href="/lesson-5" class="lesson-card">
                                <div class="lesson-card-number">"Lesson 5"</div>
                                <h3 class="lesson-card-title">"Components with Props"</h3>
                                <p class="lesson-card-desc">"Create reusable components with type-safe props."</p>
                            </a>

                            <a href="/lesson-6" class="lesson-card">
                                <div class="lesson-card-number">"Lesson 6"</div>
                                <h3 class="lesson-card-title">"HTMX Integration"</h3>
                                <p class="lesson-card-desc">"Build interactive apps with server-side rendering."</p>
                            </a>

                            <a href="/lesson-7" class="lesson-card">
                                <div class="lesson-card-number">"Lesson 7"</div>
                                <h3 class="lesson-card-title">"Layouts & Composition"</h3>
                                <p class="lesson-card-desc">"Build reusable layouts with function composition."</p>
                            </a>

                            <a href="/lesson-8" class="lesson-card">
                                <div class="lesson-card-number">"Lesson 8"</div>
                                <h3 class="lesson-card-title">"Real-World Examples"</h3>
                                <p class="lesson-card-desc">"See Azumi in action with practical applications."</p>
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
                                <div class="step-number">"1-3"</div>
                                <div class="step-content">
                                    <h4>"Fundamentals"</h4>
                                    <p>"Master the core concepts: quoting, CSS validation, and global styles."</p>
                                </div>
                            </div>
                            <div class="path-arrow">"→"</div>
                            <div class="path-step">
                                <div class="step-number">"4-5"</div>
                                <div class="step-content">
                                    <h4>"Advanced Features"</h4>
                                    <p>"Learn control flow, components, and type-safe prop patterns."</p>
                                </div>
                            </div>
                            <div class="path-arrow">"→"</div>
                            <div class="path-step">
                                <div class="step-number">"6-8"</div>
                                <div class="step-content">
                                    <h4>"Production Ready"</h4>
                                    <p>"Build real applications with HTMX, layouts, and practical examples."</p>
                                </div>
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
