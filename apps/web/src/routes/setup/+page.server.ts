import type { PageServerLoad } from './$types';
import { env } from '$env/dynamic/private';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ parent }) => {
	const { session } = await parent();
	// Check if the user has userdata
	try {
		const resp = await fetch(`${env.USERDATA_URL}?email=${session?.user?.email}`);
		const body = await resp.json();

		// If the user is logged in but doesn't have userdata, redirect them to the setup page
		if (body.size === 1) {
			throw redirect(303, '/app');
		}
	} catch (err) {
		console.log(err);
		throw redirect(303, '/setup');
	}

	return {
		session
	};
};
