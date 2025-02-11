use std::{
    net::{IpAddr, SocketAddr},
    ops::Deref,
    str::FromStr,
    sync::LazyLock,
};

use http_server::AppState;
use logger::LogType;

mod background_jobs;
mod enviorment;
mod http_server;
mod logger;

static DOMAIN: LazyLock<String> = LazyLock::new(|| enviorment::get_enviorment("DOMAIN"));
static TARGET_URL: LazyLock<String> = LazyLock::new(|| enviorment::get_enviorment("TARGET_URL"));
static IP: LazyLock<String> = LazyLock::new(|| enviorment::get_enviorment("IP"));
static PORT: LazyLock<String> = LazyLock::new(|| enviorment::get_enviorment("PORT"));

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    _ = enable_ansi_support::enable_ansi_support();
    //-----------------------------------------------------------------------
    // check enviorments
    //-----------------------------------------------------------------------
    _ = dotenv::dotenv();
    let _ = DOMAIN.deref();
    let _ = TARGET_URL.deref();
    let _ = IP.deref();
    let _ = PORT.deref();

    log!(LogType::Info, "Main", "💨 Starting up server... 💨");

    //-----------------------------------------------------------------------
    // route
    //-----------------------------------------------------------------------
    let app_state = AppState::new().await;

    let app = http_server::create_router(app_state.clone()).await;

    //-----------------------------------------------------------------------
    // run it
    //-----------------------------------------------------------------------
    background_jobs::start(app_state.clone());

    let port = u16::from_str(&PORT).expect("PORT parse error");

    let addr = SocketAddr::new(IpAddr::from_str(&IP).expect("IP parse error"), port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    crate::log!(LogType::Info, "Main", "👀 Listening on {} 👀", addr);

    let server = axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await;
    if let Err(server_error) = server {
        crate::log!(LogType::Error, "Main", "Unexpected axum-server error:\n{}", format!("{:?}", server_error));
        panic!();
    }
}
