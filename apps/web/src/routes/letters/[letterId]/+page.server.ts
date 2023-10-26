import { getLetter } from '$lib/server/letter';
import { redirect } from '@sveltejs/kit';
import { getRecipient } from '../../../lib/server/user.js';

/** @type {import('./$types').PageLoad} */
export async function load({ request, params, locals }) {
	const letterId = params.letterId;
	const letter = await getLetter({ letterId, request });

	if (!letter) {
		console.error('Get letter, no letter with id', letterId);
		throw redirect(307, '/letters');
	}

	const session = await locals.getSession();

	const recipientUserId =
		letter.to_user_id === session?.user?.id ? letter.by_user_id : letter.to_user_id;
	const recipient = await getRecipient({ request, userId: recipientUserId });

	if (!recipient) {
		console.error('Get letter, no recipient found for id', recipientUserId);
		throw redirect(307, '/letters');
	}

	return {
		letter,
		recipient
	};
}
