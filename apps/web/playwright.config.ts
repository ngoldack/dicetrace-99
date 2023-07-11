import { type PlaywrightTestConfig } from '@playwright/test';
import dotenv from 'dotenv';

if (!process.env.CI) {
	dotenv.config({ path: `.env.development.local` });
	dotenv.config({ path: `.env` });
}

const config: PlaywrightTestConfig = {
	webServer: {
		reuseExistingServer: true,
		command: 'pnpm run build && pnpm run preview',
		port: 4173
	},
	workers: process.env.CI ? 1 : '50%',
	testDir: 'tests',
	testMatch: /(.+\.)?(test|spec)\.[jt]s/,

	projects: [
		{
			name: 'Chromium',

			use: {
				browserName: 'chromium',
				connectOptions: {
					wsEndpoint: `wss://chrome.browserless.io?token=${process.env.BROWSERLESS_TOKEN}`
				}
			}
		}
	]
};

export default config;
