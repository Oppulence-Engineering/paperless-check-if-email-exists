alter table "organization"
  add column if not exists "archivedAt" timestamptz,
  add column if not exists "archivedBy" text;

create index if not exists "organization_archivedAt_idx"
  on "organization" ("archivedAt")
  where "archivedAt" is not null;
