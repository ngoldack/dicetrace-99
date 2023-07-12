import type { PageServerLoad } from './$types';
import { SOCIAL_URL, USERDATA_URL } from '$env/static/private';

export const load: PageServerLoad = async ({ params }) => {
	const { id } = params;

	let response = await fetch(`${SOCIAL_URL}/group/${id}`);
	let body = await response.json();

	const group = body.data;

	const members = [];
	for (const member of group.users) {
		response = await fetch(`${USERDATA_URL}/${member}`);
		body = await response.json();
		members.push(body.data);
	}

	return {
		group,
		members
	};
};
