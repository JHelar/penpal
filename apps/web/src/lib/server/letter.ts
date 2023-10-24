import { VITE_SERVER_API_URL } from '$env/static/private';
import {
	array,
	coerce,
	date,
	maxLength,
	minLength,
	object,
	parse,
	pick,
	string,
	uuid,
	type Output,
	flatten,
	ValiError
} from 'valibot';
import { fetch } from 'undici';

const DateSchema = coerce(date(), (input) => new Date(input as string));

export const LetterSchema = object({
	id: string([uuid()]),
	message: string([minLength(250, 'Message is to long.')]),
	subject: string([minLength(20, 'Subject is to short.'), maxLength(80, 'Subject is to long.')]),
	to_user_id: string([uuid()]),
	by_user_id: string([uuid()]),
	created_at: DateSchema,
	updated_at: DateSchema
});
export type Letter = Output<typeof LetterSchema>

export const CreateLetterSchema = pick(LetterSchema, ['message', 'subject', 'to_user_id']);

export const LettersSchema = array(LetterSchema);

export async function getLetters(request: Request) {
	const lettersResponse = await fetch(`${VITE_SERVER_API_URL}/me/letter`, {
		headers: request.headers
	});
	if (!lettersResponse.ok) {
		return [];
	}

	try {
		const letters = parse(LettersSchema, await lettersResponse.json());
		return letters;
	} catch (error) {
		console.error(error);
		return [];
	}
}

type GetLetterArgs = {
	letterId: string;
	request: Request;
};
export async function getLetter({ letterId, request }: GetLetterArgs) {
	const letterResponse = await fetch(`${VITE_SERVER_API_URL}/me/letter/${letterId}`, {
		headers: {
			cookie: request.headers.get('cookie') ?? '',
			'content-type': 'application/json'
		}
	});

	if (!letterResponse.ok) {
		console.error(letterResponse.statusText);
		return null;
	}

	try {
		const letter = parse(LetterSchema, await letterResponse.json());
		return letter;
	} catch (error) {
		console.error(flatten(error as ValiError));
		return null;
	}
}

type CreateLetterArgs = {
	letter: Output<typeof CreateLetterSchema>;
	request: Request;
};
export async function createLetter({ letter, request }: CreateLetterArgs) {
	const letterResponse = await fetch(`${VITE_SERVER_API_URL}/me/letter`, {
		headers: {
			cookie: request.headers.get('cookie') ?? '',
			'content-type': 'application/json'
		},
		method: 'post',
		body: JSON.stringify(letter)
	});

	if (!letterResponse.ok) {
		console.log(letterResponse.statusText);
		return null;
	}

	try {
		const letter = parse(LetterSchema, await letterResponse.json());
		return letter;
	} catch (error) {
		console.error(error);
		return null;
	}
}
