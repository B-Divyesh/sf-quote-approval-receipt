import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  testMatch: '**/*.spec.ts',
  fullyParallel: false,
  retries: 0,
  reporter: [['list'], ['html', { outputFolder: 'test-report', open: 'never' }]],
  use: { baseURL: 'http://127.0.0.1:4173', trace: 'retain-on-failure', screenshot: 'only-on-failure' },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'mobile-390', use: { ...devices['iPhone 13'], browserName: 'chromium', viewport: { width: 390, height: 844 } }, grep: /@mobile/ }
  ],
  webServer: {
    command: 'DATA_DIR=.test-data STATIC_DIR=dist PORT=4173 cargo run',
    url: 'http://127.0.0.1:4173/health',
    reuseExistingServer: false,
    timeout: 120_000
  }
});
