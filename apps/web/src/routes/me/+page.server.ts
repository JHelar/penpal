import type { Actions } from './$types.js';
import { flatten, safeParse } from 'valibot';
import { UpdateUserSchema, updateUser } from '$lib/server/user.js';
import { fail } from '@sveltejs/kit';

export const actions = {
	async default({ request }) {
		const data = await request.formData();
		const formData = {
			username: data.get('username'),
			display_name: data.get('display_name'),
			profile_image: data.get('profile_image')
		};

		const parseResult = safeParse(UpdateUserSchema, formData);

		if (parseResult.success) {
			const success = await updateUser({ request, payload: parseResult.output });
			if (success) {
				return {
					...formData,
					success: true,
					reason: {}
				};
			}
			return fail(400, {
				...formData,
				success: false,
				reason: {
					general: 'Ooops something went wrong!'
				}
			});
		}

		return fail(400, {
			...formData,
			success: false,
			reason: flatten(parseResult.issues).nested
		});
	}
} satisfies Actions;
