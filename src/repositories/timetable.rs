use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::models::timetable::{
    db::{DayRedefinition, ScheduleDay, Timetable},
    request::{CreateDayRedefinitionRequest, CreateTimetableRequest, UpdateTimetableRequest},
};
use crate::services::errors::{ServiceError, ServiceResult};

#[async_trait]
pub trait TimetableRepository: Send + Sync {
    async fn create_timetable(&self, request: &CreateTimetableRequest) -> ServiceResult<()>;
    async fn get_timetables(&self, organization_id: Option<Uuid>) -> ServiceResult<Vec<Timetable>>;
    async fn get_timetable(&self, master_id: Uuid) -> ServiceResult<Option<Timetable>>;
    async fn get_schedule_days(&self, master_id: Uuid) -> ServiceResult<Vec<ScheduleDay>>;
    async fn get_day_redefinitions(&self, master_id: Uuid) -> ServiceResult<Vec<DayRedefinition>>;
    async fn update_timetable(
        &self,
        master_id: Uuid,
        request: &UpdateTimetableRequest,
    ) -> ServiceResult<Timetable>;
    async fn delete_timetable(&self, master_id: Uuid) -> ServiceResult<()>;
    async fn create_day_redefinition(
        &self,
        request: &CreateDayRedefinitionRequest,
    ) -> ServiceResult<()>;
    async fn delete_day_redefinition(&self, master_id: Uuid, date: NaiveDate) -> ServiceResult<()>;
}

pub struct TimetableRepositoryImpl {
    pool: PgPool,
}

impl TimetableRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TimetableRepository for TimetableRepositoryImpl {
    async fn create_timetable(&self, request: &CreateTimetableRequest) -> ServiceResult<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(ServiceError::DatabaseError)?;

        // Insert timetable
        sqlx::query!(
            r#"
            INSERT INTO timetables (master_id, recurrence_cycle_start)
            VALUES ($1, $2)
            "#,
            request.master_id,
            request.recurrence_cycle_start
        )
        .execute(&mut *tx)
        .await
        .map_err(ServiceError::DatabaseError)?;

        // Insert schedule days
        for (day_ordinal, schedule_day) in request.schedule_days.iter().enumerate() {
            let day_data = serde_json::to_value(schedule_day).map_err(|e| {
                ServiceError::ValidationError(format!("Failed to serialize schedule day: {}", e))
            })?;

            sqlx::query!(
                r#"
                INSERT INTO schedule_days (master_id, day_ordinal, day_data)
                VALUES ($1, $2, $3)
                "#,
                request.master_id,
                day_ordinal as i32,
                day_data
            )
            .execute(&mut *tx)
            .await
            .map_err(ServiceError::DatabaseError)?;
        }

