mod examples;

use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // 🚀 Azumi Demo Server - Complete Learning Path
    let app = Router::new()
        // 🏠 Homepage - Learning Portal
        .route("/", get(examples::lessons::pages::homepage::homepage_handler))
        
        // 📚 Interactive Lessons (1-8)
        .route("/lesson-1", get(examples::lessons::lesson1::lesson1_handler))
        .route("/lesson-2", get(examples::lessons::lesson2::lesson2_handler))
        .route("/lesson-3", get(examples::lessons::lesson3::lesson3_handler))
        .route("/lesson-4", get(examples::lessons::lesson4::lesson4_handler))
        .route("/lesson-5", get(examples::lessons::lesson5::lesson5_handler))
        .route("/lesson-6", get(examples::lessons::lesson6::lesson6_handler))
        .route("/lesson-7", get(examples::lessons::lesson7::lesson7_handler))
        .route("/lesson-8", get(examples::lessons::lesson8::lesson8_handler))
        
        // 🎮 Interactive Demo Endpoints
        .route("/api/click", post(|| async { "Button clicked! 🚀" }))
        .route("/api/innerhtml", post(|| async { "Updated content!" }))
        .route("/api/append", post(|| async { "<div>New item added! ✨</div>" }))
        .route("/api/replace", post(|| async { 
            "<div style='background: #10b981; color: white; padding: 1rem; border-radius: 0.5rem; text-align: center;'>🔄 Replaced!</div>" 
        }))
        
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
    println!("📚 Interactive Lessons:");
    println!("  📖 Lesson 1:      Hello World & Quoting");
    println!("  🎨 Lesson 2:      CSS Validation & Scoping");
    println!("  🌍 Lesson 3:      Global Styles & Tokens");
    println!("  ⚡ Lesson 4:      Control Flow");
    println!("  🧩 Lesson 5:      Components with Props");
    println!("  🌐 Lesson 6:      HTMX Integration");
    println!("  🏗️  Lesson 7:      Layouts & Composition");
    println!("  🚀 Lesson 8:      Real-World Examples");
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
