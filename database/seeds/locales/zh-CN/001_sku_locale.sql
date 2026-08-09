-- ============================================================================
-- zh-CN locale seed: SKU display names and spec_json tags.
--
-- The merchandise module owns the commerce_* product tables and their base
-- rows (seeds/common/001_merchandise_bootstrap.sql). Locale-specific display
-- text lives here in the table owner's seeds/locales/{locale}/ per I18N_SPEC
-- database seed i18n, and runs AFTER the common seed in the same module.
--
-- Sku ids align with membership product references
-- (membership_product_sku / membership_package.sku_id = commerce_product_sku.id).
--
-- Tags: 10元=100算力元, daily = point_amount/30 (monthly sub rate)
-- Basic: 540/30=18, Standard: 1560/30=52, Premium: 4290/30=143, Super: 10720/30≈357
-- ============================================================================

UPDATE commerce_product_sku SET name = '基础版-连续包年', title = '基础版-连续包年', spec_json = '{"tags":["首年9.7折","18算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-annual';
UPDATE commerce_product_sku SET name = '标准版-连续包年', title = '标准版-连续包年', spec_json = '{"tags":["首年9.7折","52算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-annual';
UPDATE commerce_product_sku SET name = '巅峰版-连续包年', title = '巅峰版-连续包年', spec_json = '{"tags":["首年9.7折","143算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-annual';
UPDATE commerce_product_sku SET name = '基础版-连续包月', title = '基础版-连续包月', spec_json = '{"tags":["9.9折","18算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-monthly';
UPDATE commerce_product_sku SET name = '标准版-连续包月', title = '标准版-连续包月', spec_json = '{"tags":["9.9折","52算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-monthly';
UPDATE commerce_product_sku SET name = '巅峰版-连续包月', title = '巅峰版-连续包月', spec_json = '{"tags":["9.9折","143算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-monthly';
UPDATE commerce_product_sku SET name = '基础版-连续包季', title = '基础版-连续包季', spec_json = '{"tags":["9.8折","18算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-quarterly';
UPDATE commerce_product_sku SET name = '标准版-连续包季', title = '标准版-连续包季', spec_json = '{"tags":["9.8折","52算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-quarterly';
UPDATE commerce_product_sku SET name = '巅峰版-连续包季', title = '巅峰版-连续包季', spec_json = '{"tags":["9.8折","143算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-quarterly';
UPDATE commerce_product_sku SET name = '基础版-单月购买', title = '基础版-单月购买', spec_json = '{"tags":["18算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-basic-single';
UPDATE commerce_product_sku SET name = '标准版-单月购买', title = '标准版-单月购买', spec_json = '{"tags":["52算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-standard-single';
UPDATE commerce_product_sku SET name = '巅峰版-单月购买', title = '巅峰版-单月购买', spec_json = '{"tags":["143算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-premium-single';
UPDATE commerce_product_sku SET name = '超级版-连续包年', title = '超级版-连续包年', spec_json = '{"tags":["首年9.7折","357算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-annual';
UPDATE commerce_product_sku SET name = '超级版-连续包月', title = '超级版-连续包月', spec_json = '{"tags":["9.9折","357算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-monthly';
UPDATE commerce_product_sku SET name = '超级版-连续包季', title = '超级版-连续包季', spec_json = '{"tags":["9.8折","357算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-quarterly';
UPDATE commerce_product_sku SET name = '超级版-单月购买', title = '超级版-单月购买', spec_json = '{"tags":["357算力元/天"]}', updated_at = CURRENT_TIMESTAMP WHERE id = 'sku-super-single';
