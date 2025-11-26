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
                <style src="global.css" />
                <style src="homepage.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="hero">
                        <h1 class="hero-title">"Azumi"</h1>
                        <p class="hero-subtitle">"Type-Safe, Compile-Time Validated HTML Templates for Rust"</p>
                    </header>

                    <section>
                        <h2 class="section-title">"📚 Interactive Lessons"</h2>
                        <p>"Learn Azumi's core concepts through hands-on examples:"</p>

                        <div class="lessons-grid">
                            <a href="/lesson-0" class="lesson-card">
                                <div class="lesson-card-number">"Lesson 0"</div>
                                <h3 class="lesson-card-title">"Hello World"</h3>
                                <p class="lesson-card-desc">"Learn the fundamental quoting rules that make Azumi type-safe."</p>
                            </a>

                            <a href="/lesson-1" class="lesson-card">
                                <div class="lesson-card-number">"Lesson 1"</div>
                                <h3 class="lesson-card-title">"CSS Validation"</h3>
                                <p class="lesson-card-desc">"Catch CSS typos at compile time with location-specific errors."</p>
                            </a>

                            <a href="/lesson-2" class="lesson-card">
                                <div class="lesson-card-number">"Lesson 2"</div>
                                <h3 class="lesson-card-title">"Global Styles"</h3>
                                <p class="lesson-card-desc">"Share design tokens across components with global.css."</p>
                            </a>

                            <a href="/lesson-3" class="lesson-card">
                                <div class="lesson-card-number">"Lesson 3"</div>
                                <h3 class="lesson-card-title">"Control Flow"</h3>
                                <p class="lesson-card-desc">"Use Rust's native control flow directly in templates."</p>
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
