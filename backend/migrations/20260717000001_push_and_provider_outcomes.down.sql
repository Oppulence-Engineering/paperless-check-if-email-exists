DROP INDEX IF EXISTS idx_v1_contact_outcomes_receipt;
DROP INDEX IF EXISTS idx_v1_contact_outcomes_provider_event;
ALTER TABLE v1_contact_outcomes
    DROP COLUMN IF EXISTS correlation_status,
    DROP COLUMN IF EXISTS event_family,
    DROP COLUMN IF EXISTS provider_message_id,
    DROP COLUMN IF EXISTS provider_event_id,
    DROP COLUMN IF EXISTS receipt_id,
    DROP COLUMN IF EXISTS endpoint_id;
DROP TABLE IF EXISTS v1_provider_outcome_receipts;
DROP TABLE IF EXISTS v1_provider_endpoints;
DROP TABLE IF EXISTS v1_pipeline_push_batches;
