INSERT INTO tenants (id, name, slug, contact_email, plan_tier, status)
VALUES ('22222222-2222-4222-8222-222222222222', 'Identity migration', 'identity-migration', 'owner@audit.example', 'professional', 'active');

INSERT INTO v1_suppression_entries (tenant_id, email, canonical_email, reason, expires_at)
VALUES
('22222222-2222-4222-8222-222222222222', 'A.B+old@GoogleMail.com', 'a.b+old@googlemail.com', 'bounce', NOW() - INTERVAL '1 day'),
('22222222-2222-4222-8222-222222222222', 'a.b@gmail.com', 'a.b@gmail.com', 'complaint', NULL),
('22222222-2222-4222-8222-222222222222', 'ab+future@gmail.com', 'ab+future@gmail.com', 'unsubscribe', NOW() + INTERVAL '30 days'),
('22222222-2222-4222-8222-222222222222', 'Sales@COMPANY.EXAMPLE', 'sales@company.example', 'manual', NULL);

INSERT INTO v1_suppression_events (tenant_id, entry_id, canonical_email, event_type)
SELECT tenant_id, id, canonical_email, 'created' FROM v1_suppression_entries
WHERE tenant_id = '22222222-2222-4222-8222-222222222222';

INSERT INTO v1_contact_outcomes (tenant_id, email, canonical_email, provider, event_type, occurred_at)
VALUES ('22222222-2222-4222-8222-222222222222', 'A.B+old@GoogleMail.com', 'a.b+old@googlemail.com', 'fixture', 'bounced', NOW()),
('22222222-2222-4222-8222-222222222222', 'Sales@COMPANY.EXAMPLE', 'sales@company.example', 'fixture', 'delivered', NOW());

INSERT INTO v1_task_result (tenant_id, payload, result, canonical_email, task_state, completed_at)
VALUES ('22222222-2222-4222-8222-222222222222', '{"input":{"to_email":"Sales@COMPANY.EXAMPLE"}}',
'{"input":"Sales@COMPANY.EXAMPLE","is_reachable":"safe"}', 'sales@company.example', 'completed', NOW());
