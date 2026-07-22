import { isBlank, slugify } from "@sdkwork/utils";

import type {
  CreateProductInput,
  ProductProfile,
} from "@sdkwork/merchandise-contracts";

export function normalizeCreateProductInput(
  input: CreateProductInput,
): CreateProductInput {
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

export function formatProductHeadline(profile: ProductProfile): string {
  return `${profile.name} (${profile.status})`;
}
