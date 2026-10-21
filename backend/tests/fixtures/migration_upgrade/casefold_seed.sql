INSERT INTO tenants (id,name,slug,contact_email,plan_tier,status,used_this_period)
VALUES ('33333333-3333-4333-8333-333333333333','Casefold','casefold','owner@example.com','professional','active',7);
INSERT INTO v1_suppression_entries (tenant_id,email,canonical_email,reason,expires_at) VALUES
('33333333-3333-4333-8333-333333333333','Sales+tag@Example.com','Sales+tag@example.com','complaint',NULL),
('33333333-3333-4333-8333-333333333333','sales+tag@example.com','sales+tag@example.com','bounce',NOW()+INTERVAL '30 days');
INSERT INTO v1_suppression_events (tenant_id,entry_id,canonical_email,event_type)
SELECT tenant_id,id,canonical_email,'created' FROM v1_suppression_entries;
INSERT INTO v1_bulk_job (id,total_records,tenant_id,status) VALUES (333,2,'33333333-3333-4333-8333-333333333333','completed');
INSERT INTO v1_task_result (tenant_id,job_id,payload,result,canonical_email,task_state,completed_at) VALUES
('33333333-3333-4333-8333-333333333333',333,'{"input":{"to_email":"Sales+tag@Example.com"}}','{"input":"Sales+tag@Example.com"}','Sales+tag@example.com','completed',NOW()),
('33333333-3333-4333-8333-333333333333',333,'{"input":{"to_email":"sales+tag@example.com"}}','{"input":"sales+tag@example.com"}','sales+tag@example.com','completed',NOW());
INSERT INTO verification_outcomes (tenant_id,canonical_email,outcome_type,occurred_at,source) VALUES
('33333333-3333-4333-8333-333333333333','Sales+tag@example.com','delivered','2026-10-20T00:00:00Z','fixture'),
('33333333-3333-4333-8333-333333333333','sales+tag@example.com','delivered','2026-10-20T00:00:00Z','fixture');
