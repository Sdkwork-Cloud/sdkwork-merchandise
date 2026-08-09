-- ============================================================================
-- en-US locale seed: SKU display names and spec_json tags.
--
-- The merchandise module owns the commerce_* product tables and their base
-- rows (seeds/common/001_merchandise_bootstrap.sql). Locale-specific display
-- text lives here in the table owner's seeds/locales/{locale}/ per I18N_SPEC
-- database seed i18n, and runs AFTER the common seed in the same module.
--
-- Sku ids align with membership product references
-- (membership_product_sku / membership_package.sku_id = commerce_product_sku.id).
--
-- Tags: ¥10=100 credits, daily = point_amount/30 (monthly sub rate)
-- Basic: 540/30=18, Standard: 1560/30=52, Premium: 4290/30=143, Super: 10720/30≈357
-- ============================================================================

UPDATE commerce_product_sku SET name = 'Basic Annual', title = 'Basic Annual', spec_json = '{"tags":["3% off","18 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-annual';
UPDATE commerce_product_sku SET name = 'Standard Annual', title = 'Standard Annual', spec_json = '{"tags":["3% off","52 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-annual';
UPDATE commerce_product_sku SET name = 'Premium Annual', title = 'Premium Annual', spec_json = '{"tags":["3% off","143 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-annual';
UPDATE commerce_product_sku SET name = 'Basic Monthly', title = 'Basic Monthly', spec_json = '{"tags":["1% off","18 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-monthly';
UPDATE commerce_product_sku SET name = 'Standard Monthly', title = 'Standard Monthly', spec_json = '{"tags":["1% off","52 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-monthly';
UPDATE commerce_product_sku SET name = 'Premium Monthly', title = 'Premium Monthly', spec_json = '{"tags":["1% off","143 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-monthly';
UPDATE commerce_product_sku SET name = 'Basic Quarterly', title = 'Basic Quarterly', spec_json = '{"tags":["2% off","18 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-quarterly';
UPDATE commerce_product_sku SET name = 'Standard Quarterly', title = 'Standard Quarterly', spec_json = '{"tags":["2% off","52 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-quarterly';
UPDATE commerce_product_sku SET name = 'Premium Quarterly', title = 'Premium Quarterly', spec_json = '{"tags":["2% off","143 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-quarterly';
UPDATE commerce_product_sku SET name = 'Basic Single', title = 'Basic Single', spec_json = '{"tags":["18 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-single';
UPDATE commerce_product_sku SET name = 'Standard Single', title = 'Standard Single', spec_json = '{"tags":["52 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-single';
UPDATE commerce_product_sku SET name = 'Premium Single', title = 'Premium Single', spec_json = '{"tags":["143 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-single';
UPDATE commerce_product_sku SET name = 'Super Annual', title = 'Super Annual', spec_json = '{"tags":["3% off","357 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-annual';
UPDATE commerce_product_sku SET name = 'Super Monthly', title = 'Super Monthly', spec_json = '{"tags":["1% off","357 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-monthly';
UPDATE commerce_product_sku SET name = 'Super Quarterly', title = 'Super Quarterly', spec_json = '{"tags":["2% off","357 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-quarterly';
UPDATE commerce_product_sku SET name = 'Super Single', title = 'Super Single', spec_json = '{"tags":["357 credits/day"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-single';
