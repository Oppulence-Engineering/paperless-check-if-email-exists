alter table "identity_audit_event"
  add column if not exists "reason" text;
