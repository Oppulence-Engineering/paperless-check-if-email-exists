DROP TRIGGER IF EXISTS set_v1_usage_events_updated_at ON v1_usage_events;
DROP TRIGGER IF EXISTS set_v1_pipeline_runs_updated_at ON v1_pipeline_runs;
DROP TRIGGER IF EXISTS set_v1_pipelines_updated_at ON v1_pipelines;

DROP TABLE IF EXISTS v1_usage_events;

ALTER TABLE v1_lists
    DROP COLUMN IF EXISTS pipeline_run_id,
    DROP COLUMN IF EXISTS pipeline_id,
    DROP COLUMN IF EXISTS source_list_id;

DROP TABLE IF EXISTS v1_pipeline_runs;
DROP TABLE IF EXISTS v1_pipelines;

DROP TYPE IF EXISTS pipeline_run_status;
DROP TYPE IF EXISTS pipeline_source_type;
DROP TYPE IF EXISTS pipeline_status;
