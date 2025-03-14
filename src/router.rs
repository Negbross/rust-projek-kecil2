use crate::database::DatabaseState;
use crate::handlers::{create_product, delete_product, get_product, list_products, update_product};
use crate::middlewares::logging_middleware;
use axum::middleware::{self, };
use axum::routing::{delete, get, post, put};
use axum::Router;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

pub fn create_router(state: DatabaseState) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/products", get(list_products))
        .route("/product", post(create_product))
        .route("/update/{id}", put(update_product))
        .route("/delete", delete(delete_product))
        .route("/product/{id}", get(get_product))
        .with_state(Arc::new(state))
        // Middleware
        .layer(middleware::from_fn(logging_middleware))
        .layer(TraceLayer::new_for_http())

}