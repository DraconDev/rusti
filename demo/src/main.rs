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
        .route("/lesson-0", get(examples::lessons::pages::lesson0::lesson0_handler))
        .route("/lesson-1", get(examples::lessons::pages::lesson1::lesson1_handler))
        .route("/lesson-2", get(examples::lessons::pages::lesson2::lesson2_handler))
        .route("/lesson-3", get(examples::lessons::pages::lesson3::lesson3_handler))
        .route("/lesson-4", get(examples::lessons::pages::lesson4::lesson4_handler))
        .route("/lesson-5", get(examples::lessons::pages::lesson5::lesson5_handler))
        .route("/lesson-6", get(examples::lessons::pages::lesson6::lesson6_handler))
        .route("/lesson-7", get(examples::lessons::pages::lesson7::lesson7_handler))
        .route("/lesson-8", get(examples::lessons::pages::lesson8::lesson8_handler))
        .route("/lesson-9", get(examples::lessons::pages::lesson9::lesson9_handler))
        .route("/lesson-10", get(examples::lessons::pages::lesson10::lesson10_handler))
        .route("/lesson-11", get(examples::lessons::pages::lesson11::lesson11_handler))
        .route("/lesson-12", get(examples::lessons::pages::lesson12::lesson12_handler))
        .route("/lesson-13", get(examples::lessons::pages::lesson13::lesson13_handler))
        .route("/lesson-14", get(examples::lessons::pages::lesson14::lesson14_handler))
        .route("/lesson-15", get(examples::lessons::pages::lesson15::lesson15_handler))
        .route("/lesson-16", get(examples::lessons::pages::lesson16::lesson16_handler))
        .route("/lesson-17", get(examples::lessons::pages::lesson17::lesson17_handler))
        .route("/lesson-18", get(examples::lessons::pages::lesson18::lesson18_handler))
        .route("/lesson-19", get(examples::lessons::pages::lesson19::lesson19_handler))
        .route("/lesson-20", get(examples::lessons::pages::lesson20::lesson20_handler))
        .route("/lesson-21", get(examples::lessons::pages::lesson21::lesson21_handler))
        .route("/lesson-22", get(examples::lessons::pages::lesson22::lesson22_handler))
        .route("/lesson-23", get(examples::lessons::pages::lesson23::lesson23_handler))
        .route("/lesson-24", get(examples::lessons::pages::lesson24::lesson24_handler))
        .route("/lesson-25", get(examples::lessons::pages::lesson25::lesson25_handler))
        .route("/lesson-26", get(examples::lessons::pages::lesson26::lesson26_handler))
        .route("/lesson-27", get(examples::lessons::pages::lesson27::lesson27_handler))
        .route("/lesson-28", get(examples::lessons::pages::lesson28::lesson28_handler))
        .route("/lesson-29", get(examples::lessons::pages::lesson29::lesson29_handler))
        .route("/lesson-30", get(examples::lessons::pages::lesson30::lesson30_handler))
        
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
    println!("  ⚡ Lesson 18:     Real-Time Updates");
    println!("  🎨 Lesson 19:     Layout System Patterns");
    println!("  📚 Lesson 20:     Full Application Example");
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
