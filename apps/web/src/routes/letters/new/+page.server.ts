import { CreateLetterSchema, createLetter } from '$lib/server/letter';
import { flatten, safeParse, string, uuid } from 'valibot';
import type { Actions } from './$types';
import { fail, redirect } from '@sveltejs/kit';
import type { SelectOptionType } from 'flowbite-svelte';

export const actions = {
	default: async ({ request }) => {
		const data = await request.formData();
		const message = data.get('message');
		const subject = data.get('subject');
		const toUserId = data.get('to_user_id');

		const letterParse = safeParse(CreateLetterSchema, {
			message,
			subject,
			to_user_id: toUserId
		});

		if (letterParse.success) {
			const letter = await createLetter({ letter: letterParse.output, request });
			if (!letter) {
				return fail(400, {
					subject,
					message,
					toUserId,
					reason: {
						general: 'Ooops, something went wrong!'
					}
				});
			}
			throw redirect(302, `/letters/${letter.id}`);
		}

		return fail(400, {
			subject,
			message,
			toUserId,
			reason: flatten(letterParse.issues).nested
		});
	}
} satisfies Actions;

/** @type {import('./$types').PageLoad} */
export async function load({ url }) {
	/** @todo validate user id with server */
	const toUserId = safeParse(string([uuid()]), url.searchParams.get('toUserId'))
	const recipients: SelectOptionType<string>[] = []

	if(toUserId.success) {
		recipients.push({
			name: toUserId.output,
			value: toUserId.output
		})

		return {
			toUserId: toUserId.output,
			recipients
		}
	}

	return {
		toUserId: undefined,
		recipients
	}
}