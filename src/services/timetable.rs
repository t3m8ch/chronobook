use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    models::timetable::{
        request::{CreateDayRedefinitionRequest, CreateTimetableRequest, UpdateTimetableRequest},
        response::{TimetableOut, TimetableWithRedefinitionsOut},
    },
    services::errors::ServiceResult,
};

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
