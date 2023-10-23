import { CreateLetterSchema, createLetter } from '$lib/server/letter';
import { flatten, safeParse } from 'valibot';
import type { Actions } from './$types';
import { fail, redirect } from '@sveltejs/kit';

export const actions = {
	default: async ({ request }) => {
		const data = await request.formData();
		const message = data.get('message');
		const subject = data.get('subject');

		const letterParse = safeParse(CreateLetterSchema, {
			message,
			subject
		});

		if (letterParse.success) {
			const letter = await createLetter({ letter: letterParse.output, request });
			if (!letter) {
				return fail(400, {
					subject,
					message,
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
			reason: flatten(letterParse.issues).nested
		});
	}
} satisfies Actions;
