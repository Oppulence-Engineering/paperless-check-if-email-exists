-- Quality-gate thresholds for the job approval checklist. NULL uses the defaults (90 and 70).
ALTER TABLE tenants
    ADD COLUMN approval_excellent_threshold DOUBLE PRECISION,
    ADD COLUMN approval_good_threshold DOUBLE PRECISION,
    ADD CONSTRAINT tenants_approval_thresholds_check CHECK (
        (approval_excellent_threshold IS NULL AND approval_good_threshold IS NULL)
        OR (
            approval_excellent_threshold IS NOT NULL
            AND approval_good_threshold IS NOT NULL
            AND approval_good_threshold >= 0
            AND approval_excellent_threshold <= 100
            AND approval_excellent_threshold > approval_good_threshold
        )
    );
