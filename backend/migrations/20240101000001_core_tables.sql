-- Tabella principale per opere (manga, anime, film, musica)
CREATE TABLE works (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    work_type VARCHAR(50) NOT NULL,
    title VARCHAR(500) NOT NULL,
    original_title VARCHAR(500),
    synopsis TEXT,
    year INTEGER,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_works_type ON works(work_type);
CREATE INDEX idx_works_title ON works USING gin(to_tsvector('english', title));
