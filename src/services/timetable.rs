use async_trait::async_trait;
use chrono::NaiveDate;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::timetable::{
    request::{CreateDayRedefinitionRequest, CreateTimetableRequest, UpdateTimetableRequest},
    response::{DayRedefinitionOut, ScheduleDayOut, TimetableOut, TimetableWithRedefinitionsOut},
};
use crate::repositories::timetable::TimetableRepository;
use crate::services::errors::{ServiceError, ServiceResult};

#[async_trait]
pub trait TimetableService: Send + Sync {
    async fn create_timetable(&self, request: CreateTimetableRequest) -> ServiceResult<()>;
    async fn list_timetables(
        &self,
        organization_id: Option<Uuid>,
    ) -> ServiceResult<Vec<TimetableOut>>;
    async fn get_timetable_with_redefinitions(
        &self,
        master_id: Uuid,
    ) -> ServiceResult<TimetableWithRedefinitionsOut>;
    async fn update_timetable(
        &self,
        master_id: Uuid,
        request: UpdateTimetableRequest,
    ) -> ServiceResult<TimetableOut>;
    async fn delete_timetable(&self, master_id: Uuid) -> ServiceResult<()>;
    async fn create_day_redefinition(
        &self,
        request: CreateDayRedefinitionRequest,
    ) -> ServiceResult<()>;
    async fn delete_day_redefinition(&self, master_id: Uuid, date: NaiveDate) -> ServiceResult<()>;
}

pub struct TimetableServiceImpl {
    timetable_repository: Arc<dyn TimetableRepository>,
}

impl TimetableServiceImpl {
    pub fn new(timetable_repository: Arc<dyn TimetableRepository>) -> Self {
        Self {
            timetable_repository,
        }
    }
}

#[async_trait]
impl TimetableService for TimetableServiceImpl {
    async fn create_timetable(&self, request: CreateTimetableRequest) -> ServiceResult<()> {
        // Check if timetable already exists for this master
        if let Some(_) = self
            .timetable_repository
            .get_timetable(request.master_id)
            .await?
        {
            return Err(ServiceError::ConflictError(
                "Timetable already exists for this master".to_string(),
            ));
        }

        self.timetable_repository.create_timetable(&request).await
    }

    async fn list_timetables(
        &self,
        organization_id: Option<Uuid>,
    ) -> ServiceResult<Vec<TimetableOut>> {
        let timetables = self
            .timetable_repository
            .get_timetables(organization_id)
            .await?;

        let mut timetable_outs = Vec::new();
        for timetable in timetables {
            let schedule_days = self
                .timetable_repository
                .get_schedule_days(timetable.master_id)
                .await?;
            timetable_outs.push(TimetableOut {
                master_id: timetable.master_id,
                recurrence_cycle_start: timetable.recurrence_cycle_start,
                recurrence_cycle_days: schedule_days.len() as i32,
            });
        }

        Ok(timetable_outs)
    }

    async fn get_timetable_with_redefinitions(
        &self,
        master_id: Uuid,
    ) -> ServiceResult<TimetableWithRedefinitionsOut> {
        let timetable = self
            .timetable_repository
            .get_timetable(master_id)
            .await?
            .ok_or_else(|| ServiceError::NotFound("Timetable not found".to_string()))?;

        let schedule_days = self
            .timetable_repository
            .get_schedule_days(master_id)
            .await?;
        let redefinitions = self
            .timetable_repository
            .get_day_redefinitions(master_id)
            .await?;

        let timetable_out = TimetableOut {
            master_id: timetable.master_id,
            recurrence_cycle_start: timetable.recurrence_cycle_start,
            recurrence_cycle_days: schedule_days.len() as i32,
        };

        let schedule_days_out = schedule_days
            .into_iter()
            .map(|day| ScheduleDayOut {
                master_id: day.master_id,
                day_ordinal: day.day_ordinal,
                day_data: day.day_data,
            })
            .collect();

        let redefinitions_out = redefinitions
            .into_iter()
            .map(|redef| DayRedefinitionOut {
                master_id: redef.master_id,
                date: redef.date,
                day_data: redef.day_data,
            })
            .collect();

        Ok(TimetableWithRedefinitionsOut {
            timetable: timetable_out,
            schedule_days: schedule_days_out,
            redefinitions: redefinitions_out,
        })
    }

    async fn update_timetable(
        &self,
        master_id: Uuid,
        request: UpdateTimetableRequest,
    ) -> ServiceResult<TimetableOut> {
        let updated_timetable = self
            .timetable_repository
            .update_timetable(master_id, &request)
            .await?;

        let schedule_days = self
            .timetable_repository
            .get_schedule_days(master_id)
            .await?;
        Ok(TimetableOut {
            master_id: updated_timetable.master_id,
            recurrence_cycle_start: updated_timetable.recurrence_cycle_start,
            recurrence_cycle_days: schedule_days.len() as i32,
        })
    }

    async fn delete_timetable(&self, master_id: Uuid) -> ServiceResult<()> {
        self.timetable_repository.delete_timetable(master_id).await
    }

    async fn create_day_redefinition(
        &self,
        request: CreateDayRedefinitionRequest,
    ) -> ServiceResult<()> {
        // Check if timetable exists for this master
        if self
            .timetable_repository
            .get_timetable(request.master_id)
            .await?
            .is_none()
        {
            return Err(ServiceError::NotFound(
                "Timetable not found for this master".to_string(),
            ));
        }

        self.timetable_repository
            .create_day_redefinition(&request)
            .await
    }

    async fn delete_day_redefinition(&self, master_id: Uuid, date: NaiveDate) -> ServiceResult<()> {
        self.timetable_repository
            .delete_day_redefinition(master_id, date)
            .await
    }
}
