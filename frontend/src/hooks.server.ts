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
    const publicRoutes = ['/login', '/inscription', '/inscription-organisation', '/cgu', '/confidentialite', '/', 'derniere-page-consultee', '/plugin'];
    const isPublicRoute = publicRoutes.some(route =>
        event.url.pathname === route || event.url.pathname.startsWith(route + '/')
    );

    if (!isPublicRoute && !session) {
        throw redirect(303, '/login');
    }

    if (event.url.pathname === '/login' && session) {
        throw redirect(303, '/');
    }

    return resolve(event);
};
