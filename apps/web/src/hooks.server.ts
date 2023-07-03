import * as Sentry from '@sentry/sveltekit';
import { SvelteKitAuth } from '@auth/sveltekit';
import Auth0 from '@auth/core/providers/auth0';

import { redirect, type Handle } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import type { Provider } from '@auth/core/providers';
import { sequence } from '@sveltejs/kit/hooks';

Sentry.init({
    dsn: "https://098e1022501148f4a3e5466b7b8ea4f6@o4505465647267840.ingest.sentry.io/4505465648578560",
    tracesSampleRate: 1
})

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

export const handle: Handle = sequence(Sentry.sentryHandle(), sequence(
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
));
export const handleError = Sentry.handleErrorWithSentry();