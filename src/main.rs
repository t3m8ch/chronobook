use poem::{Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;

use crate::api::v1::{auth::AuthApi, bookings::BookingsApi};

mod api;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new((BookingsApi, AuthApi), "Chronobook API", "1.0")
        .server("http://37.233.85.221:3222/api/v1");

    let stoplight_ui = api_service.stoplight_elements();

    let app = Route::new()
        .nest("/api/v1", api_service)
        .nest("/docs/stoplight", stoplight_ui);

    Server::new(TcpListener::bind("0.0.0.0:3222"))
        .run(app)
        .await
}
