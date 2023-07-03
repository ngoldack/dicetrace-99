import { SvelteKitAuth } from '@auth/sveltekit';
import Auth0 from '@auth/core/providers/auth0';

import { redirect, type Handle } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import type { Provider } from '@auth/core/providers';
import { sequence } from '@sveltejs/kit/hooks';

const authorization: Handle = async ({ event, resolve }) => {
	// Protect any routes under /authenticated
	if (event.url.pathname.startsWith('/app')) {
		const session = await event.locals.getSession();
		if (!session) {
			throw redirect(303, '/auth/signin');
		}
	}

	// If the request is still here, just proceed as normally
	return resolve(event);
};

export const handle: Handle = sequence(
	SvelteKitAuth({
		trustHost: true,
		providers: [
			Auth0({
				issuer: env.AUTH0_ISSUER,
				clientId: env.AUTH0_CLIENT_ID,
				clientSecret: env.AUTH0_CLIENT_SECRET
			}) as Provider
		],
		callbacks: {}
	}),
	authorization
);
