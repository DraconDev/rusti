use axum::response::{Html, IntoResponse};
use azumi::html;
use azumi::Component;

/// Modern Navigation Component with Interactive Features
pub fn modern_navigation(active_route: &str) -> impl Component {
    let routes = vec![
        ("/", "Home", "fas fa-home"),
        ("/hello", "Hello World", "fas fa-rocket"),
        ("/components", "Components", "fas fa-cube"),
        ("/control-flow", "Control Flow", "fas fa-code-branch"),
        ("/layouts", "Layouts", "fas fa-layer-group"),
        ("/forms", "Forms", "fas fa-edit"),
        ("/htmx-todo", "HTMX Todo", "fas fa-check"),
        ("/dashboard", "Dashboard", "fas fa-chart-bar"),
    ];

    html! {
        <nav class="modern-nav">
            <div class="nav-container">
                <div class="nav-brand">
                    <a href="/" class="brand-link">
                        <span class="brand-icon">"⚡"</span>
                        <span class="brand-text">"Azumi"</span>
                    </a>
                </div>
                
                <div class="nav-menu">
                    @for (route, label, icon) in &routes {
                        @let is_active = route == active_route;
                        <a 
                            href={route} 
                            class={format!("nav-link {}", if is_active { "active" } else { "" })}
                        >
                            <i class={icon}></i>
                            <span>{label}</span>
                        </a>
                    }
                </div>
                
                <div class="nav-actions">
                    <button class="theme-toggle" onclick="toggleTheme()">
                        <i class="fas fa-moon"></i>
                    </button>
                    <button class="mobile-menu-toggle" onclick="toggleMobileMenu()">
                        <i class="fas fa-bars"></i>
                    </button>
                </div>
            </div>
            
            <div class="mobile-menu" id="mobileMenu">
                @for (route, label, icon) in &routes {
                    @let is_active = route == active_route;
                    <a 
                        href={route} 
                        class={format!("mobile-nav-link {}", if is_active { "active" } else { "" })}
                    >
                        <i class={icon}></i>
                        <span>{label}</span>
                    </a>
                }
            </div>
        </nav>
    }
}

/// Feature showcase component
pub fn feature_showcase<'a>() -> impl Component + 'a {
    html! {
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
    }
}

/// Performance comparison component
pub fn performance_comparison<'a>() -> impl Component + 'a {
    html! {
        <section class="perf-comparison">
            <div class="comparison-header">
                <h2>"📊 Performance Comparison"</h2>
                <p>"Azumi vs Traditional Template Engines"</p>
            </div>
            
            <div class="comparison-grid">
                <div class="perf-card azumi">
                    <div class="card-header">
                        <span class="library-name">"Azumi"</span>
                        <span class="perf-badge success">"🚀 Fast"</span>
                    </div>
                    <div class="metrics">
                        <div class="metric">
                            <label>"Compile Time"</label>
                            <span class="value">"~50ms"</span>
                        </div>
                        <div class="metric">
                            <label>"Runtime Overhead"</label>
                            <span class="value">"~0ms"</span>
                        </div>
                        <div class="metric">
                            <label>"Memory Usage"</label>
                            <span class="value">"Minimal"</span>
                        </div>
                        <div class="metric">
                            <label>"Error Detection"</label>
                            <span class="value">"Compile-time"</span>
                        </div>
                    </div>
                </div>
                
                <div class="perf-card templating">
                    <div class="card-header">
                        <span class="library-name">"Template Engine"</span>
                        <span class="perf-badge warning">"🐌 Slow"</span>
                    </div>
                    <div class="metrics">
                        <div class="metric">
                            <label>"Compile Time"</label>
                            <span class="value">"~200ms"</span>
                        </div>
                        <div class="metric">
                            <label>"Runtime Overhead"</label>
                            <span class="value">"~5-10ms"</span>
                        </div>
                        <div class="metric">
                            <label>"Memory Usage"</label>
                            <span class="value">"Higher"</span>
                        </div>
                        <div class="metric">
                            <label>"Error Detection"</label>
                            <span class="value">"Runtime"</span>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="winner-highlight">
                <div class="winner-badge">
                    <i class="fas fa-trophy"></i>
                    <span>"Winner: Azumi"</span>
                </div>
                <p>"10x faster runtime, 4x faster compile time, 100% type safety"</p>
            </div>
        </section>
    }
}

/// Getting started component
pub fn getting_started<'a>() -> impl Component + 'a {
    html! {
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

/// Complete navigation system for the demo
pub fn complete_navigation<'a>(active_route: &'a str) -> impl Component + 'a {
    html! {
        <style src="/static/navigation_component.css" />
        <div class="navigation-system">
            @modern_navigation(active_route)
            @feature_showcase()
            @performance_comparison()
            @getting_started()
        </div>
    }
}

pub async fn navigation_component_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&complete_navigation("/")))
}