import {
  formatProductHeadline,
  normalizeCreateProductInput,
} from "@sdkwork/merchandise-service";

import type {
  CreateProductInput,
  ProductProfile,
} from "@sdkwork/merchandise-contracts";

export function buildProductDraft(input: CreateProductInput): CreateProductInput {
  return normalizeCreateProductInput(input);
}

export function describeProduct(profile: ProductProfile): string {
  return formatProductHeadline(profile);
}
