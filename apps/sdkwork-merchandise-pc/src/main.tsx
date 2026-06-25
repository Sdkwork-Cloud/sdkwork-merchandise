import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { ShopAppShell } from "@sdkwork/merchandise-pc-shell";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ShopAppShell />
  </StrictMode>,
);
