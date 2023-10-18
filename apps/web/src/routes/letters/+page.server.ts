import { VITE_SERVER_API_URL } from '$env/static/private';
import { array, coerce, date, object, parse, string, uuid } from 'valibot';

const DateSchema = coerce(date(), (input) => new Date(input as string));

const LetterSchema = object({
	id: string([uuid()]),
	message: string(),
	to_user_id: string([uuid()]),
	by_user_id: string([uuid()]),
	created_at: DateSchema,
	updated_at: DateSchema
});

const LettersSchema = array(LetterSchema);

/** @type {import('./$types').PageLoad} */
export async function load({ fetch }) {
	const lettersResponse = await fetch(`${VITE_SERVER_API_URL}/me/letter`);
	if (!lettersResponse.ok) {
		return {
			letters: []
		};
	}

	const letters = parse(LettersSchema, await lettersResponse.json());
	return {
		letters
	};
}
