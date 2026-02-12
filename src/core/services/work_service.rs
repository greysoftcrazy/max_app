use crate::core::{
    models::{Work, WorkCreateDto, WorkUpdateDto},
    repositories::WorkRepository,
};
use sqlx::PgPool;
use uuid::Uuid;

pub struct WorkService {
    repo: WorkRepository,
}

impl WorkService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: WorkRepository::new(pool),
        }
    }

    pub async fn create(&self, dto: WorkCreateDto) -> Result<Work, crate::error::AppError> {
        // Валидация
        if dto.title.trim().is_empty() {
            return Err(crate::error::AppError::ValidationError("Название работы не может быть пустым".to_string()));
        }
        if dto.specialty.trim().is_empty() {
            return Err(crate::error::AppError::ValidationError("Специальность не может быть пустой".to_string()));
        }
        if dto.author_name.trim().is_empty() {
            return Err(crate::error::AppError::ValidationError("Имя автора не может быть пустым".to_string()));
        }
        if dto.supervisor_name.trim().is_empty() {
            return Err(crate::error::AppError::ValidationError("Имя руководителя не может быть пустым".to_string()));
        }
        if dto.year < 1900 || dto.year > 2100 {
            return Err(crate::error::AppError::ValidationError("Год должен быть в диапазоне 1900-2100".to_string()));
        }

        let work = self.repo.create(&dto).await?;
        Ok(work)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Work>, crate::error::AppError> {
        let work = self.repo.get_by_id(id).await?;
        Ok(work)
    }

    pub async fn search(
        &self,
        query: Option<&str>,
        specialty: Option<&str>,
        work_type: Option<&str>,
        year: Option<i32>,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Work>, crate::error::AppError> {
        let offset = ((page - 1) * limit) as i64;
        let works = self.repo.search(query, specialty, work_type, year, limit as i64, offset).await?;
        Ok(works)
    }

    pub async fn list_specialties(&self) -> Result<Vec<String>, crate::error::AppError> {
        let specialties = self.repo.list_specialties().await?;
        Ok(specialties)
    }

    pub async fn update(&self, id: Uuid, dto: WorkUpdateDto) -> Result<Option<Work>, crate::error::AppError> {
        let work = self.repo.update(id, &dto).await?;
        Ok(work)
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool, crate::error::AppError> {
        let deleted = self.repo.delete(id).await?;
        Ok(deleted)
    }
}