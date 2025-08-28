use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    models::{
        booking::{request::CreateBookingRequest, response::BookingOut},
        branch::response::BranchOut,
        master::response::MasterOut,
        organization::response::OrganizationOut,
        service::response::ServiceOut,
        timetable::request::GetWindowsQuery,
    },
    services::errors::BookingServiceError,
};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", tag = "dayType")]
pub enum DayData {
    #[serde(rename = "weekday", rename_all = "camelCase")]
    Weekday {
        branch_id: Uuid,
        working_interval: Interval,
        break_intervals: Vec<Interval>,
    },
    #[serde(rename = "weekend", rename_all = "camelCase")]
    Weekend,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct Window {
    pub id: Uuid,
    pub slots: Vec<Interval>,
    pub master: MasterOut,
    pub branch: BranchOut,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct Interval {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl Interval {
    pub fn new(start: NaiveDateTime, end: NaiveDateTime) -> Self {
        Self { start, end }
    }
}

#[mockall::automock]
#[async_trait]
pub trait BookingService: Send + Sync {
    async fn get_organization_by_name(
        &self,
        name: &str,
    ) -> Result<OrganizationOut, BookingServiceError>;

    async fn get_services(
        &self,
        organization_name: &str,
        master_ids: &[Uuid],
    ) -> Result<Vec<ServiceOut>, BookingServiceError>;

    async fn get_masters(
        &self,
        organization_name: &str,
        branch_ids: &[Uuid],
        service_ids: &[Uuid],
    ) -> Result<Vec<MasterOut>, BookingServiceError>;

    async fn get_master_by_id(&self, master_id: Uuid) -> Result<MasterOut, BookingServiceError>;

    async fn get_branches(
        &self,
        organization_name: &str,
        master_ids: &[Uuid],
    ) -> Result<Vec<BranchOut>, BookingServiceError>;

    async fn get_windows(
        &self,
        query: &GetWindowsQuery,
    ) -> Result<Vec<Window>, BookingServiceError>;

    async fn create_booking(
        &self,
        user_id: Uuid,
        request: &CreateBookingRequest,
    ) -> Result<BookingOut, BookingServiceError>;
}
