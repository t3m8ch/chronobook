use async_trait::async_trait;

#[mockall::automock]
#[async_trait]
pub trait SmsProvider: Send + Sync {
    async fn send_verification_code(&self, phone: &str, code: u32) -> Result<(), String>;
    async fn send_notification(&self, phone: &str, message: &str) -> Result<(), String>;
}

#[mockall::automock]
#[async_trait]
pub trait TelegramProvider: Send + Sync {
    async fn send_message(&self, telegram_id: i64, message: &str) -> Result<(), String>;
    async fn generate_auth_hash(&self, telegram_id: i64) -> Result<Vec<u8>, String>;
    async fn verify_auth_data(
        &self,
        telegram_id: i64,
        username: Option<String>,
        first_name: String,
        last_name: Option<String>,
        hash: String,
    ) -> Result<bool, String>;
}
