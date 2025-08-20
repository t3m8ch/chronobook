use async_trait::async_trait;
use chrono::{Duration, NaiveDate, NaiveDateTime, Timelike};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    models::{
        booking::{request::CreateBookingRequest, response::BookingOut},
        branch::{db::Branch, response::BranchOut},
        master::response::MasterOut,
        organization::response::OrganizationOut,
        service::response::ServiceOut,
        timetable::{
            db::{DayRedefinition, ScheduleDay},
            request::GetWindowsQuery,
            response::WindowOut,
        },
    },
    repositories::{auth::AuthRepository, booking::BookingRepository},
    services::errors::BookingServiceError,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "dayType")]
enum DayData {
    #[serde(rename = "weekday", rename_all = "camelCase")]
    Weekday {
        branch_id: Uuid,
        working_interval: Interval,
        break_intervals: Vec<Interval>,
    },
    #[serde(rename = "weekend")]
    Weekend,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Interval {
    start: NaiveDateTime,
    end: NaiveDateTime,
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
    ) -> Result<Vec<WindowOut>, BookingServiceError>;

    async fn create_booking(
        &self,
        user_id: Uuid,
        request: &CreateBookingRequest,
    ) -> Result<BookingOut, BookingServiceError>;
}

pub struct BookingServiceImpl {
    booking_repo: Arc<dyn BookingRepository>,
    auth_repo: Arc<dyn AuthRepository>,
}

impl BookingServiceImpl {
    pub fn new(
        booking_repo: Arc<dyn BookingRepository>,
        auth_repo: Arc<dyn AuthRepository>,
    ) -> Self {
        Self {
            booking_repo,
            auth_repo,
        }
    }

    fn round_to_15_minutes(dt: NaiveDateTime) -> NaiveDateTime {
        let minutes = dt.minute();
        let rounded_minutes = (minutes / 15) * 15;
        dt.with_minute(rounded_minutes)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
    }

    fn generate_time_slots(
        start: NaiveDateTime,
        end: NaiveDateTime,
        duration_minutes: i32,
    ) -> Vec<(NaiveDateTime, NaiveDateTime)> {
        let mut slots = Vec::new();
        let duration = Duration::minutes(duration_minutes as i64);
        let step = Duration::minutes(15);

        let mut current = Self::round_to_15_minutes(start);
        let end = Self::round_to_15_minutes(end);

        while current + duration <= end {
            slots.push((current, current + duration));
            current = current + step;
        }

        slots
    }

    fn parse_day_data(day_data: &Value) -> Option<DayData> {
        match serde_json::from_value(day_data.clone()) {
            Ok(data) => Some(data),
            Err(e) => {
                tracing::warn!("Failed to parse day_data: {}, JSON: {}", e, day_data);
                None
            }
        }
    }

    fn get_working_intervals_for_day(
        &self,
        schedule_days: &[ScheduleDay],
        day_redefinitions: &[DayRedefinition],
        timetable_start: NaiveDate,
        target_date: NaiveDate,
        branch_filter: &[Uuid],
    ) -> Vec<(Uuid, Vec<Interval>, Vec<Interval>)> {
        // Check for day redefinition first
        if let Some(redefinition) = day_redefinitions.iter().find(|dr| dr.date == target_date) {
            if let Some(day_data) = Self::parse_day_data(&redefinition.day_data) {
                match day_data {
                    DayData::Weekday {
                        branch_id,
                        working_interval,
                        break_intervals,
                    } => {
                        if branch_filter.is_empty() || branch_filter.contains(&branch_id) {
                            return vec![(branch_id, vec![working_interval], break_intervals)];
                        }
                    }
                    DayData::Weekend => return vec![],
                }
            }
        }

        // Calculate day ordinal from recurrence cycle
        let days_since_start = (target_date - timetable_start).num_days();
        if days_since_start < 0 {
            return vec![];
        }

        // Find the schedule for this day ordinal
        for schedule_day in schedule_days {
            // Check if this schedule matches the target day
            // We need to calculate which day of the cycle we're on
            let cycle_length = schedule_days.len() as i64;
            let day_in_cycle = days_since_start % cycle_length;

            if schedule_day.day_ordinal as i64 == day_in_cycle {
                if let Some(day_data) = Self::parse_day_data(&schedule_day.day_data) {
                    match day_data {
                        DayData::Weekday {
                            branch_id,
                            working_interval,
                            break_intervals,
                        } => {
                            if branch_filter.is_empty() || branch_filter.contains(&branch_id) {
                                return vec![(branch_id, vec![working_interval], break_intervals)];
                            }
                        }
                        DayData::Weekend => return vec![],
                    }
                }
            }
        }

        vec![]
    }

