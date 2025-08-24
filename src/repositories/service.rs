use async_trait::async_trait;
use sqlx::{PgPool, QueryBuilder};
use std::str::FromStr;
use uuid::Uuid;

use crate::models::service::db::Service;

#[async_trait]
pub trait ServiceRepository: Send + Sync {
    async fn create_service(
        &self,
        display_name: String,
        description: String,
        duration_minutes: i32,
        price: Option<String>,
        master_id: Option<Uuid>,
    ) -> Result<Uuid, sqlx::Error>;

    async fn find_service_by_id(&self, id: Uuid) -> Result<Option<Service>, sqlx::Error>;

    async fn find_services_by_organization(
        &self,
        organization_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Service>, sqlx::Error>;

    async fn update_service(
        &self,
        id: Uuid,
        display_name: Option<String>,
        description: Option<String>,
        duration_minutes: Option<i32>,
        price: Option<Option<String>>,
        master_id: Option<Option<Uuid>>,
    ) -> Result<Service, sqlx::Error>;

    async fn delete_service(&self, id: Uuid) -> Result<bool, sqlx::Error>;

    async fn find_organization_id_by_service(
        &self,
        service_id: Uuid,
    ) -> Result<Option<Uuid>, sqlx::Error>;
}

pub struct ServiceRepositoryImpl {
    pool: PgPool,
}

impl ServiceRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ServiceRepository for ServiceRepositoryImpl {
    async fn create_service(
        &self,
        display_name: String,
        description: String,
        duration_minutes: i32,
        price: Option<String>,
        master_id: Option<Uuid>,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();

        let price_decimal = price
            .as_ref()
            .and_then(|p| sqlx::types::BigDecimal::from_str(p).ok());

        sqlx::query!(
            r#"
            INSERT INTO services (
                id, created_at, updated_at, display_name, description, 
                duration_minutes, price, master_id
            )
            VALUES ($1, NOW(), NOW(), $2, $3, $4, $5, $6)
            "#,
            id,
            display_name,
            description,
            duration_minutes,
            price_decimal,
            master_id
        )
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    async fn find_service_by_id(&self, id: Uuid) -> Result<Option<Service>, sqlx::Error> {
        sqlx::query_as!(
            Service,
            r#"
            SELECT 
                s.id, s.created_at, s.updated_at, s.display_name, s.description,
                s.duration_minutes, s.price, s.master_id, e.organization_id
            FROM services s
            LEFT JOIN employees e ON s.master_id = e.id
            WHERE s.id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_services_by_organization(
        &self,
        organization_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Service>, sqlx::Error> {
        sqlx::query_as!(
            Service,
            r#"
            SELECT 
                s.id, s.created_at, s.updated_at, s.display_name, s.description,
                s.duration_minutes, s.price, s.master_id, e.organization_id
            FROM services s
            LEFT JOIN employees e ON s.master_id = e.id
            WHERE e.organization_id = $1 OR s.master_id IS NULL
            ORDER BY s.display_name
            LIMIT $2 OFFSET $3
            "#,
            organization_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn update_service(
        &self,
        id: Uuid,
        display_name: Option<String>,
        description: Option<String>,
        duration_minutes: Option<i32>,
        price: Option<Option<String>>,
        master_id: Option<Option<Uuid>>,
    ) -> Result<Service, sqlx::Error> {
        let mut query_builder = QueryBuilder::new("UPDATE services SET updated_at = NOW()");

        if let Some(name) = display_name {
            query_builder.push(", display_name = ");
            query_builder.push_bind(name);
        }

        if let Some(desc) = description {
            query_builder.push(", description = ");
            query_builder.push_bind(desc);
        }

        if let Some(duration) = duration_minutes {
            query_builder.push(", duration_minutes = ");
            query_builder.push_bind(duration);
        }

        if let Some(price_opt) = price {
            query_builder.push(", price = ");
            let price_decimal = price_opt.and_then(|p| sqlx::types::BigDecimal::from_str(&p).ok());
            query_builder.push_bind(price_decimal);
        }

        if let Some(master_opt) = master_id {
            query_builder.push(", master_id = ");
            query_builder.push_bind(master_opt);
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id.clone());

        query_builder.build().execute(&self.pool).await?;

        // Return updated service with organization_id
        self.find_service_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    async fn delete_service(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM services WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn find_organization_id_by_service(
        &self,
        service_id: Uuid,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT e.organization_id
            FROM services s
            LEFT JOIN employees e ON s.master_id = e.id
            WHERE s.id = $1
            "#,
            service_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|r| r.organization_id))
    }
}
