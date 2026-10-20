ALTER TABLE v1_task_result
    ADD COLUMN dispatch_id UUID,
    ADD COLUMN lease_id UUID,
    ADD COLUMN lease_expires_at TIMESTAMPTZ,
    ADD COLUMN verification_stored BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN webhook_done BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN completion_done BOOLEAN NOT NULL DEFAULT false;

UPDATE v1_task_result
SET verification_stored = true, completion_done = true
WHERE task_state IN ('completed', 'failed', 'dead_lettered', 'cancelled');

CREATE TABLE verification_dispatch (
    id UUID PRIMARY KEY,
    task_id INTEGER NOT NULL UNIQUE REFERENCES v1_task_result(id) ON DELETE CASCADE,
    payload JSONB NOT NULL,
    priority SMALLINT NOT NULL CHECK (priority BETWEEN 0 AND 255),
    attempts INTEGER NOT NULL DEFAULT 0,
    next_attempt_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX verification_dispatch_due_idx ON verification_dispatch (next_attempt_at);

CREATE INDEX verification_completion_pending_idx ON v1_task_result (updated_at)
    WHERE verification_stored AND NOT completion_done AND NOT is_duplicate AND dispatch_id IS NOT NULL;
