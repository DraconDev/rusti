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
        .route("/lesson-0", get(examples::lessons::pages::lesson0::handler))
        .route("/lesson-2", get(examples::lessons::pages::lesson2::lesson2_handler))
        .route("/lesson-3", get(examples::lessons::pages::lesson3::lesson3_handler))
        .route("/lesson-4", get(examples::lessons::pages::lesson4::lesson4_handler))
        .route("/lesson-5", get(examples::lessons::pages::lesson5::lesson5_handler))
        .route("/test-global-styles", get(examples::test_global_styles::handler))
        .route("/azumi-plus", get(examples::azumi_plus_demo::azumi_plus_demo_handler))
        .merge(azumi::action::register_actions(axum::Router::new()))
        .route("/lesson-1", get(examples::lessons::pages::lesson1::lesson1_handler))
        .route("/lesson-6", get(examples::lessons::pages::lesson6::lesson6_handler))
        .route("/lesson-7", get(examples::lessons::pages::lesson7::lesson7_handler))
        .route("/lesson-8", get(examples::lessons::pages::lesson8::lesson8_handler))
        .route("/lesson-9", get(examples::lessons::pages::lesson9::lesson9_handler))
        .route("/lesson-10", get(examples::lessons::pages::lesson10::lesson10_handler))
        .route("/lesson-11", get(examples::lessons::pages::lesson11::lesson11_handler))
        .route("/lesson-12", get(examples::lessons::pages::lesson12::lesson12_handler))
        .route("/unified-demo", get(examples::live_component_demo::unified_demo_handler))

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

    axum::serve(listener, app).await.unwrap();
}
