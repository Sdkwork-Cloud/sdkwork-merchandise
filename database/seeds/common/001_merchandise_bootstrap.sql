-- Merchandise product catalog seed for tenant 100001.
-- Physical data for the shared commerce product tables (commerce_product_spu /
-- commerce_product_sku). The catalog module declares these tables as a
-- reference-only contract; merchandise owns the commerce_ prefix and lands
-- the catalog rows here.
--
-- Sku ids align with the membership module product tables
-- (membership_product_sku.id = commerce_product_sku.id), so membership
-- packages (membership_package.sku_id) resolve through the commerce catalog
-- with the same identity in every deployment.
--
-- ON CONFLICT (id) DO UPDATE SET keeps re-seeding idempotent and allows seed
-- corrections to overwrite previously garbled rows.

INSERT INTO commerce_product_spu (
  id, tenant_id, organization_id, spu_no, name, title, product_type, status
) VALUES (
  'seed-product-membership', '100001', '0', 'membership-catalog',
  'Membership Catalog', 'Membership Catalog', 'membership', 'active'
) ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name,
  title = EXCLUDED.title,
  product_type = EXCLUDED.product_type,
  status = EXCLUDED.status,
  updated_at = CURRENT_TIMESTAMP;

INSERT INTO commerce_product_sku (
  id, tenant_id, organization_id, spu_id, sku_no, name, title, price_amount,
  original_price_amount, currency_code, fulfillment_type, inventory_tracking,
  sales_status, status, spec_json, created_at, updated_at
) VALUES
  ('sku-basic-annual', '100001', '0', 'seed-product-membership', 'basic-annual', 'Basic Annual', 'Basic Annual', '640', '660', 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["3% off","18 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-standard-annual', '100001', '0', 'seed-product-membership', 'standard-annual', 'Standard Annual', 'Standard Annual', '1839', '1896', 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["3% off","52 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-premium-annual', '100001', '0', 'seed-product-membership', 'premium-annual', 'Premium Annual', 'Premium Annual', '5040', '5196', 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["3% off","143 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-super-annual', '100001', '0', 'seed-product-membership', 'super-annual', 'Super Annual', 'Super Annual', '12606', '12996', 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["3% off","357 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-basic-monthly', '100001', '0', 'seed-product-membership', 'basic-monthly', 'Basic Monthly', 'Basic Monthly', '54', '55', 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["1% off","18 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-standard-monthly', '100001', '0', 'seed-product-membership', 'standard-monthly', 'Standard Monthly', 'Standard Monthly', '156', '158', 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["1% off","52 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-premium-monthly', '100001', '0', 'seed-product-membership', 'premium-monthly', 'Premium Monthly', 'Premium Monthly', '429', '433', 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["1% off","143 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-super-monthly', '100001', '0', 'seed-product-membership', 'super-monthly', 'Super Monthly', 'Super Monthly', '1072', '1083', 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["1% off","357 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-basic-quarterly', '100001', '0', 'seed-product-membership', 'basic-quarterly', 'Basic Quarterly', 'Basic Quarterly', '162', '165', 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["2% off","18 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-standard-quarterly', '100001', '0', 'seed-product-membership', 'standard-quarterly', 'Standard Quarterly', 'Standard Quarterly', '465', '474', 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["2% off","52 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-premium-quarterly', '100001', '0', 'seed-product-membership', 'premium-quarterly', 'Premium Quarterly', 'Premium Quarterly', '1273', '1299', 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["2% off","143 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-super-quarterly', '100001', '0', 'seed-product-membership', 'super-quarterly', 'Super Quarterly', 'Super Quarterly', '3184', '3249', 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["2% off","357 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-basic-single', '100001', '0', 'seed-product-membership', 'basic-single', 'Basic Single', 'Basic Single', '55', NULL, 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["18 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-standard-single', '100001', '0', 'seed-product-membership', 'standard-single', 'Standard Single', 'Standard Single', '158', NULL, 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["52 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-premium-single', '100001', '0', 'seed-product-membership', 'premium-single', 'Premium Single', 'Premium Single', '433', NULL, 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["143 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('sku-super-single', '100001', '0', 'seed-product-membership', 'super-single', 'Super Single', 'Super Single', '1083', NULL, 'CNY', 'membership_activation', 'untracked', 'active', 'active', '{"tags":["357 credits/day"]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT (id) DO UPDATE SET
  spu_id = EXCLUDED.spu_id,
  name = EXCLUDED.name,
  title = EXCLUDED.title,
  price_amount = EXCLUDED.price_amount,
  original_price_amount = EXCLUDED.original_price_amount,
  currency_code = EXCLUDED.currency_code,
  fulfillment_type = EXCLUDED.fulfillment_type,
  inventory_tracking = EXCLUDED.inventory_tracking,
  sales_status = EXCLUDED.sales_status,
  status = EXCLUDED.status,
  spec_json = EXCLUDED.spec_json,
  updated_at = CURRENT_TIMESTAMP;
