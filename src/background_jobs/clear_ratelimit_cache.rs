use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::time::sleep;

use crate::{http_server::AppState, logger::LogType};

pub async fn cleanup_ratelimit_cache(app_state: Arc<AppState>) {
    loop {
        let now = Instant::now();

        let mut hash_map = app_state.rate_limit.lock().await;
        let count_before = hash_map.len();

        hash_map.retain(|_, &mut timestamp| timestamp > now);

        let count_after = hash_map.len();

        drop(hash_map);

        let delay_in_ms = now.elapsed().as_micros() as f64 / 1000f64;

        if count_before - count_after != 0 {
            crate::log!(
                LogType::InfoBG,
                "RateLimitCleaner",
                "🧹 {} ip removed!\tDelay: {} ms 🧹",
                count_before - count_after,
                format!("{:.3}", delay_in_ms)
            );
        }

        sleep(Duration::from_secs(60)).await;
    }
}
