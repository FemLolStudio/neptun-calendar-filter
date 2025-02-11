use std::{
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{
    extract::{ConnectInfo, Request, State},
    http::HeaderMap,
    middleware::Next,
    response::{Html, IntoResponse, Response},
};
use reqwest::StatusCode;

use crate::http_server::{ip_manager, AppState};

const RATE_LIMIT_SECS: u64 = 5;

pub async fn limit(
    State(state): State<Arc<AppState>>,

    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,

    request: Request,
    next: Next,
) -> Response {
    let ip = ip_manager::get_user_ip(&addr, &headers);
    let now = Instant::now();

    {
        let mut hashmap = state.rate_limit.lock().await;
        if let Some(lock_until) = hashmap.get_mut(&ip) {
            if now > *lock_until {
                *lock_until = now + Duration::from_secs(RATE_LIMIT_SECS);
            } else {
                return (StatusCode::TOO_MANY_REQUESTS, Html(format!("{RATE_LIMIT_SECS} sec rate limit"))).into_response();
            }
        } else {
            hashmap.insert(ip, now);
        }
    }

    next.run(request).await
}
