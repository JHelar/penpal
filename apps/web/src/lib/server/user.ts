import { VITE_SERVER_API_URL } from '$env/static/private';
import { boolean, email, nullable, object, parse, string, uuid, type Output } from 'valibot';
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

type UpdateUserArgs = {
	username: string;
	display_name: string;
	profile_image: string;
	request: Request;
};
export async function updateUser({ request, ...args }: UpdateUserArgs) {
	const updateUserRes = await fetch(`${VITE_SERVER_API_URL}/me`, {
		method: 'POST',
		body: JSON.stringify({
			args
		}),
		headers: {
			cookie: request.headers.get('cookie') ?? '',
			'Content-Type': 'application/json'
		}
	});

	return updateUserRes.ok;
}
