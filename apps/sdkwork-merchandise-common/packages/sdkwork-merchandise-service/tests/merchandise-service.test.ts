import { describe, expect, it } from "vitest";

import { formatProductHeadline, normalizeCreateProductInput } from "../src/index.ts";

describe("merchandise service helpers", () => {
  it("normalizes slug via sdkwork-utils", () => {
    expect(normalizeCreateProductInput({ name: "Demo Merchandise", slug: "" })).toEqual({
      name: "Demo Merchandise",
      slug: "demo-merchandise",
    });
  });

  it("formats merchandise headline", () => {
    expect(
      formatProductHeadline({ id: "1", name: "Demo", slug: "demo", status: "draft" }),
    ).toBe("Demo (draft)");
  });
});
