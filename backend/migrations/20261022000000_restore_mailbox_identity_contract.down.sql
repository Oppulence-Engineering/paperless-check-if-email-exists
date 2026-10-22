-- Re-folding identities would merge distinct mailboxes; keep history and roll forward.
DO $$ BEGIN RAISE EXCEPTION 'Mailbox identity recovery requires a forward migration'; END $$;
