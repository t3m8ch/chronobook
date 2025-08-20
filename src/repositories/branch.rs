use async_trait::async_trait;
use sqlx::{PgPool, Result};
use uuid::Uuid;

use crate::models::branch::db::Branch;

#[mockall::automock]
#[async_trait]
pub trait BranchRepository: Send + Sync {
    async fn create(
        &self,
        name: String,
        description: String,
        timezone: String,
        street: String,
        house_number: String,
        apartment_number: Option<String>,
        city: String,
        region: String,
        country: String,
        address_info: Option<String>,
        organization_id: Uuid,
    ) -> Result<Branch>;

    async fn list(
        &self,
        organization_id: Option<Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Branch>>;

    async fn get(&self, id: Uuid) -> Result<Option<Branch>>;

    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        timezone: Option<String>,
        street: Option<String>,
        house_number: Option<String>,
        apartment_number: Option<Option<String>>,
        city: Option<String>,
        region: Option<String>,
        country: Option<String>,
        address_info: Option<Option<String>>,
    ) -> Result<Option<Branch>>;

    async fn delete(&self, id: Uuid) -> Result<bool>;

    async fn get_organization_id(&self, branch_id: Uuid) -> Result<Option<Uuid>>;
}

pub struct PgBranchRepository {
    pool: PgPool,
}

impl PgBranchRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BranchRepository for PgBranchRepository {
    async fn create(
        &self,
        name: String,
        description: String,
        timezone: String,
        street: String,
        house_number: String,
        apartment_number: Option<String>,
        city: String,
        region: String,
        country: String,
        address_info: Option<String>,
        organization_id: Uuid,
    ) -> Result<Branch> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now().naive_utc();

        sqlx::query_as!(
            Branch,
            r#"
            INSERT INTO branches (
                id, created_at, updated_at, display_name, description, timezone,
                street, house_number, apartment_number, city, region, country,
                address_info, organization_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING id, created_at, updated_at, display_name, description, timezone,
                      street, house_number, apartment_number, city, region, country,
                      address_info, organization_id
            "#,
            id,
            now,
            now,
            name,
            description,
            timezone,
            street,
            house_number,
            apartment_number.as_deref(),
            city,
            region,
            country,
            address_info.as_deref(),
            organization_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn list(
        &self,
        organization_id: Option<Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Branch>> {
        if let Some(org_id) = organization_id {
            sqlx::query_as!(
                Branch,
                r#"
                SELECT id, created_at, updated_at, display_name, description, timezone,
                       street, house_number, apartment_number, city, region, country,
                       address_info, organization_id
                FROM branches
                WHERE organization_id = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
                org_id,
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as!(
                Branch,
                r#"
                SELECT id, created_at, updated_at, display_name, description, timezone,
                       street, house_number, apartment_number, city, region, country,
                       address_info, organization_id
                FROM branches
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#,
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await
        }
    }

    async fn get(&self, id: Uuid) -> Result<Option<Branch>> {
        sqlx::query_as!(
            Branch,
            r#"
            SELECT id, created_at, updated_at, display_name, description, timezone,
                   street, house_number, apartment_number, city, region, country,
                   address_info, organization_id
            FROM branches
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        timezone: Option<String>,
        street: Option<String>,
        house_number: Option<String>,
        apartment_number: Option<Option<String>>,
        city: Option<String>,
        region: Option<String>,
        country: Option<String>,
        address_info: Option<Option<String>>,
    ) -> Result<Option<Branch>> {
        let now = chrono::Utc::now().naive_utc();

        // First, get the current branch to preserve unchanged fields
        let current = self.get(id).await?;
        if current.is_none() {
            return Ok(None);
        }
        let current = current.unwrap();

        // Apply updates with Option<Option<T>> pattern
        let apartment_number_value = match apartment_number {
            Some(Some(val)) => Some(val),
            Some(None) => None,
            None => current.apartment_number,
        };

        let address_info_value = match address_info {
            Some(Some(val)) => Some(val),
            Some(None) => None,
            None => current.address_info,
        };

        sqlx::query_as!(
            Branch,
            r#"
            UPDATE branches
            SET updated_at = $2,
                display_name = COALESCE($3, display_name),
                description = COALESCE($4, description),
                timezone = COALESCE($5, timezone),
                street = COALESCE($6, street),
                house_number = COALESCE($7, house_number),
                apartment_number = $8,
                city = COALESCE($9, city),
                region = COALESCE($10, region),
                country = COALESCE($11, country),
                address_info = $12
            WHERE id = $1
            RETURNING id, created_at, updated_at, display_name, description, timezone,
                      street, house_number, apartment_number, city, region, country,
                      address_info, organization_id
            "#,
            id,
            now,
            name.as_deref(),
            description.as_deref(),
            timezone.as_deref(),
            street.as_deref(),
            house_number.as_deref(),
            apartment_number_value.as_deref(),
            city.as_deref(),
            region.as_deref(),
            country.as_deref(),
            address_info_value.as_deref()
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM branches
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_organization_id(&self, branch_id: Uuid) -> Result<Option<Uuid>> {
        let result = sqlx::query!(
            r#"
            SELECT organization_id
            FROM branches
            WHERE id = $1
            "#,
            branch_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|r| r.organization_id))
    }
}
