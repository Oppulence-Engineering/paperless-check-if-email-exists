-- The existing worker auto-action hook reads this operator-managed configuration.
ALTER TABLE tenants ADD COLUMN settings JSONB NOT NULL DEFAULT '{}'::jsonb
    CHECK (jsonb_typeof(settings) = 'object');
