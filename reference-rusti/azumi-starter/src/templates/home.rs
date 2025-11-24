use rusti::{component, rusti};

pub fn home_page() -> impl rusti::Component {
    rusti! {
        <main class="w-full" role="main">
            // Hero Section
            <section class="text-center mb-16">
                <div class="inline-block p-1 bg-gradient-to-r from-cyan-400 via-purple-500 to-cyan-600 rounded-full mb-8 glow-effect">
                    <div class="glass-card rounded-full px-6 py-2">
                        <span class="text-sm font-semibold text-white">"🚀 Production Ready Platform"</span>
                    </div>
                </div>
                <h1 class="text-3xl md:text-4xl lg:text-6xl font-bold bg-gradient-to-r from-cyan-400 via-purple-500 to-cyan-300 bg-clip-text text-transparent mb-6">
                    "Rust + Axum Authentication Platform"
                </h1>
                <p class="text-base md:text-xl text-gray-300 mb-10 max-w-3xl mx-auto leading-relaxed">
                    "A complete, production-ready authentication system built with Rust, Axum, SQLx, and Rusti. Features OAuth integration, server sessions, user management, and admin dashboards."
                </p>
                <div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
                    <a href="/login" class="bg-gradient-to-r from-cyan-500 to-purple-600 hover:from-cyan-400 hover:to-purple-500 text-white font-semibold py-4 px-8 rounded-xl transition-all duration-300 transform hover:scale-105 shadow-lg glow-effect">
                        "Try Authentication"
                    </a>
                    <a href="/profile" class="bg-gradient-to-r from-green-500 to-green-600 hover:from-green-400 hover:to-green-500 text-white font-semibold py-4 px-8 rounded-xl transition-all duration-300 transform hover:scale-105 shadow-lg glow-effect">
                        "View Profile Demo"
                    </a>
                </div>
            </section>

            // Live Demo Status
            <section class="text-center mb-16">
                <div class="glass-card rounded-2xl p-6 border border-cyan-400/30 max-w-2xl mx-auto">
                    <h3 class="text-lg font-semibold text-cyan-400 mb-2">"🟢 Platform Status"</h3>
                    <p class="text-gray-300">"All authentication methods are working: Google, GitHub, Discord, Microsoft"</p>
                    <div class="flex justify-center space-x-4 mt-4">
                        @status_indicator(label = "Google OAuth".to_string())
                        @status_indicator(label = "GitHub OAuth".to_string())
                        @status_indicator(label = "Discord OAuth".to_string())
                        @status_indicator(label = "Microsoft OAuth".to_string())
                    </div>
                </div>
            </section>

            // Features Grid
            <section class="grid grid-cols-1 lg:grid-cols-3 gap-8 mb-16" aria-label="Platform features">
                @feature_card(
                    icon = "🔐".to_string(),
                    title = "OAuth Integration".to_string(),
                    description = "Secure authentication with multiple providers. Redis-backed sessions with instant validation.".to_string(),
                    features = vec![
                        "Google, GitHub, Discord, Microsoft".to_string(),
                        "Server session management".to_string(),
                        "Instant session validation".to_string(),
                        "HTTP-only secure cookies".to_string()
                    ]
                )
                @feature_card(
                    icon = "👥".to_string(),
                    title = "User Management".to_string(),
                    description = "Complete user profile system with database integration and admin controls.".to_string(),
                    features = vec![
                        "PostgreSQL database".to_string(),
                        "User profiles & avatars".to_string(),
                        "Session management".to_string(),
                        "Admin dashboard".to_string()
                    ]
                )
                @feature_card(
                    icon = "⚡".to_string(),
                    title = "Modern Architecture".to_string(),
                    description = "Built with modern tools for performance, type safety, and developer experience.".to_string(),
                    features = vec![
                        "Rust + Axum backend".to_string(),
                        "Type-safe Rusti templates".to_string(),
                        "HTMX for interactivity".to_string(),
                        "Production deployment".to_string()
                    ]
                )
            </section>

            // Technical Architecture
            <section class="glass-card rounded-2xl shadow-2xl p-8 mb-8">
                <div class="text-center mb-8">
                    <h3 class="text-3xl font-bold text-white mb-4">"System Architecture"</h3>
                    <p class="text-gray-300 text-lg">"Clean separation between main app and authentication microservice"</p>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    @architecture_card(
                        icon = "🌐".to_string(),
                        title = "Main Application".to_string(),
                        color = "cyan".to_string(),
                        items = vec![
                            ("Rust + Axum:".to_string(), "HTTP routing and handlers".to_string()),
                            ("Rusti:".to_string(), "Type-safe HTML generation".to_string()),
                            ("HTMX:".to_string(), "Dynamic interactions".to_string()),
                            ("PostgreSQL:".to_string(), "User data storage".to_string()),
                            ("SQLx:".to_string(), "Type-safe database queries".to_string())
                        ]
                    )
                    @architecture_card(
                        icon = "🔐".to_string(),
                        title = "Auth Microservice".to_string(),
                        color = "green".to_string(),
                        items = vec![
                            ("OAuth 2.0:".to_string(), "Provider integration".to_string()),
                            ("Server Sessions:".to_string(), "Redis-backed validation".to_string()),
                            ("Redis:".to_string(), "Session storage".to_string()),
                            ("Session Management:".to_string(), "Instant validation".to_string()),
                            ("Multi-provider:".to_string(), "Google, GitHub, Discord, Microsoft".to_string())
                        ]
                    )
                </div>
            </section>

            // Getting Started
            <section class="text-center" aria-label="Getting started">
                <div class="glass-card rounded-2xl shadow-2xl p-8 max-w-3xl mx-auto">
                    <h3 class="text-3xl font-bold text-white mb-4">"Ready to Get Started?"</h3>
                    <p class="text-gray-300 mb-6 text-lg">
                        "Test the authentication system and explore the codebase. Everything is production-ready and fully documented."
                    </p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center items-center mb-6">
                        <a href="/login" class="bg-gradient-to-r from-cyan-500 to-purple-600 hover:from-cyan-400 hover:to-purple-500 text-white font-semibold py-4 px-8 rounded-xl transition-all duration-300 transform hover:scale-105 shadow-lg glow-effect">
                            "Try Login System →"
                        </a>
                        <a href="/profile" class="bg-gradient-to-r from-green-500 to-green-600 hover:from-green-400 hover:to-green-500 text-white font-semibold py-4 px-8 rounded-xl transition-all duration-300 transform hover:scale-105 shadow-lg glow-effect">
                            "View Profile Demo"
                        </a>
                    </div>
                    <div class="text-sm text-gray-400">
                        "✓ OAuth 2.0 compliant  ✓ Production ready  ✓ Fully documented"
                    </div>
                </div>
            </section>
        </main>
    }
}

