INSERT INTO tenants (id, name, slug, contact_email, plan_tier, status)
VALUES ('33333333-3333-4333-8333-333333333333', 'Identity recovery', 'identity-recovery', 'owner@audit.example', 'professional', 'active');
INSERT INTO v1_suppression_entries (tenant_id, email, canonical_email, reason, expires_at)
VALUES
('33333333-3333-4333-8333-333333333333', 'Sales@COMPANY.EXAMPLE', 'Sales@company.example', 'manual', NULL),
('33333333-3333-4333-8333-333333333333', 'sales@company.example', 'sales@company.example', 'manual', NOW() + INTERVAL '30 days');

INSERT INTO v1_task_result (tenant_id, payload, canonical_email, task_state)
VALUES ('33333333-3333-4333-8333-333333333333', '{}', 'legacy@company.example', 'completed');

INSERT INTO v1_bulk_job (id, total_records, tenant_id, status)
VALUES (9901, 2, '33333333-3333-4333-8333-333333333333', 'completed');
INSERT INTO v1_lists (tenant_id, job_id, name, original_filename, file_size_bytes, email_column, status, unique_emails, deduplicated_count)
VALUES ('33333333-3333-4333-8333-333333333333', 9901, 'Distinct identities', 'identities.csv', 100, 'email', 'completed', 2, 0);
INSERT INTO v1_task_result (tenant_id, job_id, payload, canonical_email, task_state, is_duplicate, completion_done)
VALUES
('33333333-3333-4333-8333-333333333333', 9901, '{"input":{"to_email":"Sales@company.example"}}', 'Sales@company.example', 'completed', false, true),
('33333333-3333-4333-8333-333333333333', 9901, '{"input":{"to_email":"sales@company.example"}}', 'sales@company.example', 'completed', false, true);

INSERT INTO verification_outcomes (tenant_id, canonical_email, email, outcome_type, occurred_at, source)
VALUES
('33333333-3333-4333-8333-333333333333', 'Sales@company.example', 'Sales@company.example', 'delivered', '2026-10-20T00:00:00Z', 'fixture'),
('33333333-3333-4333-8333-333333333333', 'sales@company.example', 'sales@company.example', 'delivered', '2026-10-20T00:00:00Z', 'fixture');
