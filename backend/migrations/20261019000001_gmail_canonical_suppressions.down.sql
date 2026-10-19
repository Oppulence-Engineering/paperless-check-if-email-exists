-- The original spelling of a re-keyed address is still in the `email` column,
-- but rows merged by the up migration cannot be told apart from rows merged
-- for other reasons, so the re-keying is not reversed.
SELECT 1;
