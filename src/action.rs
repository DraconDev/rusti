use crate::Component;
use axum::response::IntoResponse;
use serde::de::DeserializeOwned;
use std::future::Future;

/// Trait for Azumi Actions
/// This is implemented automatically by the `#[azumi::action]` macro
pub trait Action<Input, Output> {
    fn call(input: Input) -> impl Future<Output = Output> + Send;
}

/// Helper to wrap an action result into an Axum response
/// This is used by the generated handler code
pub async fn handle_action_result<C: Component + ?Sized>(component: &C) -> impl IntoResponse {
    crate::render_to_string(component)
}

// We might need more helpers here for the macro to use
// For example, to handle the request body deserialization
