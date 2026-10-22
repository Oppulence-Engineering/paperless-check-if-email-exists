-- Identity merges preserve audit history and require deliberate recovery from mailbox_identity_v3_backup.
DO $$ BEGIN RAISE EXCEPTION 'Restore identity data from the audited backup; automatic downgrade would weaken suppression protection'; END $$;
