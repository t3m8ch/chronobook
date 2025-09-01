use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::dashboard::{db::OrganizationDashboard, response::OrganizationDashboardOut};

#[async_trait]
pub trait DashboardRepository: Send + Sync {
    async fn get_organization_dashboard(
        &self,
        organization_id: Uuid,
    ) -> Result<OrganizationDashboardOut, sqlx::Error>;
}

pub struct DashboardRepositoryImpl {
    pool: PgPool,
}

impl DashboardRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DashboardRepository for DashboardRepositoryImpl {
    async fn get_organization_dashboard(
        &self,
        organization_id: Uuid,
    ) -> Result<OrganizationDashboardOut, sqlx::Error> {
        let organization = sqlx::query_as::<_, OrganizationDashboard>(
            r#"
            SELECT
                o.id,
                o.name,
                o.display_name,
                o.description,
                EXISTS(SELECT 1 FROM branches b WHERE b.organization_id = o.id) as has_branch,
                EXISTS(
                    SELECT 1 FROM employees e
                    WHERE e.organization_id = o.id AND e.is_master = true
                ) as has_master,
                EXISTS(
                    SELECT 1 FROM timetables t
                    JOIN employees e ON t.master_id = e.id
                    WHERE e.organization_id = o.id
                ) as has_timetable,
                EXISTS(
                    SELECT 1 FROM services s
                    JOIN employees e ON s.master_id = e.id
                    WHERE e.organization_id = o.id
                ) as has_service
            FROM organizations o
            WHERE o.id = $1
            "#,
        )
        .bind(organization_id)
        .fetch_optional(&self.pool)
        .await?;

        match organization {
            Some(org) => {
                let active =
                    org.has_branch && org.has_master && org.has_timetable && org.has_service;

                Ok(OrganizationDashboardOut {
                    id: org.id.to_string(),
                    name: org.name,
                    display_name: org.display_name,
                    description: org.description,
                    active,
                    al_least_one_branch: org.has_branch,
                    al_least_one_master: org.has_master,
                    al_least_one_timetable: org.has_timetable,
                    al_least_one_service: org.has_service,
                })
            }
            None => Err(sqlx::Error::RowNotFound),
        }
    }
}
