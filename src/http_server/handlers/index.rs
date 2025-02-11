use std::sync::LazyLock;

use axum::response::Html;

static INDEX: LazyLock<String> = LazyLock::new(|| {
    std::fs::read_to_string("files/index.html")
        .unwrap_or("none".to_string())
        .replace("{domain}", &crate::DOMAIN)
});

pub async fn request_handler() -> Html<String> {
    Html(INDEX.to_string())
}
