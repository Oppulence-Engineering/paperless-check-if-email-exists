DROP INDEX IF EXISTS idx_outcome_policies_one_default;
DROP TABLE IF EXISTS v1_outcome_policies;

DROP INDEX IF EXISTS idx_verification_outcomes_campaign;
DROP INDEX IF EXISTS idx_verification_outcomes_recent;
DROP INDEX IF EXISTS idx_verification_outcomes_lookup;
DROP TABLE IF EXISTS verification_outcomes;

DROP TYPE IF EXISTS outcome_type;
