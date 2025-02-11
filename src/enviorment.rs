use crate::{log, logger::LogType};

/// Getting the enviorment variable
///
/// If the variable don't exist it **panic**
pub fn get_enviorment(key: &str) -> String {
    let res = std::env::var(key);
    match res {
        Ok(envi) => {
            log!(LogType::Info, "Enviorment", "✅ Getting '{}' enviorment is successful! ✅", key);
            envi
        }
        Err(err) => {
            log!(
                LogType::Error,
                "Enviorment",
                "🔥 Failed to get '{}' enviorment: {} 🔥",
                key,
                format!("{:?}", err)
            );
            std::process::exit(1);
        }
    }
}
