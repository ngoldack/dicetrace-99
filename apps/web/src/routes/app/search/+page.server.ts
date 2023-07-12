import type { PageServerLoad } from './$types';

import { env } from '$env/dynamic/public';
import { type Actions, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ url }) => {
	const param = url.searchParams.get('q');
	const query = param && param !== '' ? param : undefined;

	if (!query) {
		return {
			query: undefined,
			results: []
		};
	}
	const response = await fetch(`${env.PUBLIC_BGG_PROXY_URL}/api/v1/search?q=${query}`);

	try {
		if (response.ok) {
			const body = await response.json();
			return {
				query: query,
				results: body
			};
		}
	} catch (err) {
		console.log(err);
		throw new Error("Couldn't load search results", { cause: err });
	}
};

export const actions: Actions = {
	search: async ({ request }) => {
		const data = await request.formData();

		const query = data.get('query')?.toString();

		throw redirect(303, `/app/search?q=${query}`);
	}
};
