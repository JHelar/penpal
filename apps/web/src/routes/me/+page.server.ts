import { redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad} */
export async function load({ locals }) {
	const user = await locals.getSession();
	const email = user?.user?.email;
	if (!user || !email) {
		throw redirect(307, '/login');
	}

	// const backendUser = await getOrCreateUser(email)!;

	// return {
	// 	user: backendUser
	// };
}
