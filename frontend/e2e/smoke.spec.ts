import { test, expect } from '@playwright/test';

test.describe('Kicks Frontend Smoke Tests', () => {
  test('homepage loads with title', async ({ page }) => {
    await page.goto('/');
    const toolbar = page.locator('[data-testid="toolbar"]');
    await expect(toolbar).toContainText('KICKS');
    await expect(toolbar).toContainText('Guitar Workstation');
  });

  test('sidebar navigation is present', async ({ page }) => {
    await page.goto('/');
    const sidebar = page.locator('nav');
    await expect(sidebar).toBeVisible();
    await expect(sidebar).toContainText('Signal Chain');
    await expect(sidebar).toContainText('Presets');
    await expect(sidebar).toContainText('Settings');
  });

  test('clicking a nav item updates active state', async ({ page }) => {
    await page.goto('/');
    const midiBtn = page.locator('nav button', { hasText: 'MIDI' });
    await midiBtn.click();
    // The active class is applied to the clicked button
    await expect(midiBtn).toHaveClass(/bg-\[var\(--accent-bg\)\]/);
  });

  test('status bar shows version info', async ({ page }) => {
    await page.goto('/');
    const footer = page.locator('footer');
    await expect(footer).toBeVisible();
    await expect(footer).toContainText('v0.1.0');
  });

  test('engine status indicator is visible', async ({ page }) => {
    await page.goto('/');
    const toolbar = page.locator('[data-testid="toolbar"]');
    await expect(toolbar).toContainText('RUNNING');
  });
});
