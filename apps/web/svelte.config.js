import adapter from '@sveltejs/adapter-vercel';
import { vitePreprocess } from '@sveltejs/kit/vite';
/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	vitePlugin: {
		inspector: true
	},
	kit: {
		adapter: adapter(),
		version: {
			name: process?.env?.VERCEL_GIT_COMMIT_SHA ?? Date.now().toString(),
			pollInterval: 60000 // 1 min
		}
	}
};
export default config;
