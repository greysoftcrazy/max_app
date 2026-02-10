use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Work {
    pub id: Uuid,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "work_type", rename_all = "lowercase")]
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

impl WorkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkType::Article => "Статья",
            WorkType::Competition => "Конкурсная работа",
            WorkType::Essay => "Эссе",
            WorkType::Report => "Реферат",
            WorkType::Project => "Проект",
            WorkType::Presentation => "Презентация",
            WorkType::Speech => "Доклад",
            WorkType::Other => "Другое",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct WorkCreateDto {
    #[validate(length(min = 1, max = 500))]
    pub title: String,
    
    pub work_type: WorkType,
    
    #[validate(length(min = 1, max = 200))]
    pub specialty: String,
    
    #[validate(length(min = 1, max = 300))]
    pub author_name: String,
    
    #[validate(length(min = 1, max = 300))]
    pub supervisor_name: String,
    
    #[validate(range(min = 1900, max = 2100))]
    pub year: i32,
    
    pub annotation: Option<String>,
    pub keywords: Option<String>,
}