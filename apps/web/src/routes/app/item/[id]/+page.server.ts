import type { PageServerLoad } from './$types';
import { env } from '$env/dynamic/public';

export const load: PageServerLoad = async ({ params }) => {
	const { id } = params;

	try {
		const response = await fetch(`${env.PUBLIC_BGG_PROXY_URL}/api/v1/item?ids=${id}`);
		const body = await response.json();

		if (!body) throw new Error("Couldn't load item");
		if ((body as Array<unknown>).length !== 1) throw new Error("Couldn't load item");

		return {
			item: body[0]
		};
	} catch (error) {
		console.error(error);
		throw new Error("Couldn't load item", { cause: error });
	}
};
