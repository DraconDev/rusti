mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod repositories;
mod routes;
mod services;
mod templates;

use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: Config,
    pub http_client: reqwest::Client,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "azumi_starter=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded");

    // Create database connection pool
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    tracing::info!("Database connection established");

    // Run migrations
    sqlx::migrate!("./migrations").run(&db).await?;
    tracing::info!("Database migrations completed");

    // Create HTTP client
    let http_client = reqwest::Client::new();

    // Create application state
    let state = AppState {
        db,
        config: config.clone(),
        http_client,
    };

    // Create router
    let app = routes::create_routes(state).layer(TraceLayer::new_for_http());

    // Start server
    let addr = format!("0.0.0.0:{}", config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!(
        "🚀 Server running on http://localhost:{}",
        config.server_port
    );
    tracing::info!(
        "📖 Visit http://localhost:{} to access the application",
        config.server_port
    );

    axum::serve(listener, app).await?;

    Ok(())
}
