export type ShopStatus = "draft" | "pending" | "active" | "suspended";

export interface ShopProfile {
  id: string;
  name: string;
  slug: string;
  status: ShopStatus | string;
}

export interface CreateShopInput {
  name: string;
  slug: string;
}
