mod examples;

use axum::{routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        // New Rusti 2.0 Examples
        .route("/", get(examples::hello_handler))
        .route("/components", get(examples::components_handler))
        .route("/htmx-todo", get(examples::htmx_todo_handler))
        // Static files
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:80 81")
        .await
        .unwrap();

    println!("🚀 Rusti 2.0 Demo Server");
    println!("📍 http://localhost:8081");
    println!("   - / (Hello World)");
    println!("   - /components (Component Composition)");
    println!("   - /htmx-todo (HTMX Todo List)");

    axum::serve(listener, app).await.unwrap();
}