    fn subtract_intervals(
        available: Vec<(NaiveDateTime, NaiveDateTime)>,
        busy: &[(NaiveDateTime, NaiveDateTime)],
    ) -> Vec<(NaiveDateTime, NaiveDateTime)> {
        let mut result = Vec::new();

        for (avail_start, avail_end) in available {
            let mut current_intervals = vec![(avail_start, avail_end)];

            for (busy_start, busy_end) in busy {
                let mut new_intervals = Vec::new();

                for (int_start, int_end) in current_intervals {
                    // No overlap
                    if int_end <= *busy_start || int_start >= *busy_end {
                        new_intervals.push((int_start, int_end));
                        continue;
                    }

                    // Add the part before busy interval
                    if int_start < *busy_start {
                        new_intervals.push((int_start, *busy_start));
                    }

                    // Add the part after busy interval
                    if int_end > *busy_end {
                        new_intervals.push((*busy_end, int_end));
                    }
                }

                current_intervals = new_intervals;
            }

            result.extend(current_intervals);
        }

        result
    }
}

#[async_trait]
impl BookingService for BookingServiceImpl {
    async fn get_organization_by_name(
        &self,
        name: &str,
    ) -> Result<OrganizationOut, BookingServiceError> {
        let org = self
            .booking_repo
            .find_organization_by_name(name)
            .await?
            .ok_or_else(|| BookingServiceError::NotFound("Organization not found".to_string()))?;

        Ok(OrganizationOut {
            id: org.id.to_string(),
            name: org.name,
            display_name: org.display_name,
            description: org.description,
        })
    }

    async fn get_services(
        &self,
        organization_name: &str,
        master_ids: &[Uuid],
    ) -> Result<Vec<ServiceOut>, BookingServiceError> {
        let services = self
            .booking_repo
            .find_services_by_organization(organization_name, master_ids)
            .await?;

        Ok(services
            .into_iter()
            .map(|s| ServiceOut {
                id: s.id,
                name: s.display_name,
                description: s.description,
                duration_minutes: s.duration_minutes as u32,
                price: s.price.map(|p| p.to_string()),
            })
            .collect())
    }

    async fn get_masters(
        &self,
        organization_name: &str,
        branch_ids: &[Uuid],
        service_ids: &[Uuid],
    ) -> Result<Vec<MasterOut>, BookingServiceError> {
        let masters = self
            .booking_repo
            .find_masters_by_organization(organization_name, branch_ids, service_ids)
            .await?;

        Ok(masters
            .into_iter()
            .map(|m| MasterOut {
                id: m.id,
                first_name: m.first_name,
                last_name: m.last_name,
                patronymic: m.patronymic,
                contact_phone: m.contact_phone,
                contact_email: m.contact_email,
                contact_telegram: m.contact_telegram,
            })
            .collect())
    }

    async fn get_master_by_id(&self, master_id: Uuid) -> Result<MasterOut, BookingServiceError> {
        let master = self
            .booking_repo
            .find_master_by_id(master_id)
            .await?
            .ok_or_else(|| BookingServiceError::NotFound("Master not found".to_string()))?;

        Ok(MasterOut {
            id: master.id,
            first_name: master.first_name,
            last_name: master.last_name,
            patronymic: master.patronymic,
            contact_phone: master.contact_phone,
            contact_email: master.contact_email,
            contact_telegram: master.contact_telegram,
        })
    }

