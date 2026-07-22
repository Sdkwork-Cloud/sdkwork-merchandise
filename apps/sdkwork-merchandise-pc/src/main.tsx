import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { MerchandiseAppShell } from "@sdkwork/merchandise-pc-shell";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <MerchandiseAppShell />
  </StrictMode>,
);
