-- Tabella persone (autori, registi, ecc.) e associazione con le opere
CREATE TABLE people (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    original_name VARCHAR(200),
    country_code VARCHAR(2),
    birth_date DATE,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE work_people (
    work_id UUID NOT NULL REFERENCES works(id) ON DELETE CASCADE,
    person_id UUID NOT NULL REFERENCES people(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL,
    PRIMARY KEY (work_id, person_id, role)
);

CREATE INDEX idx_work_people_work ON work_people(work_id);
CREATE INDEX idx_work_people_person ON work_people(person_id);
