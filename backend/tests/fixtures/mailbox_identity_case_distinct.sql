INSERT INTO tenants (id, name, slug, contact_email, plan_tier, status)
VALUES ('33333333-3333-4333-8333-333333333333', 'Identity recovery', 'identity-recovery', 'owner@audit.example', 'professional', 'active');
INSERT INTO v1_suppression_entries (tenant_id, email, canonical_email, reason)
VALUES
('33333333-3333-4333-8333-333333333333', 'Sales@COMPANY.EXAMPLE', 'Sales@company.example', 'manual'),
('33333333-3333-4333-8333-333333333333', 'sales@company.example', 'sales@company.example', 'manual');
