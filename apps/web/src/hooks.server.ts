import { fetch } from 'undici';
import { SvelteKitAuth } from '@auth/sveltekit';
import GoogleProvider from '@auth/core/providers/google';
import {
	VITE_AUTH_SECRET,
	VITE_GOOGLE_CLIENT_ID,
	VITE_GOOGLE_CLIENT_SECRET,
	VITE_SERVER_API_URL
} from '$env/static/private';
import { sequence } from '@sveltejs/kit/hooks';
import { email, object, parse, string, uuid } from 'valibot';

const UserSchema = object({
	email: string([email()]),
	id: string([uuid()])
});

async function getOrCreateUser(email: string) {
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

const svelteAuthHandle = SvelteKitAuth({
	providers: [
		GoogleProvider({
			clientId: VITE_GOOGLE_CLIENT_ID,
			clientSecret: VITE_GOOGLE_CLIENT_SECRET,
			authorization: {
				params: {
					prompt: 'consent',
					access_type: 'offline',
					response_type: 'code'
				}
			}
		})
	],
	session: {
		strategy: 'jwt'
	},
	callbacks: {
		async signIn({ profile }) {
			const email = profile?.email;
			const verified = profile?.email_verified;

			if (verified && email) {
				const user = await getOrCreateUser(email);
				return Boolean(user);
			}
			return false;
		},
		async jwt({ profile, token, trigger }) {
			const email = profile?.email;
			const verified = profile?.email_verified;
			if (trigger === 'signIn' && email && verified) {
				const user = await getOrCreateUser(email);
				console.log({
					profile,
					token,
					trigger
				});
				if (user) {
					return {
						...token,
						ppid: user.id
					};
				}
			}
			return token;
		}
	},
	secret: VITE_AUTH_SECRET
});

export const handle = sequence(svelteAuthHandle);
