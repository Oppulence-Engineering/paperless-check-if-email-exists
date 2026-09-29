alter table "session"
  add column if not exists "stepUpMethod" text,
  add column if not exists "stepUpPurpose" text,
  add column if not exists "ssoProviderId" text,
  add column if not exists "ssoOrganizationId" text;

update "session"
set "stepUpVerifiedAt" = null
where "stepUpVerifiedAt" is not null;
