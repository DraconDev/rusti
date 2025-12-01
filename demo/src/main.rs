mod actions;
mod examples;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // 🚀 Azumi Demo Server - Complete Learning Path
    let app = Router::new()
        // 🏠 Homepage - Learning Portal
        .route("/", get(examples::lessons::pages::homepage::homepage_handler))
        
        
        // 📚 Interactive Lessons (0-20)
        // .route("/lesson-0", get(examples::lessons::pages::lesson0::lesson0_handler))
        .route("/test-global-styles", get(examples::test_global_styles::handler))
        .route("/azumi-plus", get(examples::azumi_plus_demo::azumi_plus_demo_handler))
        .merge(azumi::action::register_actions(axum::Router::new()))
        .route("/lesson-1", get(examples::lessons::pages::lesson1::lesson1_handler))

        // 🎮 Interactive Demo Endpoints
        .route("/api/click", post(|| async { "Button clicked! 🚀" }))
        .route("/api/innerhtml", post(|| async { "Updated content!" }))
        .route("/api/append", post(|| async { "<li class='todo-item'><span class='todo-text'>New task added! ✨</span><button hx-delete='/api/todos/delete' hx-target='closest .todo-item' hx-swap='outerHTML swap:0.3s' class='todo-delete'>Delete</button></li>" }))
        .route("/api/replace", post(|| async { 
            "<div style='background: #10b981; color: white; padding: 1rem; border-radius: 0.5rem; text-align: center;'>🔄 Replaced!</div>" 
        }))
        
        // HTMX Todo handlers
        .route("/api/todos/delete", axum::routing::delete(|| async { "" }))
        
        // 📁 Static files (CSS, JS)
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081")
        .await
        .expect("Failed to bind to port 8081");

    println!("🎓 Azumi Learning Platform");
    println!("=====================================");
    println!("📍 http://localhost:8081");
    println!();
    println!("🧭 Navigation:");
    println!("  🌐 /              - Learning Homepage");
    println!();
    println!("📚 Interactive Lessons (0-20):");
    println!("  📖 Lesson 0:      Fragments & Basic Structure");
    println!("  📖 Lesson 1:      Hello World & Quoting");
    println!("  🎨 Lesson 2:      CSS Validation & Scoping");
    println!("  🌍 Lesson 3:      Global Styles & Tokens");
    println!("  ⚡ Lesson 4:      Control Flow (@if, @for, @match)");
    println!("  🧩 Lesson 5:      Components with Props");
    println!("  🎭 Lesson 6:      Pattern Matching");
    println!("  📊 Lesson 7:      CSS Variables & Interpolation");
    println!("  🔄 Lesson 8:      Nested Control Flow");
    println!("  📱 Lesson 9:      HTML Structure Validation");
    println!("  🔍 Lesson 10:     Accessibility Testing");
    println!("  🎨 Lesson 11:     Button Components");
    println!("  🧪 Lesson 12:     Fragment Testing");
    println!("  🏗️  Lesson 13:     Component Composition");
    println!("  🔧 Lesson 14:     Advanced Component Patterns");
    println!("  📝 Lesson 15:     Form Components & State");
    println!("  🌐 Lesson 16:     JavaScript Integration");
    println!("  🚀 Lesson 17:     HTMX Interactive Features");
    println!("  🎨 Lesson 18:     CSS Variables");
    println!("  ♿ Lesson 19:     Accessibility Patterns");
    println!("  🔄 Lesson 20:     Conditional Classes");
    println!("  🔍 Lesson 21:     CSS Scoping Demo");
    println!("  📊 Lesson 22:     Data Tables");
    println!("  🌍 Lesson 23:     Global CSS & CDN");
    println!("  ✅ Lesson 24:     Boolean Attributes");
    println!("  📝 Lesson 25:     Schema.org JSON-LD");
    println!("  🎨 Lesson 26:     Multiple CSS Files");
    println!("  🏷️  Lesson 27:     SEO Meta Tags (head!)");
    println!("  ⚠️  Lesson 28:     Error Handling");
    println!("  🧩 Lesson 29:     Advanced Composition (Slots)");
    println!("  ⏳ Lesson 30:     Loading States");
    println!("  📝 Lesson 31:     Type-Safe Forms (Form Binding)");
    println!("  ⚡ Lesson 32:     String Optimization");
    println!("  🛡️  Lesson 33:     Strict Validation Rules");
    println!("  🏆 Lesson 34:     Capstone - Social Profile");
    println!();
    println!("🎯 Learning Path: Fundamentals → Advanced → Production");
    println!();
    println!("✅ Azumi Features Demonstrated:");
    println!("  • Type-safe HTML with compile-time validation");
    println!("  • Automatic CSS scoping & dead code prevention");
    println!("  • Component architecture with props");
    println!("  • HTMX integration for interactivity");
    println!("  • Layout composition patterns");
    println!();
    println!("🔧 Built with: Rust + Azumi + Axum + HTMX");
    println!("=====================================");

    axum::serve(listener, app).await.unwrap();
}
