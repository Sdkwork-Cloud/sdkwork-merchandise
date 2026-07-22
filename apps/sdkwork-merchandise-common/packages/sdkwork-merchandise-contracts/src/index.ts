export type ProductStatus = "draft" | "pending" | "active" | "suspended";

export interface ProductProfile {
  id: string;
  name: string;
  slug: string;
  status: ProductStatus | string;
}

export interface CreateProductInput {
  name: string;
  slug: string;
}
