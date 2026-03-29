-- Libreria personale dell'utente
CREATE TABLE user_library_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id VARCHAR(100) NOT NULL,
    work_id UUID NOT NULL REFERENCES works(id) ON DELETE CASCADE,
    owned_volumes TEXT[],
    current_episode INTEGER DEFAULT 0,
    total_episodes INTEGER,
    purchase_price DECIMAL(10,2),
    variant_notes TEXT,
    personal_rating INTEGER CHECK (personal_rating >= 1 AND personal_rating <= 10),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, work_id)
);

CREATE INDEX idx_library_user ON user_library_items(user_id);
CREATE INDEX idx_library_work ON user_library_items(work_id);
