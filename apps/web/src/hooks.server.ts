import { SvelteKitAuth } from '@auth/sveltekit';
import GoogleProvider from '@auth/core/providers/google';
import {
	VITE_AUTH_SECRET,
	VITE_GOOGLE_CLIENT_ID,
	VITE_GOOGLE_CLIENT_SECRET
} from '$env/static/private';
import { sequence } from '@sveltejs/kit/hooks';
import { getOrCreateUser } from './lib/server/user';
import type { Handle } from '@sveltejs/kit';

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
	pages: {
		signIn: '/login'
	},
	session: {
		strategy: 'jwt'
	},
	callbacks: {
		async session({ session }) {
			const email = session.user?.email;
			if (!email) {
				return session;
			}
			const user = await getOrCreateUser(email);
			if (!user) {
				return session;
			}

			return {
				expires: session.expires,
				user: {
					...user,
					profile_image: user.profile_image ?? session.user?.image,
					display_name: user.display_name ?? session.user?.name
				}
			};
		},
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
				// @todo: Super insecure, use provider and providerID!
				const user = await getOrCreateUser(email);
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

const initializeUserRedirect: Handle = async ({ event, resolve }) => {
	const session = await event.locals.getSession();
	if (session) {
		const user = session.user;
		if (user) {
			if (!user.is_initialized && event.url.pathname !== '/me') {
				return new Response('Redirect', { status: 303, headers: { Location: '/me' } });
			}
		}
	}
	return resolve(event);
};

export const handle = sequence(svelteAuthHandle, initializeUserRedirect);
