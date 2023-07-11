import type { PageServerLoad } from './$types';
import { env } from '$env/dynamic/public';

export const load: PageServerLoad = async ({ params }) => {
	const { id } = params;

	try {
		const response = await fetch(`${env.PUBLIC_BGG_PROXY_URL}/item/${id}`);
		const body = await response.json();

		return {
			item: body
		};
	} catch (error) {
		console.error(error);
		throw new Error("Couldn't load item", { cause: error });
	}
};
