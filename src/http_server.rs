use std::{sync::Arc, time::Instant};

use ahash::{HashMap, HashMapExt};
use axum::{extract::DefaultBodyLimit, middleware, routing, Router};
use handlers::{filter, index};
use middlewares::{log, rate_limit};
use tokio::sync::Mutex;
use tower_http::compression::CompressionLayer;

mod handlers;
mod ip_manager;
mod middlewares;

pub struct AppState {
    pub rate_limit: Mutex<HashMap<String, Instant>>,
}
impl AppState {
    pub async fn new() -> Arc<Self> {
        let app_state = AppState {
            rate_limit: Mutex::new(HashMap::new()),
        };
        Arc::new(app_state)
    }
}

#[allow(unused_mut)]
pub async fn create_router(app_state: Arc<AppState>) -> Router {
    let mut app: Router = Router::new()
        .route("/", routing::get(index::request_handler))
        .route("/filter/{id}", routing::get(filter::request_handler))
        .route("/inverse-filter/{id}", routing::get(filter::inverse_request_handler))
        .layer(middleware::from_fn_with_state(app_state.clone(), rate_limit::limit))
        .layer(middleware::from_fn(log::log))
        //gzip compress
        .layer(CompressionLayer::new())
        //1 MB limit
        .layer(DefaultBodyLimit::max(1024 * 1024))
        .with_state(app_state);
    app
}