#[component]
fn status_indicator(label: String) -> impl rusti::Component {
    rusti! {
        <div class="flex items-center text-sm text-green-400">
            <div class="w-2 h-2 bg-green-400 rounded-full mr-2"></div>
            {label}
        </div>
    }
}

#[component]
fn feature_card(
    icon: String,
    title: String,
    description: String,
    features: Vec<String>,
) -> impl rusti::Component {
    rusti! {
        <article class="glass-card rounded-2xl shadow-2xl p-8 hover:shadow-3xl transition-all duration-300">
            <div class="w-16 h-16 bg-gradient-to-br from-blue-400 to-blue-600 rounded-2xl flex items-center justify-center mb-6 glow-effect">
                <span class="text-2xl">{icon}</span>
            </div>
            <h3 class="text-2xl font-bold text-white mb-4">{title}</h3>
            <p class="text-gray-300 mb-6">
                {description}
            </p>
            <ul class="space-y-2 text-sm text-gray-400">
                @for feature in &features {
                    <li>"• " {feature}</li>
                }
            </ul>
        </article>
    }
}

#[component]
fn architecture_card(
    icon: String,
    title: String,
    color: String,
    items: Vec<(String, String)>,
) -> impl rusti::Component {
    // Pre-build the class strings
    let card_classes = match color.as_str() {
        "cyan" => "glass-card rounded-xl p-6 border border-cyan-400/30",
        "green" => "glass-card rounded-xl p-6 border border-green-400/30",
        _ => "glass-card rounded-xl p-6 border border-gray-400/30",
    };

    let title_classes = match color.as_str() {
        "cyan" => "text-xl font-bold mb-4 flex items-center text-cyan-400",
        "green" => "text-xl font-bold mb-4 flex items-center text-green-400",
        _ => "text-xl font-bold mb-4 flex items-center text-gray-400",
    };

    rusti! {
        <div class={card_classes}>
            <h4 class={title_classes}>
                <span class="text-2xl mr-3">{icon}</span>
                {title}
            </h4>
            <ul class="space-y-2 text-gray-300">
                @for (key, value) in &items {
                    <li>"• " <strong>{key}</strong> " " {value}</li>
                }
            </ul>
        </div>
    }
}
