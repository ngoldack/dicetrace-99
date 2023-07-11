import type { PageServerLoad } from './$types';
import { env } from '$env/dynamic/public';

export const load: PageServerLoad = async ({ parent }) => {
	const { userdata } = await parent();

	try {
		const response = await fetch(
			`${env.PUBLIC_BGG_PROXY_URL}/api/v1/collection?username=${userdata.bgg_username}`
		);
		const body = await response.json();
		return {
			collection: body
		};
	} catch (error) {
		throw new Error("Couldn't load collection", { cause: error });
	}
};
