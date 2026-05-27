import { defineConfig, devices } from '@playwright/test';

/**
 * Playwright configuration for the Kicks Tauri frontend.
 *
 * In dev mode we target the Vite dev server so the frontend can be tested
 * without building the full Tauri binary. In CI the same tests can be run
 * against a built Tauri app by overriding the `webServer` / `use.baseURL`.
 */
export default defineConfig({
  testDir: './e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'list',
  use: {
    baseURL: 'http://localhost:5173',
    trace: 'on-first-retry',
    // The Tauri frontend uses CSS custom properties; avoid color-scheme issues
    colorScheme: 'dark',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },
  ],
  webServer: {
    command: 'npm run dev',
    url: 'http://localhost:5173',
    reuseExistingServer: !process.env.CI,
    timeout: 120 * 1000,
  },
});
