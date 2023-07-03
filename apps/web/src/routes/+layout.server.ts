import type { LayoutServerLoad } from './$types';

import type { Game } from 'types/game';

export const load: LayoutServerLoad = async (event) => {
	const session = await event.locals.getSession();

	const g: Game;

	return {
		session
	};
};
