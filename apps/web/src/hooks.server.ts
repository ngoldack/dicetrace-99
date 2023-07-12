import { SvelteKitAuth } from '@auth/sveltekit';
import Auth0 from '@auth/core/providers/auth0';

import { redirect, type Handle } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import type { Provider } from '@auth/core/providers';
import { sequence } from '@sveltejs/kit/hooks';

const authorization: Handle = async ({ event, resolve }) => {
	// If the path is not /app or /setup, we don't need to check for authorization
	if (!event.url.pathname.startsWith('/app') && event.url.pathname !== '/setup') {
		return resolve(event);
	}

	// Otherwise, we need to check if the user is logged in
	const session = await event.locals.getSession();
	// If the user is not logged in, redirect them to the login page
	if (!session) {
		throw redirect(303, '/auth/signin');
	}

	// User is already logged in, granting access to setup page
	if (session && event.url.pathname === '/setup') {
		return resolve(event);
	}

	// Check if the user has userdata
	try {
		const resp = await fetch(`${env.USERDATA_URL}?email=${session?.user?.email}`);
		const body = await resp.json();

		// If the user is logged in but doesn't have userdata, redirect them to the setup page
		if (body.size === 0) {
			throw redirect(303, '/setup');
		}
	} catch (err) {
		console.log(err);
		throw redirect(303, '/setup');
	}

	return resolve(event);
};

export const handle: Handle = sequence(
	sequence(
		SvelteKitAuth({
			trustHost: true,
			secret: env.AUTH_SECRET ?? '82ac8d6c8694a5232abe4af68e121400',
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
	)
);

export const handleError = (error: Error) => {
	return {
		code: 500,
		message: error.message ?? 'Unknown error',
		id: crypto.randomUUID()
	} satisfies App.Error;
};
