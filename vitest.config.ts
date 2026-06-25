import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  resolve: {
    alias: {
      "@sdkwork/utils": path.resolve(root, "../sdkwork-utils/packages/sdkwork-utils-typescript/src/index.ts"),
      "@sdkwork/merchandise-contracts": path.resolve(
        root,
        "packages/common/merchandise/sdkwork-merchandise-contracts/src/index.ts",
      ),
    },
  },
  test: {
    environment: "node",
    include: ["packages/common/merchandise/**/*.test.ts"],
  },
});
