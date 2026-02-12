-- Add migration script here
-- Создание ENUM типа для вида работы
CREATE TYPE work_type AS ENUM (
    'article', 'competition', 'essay', 'report', 
    'project', 'presentation', 'speech', 'other'
);

-- Создание таблицы работ
CREATE TABLE works (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(500) NOT NULL,
    work_type work_type NOT NULL,
    specialty VARCHAR(200) NOT NULL,
    author_name VARCHAR(300) NOT NULL,
    supervisor_name VARCHAR(300) NOT NULL,
    year INTEGER NOT NULL CHECK (year >= 1900 AND year <= 2100),
    annotation TEXT,
    keywords TEXT,
    file_path VARCHAR(1000) NOT NULL,
    thumbnail_path VARCHAR(1000),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    search_vector TSVECTOR
);

-- Индексы для ускорения поиска
CREATE INDEX idx_works_specialty ON works(specialty);
CREATE INDEX idx_works_work_type ON works(work_type);
CREATE INDEX idx_works_year ON works(year);
CREATE INDEX idx_works_author ON works(author_name);
CREATE INDEX idx_works_supervisor ON works(supervisor_name);
CREATE INDEX idx_works_search ON works USING GIN(search_vector);

-- Триггер для обновления вектора поиска
CREATE FUNCTION update_search_vector() RETURNS trigger AS $$
BEGIN
    NEW.search_vector := 
        setweight(to_tsvector('russian', coalesce(NEW.title, '')), 'A') ||
        setweight(to_tsvector('russian', coalesce(NEW.annotation, '')), 'B') ||
        setweight(to_tsvector('russian', coalesce(NEW.keywords, '')), 'B') ||
        setweight(to_tsvector('russian', coalesce(NEW.author_name, '')), 'C') ||
        setweight(to_tsvector('russian', coalesce(NEW.supervisor_name, '')), 'C');
    RETURN NEW;
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_search_vector
    BEFORE INSERT OR UPDATE ON works
    FOR EACH ROW EXECUTE FUNCTION update_search_vector();