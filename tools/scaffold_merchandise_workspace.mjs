#!/usr/bin/env node
/**
 * Bootstrap sdkwork-merchandise workspace aligned with sdkwork-specs.
 * Run: node tools/scaffold_merchandise_workspace.mjs
 */
import { mkdir, writeFile, access } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

async function exists(filePath) {
  try {
    await access(filePath);
    return true;
  } catch {
    return false;
  }
}

async function writeIfMissing(relativePath, content) {
  const fullPath = path.join(root, relativePath);
  if (await exists(fullPath)) {
    return false;
  }
  await mkdir(path.dirname(fullPath), { recursive: true });
  await writeFile(fullPath, content, "utf8");
  return true;
}

async function writeAlways(relativePath, content) {
  const fullPath = path.join(root, relativePath);
  await mkdir(path.dirname(fullPath), { recursive: true });
  await writeFile(fullPath, content, "utf8");
}

const placeholderDirs = [
  "apis/README.md",
  "apis/app-api/merchandise/examples/.gitkeep",
  "apis/app-api/merchandise/changelogs/.gitkeep",
  "apis/backend-api/merchandise/examples/.gitkeep",
  "apis/backend-api/merchandise/changelogs/.gitkeep",
  "apps/README.md",
  "crates/README.md",
  "sdks/README.md",
  "jobs/README.md",
  "tools/README.md",
  "plugins/README.md",
  "examples/README.md",
  "configs/README.md",
  "deployments/README.md",
  "scripts/README.md",
  "docs/README.md",
  "tests/README.md",
  ".sdkwork/README.md",
  ".sdkwork/skills/README.md",
  ".sdkwork/plugins/README.md",
];

for (const dir of placeholderDirs) {
  const content = dir.endsWith(".gitkeep") ? "" : `# ${path.basename(path.dirname(dir)) || path.basename(dir, ".md")}\n\nSee \`../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md\`.\n`;
  await writeIfMissing(dir, content);
}

await writeAlways(
  "AGENTS.md",
  `# Repository Guidelines

## SDKWORK Soul

Read \`../sdkwork-specs/SOUL.md\` before executing tasks in this root.

## SDKWORK Standards

- \`../sdkwork-specs/README.md\`
- \`../sdkwork-specs/SOUL.md\`
- \`../sdkwork-specs/AGENTS_SPEC.md\`
- \`../sdkwork-specs/WEB_FRAMEWORK_SPEC.md\`
- \`../sdkwork-specs/DATABASE_FRAMEWORK_SPEC.md\`

## Application Identity

Application manifests live under \`apps/*/sdkwork.app.config.json\`. This repository root is the commerce merchandise capability workspace (\`domain: commerce\`, \`capability: merchandise\`).

## Project Rules

- Canonical domain: \`commerce\`; capability: \`merchandise\` (\`DOMAIN_SPEC.md\`).
- Database table prefix: \`commerce_\` for merchandise-owned tables.
- App API prefix: \`/app/v3/api/catalog/products\`.
- Backend API prefix: \`/backend/v3/api/catalog/products\`.
- Rust HTTP runtimes integrate \`sdkwork-web-framework\`; database lifecycle uses \`sdkwork-database\`.
- TypeScript packages consume \`@sdkwork/utils-typescript\` for shared helpers — no local duplicates.
- \`sdkwork-discovery\` is deferred until RPC/cloud-split deployment exists.
- Generated SDK output under \`sdks/**/generated/**\` is generator-owned.

## Verification

\`\`\`bash
pnpm verify
pnpm db:validate
\`\`\`
`,
);

for (const shim of ["CLAUDE.md", "GEMINI.md", "CODEX.md"]) {
  await writeIfMissing(
    shim,
    `# ${shim.replace(".md", "")} Compatibility Shim

Read \`AGENTS.md\` in this directory. Do not duplicate SDKWork rules here.
`,
  );
}

await writeAlways(
  "README.md",
  `# sdkwork-merchandise

SDKWork commerce **merchandise** capability application: merchant merchandise configuration, onboarding, and operator console.

- Standards: \`../sdkwork-specs/README.md\`
- Domain: \`commerce\` / capability: \`merchandise\`
- PC app: \`apps/sdkwork-merchandise-pc/\`
- HTTP API: \`crates/sdkwork-merchandise-standalone-gateway/\`
- Database: \`database/\` via \`sdkwork-database\`

## Quick start

\`\`\`bash
pnpm install
pnpm verify
pnpm --dir apps/sdkwork-merchandise-pc dev
\`\`\`
`,
);

await writeAlways(
  "specs/component.spec.json",
  JSON.stringify(
    {
      schemaVersion: 1,
      kind: "sdkwork.component.spec",
      component: {
        name: "sdkwork-merchandise-workspace",
        displayName: "SDKWork Merchandise Workspace",
        version: "0.1.0",
        type: "rust-crate",
        root: "sdkwork-merchandise",
        domain: "commerce",
        capability: "merchandise",
        languages: ["typescript", "rust"],
        generated: false,
        manifests: ["package.json", "Cargo.toml"],
      },
      canonicalSpecs: [
        { file: "WEB_FRAMEWORK_SPEC.md", path: "../sdkwork-specs/WEB_FRAMEWORK_SPEC.md" },
        { file: "DATABASE_FRAMEWORK_SPEC.md", path: "../sdkwork-specs/DATABASE_FRAMEWORK_SPEC.md" },
        { file: "API_SPEC.md", path: "../sdkwork-specs/API_SPEC.md" },
        { file: "APP_PC_ARCHITECTURE_SPEC.md", path: "../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md" },
      ],
      contracts: {
        publicExports: ["."],
        runtimeEntrypoints: ["package.json#scripts.verify"],
        routeManifest: "sdks/_route-manifests/app-api/sdkwork-merchandise-standalone-gateway.route-manifest.json",
        sdkClients: [],
        events: [],
        configKeys: [],
      },
      verification: { commands: ["pnpm verify"] },
    },
    null,
    2,
  ) + "\n",
);

await writeAlways(
  "specs/README.md",
  `# sdkwork-merchandise component specs

Local narrowing rules for the commerce merchandise capability workspace. Canonical standards remain in \`../sdkwork-specs/\`.
`,
);

console.log("[scaffold_merchandise_workspace] base structure written");