    async fn get_branches(
        &self,
        organization_name: &str,
        master_ids: &[Uuid],
    ) -> Result<Vec<BranchOut>, BookingServiceError> {
        let branches = self
            .booking_repo
            .find_branches_by_organization(organization_name, master_ids)
            .await?;

        Ok(branches
            .into_iter()
            .map(|b| BranchOut {
                id: b.id,
                name: b.display_name,
                description: b.description,
                timezone: b.timezone,
                street: b.street,
                house_number: b.house_number,
                apartment_number: b.apartment_number.unwrap_or_default(),
                city: b.city,
                region: b.region,
                country: b.country,
                address_info: b.address_info,
            })
            .collect())
    }

    async fn get_windows(
        &self,
        query: &GetWindowsQuery,
    ) -> Result<Vec<WindowOut>, BookingServiceError> {
        // Get organization to validate it exists
        let org = self
            .booking_repo
            .find_organization_by_name(&query.organization_name)
            .await?
            .ok_or_else(|| BookingServiceError::NotFound("Organization not found".to_string()))?;

        // Get service to know duration
        let service = self
            .booking_repo
            .find_service_by_id(query.service_id)
            .await?
            .ok_or_else(|| BookingServiceError::NotFound("Service not found".to_string()))?;

        // Get all relevant masters
        let masters = if query.masters.is_empty() {
            // Get all masters for organization that work in specified branches (or all branches)
            self.booking_repo
                .find_masters_by_organization(&query.organization_name, &query.branches, &[])
                .await?
        } else {
            // Get specific masters
            let mut masters = Vec::new();
            for master_id in &query.masters {
                if let Some(master) = self.booking_repo.find_master_by_id(*master_id).await? {
                    masters.push(master);
                }
            }
            masters
        };

        // Get all branches for mapping
        let all_branches = self
            .booking_repo
            .find_branches_by_organization(&query.organization_name, &[])
            .await?;
        let branch_map: HashMap<Uuid, Branch> =
            all_branches.into_iter().map(|b| (b.id, b)).collect();

        // Get existing bookings for the time range
        let master_ids: Vec<Uuid> = masters.iter().map(|m| m.id).collect();
        let bookings = self
            .booking_repo
            .find_bookings(
                org.id,
                &master_ids,
                &query.branches,
                query.min_datetime,
                query.max_datetime,
            )
            .await?;

        // Group bookings by master
        let mut bookings_by_master: HashMap<Uuid, Vec<(NaiveDateTime, NaiveDateTime)>> =
            HashMap::new();
        for booking in bookings {
            bookings_by_master
                .entry(booking.master_id)
                .or_insert_with(Vec::new)
                .push((booking.started_at, booking.ended_at));
        }

        // Calculate windows for each master
        let mut windows = Vec::new();

        for master in masters {
            // Get master's timetable
            let timetable = match self.booking_repo.find_master_timetable(master.id).await? {
                Some(t) => t,
                None => continue, // Skip masters without timetables
            };

            // Get schedule days
            let schedule_days = self
                .booking_repo
                .find_master_schedule_days(master.id)
                .await?;
            if schedule_days.is_empty() {
                continue;
            }

            // Get day redefinitions for the period
            let day_redefinitions = self
                .booking_repo
                .find_master_day_redefinitions(master.id, query.min_datetime, query.max_datetime)
                .await?;

            // Process each day in the range
            let mut current_date = query.min_datetime.date();
            let end_date = query.max_datetime.date();

            let mut master_slots = Vec::new();
            let mut branches_used = HashSet::new();

            while current_date <= end_date {
                // Get working intervals for this day
                let day_intervals = self.get_working_intervals_for_day(
                    &schedule_days,
                    &day_redefinitions,
                    timetable.recurrence_cycle_start,
                    current_date,
                    &query.branches,
                );

                for (branch_id, working_intervals, break_intervals) in day_intervals {
                    branches_used.insert(branch_id);

                    // Convert intervals to the current date
                    for working_interval in working_intervals {
                        let work_start = current_date.and_time(working_interval.start.time());
                        let work_end = current_date.and_time(working_interval.end.time());

                        // Ensure we're within the query range
                        let effective_start = work_start.max(query.min_datetime);
                        let effective_end = work_end.min(query.max_datetime);

                        if effective_start >= effective_end {
                            continue;
                        }

                        // Generate all possible slots for this interval
                        let available_slots = Self::generate_time_slots(
                            effective_start,
                            effective_end,
                            service.duration_minutes,
                        );

                        // Subtract break intervals
                        let break_intervals_today: Vec<(NaiveDateTime, NaiveDateTime)> =
                            break_intervals
                                .iter()
                                .map(|bi| {
                                    (
                                        current_date.and_time(bi.start.time()),
                                        current_date.and_time(bi.end.time()),
                                    )
                                })
                                .collect();

                        let slots_after_breaks =
                            Self::subtract_intervals(available_slots, &break_intervals_today);

                        // Subtract existing bookings
                        let empty_vec = Vec::new();
                        let master_bookings =
                            bookings_by_master.get(&master.id).unwrap_or(&empty_vec);
                        let final_slots =
                            Self::subtract_intervals(slots_after_breaks, master_bookings);

                        master_slots.extend(final_slots);
                    }
                }

                current_date = current_date.succ_opt().unwrap();
            }

            // Create window for this master if there are any slots
            if !master_slots.is_empty() {
                // For simplicity, we'll return one window per master with all their slots
                // In production, you might want to group by branch or day
                let branch_id = branches_used.into_iter().next();

                if let Some(branch_id) = branch_id {
                    if let Some(branch) = branch_map.get(&branch_id) {
                        windows.push(WindowOut {
                            id: Uuid::new_v4(), // Generate unique ID for this window
                            slots: master_slots,
                            master: MasterOut {
                                id: master.id,
                                first_name: master.first_name,
                                last_name: master.last_name,
                                patronymic: master.patronymic,
                                contact_phone: master.contact_phone,
                                contact_email: master.contact_email,
                                contact_telegram: master.contact_telegram,
                            },
                            branch: BranchOut {
                                id: branch.id,
                                name: branch.display_name.clone(),
                                description: branch.description.clone(),
                                timezone: branch.timezone.clone(),
                                street: branch.street.clone(),
                                house_number: branch.house_number.clone(),
                                apartment_number: branch
                                    .apartment_number
                                    .clone()
                                    .unwrap_or_default(),
                                city: branch.city.clone(),
                                region: branch.region.clone(),
                                country: branch.country.clone(),
                                address_info: branch.address_info.clone(),
                            },
                        });
                    }
                }
            }
        }

        Ok(windows)
    }

