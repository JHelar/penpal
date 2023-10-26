import { VITE_SERVER_API_URL } from '$env/static/private';
import { boolean, email, nullable, object, parse, string, uuid, type Output, url } from 'valibot';
import { DateSchema } from './letter';

const UserSchema = object({
	email: string([email()]),
	id: string([uuid()]),
	username: nullable(string()),
	display_name: nullable(string()),
	profile_image: nullable(string()),
	is_initialized: boolean(),
	created_at: DateSchema,
	updated_at: DateSchema
});
export type User = Output<typeof UserSchema>;

export async function getOrCreateUser(email: string) {
	const signInRes = await fetch(`${VITE_SERVER_API_URL}/signIn`, {
		method: 'POST',
		body: JSON.stringify({
			email
		}),
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!signInRes.ok) {
		return null;
	}

	return parse(UserSchema, await signInRes.json());
}

export const UpdateUserSchema = object({
	username: string(),
	display_name: string(),
	profile_image: string([url()])
});

type UpdateUserArgs = {
	payload: Output<typeof UpdateUserSchema>;
	request: Request;
};
export async function updateUser({ request, payload }: UpdateUserArgs) {
	const updateUserRes = await fetch(`${VITE_SERVER_API_URL}/me`, {
		method: 'POST',
		body: JSON.stringify(payload),
		headers: {
			cookie: request.headers.get('cookie') ?? '',
			'content-type': 'application/json'
		}
	});

	if (!updateUserRes.ok) {
		console.error(updateUserRes.statusText, updateUserRes.status);
	}

	return updateUserRes.ok;
}

const RecipientSchema = object({
	id: string([uuid()]),
	display_name: string(),
	profile_image: string()
});

export type Recipient = Output<typeof RecipientSchema>;

type GetRandomRecipientArgs = {
	request: Request;
};

export async function getRandomRecipient({ request }: GetRandomRecipientArgs) {
	const result = await fetch(`${VITE_SERVER_API_URL}/me/random_recipient`, {
		method: 'get',
		headers: {
			cookie: request.headers.get('cookie') ?? '',
			'content-type': 'application/json'
		}
	});

	if (!result.ok) {
		console.log(result.statusText);
		return null;
	}

	return parse(RecipientSchema, await result.json());
}
