import { buildShopDraft, describeShop } from "@sdkwork/merchandise-pc-core";

const demoShop = buildShopDraft({ name: "示例店铺", slug: "" });

export function ShopAppShell() {
  const headline = describeShop({
    id: "demo",
    name: demoShop.name,
    slug: demoShop.slug,
    status: "draft",
  });

  return (
    <main className="merchandise-shell">
      <section className="merchandise-card">
        <h1>SDKWork Merchandise</h1>
        <p>{headline}</p>
        <p>Merchant merchandise console scaffold aligned with sdkwork-specs.</p>
      </section>
    </main>
  );
}
