-- SDKWork merchandise baseline (commerce_shop core tables)
CREATE TABLE IF NOT EXISTS commerce_shop (
  id UUID PRIMARY KEY,
  tenant_id UUID NOT NULL,
  organization_id UUID NOT NULL,
  name TEXT NOT NULL,
  slug TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'draft',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_commerce_shop_tenant_slug
  ON commerce_shop (tenant_id, slug);

CREATE TABLE IF NOT EXISTS commerce_shop_application (
  id UUID PRIMARY KEY,
  shop_id UUID NOT NULL REFERENCES commerce_shop(id),
  applicant_user_id UUID NOT NULL,
  status TEXT NOT NULL DEFAULT 'pending',
  submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS commerce_shop_status_event (
  id UUID PRIMARY KEY,
  shop_id UUID NOT NULL REFERENCES commerce_shop(id),
  from_status TEXT,
  to_status TEXT NOT NULL,
  actor_user_id UUID,
  recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
