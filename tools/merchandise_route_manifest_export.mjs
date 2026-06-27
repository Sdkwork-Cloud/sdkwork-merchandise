#!/usr/bin/env node
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const checkMode = process.argv.includes("--check");

const manifestPath = path.join(
  root,
  "sdks/_route-manifests/app-api/sdkwork-merchandise-standalone-gateway.route-manifest.json",
);
const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));

if (!Array.isArray(manifest.routes) || manifest.routes.length === 0) {
  console.error("[merchandise_route_manifest_export] route manifest must contain routes");
  process.exit(1);
}

for (const route of manifest.routes) {
  if (!route.requestContext || !route.apiSurface || !route.operationId) {
    console.error("[merchandise_route_manifest_export] route missing framework metadata", route);
    process.exit(1);
  }
}

console.log(`[merchandise_route_manifest_export] ${checkMode ? "check ok" : "export ok"} (${manifest.routes.length} routes)`);
