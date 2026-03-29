-- Trigger per aggiornamento automatico del campo updated_at
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER works_updated_at BEFORE UPDATE ON works
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER library_updated_at BEFORE UPDATE ON user_library_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
