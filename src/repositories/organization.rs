use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::models::organization::db::Organization;

#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    async fn create_organization(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        name: &str,
        display_name: &str,
        description: Option<&str>,
    ) -> Result<Organization, sqlx::Error>;

    async fn organization_exists_by_name(&self, name: &str) -> Result<bool, sqlx::Error>;
}

pub struct OrganizationRepositoryImpl {
    pool: PgPool,
}

impl OrganizationRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrganizationRepository for OrganizationRepositoryImpl {
    async fn create_organization(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        name: &str,
        display_name: &str,
        description: Option<&str>,
    ) -> Result<Organization, sqlx::Error> {
        let id = Uuid::new_v4();
        let created_at = Utc::now().naive_utc();
        let organization = sqlx::query_as!(
            Organization,
            r#"
            INSERT INTO organizations (id, name, display_name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, display_name, description
            "#,
            id,
            name,
            display_name,
            description,
            created_at,
            created_at,
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(organization)
    }

    async fn organization_exists_by_name(&self, name: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM organizations WHERE name = $1) as "exists!"
            "#,
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists)
    }
}
