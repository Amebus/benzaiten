-- Immagini associate alle opere (copertine, screenshot, ecc.)
CREATE TABLE images (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    work_id UUID NOT NULL REFERENCES works(id) ON DELETE CASCADE,
    s3_key VARCHAR(500) NOT NULL,
    kind VARCHAR(50) NOT NULL,
    display_order INTEGER DEFAULT 0,
    width INTEGER,
    height INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_images_work ON images(work_id);
