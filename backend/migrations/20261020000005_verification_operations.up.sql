ALTER TABLE v1_task_result
    ADD COLUMN operation_id UUID UNIQUE,
    ADD COLUMN quota_period_end TIMESTAMPTZ;

ALTER TABLE idempotency_keys
    ADD COLUMN operation_id UUID;

CREATE INDEX idempotency_keys_operation_id ON idempotency_keys (operation_id)
    WHERE operation_id IS NOT NULL;
