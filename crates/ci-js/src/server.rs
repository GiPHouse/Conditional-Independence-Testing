use crate::handler::run_test;
use axum::{routing::post, Router};
use tower_http::cors::CorsLayer;

pub fn build_router() -> Router {
    Router::new()
    .route("/tests/{test_name}", post(run_test))
    .layer(CorsLayer::permissive())
}