    async fn create_booking(
        &self,
        user_id: Uuid,
        request: &CreateBookingRequest,
    ) -> Result<BookingOut, BookingServiceError> {
        // Validate times are 15-minute aligned
        if request.start.minute() % 15 != 0 || request.start.second() != 0 {
            return Err(BookingServiceError::ValidationError(
                "Start time must be aligned to 15-minute intervals".to_string(),
            ));
        }
        if request.end.minute() % 15 != 0 || request.end.second() != 0 {
            return Err(BookingServiceError::ValidationError(
                "End time must be aligned to 15-minute intervals".to_string(),
            ));
        }

        // Find organization by name
        let organization = self
            .booking_repo
            .find_organization_by_name(&request.organization_name)
            .await?
            .ok_or_else(|| BookingServiceError::NotFound("Organization not found".to_string()))?;

        // Find or create customer for this user and organization
        let customer = match self
            .auth_repo
            .find_customer(user_id, organization.id)
            .await?
        {
            Some(customer) => customer,
            None => {
                // Auto-create customer for authenticated user
                self.auth_repo
                    .create_customer(user_id, organization.id)
                    .await?
            }
        };

        // Check for conflicts
        let master = self
            .booking_repo
            .find_master_by_id(request.master_id)
            .await?
            .ok_or_else(|| BookingServiceError::NotFound("Master not found".to_string()))?;

        // Verify that the master belongs to the same organization
        if master.organization_id != organization.id {
            return Err(BookingServiceError::ValidationError(
                "Master does not belong to this organization".to_string(),
            ));
        }

        let conflicts = self
            .booking_repo
            .find_bookings(
                organization.id,
                &[request.master_id],
                &[],
                request.start,
                request.end,
            )
            .await?;

        if !conflicts.is_empty() {
            return Err(BookingServiceError::ConflictError(
                "Time slot is already booked".to_string(),
            ));
        }

        // Convert notify methods
        let notify_methods: Vec<String> = request
            .notify_methods
            .iter()
            .map(|m| match m {
                crate::models::booking::request::NotifyMethod::Sms => "sms".to_string(),
                crate::models::booking::request::NotifyMethod::Telegram => "telegram".to_string(),
            })
            .collect();

        // Create booking
        let booking = self
            .booking_repo
            .create_booking(
                customer.id,
                request.service_id,
                request.master_id,
                request.branch_id,
                request.start,
                request.end,
                &notify_methods,
            )
            .await?;

        Ok(BookingOut {
            id: booking.id,
            service_id: booking.service_id,
            master_id: booking.master_id,
            branch_id: booking.branch_id,
            start: booking.started_at,
            end: booking.ended_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveTime;

    #[test]
    fn test_round_to_15_minutes() {
        let test_cases = vec![
            ("2024-01-01T10:00:00", "2024-01-01T10:00:00"),
            ("2024-01-01T10:01:00", "2024-01-01T10:00:00"),
            ("2024-01-01T10:14:59", "2024-01-01T10:00:00"),
            ("2024-01-01T10:15:00", "2024-01-01T10:15:00"),
            ("2024-01-01T10:16:00", "2024-01-01T10:15:00"),
            ("2024-01-01T10:29:59", "2024-01-01T10:15:00"),
            ("2024-01-01T10:30:00", "2024-01-01T10:30:00"),
            ("2024-01-01T10:44:59", "2024-01-01T10:30:00"),
            ("2024-01-01T10:45:00", "2024-01-01T10:45:00"),
            ("2024-01-01T10:59:59", "2024-01-01T10:45:00"),
        ];

        for (input, expected) in test_cases {
            let dt = NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S").unwrap();
            let expected_dt = NaiveDateTime::parse_from_str(expected, "%Y-%m-%dT%H:%M:%S").unwrap();
            let result = BookingServiceImpl::round_to_15_minutes(dt);
            assert_eq!(result, expected_dt, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_generate_time_slots_one_hour_service() {
        let start =
            NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2024-01-01T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let slots = BookingServiceImpl::generate_time_slots(start, end, 60);

        assert_eq!(slots.len(), 5);
        assert_eq!(
            slots[0],
            (
                NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T11:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
            )
        );
        assert_eq!(
            slots[1],
            (
                NaiveDateTime::parse_from_str("2024-01-01T10:15:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T11:15:00", "%Y-%m-%dT%H:%M:%S").unwrap()
            )
        );
        assert_eq!(
            slots[4],
            (
                NaiveDateTime::parse_from_str("2024-01-01T11:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
            )
        );
    }

    #[test]
    fn test_generate_time_slots_30_minute_service() {
        let start =
            NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2024-01-01T11:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let slots = BookingServiceImpl::generate_time_slots(start, end, 30);

        assert_eq!(slots.len(), 3);
        assert_eq!(
            slots[0],
            (
                NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T10:30:00", "%Y-%m-%dT%H:%M:%S").unwrap()
            )
        );
        assert_eq!(
            slots[1],
            (
                NaiveDateTime::parse_from_str("2024-01-01T10:15:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T10:45:00", "%Y-%m-%dT%H:%M:%S").unwrap()
            )
        );
        assert_eq!(
            slots[2],
            (
                NaiveDateTime::parse_from_str("2024-01-01T10:30:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T11:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
            )
        );
    }

    #[test]
    fn test_generate_time_slots_15_minute_service() {
        let start =
            NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2024-01-01T10:45:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let slots = BookingServiceImpl::generate_time_slots(start, end, 15);

        assert_eq!(slots.len(), 3);
        assert_eq!(
            slots[0],
            (
                NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T10:15:00", "%Y-%m-%dT%H:%M:%S").unwrap()
            )
        );
    }

    #[test]
    fn test_generate_time_slots_not_aligned_times() {
        let start =
            NaiveDateTime::parse_from_str("2024-01-01T10:03:17", "%Y-%m-%dT%H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2024-01-01T11:58:42", "%Y-%m-%dT%H:%M:%S").unwrap();

        let slots = BookingServiceImpl::generate_time_slots(start, end, 60);

        // Should round start to 10:00 and end to 11:45
        // For 60-minute service: 10:00-11:00, 10:15-11:15, 10:30-11:30, 10:45-11:45
        assert_eq!(slots.len(), 4);
        assert_eq!(
            slots[0],
            (
                NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T11:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
            )
        );
        assert_eq!(
            slots[3],
            (
                NaiveDateTime::parse_from_str("2024-01-01T10:45:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T11:45:00", "%Y-%m-%dT%H:%M:%S").unwrap()
            )
        );
    }

    #[test]
    fn test_subtract_intervals_no_overlap() {
        let available = vec![(
            NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-01-01T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )];

        let busy = vec![
            (
                NaiveDateTime::parse_from_str("2024-01-01T08:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T09:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            ),
            (
                NaiveDateTime::parse_from_str("2024-01-01T13:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T14:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            ),
        ];

        let result = BookingServiceImpl::subtract_intervals(available.clone(), &busy);
        assert_eq!(result, available);
    }

    #[test]
    fn test_subtract_intervals_complete_overlap() {
        let available = vec![(
            NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-01-01T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )];

        let busy = vec![(
            NaiveDateTime::parse_from_str("2024-01-01T09:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-01-01T13:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )];

        let result = BookingServiceImpl::subtract_intervals(available, &busy);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_subtract_intervals_partial_overlap_start() {
        let available = vec![(
            NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-01-01T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )];

        let busy = vec![(
            NaiveDateTime::parse_from_str("2024-01-01T09:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-01-01T10:30:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )];

        let result = BookingServiceImpl::subtract_intervals(available, &busy);
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            (
                NaiveDateTime::parse_from_str("2024-01-01T10:30:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            )
        );
    }

    #[test]
    fn test_subtract_intervals_partial_overlap_end() {
        let available = vec![(
            NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-01-01T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )];

        let busy = vec![(
            NaiveDateTime::parse_from_str("2024-01-01T11:30:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-01-01T13:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )];

        let result = BookingServiceImpl::subtract_intervals(available, &busy);
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            (
                NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T11:30:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            )
        );
    }

    #[test]
    fn test_subtract_intervals_middle_overlap() {
        let available = vec![(
            NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-01-01T14:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )];

        let busy = vec![(
            NaiveDateTime::parse_from_str("2024-01-01T11:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-01-01T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )];

        let result = BookingServiceImpl::subtract_intervals(available, &busy);
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            (
                NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T11:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            )
        );
        assert_eq!(
            result[1],
            (
                NaiveDateTime::parse_from_str("2024-01-01T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T14:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            )
        );
    }

    #[test]
    fn test_subtract_intervals_multiple_busy_periods() {
        let available = vec![(
            NaiveDateTime::parse_from_str("2024-01-01T09:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-01-01T17:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )];

        // Simulate breaks and existing bookings
        let busy = vec![
            // Morning booking
            (
                NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T11:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            ),
            // Lunch break
            (
                NaiveDateTime::parse_from_str("2024-01-01T13:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T14:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            ),
            // Afternoon booking
            (
                NaiveDateTime::parse_from_str("2024-01-01T15:30:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T16:30:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            ),
        ];

        let result = BookingServiceImpl::subtract_intervals(available, &busy);
        assert_eq!(result.len(), 4);

        // Check resulting free intervals
        assert_eq!(
            result[0],
            (
                NaiveDateTime::parse_from_str("2024-01-01T09:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            )
        );
        assert_eq!(
            result[1],
            (
                NaiveDateTime::parse_from_str("2024-01-01T11:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T13:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            )
        );
        assert_eq!(
            result[2],
            (
                NaiveDateTime::parse_from_str("2024-01-01T14:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T15:30:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            )
        );
        assert_eq!(
            result[3],
            (
                NaiveDateTime::parse_from_str("2024-01-01T16:30:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T17:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            )
        );
    }

    #[test]
    fn test_simple_interval_parse() {
        // First test if we can parse a simple Interval
        let interval_json = serde_json::json!({
            "start": "2024-01-01T09:00:00",
            "end": "2024-01-01T18:00:00"
        });

        let interval_result = serde_json::from_value::<Interval>(interval_json);
        if let Err(e) = &interval_result {
            println!("Failed to parse Interval: {}", e);
        }
        assert!(interval_result.is_ok());
    }

    #[test]
    fn test_parse_day_data_weekday() {
        let json = serde_json::json!({
            "dayType": "weekday",
            "branchId": "550e8400-e29b-41d4-a716-446655440000",
            "workingInterval": {
                "start": "2024-01-01T09:00:00",
                "end": "2024-01-01T18:00:00"
            },
            "breakIntervals": [
                {
                    "start": "2024-01-01T13:00:00",
                    "end": "2024-01-01T14:00:00"
                }
            ]
        });

        let result = BookingServiceImpl::parse_day_data(&json);
        if result.is_none() {
            println!("Failed to parse JSON: {}", json);
        }
        assert!(result.is_some());

        match result.unwrap() {
            DayData::Weekday {
                branch_id,
                working_interval,
                break_intervals,
            } => {
                assert_eq!(
                    branch_id.to_string(),
                    "550e8400-e29b-41d4-a716-446655440000"
                );
                assert_eq!(
                    working_interval.start.time(),
                    NaiveTime::from_hms_opt(9, 0, 0).unwrap()
                );
                assert_eq!(
                    working_interval.end.time(),
                    NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                );
                assert_eq!(break_intervals.len(), 1);
                assert_eq!(
                    break_intervals[0].start.time(),
                    NaiveTime::from_hms_opt(13, 0, 0).unwrap()
                );
                assert_eq!(
                    break_intervals[0].end.time(),
                    NaiveTime::from_hms_opt(14, 0, 0).unwrap()
                );
            }
            DayData::Weekend => panic!("Expected Weekday, got Weekend"),
        }
    }

    #[test]
    fn test_parse_day_data_weekend() {
        let json = serde_json::json!({
            "dayType": "weekend"
        });

        let result = BookingServiceImpl::parse_day_data(&json);
        assert!(result.is_some());

        match result.unwrap() {
            DayData::Weekend => {} // Success
            DayData::Weekday { .. } => panic!("Expected Weekend, got Weekday"),
        }
    }

    #[test]
    fn test_complex_scenario_with_breaks_and_bookings() {
        // Simulate a working day from 9:00 to 18:00 with lunch break 13:00-14:00
        // And existing bookings at 10:00-11:00 and 15:00-16:00
        // Service duration is 1 hour

        let available = vec![(
            NaiveDateTime::parse_from_str("2024-01-01T09:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-01-01T18:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )];

        // First subtract lunch break
        let lunch_break = vec![(
            NaiveDateTime::parse_from_str("2024-01-01T13:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-01-01T14:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )];

        let after_break = BookingServiceImpl::subtract_intervals(available, &lunch_break);

        // Then subtract existing bookings
        let bookings = vec![
            (
                NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T11:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            ),
            (
                NaiveDateTime::parse_from_str("2024-01-01T15:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T16:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            ),
        ];

        let final_free = BookingServiceImpl::subtract_intervals(after_break, &bookings);

        // Now generate 1-hour slots with 15-minute steps
        let mut all_slots = Vec::new();
        for (start, end) in final_free {
            let slots = BookingServiceImpl::generate_time_slots(start, end, 60);
            all_slots.extend(slots);
        }

        // We should have:
        // 09:00-10:00 (1 slot starting at 09:00)
        // 11:00-13:00 (5 slots: 11:00, 11:15, 11:30, 11:45, 12:00)
        // 14:00-15:00 (1 slot starting at 14:00)
        // 16:00-18:00 (5 slots: 16:00, 16:15, 16:30, 16:45, 17:00)
        assert_eq!(all_slots.len(), 12);

        // Verify first available slot
        assert_eq!(
            all_slots[0],
            (
                NaiveDateTime::parse_from_str("2024-01-01T09:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            )
        );

        // Verify last available slot
        assert_eq!(
            all_slots[11],
            (
                NaiveDateTime::parse_from_str("2024-01-01T17:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2024-01-01T18:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            )
        );
    }
}
