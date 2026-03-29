DROP TABLE IF EXISTS bounce_risk_domain_cache;

DROP INDEX IF EXISTS idx_v1_task_result_tenant_canonical_completed;

ALTER TABLE v1_task_result
    DROP COLUMN IF EXISTS bounce_risk_signals,
    DROP COLUMN IF EXISTS bounce_risk_model_version,
    DROP COLUMN IF EXISTS bounce_risk_action,
    DROP COLUMN IF EXISTS bounce_risk_confidence,
    DROP COLUMN IF EXISTS bounce_risk_category,
    DROP COLUMN IF EXISTS bounce_risk_score;
