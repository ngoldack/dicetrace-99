import { env } from '$env/dynamic/private';
import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ parent }) => {
	const { session } = await parent();
	let body;

	// Check if the user has userdata
	try {
		const resp = await fetch(`${env.USERDATA_URL}?email=${session?.user?.email}`);
		body = await resp.json();
	} catch (err) {
		throw redirect(303, '/setup');
	}

	return {
		session,
		userdata: body.data[0]
	};
};
