do $$
begin
  if exists (select 1 from "scimProvider")
    or exists (select 1 from "scim_group")
    or exists (select 1 from "scim_group_member")
    or exists (select 1 from "scim_group_role_mapping") then
    raise exception 'Better Auth 1.7 SCIM cutover requires empty legacy SCIM tables';
  end if;
end
$$;

drop table "scim_group_member";
drop table "scim_group_role_mapping";
drop table "scim_group";
drop table "scimProvider";

alter table "jwks" add column "alg" text;
alter table "jwks" add column "crv" text;

create table "scimManagedConnection" (
  "id" text not null primary key,
  "creationRequestId" text not null unique,
  "connectionId" text not null unique,
  "provisioningDomainId" text not null,
  "status" text not null,
  "revision" integer not null,
  "createdAt" timestamptz not null,
  "createdBy" text not null,
  "decommissionStartedAt" timestamptz,
  "decommissionStartedBy" text,
  "decommissionedAt" timestamptz,
  "decommissionedBy" text
);

create table "scimManagedCredential" (
  "id" text not null primary key,
  "connectionRecordId" text not null references "scimManagedConnection" ("id") on delete cascade,
  "credentialId" text not null unique,
  "tokenDigest" text not null,
  "hashVersion" text not null,
  "activeSlotKey" text not null unique,
  "status" text not null,
  "serializedScopes" text not null,
  "expiresAt" timestamptz not null,
  "createdAt" timestamptz not null,
  "createdBy" text not null,
  "lastUsedAt" timestamptz,
  "revokedAt" timestamptz,
  "revokedBy" text,
  "decommissionedAt" timestamptz
);

create table "scimManagedConnectionEvent" (
  "id" text not null primary key,
  "connectionRecordId" text not null references "scimManagedConnection" ("id") on delete cascade,
  "eventKey" text not null unique,
  "sequence" integer not null,
  "type" text not null,
  "actorId" text not null,
  "credentialId" text,
  "createdAt" timestamptz not null
);

create table "scimConnectionBinding" (
  "id" text not null primary key,
  "connectionId" text not null,
  "connectionKey" text not null unique,
  "provisioningDomainId" text not null,
  "createdAt" timestamptz not null,
  "decommissionedAt" timestamptz,
  "decommissionStatus" text not null,
  "decommissionCursorUserId" text,
  "decommissionReconciledUserCount" integer not null,
  "decommissionBatchCount" integer not null,
  "decommissionRevision" integer not null,
  "decommissionCompletedAt" timestamptz,
  "decommissionLeaseId" text,
  "decommissionLeaseExpiresAt" timestamptz
);

create table "scimIdentityTombstone" (
  "id" text not null primary key,
  "connectionId" text not null,
  "provisioningDomainId" text not null,
  "externalId" text not null,
  "externalIdKey" text not null unique,
  "userId" text not null references "user" ("id") on delete cascade,
  "profile" text not null,
  "deletedAt" timestamptz not null
);

create table "scimSubject" (
  "id" text not null primary key,
  "userId" text not null unique references "user" ("id") on delete cascade,
  "profileSourceId" text,
  "revision" integer not null,
  "createdAt" timestamptz not null,
  "updatedAt" timestamptz not null
);

create table "scimUser" (
  "id" text not null primary key,
  "connectionId" text not null,
  "provisioningDomainId" text not null,
  "userId" text not null references "user" ("id") on delete cascade,
  "connectionUserKey" text not null unique,
  "userName" text not null,
  "userNameKey" text not null unique,
  "primaryEmail" text not null,
  "workEmailValueIndex" text not null,
  "emailValueIndex" text not null,
  "displayName" text not null,
  "formattedName" text not null,
  "givenName" text,
  "familyName" text,
  "serializedEmails" text not null,
  "serializedAttributes" text,
  "externalId" text,
  "externalIdKey" text unique,
  "active" boolean not null,
  "orderKey" text not null unique,
  "createdAt" timestamptz not null,
  "updatedAt" timestamptz not null
);

create table "scimProjectionGrant" (
  "id" text not null primary key,
  "connectionId" text not null,
  "provisioningDomainId" text not null,
  "scimUserId" text not null references "scimUser" ("id") on delete cascade,
  "userId" text not null references "user" ("id") on delete cascade,
  "sourceKind" text not null,
  "sourceId" text not null,
  "sourceValue" text,
  "role" text not null,
  "grantKey" text not null unique,
  "createdAt" timestamptz not null,
  "updatedAt" timestamptz not null
);

create table "scimGroup" (
  "id" text not null primary key,
  "connectionId" text not null,
  "provisioningDomainId" text not null,
  "revision" integer not null,
  "displayName" text not null,
  "displayNameKey" text not null unique,
  "externalId" text,
  "externalIdKey" text unique,
  "orderKey" text not null unique,
  "createdAt" timestamptz not null,
  "updatedAt" timestamptz not null
);

create table "scimGroupMember" (
  "id" text not null primary key,
  "connectionId" text not null,
  "groupId" text not null references "scimGroup" ("id") on delete cascade,
  "scimUserId" text not null references "scimUser" ("id") on delete cascade,
  "membershipKey" text not null unique,
  "createdAt" timestamptz not null
);

create table "scim_group_role_mapping" (
  "connectionId" text not null references "scimManagedConnection" ("connectionId") on delete cascade,
  "organizationId" text not null references "organization" ("id") on delete cascade,
  "groupExternalId" text not null,
  "role" text not null check ("role" in ('admin', 'member')),
  primary key ("connectionId", "groupExternalId")
);

create index "scimManagedConnection_provisioningDomainId_idx" on "scimManagedConnection" ("provisioningDomainId");
create index "scimManagedCredential_connectionRecordId_idx" on "scimManagedCredential" ("connectionRecordId");
create index "scimManagedConnectionEvent_connectionRecordId_idx" on "scimManagedConnectionEvent" ("connectionRecordId");
create index "scimConnectionBinding_connectionId_idx" on "scimConnectionBinding" ("connectionId");
create index "scimIdentityTombstone_connectionId_idx" on "scimIdentityTombstone" ("connectionId");
create index "scimIdentityTombstone_provisioningDomainId_idx" on "scimIdentityTombstone" ("provisioningDomainId");
create index "scimIdentityTombstone_userId_idx" on "scimIdentityTombstone" ("userId");
create index "scimSubject_profileSourceId_idx" on "scimSubject" ("profileSourceId");
create index "scimUser_connectionId_idx" on "scimUser" ("connectionId");
create index "scimUser_provisioningDomainId_idx" on "scimUser" ("provisioningDomainId");
create index "scimUser_userId_idx" on "scimUser" ("userId");
create index "scimProjectionGrant_connectionId_idx" on "scimProjectionGrant" ("connectionId");
create index "scimProjectionGrant_provisioningDomainId_idx" on "scimProjectionGrant" ("provisioningDomainId");
create index "scimProjectionGrant_scimUserId_idx" on "scimProjectionGrant" ("scimUserId");
create index "scimProjectionGrant_userId_idx" on "scimProjectionGrant" ("userId");
create index "scimGroup_connectionId_idx" on "scimGroup" ("connectionId");
create index "scimGroup_provisioningDomainId_idx" on "scimGroup" ("provisioningDomainId");
create index "scimGroupMember_connectionId_idx" on "scimGroupMember" ("connectionId");
create index "scimGroupMember_groupId_idx" on "scimGroupMember" ("groupId");
create index "scimGroupMember_scimUserId_idx" on "scimGroupMember" ("scimUserId");
