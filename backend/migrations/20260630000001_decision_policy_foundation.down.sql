DROP INDEX IF EXISTS idx_v1_task_result_source_key;
DROP INDEX IF EXISTS idx_v1_task_result_policy_decision;
DROP INDEX IF EXISTS idx_v1_task_result_recommendation_action;

ALTER TABLE v1_task_result
    DROP COLUMN IF EXISTS source_key,
    DROP COLUMN IF EXISTS policy_evaluated_at,
    DROP COLUMN IF EXISTS policy_decision,
    DROP COLUMN IF EXISTS policy_evaluation,
    DROP COLUMN IF EXISTS policy_profile_key,
    DROP COLUMN IF EXISTS policy_mode,
    DROP COLUMN IF EXISTS recommendation_priority,
    DROP COLUMN IF EXISTS recommendation_confidence,
    DROP COLUMN IF EXISTS recommendation_action,
    DROP COLUMN IF EXISTS recommendation;

ALTER TABLE v1_lists
    DROP COLUMN IF EXISTS source_key,
    DROP COLUMN IF EXISTS policy_profile_key,
    DROP COLUMN IF EXISTS policy_mode;

ALTER TABLE v1_bulk_job
    DROP COLUMN IF EXISTS source_key,
    DROP COLUMN IF EXISTS policy_profile_key,
    DROP COLUMN IF EXISTS policy_mode;

DROP TABLE IF EXISTS tenant_policy_profiles;

ALTER TABLE tenants
    DROP COLUMN IF EXISTS default_policy_mode;
