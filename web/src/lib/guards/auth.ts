
export async function authGuard({ page }: LoadInput): Promise<LoadOutput> {
	const loggedIn = auth.loggedIn;

	if (loggedIn && page.path === '/login') {
		return { status: 302, redirect: '/' };
	} else if (loggedIn || page.path === '/login') {
		return {};
	} else {
		return { status: 302, redirect: '/login' }
	}
}