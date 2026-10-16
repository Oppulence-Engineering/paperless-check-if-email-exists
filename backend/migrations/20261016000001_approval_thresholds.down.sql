ALTER TABLE tenants
    DROP CONSTRAINT IF EXISTS tenants_approval_thresholds_check,
    DROP COLUMN IF EXISTS approval_good_threshold,
    DROP COLUMN IF EXISTS approval_excellent_threshold;
