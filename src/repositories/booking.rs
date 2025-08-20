use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{PgPool, Result};
use uuid::Uuid;

use crate::models::{
    auth::db::Organization,
    booking::db::Booking,
    branch::db::Branch,
    employee::db::EmployeeWithProfile,
    service::db::Service,
    timetable::db::{DayRedefinition, ScheduleDay, Timetable},
};

#[mockall::automock]
#[async_trait]
pub trait BookingRepository: Send + Sync {
    async fn find_organization_by_name(&self, name: &str) -> Result<Option<Organization>>;

    async fn find_services_by_organization(
        &self,
        organization_name: &str,
        master_ids: &[Uuid],
    ) -> Result<Vec<Service>>;

    async fn find_masters_by_organization(
        &self,
        organization_name: &str,
        branch_ids: &[Uuid],
        service_ids: &[Uuid],
    ) -> Result<Vec<EmployeeWithProfile>>;

    async fn find_master_by_id(&self, master_id: Uuid) -> Result<Option<EmployeeWithProfile>>;

    async fn find_branches_by_organization(
        &self,
        organization_name: &str,
        master_ids: &[Uuid],
    ) -> Result<Vec<Branch>>;

    async fn find_master_timetable(&self, master_id: Uuid) -> Result<Option<Timetable>>;

    async fn find_master_schedule_days(&self, master_id: Uuid) -> Result<Vec<ScheduleDay>>;

    async fn find_master_day_redefinitions(
        &self,
        master_id: Uuid,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
    ) -> Result<Vec<DayRedefinition>>;

