mod examples;

use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Azumi 2.0 Demo Server
    let app = Router::new()
        // Example Routes
        .route("/", get(examples::homepage_handler))
        .route("/hello", get(examples::hello_handler))
        .route("/components", get(examples::components_handler))
        .route("/htmx-todo", get(examples::htmx_todo_handler))
        // HTMX API endpoints
        .route("/api/todos", post(examples::add_todo_handler))
        .route("/api/todos/:id", delete(examples::delete_todo_handler))
        // Static files (CSS, JS)
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081")
        .await
        .expect("Failed to bind to port 8081");

    println!("🚀 Rusti 2.0 Demo Server");
    println!("=====================================");
    println!("📍 http://localhost:8081");
    println!();
    println!("Examples:");
    println!("  • / - Hello World (basic quoting demo)");
    println!("  • /components - Component composition");
    println!("  • /htmx-todo - HTMX server-side rendering");
    println!();
    println!("All examples follow Rusti 2.0 rules:");
    println!("  ✓ Mandatory double quotes");
    println!("  ✓ External CSS/JS files");
    println!("  ✓ Type-safe components");
    println!("=====================================");

    axum::serve(listener, app).await.unwrap();
}
