use axum::Router;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use utoipa::Modify;
use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_scalar::{Scalar, Servable};

use crate::api::v1::{admin, auth, bookings};
use crate::repositories::auth::PgAuthRepository;
use crate::services::auth::AuthServiceImpl;
use crate::services::jwt::JwtManager;
use crate::services::providers::MockSmsProvider;
use crate::services::providers::MockTelegramProvider;

mod api;
mod models;
mod repositories;
mod services;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<dyn crate::services::auth::AuthService>,
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "bookings", description = "Booking management endpoints - GET endpoints are public, POST endpoints require authentication"),
        (name = "admin", description = "Admin endpoints - Access controlled by user roles: Root (all access), Owner (organization scope), Manager (branch scope), Master (own schedule and customers)"),
    ),
    info(
        title = "Chronobook API",
        version = "1.0.0",
        description = "Booking management system for beauty salons\n\n## Access Control\n\nThe API uses role-based access control with the following roles:\n\n- **Root**: System-wide administrator with full access to all resources\n- **Owner**: Organization administrator with full access to organization resources (branches, employees, services, notifications)\n- **Manager**: Branch administrator with access to branch-specific resources and can manage branch notifications\n- **Master**: Service provider who can edit own schedule, manage own customers, and view organization data\n\n### Authorization Header\n\nAll protected endpoints require a Bearer token in the Authorization header:\n```\nAuthorization: Bearer <JWT_TOKEN>\n```",
        contact(
            name = "Chronobook Support",
            email = "support@chronobook.com"
        ),
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let server_addr = std::env::var("SERVER_ADDR").unwrap_or("0.0.0.0:3222".to_string());

    let sms_provider = Arc::new({
        let mut sms_provider = MockSmsProvider::new();
        sms_provider.expect_send_notification().return_const(Ok(()));
        sms_provider
            .expect_send_verification_code()
            .return_const(Ok(()));
        sms_provider
    });

    let telegram_provider = Arc::new({
        let mut telegram_provider = MockTelegramProvider::new();
        telegram_provider.expect_send_message().return_const(Ok(()));
        telegram_provider
            .expect_send_notification()
            .return_const(Ok(()));
        telegram_provider
            .expect_generate_auth_hash()
            .return_const(Ok(Vec::new()));
        telegram_provider
            .expect_verify_auth_data()
            .return_const(Ok(true));
        telegram_provider
    });

    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await?;

    let state = Arc::new(AppState {
        auth_service: Arc::new(AuthServiceImpl::new(
            Arc::new(PgAuthRepository::new(pg_pool)),
            sms_provider.clone(),
            telegram_provider.clone(),
            Arc::new(JwtManager::new()),
        )),
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
