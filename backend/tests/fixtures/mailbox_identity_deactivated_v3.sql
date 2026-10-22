INSERT INTO tenants (id, name, slug, contact_email, plan_tier, status)
VALUES ('44444444-4444-4444-8444-444444444444', 'Deactivated recovery', 'deactivated-recovery', 'owner@audit.example', 'professional', 'active');
INSERT INTO v1_suppression_entries (tenant_id, email, canonical_email, reason, status)
VALUES
('44444444-4444-4444-8444-444444444444', 'Sales@company.example', 'sales@company.example', 'manual', 'inactive'),
('44444444-4444-4444-8444-444444444444', 'sales@company.example', 'sales@company.example', 'manual', 'merged');
INSERT INTO mailbox_identity_v3_backup (table_name, row_id, previous)
SELECT 'v1_suppression_entries', id, to_jsonb(s) || jsonb_build_object('status', 'active', 'canonical_email', email)
FROM v1_suppression_entries s WHERE tenant_id = '44444444-4444-4444-8444-444444444444';
UPDATE v1_suppression_entries SET metadata = jsonb_build_object('identity_v3_survivor',
 (SELECT id FROM v1_suppression_entries WHERE tenant_id = '44444444-4444-4444-8444-444444444444' AND status = 'inactive'))
WHERE tenant_id = '44444444-4444-4444-8444-444444444444' AND status = 'merged';
