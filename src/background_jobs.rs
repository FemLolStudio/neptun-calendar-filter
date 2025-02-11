use std::sync::Arc;

use super::http_server::AppState;
mod clear_ratelimit_cache;

pub fn start(app_state: Arc<AppState>) {
    tokio::spawn(clear_ratelimit_cache::cleanup_ratelimit_cache(app_state.clone()));
}
