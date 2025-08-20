use axum::http::HeaderValue;
use chrono::Duration;
use dotenv::dotenv;

pub struct Config {
    pub server_addr: String,
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
    pub jwt_access_duration: Duration,
    pub jwt_refresh_duration: Duration,
    pub jwt_cookie_allow_origin: HeaderValue,
    pub database_url: String,
    pub redis_url: String,
    pub telegram_hash_secret: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();

        let mut errors: Vec<String> = Vec::new();

        let server_addr = std::env::var("SERVER_ADDR").unwrap_or("0.0.0.0:3222".to_string());
        let _allow_origin =
            std::env::var("ALLOW_ORIGIN").unwrap_or("http://localhost:3222".to_string());
        let access_secret = std::env::var("JWT_ACCESS_SECRET");
        let refresh_secret = std::env::var("JWT_REFRESH_SECRET");
        let access_duration_minutes: Result<i64, _> =
            std::env::var("JWT_ACCESS_EXPIRATION_MINUTES")
                .unwrap_or("15".to_string())
                .parse();
        let refresh_duration_days: Result<i64, _> = std::env::var("JWT_REFRESH_EXPIRATION_DAYS")
            .unwrap_or("7".to_string())
            .parse();
        let jwt_cookie_allow_origin = std::env::var("JWT_COOKIE_ALLOW_ORIGIN")
            .unwrap_or("http://localhost:3222".to_string())
            .parse::<HeaderValue>();
        let database_url = std::env::var("DATABASE_URL");
        let redis_url = std::env::var("REDIS_URL").unwrap_or("redis://127.0.0.1:6379".to_string());
        let telegram_hash_secret = std::env::var("TELEGRAM_HASH_SECRET");

        if let Err(_) = access_secret.clone() {
            errors.push("JWT_ACCESS_SECRET is not set".to_string());
        }

        if let Err(_) = refresh_secret.clone() {
            errors.push("JWT_REFRESH_SECRET is not set".to_string());
        }

        if let Err(_) = access_duration_minutes.clone() {
            errors.push("JWT_ACCESS_EXPIRATION_MINUTES must be a valid integer".to_string());
        }

        if let Err(_) = refresh_duration_days.clone() {
            errors.push("JWT_REFRESH_EXPIRATION_DAYS must be a valid integer".to_string());
        }

        if jwt_cookie_allow_origin.is_err() {
            errors.push("JWT_COOKIE_ALLOW_ORIGIN is not set".to_string());
        }

        if let Err(_) = database_url.clone() {
            errors.push("DATABASE_URL is not set".to_string());
        }

        if let Err(_) = telegram_hash_secret.clone() {
            errors.push("TELEGRAM_HASH_SECRET is not set".to_string());
        }

        if !errors.is_empty() {
            return Err(anyhow::Error::msg(format!(
                "Failed to load configuration. Errors: {:#?}",
                errors
            )));
        }

        Ok(Config {
            server_addr,
            jwt_access_secret: access_secret?,
            jwt_refresh_secret: refresh_secret?,
            jwt_access_duration: Duration::minutes(access_duration_minutes?),
            jwt_refresh_duration: Duration::days(refresh_duration_days?),
            jwt_cookie_allow_origin: jwt_cookie_allow_origin?,
            database_url: database_url?,
            redis_url,
            telegram_hash_secret: telegram_hash_secret?,
        })
    }
}
