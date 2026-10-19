create table "user" ("id" text not null primary key, "name" text not null, "email" text not null unique, "emailVerified" boolean not null, "image" text, "createdAt" timestamptz default CURRENT_TIMESTAMP not null, "updatedAt" timestamptz default CURRENT_TIMESTAMP not null, "role" text, "banned" boolean, "banReason" text, "banExpires" timestamptz, "twoFactorEnabled" boolean, "legacyWorkosUserId" text unique);

create table "session" ("id" text not null primary key, "expiresAt" timestamptz not null, "token" text not null unique, "createdAt" timestamptz default CURRENT_TIMESTAMP not null, "updatedAt" timestamptz not null, "ipAddress" text, "userAgent" text, "userId" text not null references "user" ("id") on delete cascade, "activeOrganizationId" text, "impersonatedBy" text, "authenticationMethod" text, "stepUpVerifiedAt" timestamptz);

create table "account" ("id" text not null primary key, "accountId" text not null, "providerId" text not null, "userId" text not null references "user" ("id") on delete cascade, "accessToken" text, "refreshToken" text, "idToken" text, "accessTokenExpiresAt" timestamptz, "refreshTokenExpiresAt" timestamptz, "scope" text, "password" text, "createdAt" timestamptz default CURRENT_TIMESTAMP not null, "updatedAt" timestamptz not null);

create table "verification" ("id" text not null primary key, "identifier" text not null, "value" text not null, "expiresAt" timestamptz not null, "createdAt" timestamptz default CURRENT_TIMESTAMP not null, "updatedAt" timestamptz default CURRENT_TIMESTAMP not null);

create table "organization" ("id" text not null primary key, "name" text not null, "slug" text not null unique, "logo" text, "createdAt" timestamptz not null, "metadata" text, "legacyWorkosOrganizationId" text unique, "requireSso" boolean, "requireAdminStepUp" boolean, "maxSessionAgeSeconds" integer, "idleTimeoutSeconds" integer, "brandName" text, "brandLogoUrl" text, "brandWordmarkUrl" text, "brandFaviconUrl" text, "brandPrimaryColor" text, "brandAccentColor" text, "supportEmail" text, "documentationUrl" text, "termsUrl" text, "privacyUrl" text);

create table "member" ("id" text not null primary key, "organizationId" text not null references "organization" ("id") on delete cascade, "userId" text not null references "user" ("id") on delete cascade, "role" text not null, "createdAt" timestamptz not null);

create table "invitation" ("id" text not null primary key, "organizationId" text not null references "organization" ("id") on delete cascade, "email" text not null, "role" text, "status" text not null, "expiresAt" timestamptz not null, "createdAt" timestamptz default CURRENT_TIMESTAMP not null, "inviterId" text not null references "user" ("id") on delete cascade);

create table "passkey" ("id" text not null primary key, "name" text, "publicKey" text not null, "userId" text not null references "user" ("id") on delete cascade, "credentialID" text not null, "counter" integer not null, "deviceType" text not null, "backedUp" boolean not null, "transports" text, "createdAt" timestamptz, "aaguid" text);

create table "twoFactor" ("id" text not null primary key, "secret" text not null, "backupCodes" text not null, "userId" text not null references "user" ("id") on delete cascade, "verified" boolean, "failedVerificationCount" integer, "lockedUntil" timestamptz);

create table "ssoProvider" ("id" text not null primary key, "issuer" text not null, "oidcConfig" text, "samlConfig" text, "userId" text not null references "user" ("id") on delete cascade, "providerId" text not null unique, "organizationId" text, "domain" text not null);

create table "scimProvider" ("id" text not null primary key, "providerId" text not null unique, "scimToken" text not null unique, "organizationId" text);

create table "jwks" ("id" text not null primary key, "publicKey" text not null, "privateKey" text not null, "createdAt" timestamptz not null, "expiresAt" timestamptz);

create table "rateLimit" ("id" text not null primary key, "key" text not null unique, "count" integer not null, "lastRequest" bigint not null);

create index "session_userId_idx" on "session" ("userId");

create index "account_userId_idx" on "account" ("userId");

create index "verification_identifier_idx" on "verification" ("identifier");

create index "member_organizationId_idx" on "member" ("organizationId");

create index "member_userId_idx" on "member" ("userId");

create unique index "member_organization_user_unique"
  on "member" ("organizationId", "userId");

create index "invitation_organizationId_idx" on "invitation" ("organizationId");

create index "invitation_email_idx" on "invitation" ("email");

create index "passkey_userId_idx" on "passkey" ("userId");

create index "passkey_credentialID_idx" on "passkey" ("credentialID");

create index "twoFactor_secret_idx" on "twoFactor" ("secret");

create index "twoFactor_userId_idx" on "twoFactor" ("userId");

create table "scim_group" (
  "id" uuid not null primary key,
  "providerId" text not null references "scimProvider" ("providerId") on delete cascade,
  "organizationId" text not null references "organization" ("id") on delete cascade,
  "externalId" text not null,
  "displayName" text not null,
  "createdAt" timestamptz default CURRENT_TIMESTAMP not null,
  "updatedAt" timestamptz default CURRENT_TIMESTAMP not null,
  unique ("providerId", "externalId")
);

create table "scim_group_member" (
  "groupId" uuid not null references "scim_group" ("id") on delete cascade,
  "userId" text not null references "user" ("id") on delete cascade,
  primary key ("groupId", "userId")
);

create table "scim_group_role_mapping" (
  "providerId" text not null references "scimProvider" ("providerId") on delete cascade,
  "groupExternalId" text not null,
  "role" text not null check ("role" in ('admin', 'member')),
  primary key ("providerId", "groupExternalId")
);

create table "identity_audit_event" (
  "id" uuid not null primary key,
  "actorId" text,
  "organizationId" text,
  "action" text not null,
  "targetId" text,
  "result" text not null check ("result" in ('success', 'failure')),
  "requestId" text,
  "createdAt" timestamptz default CURRENT_TIMESTAMP not null
);

create index "identity_audit_event_organization_created_idx"
  on "identity_audit_event" ("organizationId", "createdAt" desc);

create function reject_identity_audit_mutation() returns trigger language plpgsql as $$
begin
  raise exception 'identity audit events are append-only';
end
$$;

create trigger identity_audit_event_append_only
before update or delete on "identity_audit_event"
for each row execute function reject_identity_audit_mutation();
