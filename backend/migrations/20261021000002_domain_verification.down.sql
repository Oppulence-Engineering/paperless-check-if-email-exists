ALTER TABLE tenant_domains DROP CONSTRAINT tenant_domain_verified_activation;
ALTER TABLE tenant_domains ALTER COLUMN is_active SET DEFAULT true;
DROP TRIGGER reset_domain_verification ON tenant_domains;
DROP FUNCTION reset_domain_verification();
ALTER TABLE tenant_domains DROP COLUMN verification_token;
