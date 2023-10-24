import type { User as BackendUser } from '$lib/server/user';

declare module '@auth/core/types' {
	interface User extends BackendUser {}
	interface Session extends DefaultSession {
		user?: User;
	}
}
