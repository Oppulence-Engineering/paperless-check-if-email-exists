-- True when the tenant's plan came from billing (the plan claim in the web
-- session token). A Free claim downgrades only such a plan, so a paid tier
-- that an operator set through the admin API is kept.
ALTER TABLE tenants ADD COLUMN plan_from_billing BOOLEAN NOT NULL DEFAULT false;
