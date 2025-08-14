use poem::{Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;

use crate::api::v1::bookings::BookingsApi;

mod api;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new(BookingsApi, "Chronobook API", "1.0")
        .server("http://37.233.85.221:3222/api/v1");

    let stoplight_ui = api_service.stoplight_elements();

    let app = Route::new()
        .nest("/api/v1", api_service)
        .nest("/docs/stoplight", stoplight_ui);

    Server::new(TcpListener::bind("0.0.0.0:3222"))
        .run(app)
        .await
}
