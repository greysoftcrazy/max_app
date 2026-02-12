use sqlx::PgPool;
use uuid::Uuid;
use crate::core::models::{Work, WorkCreateDto, WorkUpdateDto, WorkType};

pub struct WorkRepository {
    pool: PgPool,
}

impl WorkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Создание новой работы
    pub async fn create(&self, dto: &WorkCreateDto) -> Result<Work, sqlx::Error> {
        let work_type_str = match dto.work_type {
            WorkType::Article => "article",
            WorkType::Competition => "competition",
            WorkType::Essay => "essay",
            WorkType::Report => "report",
            WorkType::Project => "project",
            WorkType::Presentation => "presentation",
            WorkType::Speech => "speech",
            WorkType::Other => "other",
        };

        let work = sqlx::query_as(
            r#"
            INSERT INTO works (
                title, work_type, specialty, author_name, supervisor_name, 
                year, annotation, keywords, file_path, thumbnail_path
            )
            VALUES ($1, $2::work_type, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, title, work_type, specialty, author_name, supervisor_name, 
                      year, annotation, keywords, file_path, thumbnail_path, created_at, updated_at
            "#,
        )
        .bind(&dto.title)
        .bind(work_type_str)
        .bind(&dto.specialty)
        .bind(&dto.author_name)
        .bind(&dto.supervisor_name)
        .bind(dto.year)
        .bind(&dto.annotation)
        .bind(&dto.keywords)
        .bind(&dto.file_path)
        .bind(&dto.thumbnail_path)
        .fetch_one(&self.pool)
        .await?;

        Ok(work)
    }

    /// Получение работы по ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Work>, sqlx::Error> {
        let work = sqlx::query_as(
            "SELECT id, title, work_type, specialty, author_name, supervisor_name, 
                    year, annotation, keywords, file_path, thumbnail_path, created_at, updated_at 
             FROM works WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(work)
    }

    /// Поиск работ с фильтрами
    pub async fn search(
        &self,
        query: Option<&str>,
        specialty: Option<&str>,
        work_type: Option<&str>,
        year: Option<i32>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Work>, sqlx::Error> {
        let mut sql = String::from("SELECT id, title, work_type, specialty, author_name, supervisor_name, year, annotation, keywords, file_path, thumbnail_path, created_at, updated_at FROM works WHERE 1=1");
        let mut args = Vec::new();
        let mut param_index = 1;

        // Поиск по полнотекстовому индексу
        if let Some(q) = query {
            // Преобразуем запрос в формат tsquery: заменяем пробелы на &
            let ts_query = q.replace(' ', " & ");
            sql.push_str(&format!(" AND search_vector @@ to_tsquery('russian', ${})", param_index));
            args.push(ts_query);
            param_index += 1;
        }

        // Фильтр по специальности
        if let Some(s) = specialty {
            sql.push_str(&format!(" AND specialty ILIKE ${}", param_index));
            args.push(format!("%{}%", s));
            param_index += 1;
        }

        // Фильтр по типу работы
        if let Some(wt) = work_type {
            sql.push_str(&format!(" AND work_type = ${}::work_type", param_index));
            args.push(wt.to_string());
            param_index += 1;
        }

        // Фильтр по году
        if let Some(y) = year {
            sql.push_str(&format!(" AND year = ${}", param_index));
            args.push(y.to_string());
            param_index += 1;
        }

        sql.push_str(&format!(" ORDER BY created_at DESC LIMIT ${} OFFSET ${}", param_index, param_index + 1));

        let mut query = sqlx::query_as::<_, Work>(&sql);

        for arg in args {
            query = query.bind(arg);
        }
        query = query.bind(limit);
        query = query.bind(offset);

        let works = query.fetch_all(&self.pool).await?;

        Ok(works)
    }

    /// Получение уникальных специальностей
    pub async fn list_specialties(&self) -> Result<Vec<String>, sqlx::Error> {
        let specialties = sqlx::query_scalar(
            "SELECT DISTINCT specialty FROM works ORDER BY specialty",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(specialties)
    }

    /// Обновление работы (простой и надёжный вариант с COALESCE)
    pub async fn update(&self, id: Uuid, dto: &WorkUpdateDto) -> Result<Option<Work>, sqlx::Error> {
        // Преобразуем тип работы в строку для привязки
        let work_type_str = dto.work_type.as_ref().map(|wt| match wt {
            WorkType::Article => "article",
            WorkType::Competition => "competition",
            WorkType::Essay => "essay",
            WorkType::Report => "report",
            WorkType::Project => "project",
            WorkType::Presentation => "presentation",
            WorkType::Speech => "speech",
            WorkType::Other => "other",
        });

        let work = sqlx::query_as(
            r#"
            UPDATE works 
            SET 
                title = COALESCE($1, title),
                work_type = COALESCE($2::work_type, work_type),
                specialty = COALESCE($3, specialty),
                author_name = COALESCE($4, author_name),
                supervisor_name = COALESCE($5, supervisor_name),
                year = COALESCE($6, year),
                annotation = COALESCE($7, annotation),
                keywords = COALESCE($8, keywords),
                thumbnail_path = COALESCE($9, thumbnail_path),
                updated_at = NOW()
            WHERE id = $10
            RETURNING id, title, work_type, specialty, author_name, supervisor_name, 
                      year, annotation, keywords, file_path, thumbnail_path, created_at, updated_at
            "#,
        )
        .bind(dto.title.as_ref())
        .bind(work_type_str)
        .bind(dto.specialty.as_ref())
        .bind(dto.author_name.as_ref())
        .bind(dto.supervisor_name.as_ref())
        .bind(dto.year)
        .bind(dto.annotation.as_ref())
        .bind(dto.keywords.as_ref())
        .bind(dto.thumbnail_path.as_ref())
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(work)
    }

    /// Удаление работы
    pub async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM works WHERE id = $1",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}