import { test, expect } from '@playwright/test';

test('test page title', async ({ page }) => {
	await page.goto('/');
	const pageTitle = await page.title();
	expect(pageTitle).toBe('dicetrace');
});
