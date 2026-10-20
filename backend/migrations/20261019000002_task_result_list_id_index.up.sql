-- List views, downloads, remediation and list finalization all filter task
-- results with `(extra->>'list_id')::INTEGER = $1`. Without an index on that
-- expression each of them scans every task result of every tenant.
--
-- The expression matches the queries exactly so the planner can use it. The
-- cast cannot fail on existing rows: those same queries would already error
-- on a non-integer list_id.
CREATE INDEX IF NOT EXISTS idx_v1_task_result_list_id
    ON v1_task_result (((extra->>'list_id')::INTEGER));
