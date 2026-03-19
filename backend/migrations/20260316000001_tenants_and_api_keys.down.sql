DROP TRIGGER IF EXISTS set_api_keys_updated_at ON api_keys;
DROP TRIGGER IF EXISTS set_tenants_updated_at ON tenants;
DROP FUNCTION IF EXISTS update_updated_at_column();
DROP TABLE IF EXISTS api_keys;
DROP TABLE IF EXISTS tenants;
DROP TYPE IF EXISTS api_key_status;
DROP TYPE IF EXISTS plan_tier;
DROP TYPE IF EXISTS tenant_status;
