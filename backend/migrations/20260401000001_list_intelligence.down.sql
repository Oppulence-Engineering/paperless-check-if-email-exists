DROP TRIGGER IF EXISTS set_v1_saved_segments_updated_at ON v1_saved_segments;
DROP TRIGGER IF EXISTS set_v1_score_policies_updated_at ON v1_score_policies;
DROP TRIGGER IF EXISTS set_v1_alerts_updated_at ON v1_alerts;

DROP TABLE IF EXISTS v1_saved_segments;

ALTER TABLE v1_lists
    DROP COLUMN IF EXISTS policy_id;

DROP TABLE IF EXISTS v1_score_policies;
DROP TABLE IF EXISTS v1_alerts;
DROP TABLE IF EXISTS verification_change_events;
