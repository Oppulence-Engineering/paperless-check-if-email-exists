ALTER TABLE tenant_domains ADD COLUMN verification_token TEXT NOT NULL DEFAULT
    replace(gen_random_uuid()::text, '-', '') || replace(gen_random_uuid()::text, '-', '');
CREATE FUNCTION reset_domain_verification() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF TG_OP = 'UPDATE' AND NEW.domain IS DISTINCT FROM OLD.domain THEN
  NEW.is_verified := false; NEW.is_active := false;
  NEW.verification_token := replace(gen_random_uuid()::text, '-', '') || replace(gen_random_uuid()::text, '-', '');
 END IF;
 IF NOT NEW.is_verified THEN NEW.is_active := false; END IF;
 RETURN NEW;
END $$;
CREATE TRIGGER reset_domain_verification BEFORE INSERT OR UPDATE ON tenant_domains
FOR EACH ROW EXECUTE FUNCTION reset_domain_verification();

UPDATE tenant_domains SET is_active=false WHERE NOT is_verified;
ALTER TABLE tenant_domains ALTER COLUMN is_active SET DEFAULT false;
ALTER TABLE tenant_domains ADD CONSTRAINT tenant_domain_verified_activation CHECK (NOT is_active OR is_verified);
