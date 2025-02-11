use std::{net::SocketAddr, time::Instant};

use crate::logger::LogType;
use axum::{
    body::{Body, Bytes},
    extract::{ConnectInfo, OriginalUri, Request},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;

use super::super::ip_manager::get_user_ip;

pub async fn log(
    OriginalUri(path): OriginalUri,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,

    request: Request,
    next: Next,
) -> Response {
    let now = Instant::now();
    let mut response = next.run(request).await;
    let delay_in_ms = now.elapsed().as_micros() as f64 / 1000f64;
    let ip = get_user_ip(&addr, &headers);

    if response.status() == StatusCode::OK || response.status() == StatusCode::PERMANENT_REDIRECT {
        crate::log!(
            LogType::Info,
            "Request",
            "🔷 Code: {}\tIP: {}\tDelay: {} ms\tRequest: {} 🔷",
            response.status(),
            ip,
            format!("{:.3}", delay_in_ms),
            path.path()
        );
    } else {
        // Buffer the response body
        let (parts, body) = response.into_parts();
        let (bytes, body_string_option) = get_body(body).await;

        let mut body_string = body_string_option.unwrap_or(String::from("<none>"));
        if body_string.is_empty() {
            body_string = String::from("<empty>");
        }
        crate::log!(
            LogType::Warning,
            "Request",
            "🛑 Code: {}\tIP:{}\tDelay: {} ms\tRequest: {}\tBody: {} 🛑",
            parts.status,
            ip,
            format!("{:.3}", delay_in_ms),
            path.path(),
            body_string
        );
        if let Some(bytes) = bytes {
            response = Response::from_parts(parts, Body::from(bytes)).into_response();
        } else {
            response = parts.into_response()
        }
    }

    response
}

pub async fn get_body<B>(body: B) -> (Option<Bytes>, Option<String>)
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes_res = body.collect().await;
    if let Ok(bytes) = bytes_res {
        let bytes = bytes.to_bytes();
        if let Ok(body) = std::str::from_utf8(&bytes) {
            let restr = body.to_owned();
            (Some(bytes), Some(restr))
        } else {
            (Some(bytes), None)
        }
    } else {
        (None, None)
    }
}
