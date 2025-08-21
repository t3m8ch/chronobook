use async_trait::async_trait;
use sqlx::{PgPool, Result};
use uuid::Uuid;

use crate::models::employee::db::{Employee, EmployeeWithProfile};

#[mockall::automock]
#[async_trait]
pub trait EmployeeRepository: Send + Sync {
    async fn create(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        contact_phone: Option<String>,
        contact_email: Option<String>,
        contact_telegram: Option<String>,
        is_owner: bool,
        is_manager: bool,
        is_master: bool,
        manager_branch_id: Option<Uuid>,
    ) -> Result<Employee>;

    async fn list(
        &self,
        organization_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<EmployeeWithProfile>>;

    async fn get(&self, id: Uuid) -> Result<Option<EmployeeWithProfile>>;

    async fn get_by_user_and_org(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
    ) -> Result<Option<Employee>>;

    async fn update(
        &self,
        id: Uuid,
        contact_phone: Option<Option<String>>,
        contact_email: Option<Option<String>>,
        contact_telegram: Option<Option<String>>,
        user_id: Option<Uuid>,
        is_owner: Option<bool>,
        is_manager: Option<bool>,
        is_master: Option<bool>,
        manager_branch_id: Option<Option<Uuid>>,
    ) -> Result<Option<EmployeeWithProfile>>;

    async fn delete(&self, id: Uuid) -> Result<bool>;

    async fn get_organization_id(&self, employee_id: Uuid) -> Result<Option<Uuid>>;
}

pub struct PgEmployeeRepository {
    pool: PgPool,
}

impl PgEmployeeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EmployeeRepository for PgEmployeeRepository {
    async fn create(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        contact_phone: Option<String>,
        contact_email: Option<String>,
        contact_telegram: Option<String>,
        is_owner: bool,
        is_manager: bool,
        is_master: bool,
        manager_branch_id: Option<Uuid>,
    ) -> Result<Employee> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now().naive_utc();

        sqlx::query_as!(
            Employee,
            r#"
            INSERT INTO employees (
                id, created_at, updated_at, contact_phone, contact_email, contact_telegram,
                is_owner, is_manager, is_master, organization_id, manager_branch_id, user_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id, created_at, updated_at, contact_phone, contact_email, contact_telegram,
                      is_owner, is_manager, is_master, organization_id, manager_branch_id, user_id
            "#,
            id,
            now,
            now,
            contact_phone,
            contact_email,
            contact_telegram,
            is_owner,
            is_manager,
            is_master,
            organization_id,
            manager_branch_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn list(
        &self,
        organization_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<EmployeeWithProfile>> {
        sqlx::query_as!(
            EmployeeWithProfile,
            r#"
            SELECT 
                e.id,
                e.created_at as employee_created_at,
                e.updated_at as employee_updated_at,
                e.contact_phone,
                e.contact_email,
                e.contact_telegram,
                e.is_owner,
                e.is_manager,
                e.is_master,
                e.organization_id,
                e.manager_branch_id,
                e.user_id,
                up.first_name,
                up.last_name,
                up.patronymic,
                up.created_at as profile_created_at,
                up.updated_at as profile_updated_at
            FROM employees e
            INNER JOIN user_profiles up ON e.user_id = up.user_id
            WHERE e.organization_id = $1
            ORDER BY e.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            organization_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn get(&self, id: Uuid) -> Result<Option<EmployeeWithProfile>> {
        sqlx::query_as!(
            EmployeeWithProfile,
            r#"
            SELECT 
                e.id,
                e.created_at as employee_created_at,
                e.updated_at as employee_updated_at,
                e.contact_phone,
                e.contact_email,
                e.contact_telegram,
                e.is_owner,
                e.is_manager,
                e.is_master,
                e.organization_id,
                e.manager_branch_id,
                e.user_id,
                up.first_name,
                up.last_name,
                up.patronymic,
                up.created_at as profile_created_at,
                up.updated_at as profile_updated_at
            FROM employees e
            INNER JOIN user_profiles up ON e.user_id = up.user_id
            WHERE e.id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_by_user_and_org(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
    ) -> Result<Option<Employee>> {
        sqlx::query_as!(
            Employee,
            r#"
            SELECT id, created_at, updated_at, contact_phone, contact_email, contact_telegram,
                   is_owner, is_manager, is_master, organization_id, manager_branch_id, user_id
            FROM employees
            WHERE user_id = $1 AND organization_id = $2
            "#,
            user_id,
            organization_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn update(
        &self,
        id: Uuid,
        contact_phone: Option<Option<String>>,
        contact_email: Option<Option<String>>,
        contact_telegram: Option<Option<String>>,
        user_id: Option<Uuid>,
        is_owner: Option<bool>,
        is_manager: Option<bool>,
        is_master: Option<bool>,
        manager_branch_id: Option<Option<Uuid>>,
    ) -> Result<Option<EmployeeWithProfile>> {
        let now = chrono::Utc::now().naive_utc();

        // First, get the current employee to preserve unchanged fields
        let current = sqlx::query_as!(
            Employee,
            r#"
            SELECT id, created_at, updated_at, contact_phone, contact_email, contact_telegram,
                   is_owner, is_manager, is_master, organization_id, manager_branch_id, user_id
            FROM employees
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if current.is_none() {
            return Ok(None);
        }
        let current = current.unwrap();

        // Apply updates with Option<Option<T>> pattern for nullable fields
        let contact_phone_value = match contact_phone {
            Some(Some(val)) => Some(val),
            Some(None) => None,
            None => current.contact_phone,
        };

        let contact_email_value = match contact_email {
            Some(Some(val)) => Some(val),
            Some(None) => None,
            None => current.contact_email,
        };

        let contact_telegram_value = match contact_telegram {
            Some(Some(val)) => Some(val),
            Some(None) => None,
            None => current.contact_telegram,
        };

        let manager_branch_id_value = match manager_branch_id {
            Some(Some(val)) => Some(val),
            Some(None) => None,
            None => current.manager_branch_id,
        };

        // Update the employee
        sqlx::query!(
            r#"
            UPDATE employees
            SET updated_at = $2,
                contact_phone = $3,
                contact_email = $4,
                contact_telegram = $5,
                user_id = COALESCE($6, user_id),
                is_owner = COALESCE($7, is_owner),
                is_manager = COALESCE($8, is_manager),
                is_master = COALESCE($9, is_master),
                manager_branch_id = $10
            WHERE id = $1
            "#,
            id,
            now,
            contact_phone_value,
            contact_email_value,
            contact_telegram_value,
            user_id,
            is_owner,
            is_manager,
            is_master,
            manager_branch_id_value
        )
        .execute(&self.pool)
        .await?;

        // Return the updated employee with profile
        self.get(id).await
    }

    async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM employees
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_organization_id(&self, employee_id: Uuid) -> Result<Option<Uuid>> {
        let result = sqlx::query!(
            r#"
            SELECT organization_id
            FROM employees
            WHERE id = $1
            "#,
            employee_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|r| r.organization_id))
    }
}