        tx.commit().await.map_err(ServiceError::DatabaseError)?;
        Ok(())
    }

    async fn get_timetables(&self, organization_id: Option<Uuid>) -> ServiceResult<Vec<Timetable>> {
        let timetables = if let Some(org_id) = organization_id {
            sqlx::query_as!(
                Timetable,
                r#"
                SELECT t.master_id, t.recurrence_cycle_start
                FROM timetables t
                JOIN employees e ON t.master_id = e.id
                WHERE e.organization_id = $1
                ORDER BY t.recurrence_cycle_start DESC
                "#,
                org_id
            )
            .fetch_all(&self.pool)
            .await
            .map_err(ServiceError::DatabaseError)?
        } else {
            sqlx::query_as!(
                Timetable,
                r#"
                SELECT master_id, recurrence_cycle_start
                FROM timetables
                ORDER BY recurrence_cycle_start DESC
                "#
            )
            .fetch_all(&self.pool)
            .await
            .map_err(ServiceError::DatabaseError)?
        };

        Ok(timetables)
    }

    async fn get_timetable(&self, master_id: Uuid) -> ServiceResult<Option<Timetable>> {
        let timetable = sqlx::query_as!(
            Timetable,
            r#"
            SELECT master_id, recurrence_cycle_start
            FROM timetables
            WHERE master_id = $1
            "#,
            master_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(ServiceError::DatabaseError)?;

        Ok(timetable)
    }

    async fn get_schedule_days(&self, master_id: Uuid) -> ServiceResult<Vec<ScheduleDay>> {
        let schedule_days = sqlx::query_as!(
            ScheduleDay,
            r#"
            SELECT master_id, created_at, updated_at, day_ordinal, day_data
            FROM schedule_days
            WHERE master_id = $1
            ORDER BY day_ordinal
            "#,
            master_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(ServiceError::DatabaseError)?;

        Ok(schedule_days)
    }

    async fn get_day_redefinitions(&self, master_id: Uuid) -> ServiceResult<Vec<DayRedefinition>> {
        let redefinitions = sqlx::query_as!(
            DayRedefinition,
            r#"
            SELECT master_id, created_at, updated_at, date, day_data
            FROM day_redefinitions
            WHERE master_id = $1
            ORDER BY date DESC
            "#,
            master_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(ServiceError::DatabaseError)?;

        Ok(redefinitions)
    }

    async fn update_timetable(
        &self,
        master_id: Uuid,
        request: &UpdateTimetableRequest,
    ) -> ServiceResult<Timetable> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(ServiceError::DatabaseError)?;

        // Update timetable if needed
        if let Some(cycle_start) = request.recurrence_cycle_start {
            sqlx::query!(
                r#"
                UPDATE timetables
                SET recurrence_cycle_start = $2
                WHERE master_id = $1
                "#,
                master_id,
                cycle_start
            )
            .execute(&mut *tx)
            .await
            .map_err(ServiceError::DatabaseError)?;
        }

        // Update schedule days if provided
        if let Some(schedule_days) = &request.schedule_days {
            // Delete existing schedule days
            sqlx::query!("DELETE FROM schedule_days WHERE master_id = $1", master_id)
                .execute(&mut *tx)
                .await
                .map_err(ServiceError::DatabaseError)?;

            // Insert new schedule days
            for (day_ordinal, schedule_day) in schedule_days.iter().enumerate() {
                let day_data = serde_json::to_value(schedule_day).map_err(|e| {
                    ServiceError::ValidationError(format!(
                        "Failed to serialize schedule day: {}",
                        e
                    ))
                })?;

                sqlx::query!(
                    r#"
                    INSERT INTO schedule_days (master_id, day_ordinal, day_data)
                    VALUES ($1, $2, $3)
                    "#,
                    master_id,
                    day_ordinal as i32,
                    day_data
                )
                .execute(&mut *tx)
                .await
                .map_err(ServiceError::DatabaseError)?;
            }

            // No need to update duration - it's calculated from schedule_days count
        }

        tx.commit().await.map_err(ServiceError::DatabaseError)?;

        // Fetch and return updated timetable
        let timetable = self
            .get_timetable(master_id)
            .await?
            .ok_or_else(|| ServiceError::NotFound("Timetable not found".to_string()))?;

        Ok(timetable)
    }

    async fn delete_timetable(&self, master_id: Uuid) -> ServiceResult<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(ServiceError::DatabaseError)?;

        // Delete schedule days first (FK constraint)
        sqlx::query!("DELETE FROM schedule_days WHERE master_id = $1", master_id)
            .execute(&mut *tx)
            .await
            .map_err(ServiceError::DatabaseError)?;

        // Delete day redefinitions
        sqlx::query!(
            "DELETE FROM day_redefinitions WHERE master_id = $1",
            master_id
        )
        .execute(&mut *tx)
        .await
        .map_err(ServiceError::DatabaseError)?;

        // Delete timetable
        let result = sqlx::query!("DELETE FROM timetables WHERE master_id = $1", master_id)
            .execute(&mut *tx)
            .await
            .map_err(ServiceError::DatabaseError)?;

        if result.rows_affected() == 0 {
            return Err(ServiceError::NotFound("Timetable not found".to_string()));
        }

        tx.commit().await.map_err(ServiceError::DatabaseError)?;
        Ok(())
    }

    async fn create_day_redefinition(
        &self,
        request: &CreateDayRedefinitionRequest,
    ) -> ServiceResult<()> {
        let day_data = serde_json::to_value(&request.schedule_day).map_err(|e| {
            ServiceError::ValidationError(format!("Failed to serialize schedule day: {}", e))
        })?;

        sqlx::query!(
            r#"
            INSERT INTO day_redefinitions (master_id, date, day_data)
            VALUES ($1, $2, $3)
            ON CONFLICT (master_id, date)
            DO UPDATE SET day_data = EXCLUDED.day_data, updated_at = NOW()
            "#,
            request.master_id,
            request.date,
            day_data
        )
        .execute(&self.pool)
        .await
        .map_err(ServiceError::DatabaseError)?;

        Ok(())
    }

    async fn delete_day_redefinition(&self, master_id: Uuid, date: NaiveDate) -> ServiceResult<()> {
        let result = sqlx::query!(
            "DELETE FROM day_redefinitions WHERE master_id = $1 AND date = $2",
            master_id,
            date
        )
        .execute(&self.pool)
        .await
        .map_err(ServiceError::DatabaseError)?;

        if result.rows_affected() == 0 {
            return Err(ServiceError::NotFound(
                "Day redefinition not found".to_string(),
            ));
        }

        Ok(())
    }
}
