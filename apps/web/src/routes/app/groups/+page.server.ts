import type { PageServerLoad } from './$types';
import { SOCIAL_URL } from '$env/static/private';

export const load: PageServerLoad = async ({ parent }) => {
	const { userdata } = await parent();

	const response = await fetch(`${SOCIAL_URL}/group?user_id=${userdata.id}`);
	const body = await response.json();

	return {
		groups: body.data
	};
};
