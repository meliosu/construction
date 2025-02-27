use std::ops::Deref;

use crate::db::Database;
use crate::types::*;

impl Database {
    pub async fn get_employees(&self) -> sqlx::Result<Vec<EmployeeInfo>> {
        sqlx::query_as::<_, EmployeeInfo>(
            "SELECT
                id,
                first_name,
                last_name,
                middle_name,
                gender,
                photo,
                salary,
                phone_number
            FROM employee;",
        )
        .fetch_all(self.deref())
        .await
    }

    pub async fn get_sites(&self) -> sqlx::Result<Vec<SiteInfo>> {
        sqlx::query_as::<_, SiteInfo>(
            "SELECT
                id,
                type,
                location_lat,
                location_lng,
                risk_level,
                description
            FROM sites;",
        )
        .fetch_all(self.deref())
        .await
    }

    pub async fn get_departments(&self) -> sqlx::Result<Vec<DepartmentInfo>> {
        sqlx::query_as::<_, DepartmentInfo>(
            "SELECT
                department.id AS id,
                CONCAT(employee.first_name, ' ', employee.last_name) as supervisor_name
            FROM department
            JOIN employee ON department.supervisor_id = employee.id;",
        )
        .fetch_all(self.deref())
        .await
    }

    pub async fn get_equipment(&self) -> sqlx::Result<Vec<EquipmentInfo>> {
        sqlx::query_as::<_, EquipmentInfo>(
            "SELECT
                id,
                name,
                amount,
                purhcase_date,
                purchase_cost,
                fuel_type
            FROM equipment;",
        )
        .fetch_all(self.deref())
        .await
    }
}
