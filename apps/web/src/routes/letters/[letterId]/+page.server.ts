import { getLetter } from '$lib/server/letter';

/** @type {import('./$types').PageLoad} */
export async function load({ request, params }) {
	const letterId = params.letterId;
	const letter = await getLetter({ letterId, request });
	return {
		letter
	};
}
