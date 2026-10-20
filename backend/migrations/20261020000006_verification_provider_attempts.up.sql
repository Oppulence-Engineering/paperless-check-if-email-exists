-- Record intent before a paid HTTP call; a lost response must not trigger another purchase.
CREATE TABLE verification_provider_attempts (
    dispatch_id UUID PRIMARY KEY,
    task_id INTEGER NOT NULL REFERENCES v1_task_result(id) ON DELETE CASCADE,
    response JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX verification_provider_attempts_task_idx ON verification_provider_attempts(task_id);
