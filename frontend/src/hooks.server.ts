import { redirect } from '@sveltejs/kit';
import type { Handle } from '@sveltejs/kit';
import { getAccount } from '$lib/server/session';

export const handle: Handle = async ({ event, resolve }) => {
    const session = event.cookies.get('greenscoreweb_sessions');

    if (session) {
        const account = await getAccount(session);
        if (account) {
            event.locals.user = account;
        }
    }

    // Routes publiques
    const publicRoutes = ['/login', '/inscription', '/inscription-organisation', '/cgu', '/confidentialite', '/', '/plugin'];
    let isPublicRoute = publicRoutes.some(route =>
        event.url.pathname === route || event.url.pathname.startsWith(route + '/')
    );

    if (event.url.pathname === '/derniere-page-consultee' && event.url.search.length > 1) {
        isPublicRoute = true;
    }

    if (!isPublicRoute && !session) {
        throw redirect(303, '/login');
    }

    if (event.url.pathname === '/login' && session) {
        throw redirect(303, '/');
    }

    return resolve(event);
};
