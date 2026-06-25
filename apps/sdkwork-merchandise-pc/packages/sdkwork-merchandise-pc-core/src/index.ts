import { formatShopHeadline, normalizeCreateShopInput } from "@sdkwork/merchandise-service";

import type { CreateShopInput, ShopProfile } from "@sdkwork/merchandise-contracts";

export function buildShopDraft(input: CreateShopInput): CreateShopInput {
  return normalizeCreateShopInput(input);
}

export function describeShop(profile: ShopProfile): string {
  return formatShopHeadline(profile);
}
