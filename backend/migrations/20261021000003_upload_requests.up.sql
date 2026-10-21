CREATE TABLE v1_list_upload_requests (
 tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
 request_key TEXT NOT NULL, request_hash BYTEA NOT NULL, response JSONB NOT NULL,
 created_at TIMESTAMPTZ NOT NULL DEFAULT now(), PRIMARY KEY (tenant_id, request_key)
);
