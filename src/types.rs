#![allow(unused)]

use sqlx::{postgres::types::PgPoint, types::time::Date};

#[derive(sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "employee_class", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EmployeeClass {
    Worker,
    TechnicalPersonnel,
}

#[derive(sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "gender", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
}

#[derive(sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "managment_position", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ManagementPosition {
    Master,
    Foreman,
}

#[derive(sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "technical_qualification", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TechnicalQualification {
    Technician,
    Technologist,
    Engineer,
}

#[derive(sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "worker_profession", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum WorkerProfession {
    Electrician,
    Plumber,
    Welder,
    Driver,
    Mason,
}

#[derive(sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "site_risk_level", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SiteRiskLevel {
    Low,
    Medium,
    High,
}

#[derive(sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "site_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SiteType {
    PowerPlant,
    Road,
    Housing,
    Bridge,
    Park,
}

#[derive(sqlx::FromRow)]
pub struct EmployeeInfo {
    pub id: i32,
    pub class: EmployeeClass,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub photo: Option<String>,
    pub salary: i32,
    pub phone_number: String,
}

#[derive(sqlx::FromRow)]
pub struct SiteInfo {
    pub id: i32,
    pub r#type: SiteType,
    pub location: PgPoint,
    pub risk_level: SiteRiskLevel,
    pub description: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct DepartmentInfo {
    pub id: i32,
    pub supervisor_name: String,
}

#[derive(sqlx::FromRow)]
pub struct EquipmentInfo {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub purhcase_date: Date,
    pub purchase_cost: i32,
    pub fuel_type: Option<String>,
}

// Entities

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub gender: Gender,
    pub photo: Option<String>,
    pub salary: i32,
    pub phone_number: String,

    #[serde(flatten)]
    pub kind: EmployeeKind,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "class")]
pub enum EmployeeKind {
    Worker(Worker),
    TechnicalPersonnel(TechnicalPersonnel),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Worker {
    pub union_name: Option<String>,

    #[serde(flatten)]
    pub kind: WorkerKind,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "profession")]
pub enum WorkerKind {
    Electrician {
        voltage_specialization: String,
    },
    Plumbet {
        pipe_specialization: String,
    },
    Welder {
        welding_machine: String,
    },
    Driver {
        vehicle_type: String,
        number_of_accidents: i32,
    },
    Mason {
        hq_restoration_skills: bool,
    },
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TechnicalPersonnel {
    pub position: ManagementPosition,
    pub education_level: String,
    pub software_skills: Option<String>,
    pub is_project_manager: bool,

    #[serde(flatten)]
    pub kind: TechnicalPersonnelKind,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "qualification")]
pub enum TechnicalPersonnelKind {
    Technician { safety_training_level: String },
    Technologist { management_tools: String },
    Engineer { pe_license_id: i32 },
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Site {
    id: i32,
    area_id: i32,
    client_id: i32,
    location: (f64, f64),
    risk_level: SiteRiskLevel,
    description: Option<String>,

    #[serde(flatten)]
    kind: SiteKind,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum SiteKind {
    PowerPlant {
        energy_output: i32,
        energy_source: String,
        is_grid_connected: bool,
    },
    Road {
        length: i32,
        lanes: i32,
        surface: String,
    },
    Housing {
        number_of_floors: i32,
        number_of_entrances: i32,
        housing_type: String,
        energy_efficiency: char,
    },
    Bridge {
        length: i32,
        road_material: String,
        max_load: i32,
    },
    Park {
        area: f64,
        has_playground: bool,
        has_lighting: bool,
    },
}
