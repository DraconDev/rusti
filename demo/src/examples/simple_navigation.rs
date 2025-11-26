use axum::response::{Html, IntoResponse};
use azumi::html;
use azumi::Component;

/// Simple Navigation Component
pub fn simple_navigation() -> impl Component {
    html! {
        <style src="/static/navigation_component.css" />
        <nav class="modern-nav">
            <div class="nav-container">
                <div class="nav-brand">
                    <a href="/" class="brand-link">
                        <span class="brand-icon">"⚡"</span>
                        <span class="brand-text">"Azumi"</span>
                    </a>
                </div>
                
                <div class="nav-menu">
                    <a href="/" class="nav-link active">
                        <i class="fas fa-home"></i>
                        <span>"Home"</span>
                    </a>
                    <a href="/hello" class="nav-link">
                        <i class="fas fa-rocket"></i>
                        <span>"Hello World"</span>
                    </a>
                    <a href="/components" class="nav-link">
                        <i class="fas fa-cube"></i>
                        <span>"Components"</span>
                    </a>
                    <a href="/control-flow" class="nav-link">
                        <i class="fas fa-code-branch"></i>
                        <span>"Control Flow"</span>
                    </a>
                    <a href="/forms" class="nav-link">
                        <i class="fas fa-edit"></i>
                        <span>"Forms"</span>
                    </a>
                    <a href="/dashboard" class="nav-link">
                        <i class="fas fa-chart-bar"></i>
                        <span>"Dashboard"</span>
                    </a>
                </div>
                
                <div class="nav-actions">
                    <button class="theme-toggle" onclick="toggleTheme()">
                        <i class="fas fa-moon"></i>
                    </button>
                </div>
            </div>
        </nav>
        
        <section class="feature-showcase">
            <div class="showcase-header">
                <h2>"🚀 Why Choose Azumi?"</h2>
                <p>"Built for modern Rust web development"</p>
            </div>
            
            <div class="feature-grid">
                <div class="feature-card">
                    <div class="feature-icon">
                        <i class="fas fa-magic"></i>
                    </div>
                    <h3>"CSS Auto-Scoping"</h3>
                    <p>"Automatic style isolation prevents conflicts. No more CSS cascade headaches."</p>
                    <div class="feature-demo">
                        <div class="demo-scope-1">
                            <span>"Scope 1"</span>
                        </div>
                        <div class="demo-scope-2">
                            <span>"Scope 2"</span>
                        </div>
                    </div>
                </div>
                
                <div class="feature-card">
                    <div class="feature-icon">
                        <i class="fas fa-brain"></i>
                    </div>
                    <h3>"AI-Optimized"</h3>
                    <p>"95% AI generation success rate. Designed for AI-assisted development."</p>
                    <div class="feature-demo">
                        <div class="ai-example">
                            <code>{"// AI Prompt: \"User card component\"\n@UserCard(name=\"Alice\", role=\"Admin\")"}</code>
                        </div>
                    </div>
                </div>
                
                <div class="feature-card">
                    <div class="feature-icon">
                        <i class="fas fa-shield-alt"></i>
                    </div>
                    <h3>"Type Safety"</h3>
                    <p>"Compile-time error detection. Catch bugs before they reach production."</p>
                    <div class="feature-demo">
                        <div class="compile-demo">
                            <span class="success">"✓ Compiles successfully"</span>
                        </div>
                    </div>
                </div>
                
                <div class="feature-card">
                    <div class="feature-icon">
                        <i class="fas fa-bolt"></i>
                    </div>
                    <h3>"Zero Runtime"</h3>
                    <p>"All processing happens at compile time. Maximum performance guaranteed."</p>
                    <div class="feature-demo">
                        <div class="perf-metrics">
                            <span>"~0ms runtime"</span>
                            <span>"✓ Instant render"</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
        
        <section class="getting-started">
            <div class="start-header">
                <h2>"🎯 Ready to Get Started?"</h2>
                <p>"Three ways to begin your Azumi journey"</p>
            </div>
            
            <div class="start-options">
                <div class="start-option">
                    <div class="option-icon">
                        <i class="fas fa-play-circle"></i>
                    </div>
                    <h3>"1. Try the Demo"</h3>
                    <p>"Explore 25+ interactive examples right now"</p>
                    <a href="/" class="option-button primary">"Start Exploring"</a>
                </div>
                
                <div class="start-option">
                    <div class="option-icon">
                        <i class="fas fa-book"></i>
                    </div>
                    <h3>"2. Read the Guide"</h3>
                    <p>"Learn the fundamentals with step-by-step tutorials"</p>
                    <a href="/hello" class="option-button secondary">"Start Learning"</a>
                </div>
                
                <div class="start-option">
                    <div class="option-icon">
                        <i class="fas fa-code"></i>
                    </div>
                    <h3>"3. Build Something"</h3>
                    <p>"Create your first Azumi component"</p>
                    <a href="/components" class="option-button tertiary">"Build Now"</a>
                </div>
            </div>
        </section>
    }
}

pub async fn simple_navigation_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&simple_navigation()))
}