use dotenv::dotenv;
use poem::{Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;
use tracing_subscriber::EnvFilter;

use crate::api::v1::{auth::AuthApi, bookings::BookingsApi};

mod api;
mod models;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let server_addr = std::env::var("SERVER_ADDR").unwrap_or("0.0.0.0:3222".to_string());
    let openapi_addr = std::env::var("OPENAPI_ADDR").unwrap_or(server_addr.clone());

    let api_service = OpenApiService::new((BookingsApi, AuthApi), "Chronobook API", "1.0")
        .server(format!("http://{openapi_addr}/api/v1"));

    let stoplight_ui = api_service.stoplight_elements();

    let app = Route::new()
        .nest("/api/v1", api_service)
        .nest("/docs/stoplight", stoplight_ui);

    Server::new(TcpListener::bind(server_addr)).run(app).await
}
