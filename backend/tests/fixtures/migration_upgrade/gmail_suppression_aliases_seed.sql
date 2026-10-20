-- Suppressions written before 20261019000001, keyed by the lowercased
-- address. Each group below is one Gmail mailbox spelled several ways, and
-- the migration re-keys every spelling to the same canonical address.
INSERT INTO tenants (id, name, slug, contact_email, plan_tier, status)
VALUES ('6a1f0c3e-6c55-4f4e-9f0b-2c1d7a9e5b01', 'Gmail Alias Tenant', 'gmail-alias-tenant',
        'owner@gmail-alias-tenant.test', 'professional', 'active');

-- A permanent entry, and a newer one for another spelling that has expired.
INSERT INTO v1_suppression_entries
    (tenant_id, email, canonical_email, status, reason, reason_code, source_type, created_at, expires_at)
VALUES
    ('6a1f0c3e-6c55-4f4e-9f0b-2c1d7a9e5b01', 'user@gmail.com', 'user@gmail.com', 'active',
     'bounce', 'bounce', 'outcome', NOW() - INTERVAL '30 days', NULL),
    ('6a1f0c3e-6c55-4f4e-9f0b-2c1d7a9e5b01', 'u.ser@gmail.com', 'u.ser@gmail.com', 'active',
     'manual', 'manual', 'manual', NOW() - INTERVAL '1 day', NOW() - INTERVAL '1 hour');

-- Two temporary entries: an older one that runs longer and a newer one that
-- ends tomorrow.
INSERT INTO v1_suppression_entries
    (tenant_id, email, canonical_email, status, reason, reason_code, source_type, created_at, expires_at)
VALUES
    ('6a1f0c3e-6c55-4f4e-9f0b-2c1d7a9e5b01', 'tempone@gmail.com', 'tempone@gmail.com', 'active',
     'manual', 'manual', 'manual', NOW() - INTERVAL '10 days', NOW() + INTERVAL '60 days'),
    ('6a1f0c3e-6c55-4f4e-9f0b-2c1d7a9e5b01', 'temp.one+news@googlemail.com', 'temp.one+news@googlemail.com', 'active',
     'manual', 'manual', 'manual', NOW() - INTERVAL '2 days', NOW() + INTERVAL '1 day');

-- A removed entry next to an active one: only the active one suppresses.
INSERT INTO v1_suppression_entries
    (tenant_id, email, canonical_email, status, reason, reason_code, source_type, created_at, expires_at)
VALUES
    ('6a1f0c3e-6c55-4f4e-9f0b-2c1d7a9e5b01', 'removed@gmail.com', 'removed@gmail.com', 'inactive',
     'manual', 'manual', 'manual', NOW() - INTERVAL '5 days', NULL),
    ('6a1f0c3e-6c55-4f4e-9f0b-2c1d7a9e5b01', 're.moved@gmail.com', 're.moved@gmail.com', 'active',
     'complaint', 'complaint', 'outcome', NOW() - INTERVAL '3 days', NULL);