    async fn find_bookings(
        &self,
        organization_id: Uuid,
        master_ids: &[Uuid],
        branch_ids: &[Uuid],
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> Result<Vec<Booking>>;

    async fn find_service_by_id(&self, service_id: Uuid) -> Result<Option<Service>>;

    async fn create_booking(
        &self,
        customer_id: Uuid,
        service_id: Uuid,
        master_id: Uuid,
        branch_id: Uuid,
        started_at: NaiveDateTime,
        ended_at: NaiveDateTime,
        notify_methods: &[String],
    ) -> Result<Booking>;
}

pub struct PgBookingRepository {
    pool: PgPool,
}

impl PgBookingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BookingRepository for PgBookingRepository {
    async fn find_organization_by_name(&self, name: &str) -> Result<Option<Organization>> {
        sqlx::query_as!(
            Organization,
            r#"
            SELECT id, created_at, updated_at, name, display_name, description
            FROM organizations
            WHERE name = $1
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_services_by_organization(
        &self,
        organization_name: &str,
        master_ids: &[Uuid],
    ) -> Result<Vec<Service>> {
        if master_ids.is_empty() {
            // No filter, return all services for organization
            sqlx::query_as!(
                Service,
                r#"
                SELECT s.id, s.created_at, s.updated_at, s.display_name, 
                       s.description, s.duration_minutes, s.price, s.master_id
                FROM services s
                LEFT JOIN employees e ON s.master_id = e.id
                LEFT JOIN organizations o ON e.organization_id = o.id
                WHERE o.name = $1 OR s.master_id IS NULL
                GROUP BY s.id
                "#,
                organization_name
            )
            .fetch_all(&self.pool)
            .await
        } else {
            // Filter by master IDs (OR logic)
            sqlx::query_as!(
                Service,
                r#"
                SELECT DISTINCT s.id, s.created_at, s.updated_at, s.display_name, 
                       s.description, s.duration_minutes, s.price, s.master_id
                FROM services s
                WHERE s.master_id = ANY($1) OR s.master_id IS NULL
                "#,
                master_ids
            )
            .fetch_all(&self.pool)
            .await
        }
    }

    async fn find_masters_by_organization(
        &self,
        organization_name: &str,
        branch_ids: &[Uuid],
        service_ids: &[Uuid],
    ) -> Result<Vec<EmployeeWithProfile>> {
        // Base query for all cases
        if branch_ids.is_empty() && service_ids.is_empty() {
            sqlx::query_as!(
                EmployeeWithProfile,
                r#"
                SELECT e.id, e.created_at as employee_created_at, e.updated_at as employee_updated_at,
                       e.contact_phone, e.contact_email, e.contact_telegram,
                       e.is_owner, e.is_manager, e.is_master,
                       e.organization_id, e.manager_branch_id, e.user_id,
                       up.first_name, up.last_name, up.patronymic,
                       up.created_at as profile_created_at, up.updated_at as profile_updated_at
                FROM employees e
                INNER JOIN organizations o ON e.organization_id = o.id
                INNER JOIN user_profiles up ON e.user_id = up.user_id
                WHERE o.name = $1 AND e.is_master = true
                "#,
                organization_name
            )
            .fetch_all(&self.pool)
            .await
        } else if !branch_ids.is_empty() && service_ids.is_empty() {
            sqlx::query_as!(
                EmployeeWithProfile,
                r#"
                SELECT DISTINCT e.id, e.created_at as employee_created_at, e.updated_at as employee_updated_at,
                       e.contact_phone, e.contact_email, e.contact_telegram,
                       e.is_owner, e.is_manager, e.is_master,
                       e.organization_id, e.manager_branch_id, e.user_id,
                       up.first_name, up.last_name, up.patronymic,
                       up.created_at as profile_created_at, up.updated_at as profile_updated_at
                FROM employees e
                INNER JOIN organizations o ON e.organization_id = o.id
                INNER JOIN user_profiles up ON e.user_id = up.user_id
                WHERE o.name = $1 AND e.is_master = true
                AND EXISTS (
                    SELECT 1 FROM schedule_days sd
                    WHERE sd.master_id = e.id 
                    AND (sd.day_data->>'branch_id')::uuid = ANY($2)
                )
                "#,
                organization_name,
                branch_ids
            )
            .fetch_all(&self.pool)
            .await
        } else if branch_ids.is_empty() && !service_ids.is_empty() {
            sqlx::query_as!(
                EmployeeWithProfile,
                r#"
                SELECT DISTINCT e.id, e.created_at as employee_created_at, e.updated_at as employee_updated_at,
                       e.contact_phone, e.contact_email, e.contact_telegram,
                       e.is_owner, e.is_manager, e.is_master,
                       e.organization_id, e.manager_branch_id, e.user_id,
                       up.first_name, up.last_name, up.patronymic,
                       up.created_at as profile_created_at, up.updated_at as profile_updated_at
                FROM employees e
                INNER JOIN organizations o ON e.organization_id = o.id
                INNER JOIN user_profiles up ON e.user_id = up.user_id
                WHERE o.name = $1 AND e.is_master = true
                AND EXISTS (
                    SELECT 1 FROM services s
                    WHERE (s.master_id = e.id OR s.master_id IS NULL)
                    AND s.id = ANY($2)
                )
                "#,
                organization_name,
                service_ids
            )
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as!(
                EmployeeWithProfile,
                r#"
                SELECT DISTINCT e.id, e.created_at as employee_created_at, e.updated_at as employee_updated_at,
                       e.contact_phone, e.contact_email, e.contact_telegram,
                       e.is_owner, e.is_manager, e.is_master,
                       e.organization_id, e.manager_branch_id, e.user_id,
                       up.first_name, up.last_name, up.patronymic,
                       up.created_at as profile_created_at, up.updated_at as profile_updated_at
                FROM employees e
                INNER JOIN organizations o ON e.organization_id = o.id
                INNER JOIN user_profiles up ON e.user_id = up.user_id
                WHERE o.name = $1 AND e.is_master = true
                AND EXISTS (
                    SELECT 1 FROM schedule_days sd
                    WHERE sd.master_id = e.id 
                    AND (sd.day_data->>'branch_id')::uuid = ANY($2)
                )
                AND EXISTS (
                    SELECT 1 FROM services s
                    WHERE (s.master_id = e.id OR s.master_id IS NULL)
                    AND s.id = ANY($3)
                )
                "#,
                organization_name,
                branch_ids,
                service_ids
            )
            .fetch_all(&self.pool)
            .await
        }
    }

    async fn find_master_by_id(&self, master_id: Uuid) -> Result<Option<EmployeeWithProfile>> {
        sqlx::query_as!(
            EmployeeWithProfile,
            r#"
            SELECT e.id, e.created_at as employee_created_at, e.updated_at as employee_updated_at,
                   e.contact_phone, e.contact_email, e.contact_telegram,
                   e.is_owner, e.is_manager, e.is_master,
                   e.organization_id, e.manager_branch_id, e.user_id,
                   up.first_name, up.last_name, up.patronymic,
                   up.created_at as profile_created_at, up.updated_at as profile_updated_at
            FROM employees e
            INNER JOIN user_profiles up ON e.user_id = up.user_id
            WHERE e.id = $1 AND e.is_master = true
            "#,
            master_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_branches_by_organization(
        &self,
        organization_name: &str,
        master_ids: &[Uuid],
    ) -> Result<Vec<Branch>> {
        if master_ids.is_empty() {
            // No filter, return all branches for organization
            sqlx::query_as!(
                Branch,
                r#"
                SELECT b.id, b.created_at, b.updated_at, b.display_name, b.description,
                       b.timezone, b.street, b.house_number, b.apartment_number,
                       b.city, b.region, b.country, b.address_info, b.organization_id
                FROM branches b
                INNER JOIN organizations o ON b.organization_id = o.id
                WHERE o.name = $1
                "#,
                organization_name
            )
            .fetch_all(&self.pool)
            .await
        } else {
            // Filter by master IDs - find branches where masters work
            sqlx::query_as!(
                Branch,
                r#"
                SELECT DISTINCT b.id, b.created_at, b.updated_at, b.display_name, b.description,
                       b.timezone, b.street, b.house_number, b.apartment_number,
                       b.city, b.region, b.country, b.address_info, b.organization_id
                FROM branches b
                INNER JOIN organizations o ON b.organization_id = o.id
                WHERE o.name = $1
                AND EXISTS (
                    SELECT 1 FROM schedule_days sd
                    WHERE sd.master_id = ANY($2)
                    AND (sd.day_data->>'branch_id')::uuid = b.id
                )
                "#,
                organization_name,
                master_ids
            )
            .fetch_all(&self.pool)
            .await
        }
    }

    async fn find_master_timetable(&self, master_id: Uuid) -> Result<Option<Timetable>> {
        sqlx::query_as!(
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
    }

    async fn find_master_schedule_days(&self, master_id: Uuid) -> Result<Vec<ScheduleDay>> {
        sqlx::query_as!(
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
    }

    async fn find_master_day_redefinitions(
        &self,
        master_id: Uuid,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
    ) -> Result<Vec<DayRedefinition>> {
        sqlx::query_as!(
            DayRedefinition,
            r#"
            SELECT master_id, created_at, updated_at, date, day_data
            FROM day_redefinitions
            WHERE master_id = $1 AND date >= $2::date AND date <= $3::date
            ORDER BY date
            "#,
            master_id,
            start_date.date(),
            end_date.date()
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_bookings(
        &self,
        organization_id: Uuid,
        master_ids: &[Uuid],
        branch_ids: &[Uuid],
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> Result<Vec<Booking>> {
        if master_ids.is_empty() && branch_ids.is_empty() {
            // Get all bookings for organization in time range
            sqlx::query_as!(
                Booking,
                r#"
                SELECT b.id, b.created_at, b.updated_at, b.customer_id, b.service_id,
                       b.master_id, b.branch_id, b.started_at, b.ended_at,
                       b.notify_methods as "notify_methods: Vec<String>"
                FROM bookings b
                INNER JOIN employees e ON b.master_id = e.id
                WHERE e.organization_id = $1
                AND b.ended_at > $2
                AND b.started_at < $3
                "#,
                organization_id,
                start_time,
                end_time
            )
            .fetch_all(&self.pool)
            .await
        } else if !master_ids.is_empty() && branch_ids.is_empty() {
            // Filter by masters only
            sqlx::query_as!(
                Booking,
                r#"
                SELECT id, created_at, updated_at, customer_id, service_id,
                       master_id, branch_id, started_at, ended_at,
                       notify_methods as "notify_methods: Vec<String>"
                FROM bookings
                WHERE master_id = ANY($1)
                AND ended_at > $2
                AND started_at < $3
                "#,
                master_ids,
                start_time,
                end_time
            )
            .fetch_all(&self.pool)
            .await
        } else if master_ids.is_empty() && !branch_ids.is_empty() {
            // Filter by branches only
            sqlx::query_as!(
                Booking,
                r#"
                SELECT id, created_at, updated_at, customer_id, service_id,
                       master_id, branch_id, started_at, ended_at,
                       notify_methods as "notify_methods: Vec<String>"
                FROM bookings
                WHERE branch_id = ANY($1)
                AND ended_at > $2
                AND started_at < $3
                "#,
                branch_ids,
                start_time,
                end_time
            )
            .fetch_all(&self.pool)
            .await
        } else {
            // Filter by both masters and branches
            sqlx::query_as!(
                Booking,
                r#"
                SELECT id, created_at, updated_at, customer_id, service_id,
                       master_id, branch_id, started_at, ended_at,
                       notify_methods as "notify_methods: Vec<String>"
                FROM bookings
                WHERE master_id = ANY($1)
                AND branch_id = ANY($2)
                AND ended_at > $3
                AND started_at < $4
                "#,
                master_ids,
                branch_ids,
                start_time,
                end_time
            )
            .fetch_all(&self.pool)
            .await
        }
    }

    async fn find_service_by_id(&self, service_id: Uuid) -> Result<Option<Service>> {
        sqlx::query_as!(
            Service,
            r#"
            SELECT id, created_at, updated_at, display_name, description,
                   duration_minutes, price, master_id
            FROM services
            WHERE id = $1
            "#,
            service_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn create_booking(
        &self,
        customer_id: Uuid,
        service_id: Uuid,
        master_id: Uuid,
        branch_id: Uuid,
        started_at: NaiveDateTime,
        ended_at: NaiveDateTime,
        notify_methods: &[String],
    ) -> Result<Booking> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now().naive_utc();

        sqlx::query_as!(
            Booking,
            r#"
            INSERT INTO bookings (id, created_at, updated_at, customer_id, service_id,
                                 master_id, branch_id, started_at, ended_at, notify_methods)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::text[]::notify_method[])
            RETURNING id, created_at, updated_at, customer_id, service_id,
                      master_id, branch_id, started_at, ended_at,
                      notify_methods as "notify_methods: Vec<String>"
            "#,
            id,
            now,
            now,
            customer_id,
            service_id,
            master_id,
            branch_id,
            started_at,
            ended_at,
            notify_methods as _
        )
        .fetch_one(&self.pool)
        .await
    }
}
