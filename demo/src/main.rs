mod examples;

use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Azumi Demo Server
    let app = Router::new()
        // Example Routes
        .route("/", get(examples::homepage::homepage_handler))
        // Educational lessons
        .route("/lesson-1", get(examples::lesson1::lesson1_handler))
        // Basic examples
        .route("/hello", get(examples::hello::hello_handler))
        .route("/tailwind", get(examples::tailwind::tailwind_handler))
        // Core concepts
        .route("/components", get(examples::components::components_handler))
        .route("/control-flow", get(examples::control_flow::control_flow_handler))
        .route("/layouts", get(examples::layouts::layouts_handler))
        // Interactive features
        .route("/forms", get(examples::forms::forms_handler).post(examples::forms::forms_handler))
        .route("/htmx-todo", get(examples::htmx_todo::htmx_todo_handler))
        .route("/api/todos", post(examples::htmx_todo::add_todo_handler))
        .route(
            "/api/todos/:id",
            delete(examples::htmx_todo::delete_todo_handler),
        )
        // Complex applications
        .route("/dashboard", get(examples::dashboard::dashboard_handler))
        // Navigation component demo
        .route("/navigation", get(examples::simple_navigation::simple_navigation_handler))
        // Static files (CSS, JS)
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081")
        .await
        .expect("Failed to bind to port 8081");

    println!("🚀 Azumi Demo Server");
    println!("=====================================");
    println!("📍 http://localhost:8081");
    println!();
    println!("Examples:");
    println!("  • / - Hello World (basic quoting demo)");
    println!("  • /components - Basic component composition");
    println!("  • /advanced-components - Forms, modals, validation");
    println!("  • /htmx-todo - HTMX server-side rendering");
    println!();
    println!("All examples follow Azumi rules:");
    println!("  ✓ Mandatory double quotes");
    println!("  ✓ External CSS/JS files");
    println!("  ✓ Type-safe components");
    println!("=====================================");

    axum::serve(listener, app).await.unwrap();
}
