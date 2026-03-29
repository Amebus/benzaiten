-- Stato di rilascio delle opere per paese
CREATE TABLE work_release_status (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    work_id UUID NOT NULL REFERENCES works(id) ON DELETE CASCADE,
    country_code VARCHAR(2) NOT NULL,
    status VARCHAR(50) NOT NULL,
    started_at DATE,
    completed_at DATE,
    UNIQUE(work_id, country_code)
);

CREATE INDEX idx_release_status_work ON work_release_status(work_id);
