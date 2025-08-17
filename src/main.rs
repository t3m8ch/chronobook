use axum::Router;
use dotenv::dotenv;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::api::v1::{admin, auth, bookings};

mod api;
mod models;

#[derive(Clone, Debug)]
pub struct AppState {
    // Add your shared state here (e.g., database pool)
}

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "bookings", description = "Booking management endpoints"),
        (name = "admin", description = "Admin endpoints"),
    ),
    info(
        title = "Chronobook API",
        version = "1.0.0",
        description = "Booking management system for beauty salons",
        contact(
            name = "Chronobook Support",
            email = "support@chronobook.com"
        ),
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let server_addr = std::env::var("SERVER_ADDR").unwrap_or("0.0.0.0:3222".to_string());

    let state = Arc::new(AppState {
        // Initialize your state here
    });

    // Build the routers with OpenApiRouter
    let (auth_router, auth_api) = auth::router().split_for_parts();
    let (bookings_router, bookings_api) = bookings::router().split_for_parts();
    let (admin_router, admin_api) = admin::router().split_for_parts();

    // Merge all OpenAPI docs
    let api = ApiDoc::openapi()
        .nest("/api/v1/auth", auth_api)
        .nest("/api/v1/bookings", bookings_api)
        .nest("/api/v1/admin", admin_api);

    // Build the application with all routes
    let app = Router::new()
        // Auth routes
        .nest("/api/v1/auth", auth_router)
        // Bookings routes
        .nest("/api/v1/bookings", bookings_router)
        // Admin routes
        .nest("/api/v1/admin", admin_router)
        .with_state(state)
        // Add Scalar UI for OpenAPI documentation
        .merge(Scalar::with_url("/docs/scalar", api.clone()))
        // Add OpenAPI spec endpoint
        .route(
            "/api/v1/openapi.json",
            axum::routing::get(move || async move { axum::Json(api) }),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&server_addr).await?;

    tracing::info!("🚀 Server running at http://{}", server_addr);
    tracing::info!("📚 API documentation at http://{}/docs/scalar", server_addr);
    tracing::info!(
        "📋 OpenAPI spec at http://{}/api/v1/openapi.json",
        server_addr
    );

    axum::serve(listener, app).await?;

    Ok(())
}
