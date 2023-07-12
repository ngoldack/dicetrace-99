import type { PageServerLoad } from './$types';
import { env } from '$env/dynamic/public';

export const load: PageServerLoad = async ({ parent, url }) => {
	const { userdata } = await parent();

	const username = url.searchParams.get('username') ?? userdata.bgg_username;

	try {
		const response = await fetch(
			`${env.PUBLIC_BGG_PROXY_URL}/api/v1/collection?username=${username}`
		);
		const body = await response.json();
		return {
			username: username,
			collection: body
		};
	} catch (error) {
		throw new Error("Couldn't load collection", { cause: error });
	}
};
