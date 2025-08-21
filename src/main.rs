use axum::Router;
use axum::extract::FromRef;
use axum::http::Method;
use axum::http::header;
use axum_extra::extract::cookie::SameSite;
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
use crate::config::Config;
use crate::repositories::auth::PgAuthRepository;
use crate::repositories::booking::PgBookingRepository;
use crate::repositories::branch::PgBranchRepository;
use crate::repositories::employee::PgEmployeeRepository;
use crate::repositories::token::{RedisTokenRepository, TokenRepository};
use crate::services::auth::AuthService;
use crate::services::auth::AuthServiceImpl;
use crate::services::booking::{BookingService, BookingServiceImpl};
use crate::services::branch::{BranchService, BranchServiceImpl};
use crate::services::employee::{EmployeeService, EmployeeServiceImpl};
use crate::services::jwt::JwtManager;
use crate::services::providers::MockSmsProvider;
use crate::services::providers::MockTelegramProvider;

mod api;
mod config;
mod extractors;
mod models;
mod repositories;
mod services;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub auth_service: Arc<dyn AuthService>,
    pub booking_service: Arc<dyn BookingService>,
    pub branch_service: Arc<dyn BranchService>,
    pub employee_service: Arc<dyn EmployeeService>,
    pub jwt_manager: Arc<JwtManager>,
    pub jwt_cookie_settings: JwtCookieSettings,
    pub without_validation_arguments: (),
}

#[derive(Clone, Debug)]
pub struct JwtCookieSettings {
    pub cookie_name: String,
    pub http_only: bool,
    pub secure: bool,
    pub same_site: SameSite,
    pub max_age: chrono::Duration,
    pub path: String,
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
    let config = Config::from_env()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

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

    let redis_token_repository: Arc<dyn TokenRepository> = Arc::new(
        RedisTokenRepository::new(&config.redis_url)
            .map_err(|e| anyhow::Error::msg(format!("Failed to connect to Redis: {}", e)))?,
    );

    let jwt_manager = Arc::new(
        JwtManager::builder()
            .access_secret(config.jwt_access_secret)
            .refresh_secret(config.jwt_refresh_secret)
            .access_duration(config.jwt_access_duration)
            .refresh_duration(config.jwt_refresh_duration)
            .token_repository(redis_token_repository)
            .build(),
    );

    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let auth_repository = Arc::new(PgAuthRepository::new(pg_pool.clone()));
    let booking_repository = Arc::new(PgBookingRepository::new(pg_pool.clone()));
    let branch_repository = Arc::new(PgBranchRepository::new(pg_pool.clone()));
    let employee_repository = Arc::new(PgEmployeeRepository::new(pg_pool.clone()));

    let state = AppState {
        auth_service: Arc::new(AuthServiceImpl::new(
            auth_repository.clone(),
            sms_provider.clone(),
            telegram_provider.clone(),
            jwt_manager.clone(),
            config.telegram_hash_secret.clone(),
        )),
        booking_service: Arc::new(BookingServiceImpl::new(
            booking_repository,
            auth_repository.clone(),
        )),
        branch_service: Arc::new(BranchServiceImpl::new(
            branch_repository,
            auth_repository.clone(),
        )),
        employee_service: Arc::new(EmployeeServiceImpl::new(
            employee_repository,
            auth_repository,
        )),
        jwt_manager: jwt_manager.clone(),
        jwt_cookie_settings: if cfg!(debug_assertions) {
            JwtCookieSettings {
                cookie_name: "refresh_token".to_string(),
                http_only: true,
                secure: false,
                same_site: SameSite::Lax,
                max_age: config.jwt_refresh_duration,
                path: "/".to_string(),
            }
        } else {
            JwtCookieSettings {
                cookie_name: "refresh_token".to_string(),
                http_only: true,
                secure: true,
                same_site: SameSite::Lax,
                max_age: config.jwt_refresh_duration,
                path: "/".to_string(),
            }
        },
        without_validation_arguments: (),
    };

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
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_origin(config.jwt_cookie_allow_origin)
                .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        )
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&config.server_addr).await?;

    tracing::info!("🚀 Server running at http://{}", config.server_addr);
    tracing::info!(
        "📚 API documentation at http://{}/docs/scalar",
        config.server_addr
    );
    tracing::info!(
        "📋 OpenAPI spec at http://{}/api/v1/openapi.json",
        config.server_addr
    );

    axum::serve(listener, app).await?;

    Ok(())
}
