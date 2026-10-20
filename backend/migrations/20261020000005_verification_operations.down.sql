ALTER TABLE idempotency_keys DROP COLUMN operation_id;
ALTER TABLE v1_task_result DROP COLUMN operation_id, DROP COLUMN quota_period_end;
