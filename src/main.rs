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
    paths(
        // Auth paths
        auth::customer::login_phone,
        auth::customer::verify_phone,
        auth::customer::login_telegram,
        auth::customer::verify_telegram,
        auth::customer::refresh,
        auth::employee::login_phone,
        auth::employee::verify_phone,
        auth::employee::login_telegram,
        auth::employee::verify_telegram,
        auth::employee::refresh,
        // Bookings paths
        bookings::get_services,
        bookings::get_masters,
        bookings::get_branches,
        // Admin paths
        admin::create_organization,
        admin::get_organization_dashboard,
        admin::create_branch,
        admin::get_branches,
        admin::get_masters,
        admin::create_employee,
        admin::create_service,
    ),
    components(
        schemas(
            // Error models
            crate::models::error::ApiError,
            // Auth models
            crate::models::auth::request::PhoneLoginRequest,
            crate::models::auth::request::PhoneVerifyRequest,
            crate::models::auth::request::UpdateProfileRequest,
            crate::models::auth::request::RefreshTokenRequest,
            crate::models::auth::request::TelegramAuthRequest,
            crate::models::auth::response::TelegramVerifyHash,
            crate::models::auth::response::AccessToken,
            crate::models::auth::response::PhoneLoginOk,
            // Booking models
            crate::models::booking::request::CreateBookingRequest,
            crate::models::booking::request::NotifyMethod,
            crate::models::booking::request::WindowOut,
            crate::models::booking::request::SlotOut,
            crate::models::booking::response::BookingOut,
            // Branch models
            crate::models::branch::request::CreateBranchRequest,
            crate::models::branch::response::BranchOut,
            crate::models::branch::response::CreateBranchOut,
            // Service models
            crate::models::service::request::CreateServiceRequest,
            crate::models::service::response::ServiceOut,
            crate::models::service::response::CreateServiceOut,
            // Master models
            crate::models::master::response::MasterOut,
            // Employee models
            crate::models::employee::request::CreateEmployeeRequest,
            crate::models::employee::response::CreateEmployeeOut,
            // Organization models
            crate::models::organization::response::OrganizationOut,
            // Dashboard models
            crate::models::dashboard::response::OrganizationDashboardOut,
            // Timetable models
            crate::models::timetable::request::CreateTimetableRequest,
            crate::models::timetable::request::CreateDayRedefinitionRequest,
            crate::models::timetable::request::ScheduleDayIn,
            crate::models::timetable::request::Interval,
            crate::models::timetable::request::ScheduleDayType,
            crate::models::timetable::response::CreateTimetableOut,
        )
    ),
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

    // Build the application with all routes
    let app = Router::new()
        // Auth routes
        .nest("/api/v1/auth", auth::router())
        // Bookings routes
        .nest("/api/v1/bookings", bookings::router())
        // Admin routes
        .nest("/api/v1/admin", admin::router())
        .with_state(state)
        // Add Scalar UI for OpenAPI documentation
        .merge(Scalar::with_url("/docs/scalar", ApiDoc::openapi()))
        // Add OpenAPI spec endpoint
        .route(
            "/api/v1/openapi.json",
            axum::routing::get(|| async { axum::Json(ApiDoc::openapi()) }),
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
