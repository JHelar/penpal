import { CreateLetterSchema, createLetter } from '$lib/server/letter';
import { flatten, safeParse, string, uuid } from 'valibot';
import type { Actions } from './$types';
import { fail, redirect } from '@sveltejs/kit';
import { getRandomRecipient, getRecipient, type Recipient } from '$lib/server/user';

export const actions = {
	default: async ({ request }) => {
		const formData = Object.fromEntries((await request.formData()).entries());
		const letterParse = safeParse(CreateLetterSchema, formData);

		if (letterParse.success) {
			const letter = await createLetter({ letter: letterParse.output, request });
			if (!letter) {
				return fail(400, {
					formData,
					reason: {
						general: 'Ooops, something went wrong!'
					}
				});
			}
			throw redirect(302, `/letters/${letter.id}`);
		}

		return fail(400, {
			formData,
			reason: flatten(letterParse.issues).nested
		});
	}
} satisfies Actions;

/** @type {import('./$types').PageLoad} */
export async function load({ url, request }) {
	/** @todo validate user id with server */
	const toUserId = safeParse(string([uuid()]), url.searchParams.get('toUserId'));

	if (toUserId.success) {
		const recipient = await getRecipient({ request, userId: toUserId.output });
		if (recipient) {
			return {
				recipient
			};
		}
	}

	const recipient = await getRandomRecipient({ request });
	if (!recipient) {
		throw new Error('No recipient found...');
	}

	return {
		recipient
	};
}
