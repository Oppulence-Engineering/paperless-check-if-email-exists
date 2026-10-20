-- Returning to universal case folding would merge newly distinct mailboxes.
-- Roll forward or restore a pre-migration backup instead of discarding identity.
DO $$ BEGIN
    RAISE EXCEPTION 'Mailbox identity migration is lossless but not automatically reversible; roll forward or restore a backup';
END $$;
