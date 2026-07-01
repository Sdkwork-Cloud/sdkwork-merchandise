import { isBlank, slugify } from "@sdkwork/utils";

import type { CreateShopInput, ShopProfile } from "@sdkwork/merchandise-contracts";

export function normalizeCreateShopInput(input: CreateShopInput): CreateShopInput {
  const name = input.name.trim();
  const slug = slugify(input.slug.trim() || name);
  if (isBlank(name)) {
    throw new Error("merchandise name is required");
  }
  if (isBlank(slug)) {
    throw new Error("merchandise slug is required");
  }
  return { name, slug };
}

export function formatShopHeadline(profile: ShopProfile): string {
  return `${profile.name} (${profile.status})`;
}
