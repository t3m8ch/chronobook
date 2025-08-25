use async_trait::async_trait;
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
        let organization = sqlx::query_as::<_, Organization>(
            r#"
            INSERT INTO organizations (id, name, display_name, description)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, display_name, description
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(name)
        .bind(display_name)
        .bind(description)
        .fetch_one(&mut **tx)
        .await?;

        Ok(organization)
    }

    async fn organization_exists_by_name(&self, name: &str) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(SELECT 1 FROM organizations WHERE name = $1)
            "#,
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }
}
