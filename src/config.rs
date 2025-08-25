use axum::http::HeaderValue;
use chrono::Duration;
use dotenv::dotenv;
use serde::{Deserialize, Deserializer, de};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_server_addr")]
    pub server_addr: String,

    pub jwt_access_secret: String,

    pub jwt_refresh_secret: String,

    #[serde(
        default = "default_jwt_access_duration",
        rename = "JWT_ACCESS_DURATION_MINUTES",
        deserialize_with = "minutes"
    )]
    pub jwt_access_duration: Duration,

    #[serde(
        default = "default_jwt_refresh_duration",
        rename = "JWT_REFRESH_DURATION_DAYS",
        deserialize_with = "days"
    )]
    pub jwt_refresh_duration: Duration,

    #[serde(
        default = "default_jwt_cookie_allow_origin",
        deserialize_with = "header_value"
    )]
    pub jwt_cookie_allow_origin: HeaderValue,

    pub database_url: String,

    #[serde(default = "default_redis_url")]
    pub redis_url: String,

    pub telegram_hash_secret: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();
        envy::from_env().map_err(|e| anyhow::anyhow!("Failed to load environment variables: {e}"))
    }
}

fn default_server_addr() -> String {
    "0.0.0.0:3222".to_string()
}

fn default_jwt_access_duration() -> Duration {
    Duration::minutes(15)
}

fn default_jwt_refresh_duration() -> Duration {
    Duration::days(7)
}

fn default_jwt_cookie_allow_origin() -> HeaderValue {
    HeaderValue::from_static("http://localhost:3222")
}

fn default_redis_url() -> String {
    "redis://localhost:6379".to_string()
}

fn header_value<'de, D>(deserializer: D) -> Result<HeaderValue, D::Error>
where
    D: Deserializer<'de>,
{
    let value: String = Deserialize::deserialize(deserializer)?;
    HeaderValue::from_str(&value)
        .map_err(|e| de::Error::custom(format!("Invalid header value: {e}")))
}

fn minutes<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Deserialize::deserialize(deserializer)?;
    Ok(Duration::minutes(value))
}

fn days<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Deserialize::deserialize(deserializer)?;
    Ok(Duration::days(value))
}
