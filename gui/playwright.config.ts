import { defineConfig } from "@playwright/test";

// E2E runs against the Vite dev server in a real browser; without Tauri the
// frontend automatically uses its deterministic mock backend. (tauri-driver
// has no macOS support, so the full native shell isn't e2e-testable here.)
export default defineConfig({
  testDir: "e2e",
  use: {
    baseURL: "http://localhost:1421",
  },
  webServer: {
    command: "npx vite --port 1421 --strictPort",
    port: 1421,
    reuseExistingServer: false,
  },
});
