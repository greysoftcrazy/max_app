use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Work {
    pub id: Uuid,
    pub title: String,
    #[sqlx(rename = "work_type")]
    pub work_type: WorkType,
    pub specialty: String,
    pub author_name: String,
    pub supervisor_name: String,
    pub year: i32,
    pub annotation: Option<String>,
    pub keywords: Option<String>,
    pub file_path: String,
    pub thumbnail_path: Option<String>,
    #[sqlx(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    #[sqlx(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "work_type")]
#[sqlx(rename_all = "lowercase")]
pub enum WorkType {
    Article,
    Competition,
    Essay,
    Report,
    Project,
    Presentation,
    Speech,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCreateDto {
    pub title: String,
    pub work_type: WorkType,
    pub specialty: String,
    pub author_name: String,
    pub supervisor_name: String,
    pub year: i32,
    pub annotation: Option<String>,
    pub keywords: Option<String>,
    pub file_path: String,
    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkUpdateDto {
    pub title: Option<String>,
    pub work_type: Option<WorkType>,
    pub specialty: Option<String>,
    pub author_name: Option<String>,
    pub supervisor_name: Option<String>,
    pub year: Option<i32>,
    pub annotation: Option<String>,
    pub keywords: Option<String>,
    pub thumbnail_path: Option<String>,
}