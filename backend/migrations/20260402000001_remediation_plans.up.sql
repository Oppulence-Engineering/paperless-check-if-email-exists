-- This migration does nothing on purpose.
--
-- It once created v1_remediation_plans, v1_remediation_rows, and
-- v1_remediation_exports. Staging and production created these tables with
-- 20260630000002 before this file reached them, so it failed there with
-- "relation already exists". 20260630000002 creates the tables on every
-- database, in the shape that the code uses.
SELECT 1;
