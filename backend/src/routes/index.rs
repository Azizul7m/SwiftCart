// Define application routes
use crate::handlers::get_product;
use axum::{Router, routing::get};

/// Create the application router
pub async fn app() -> Router {
    Router::new().route("/api/products", get(get_product))
}
