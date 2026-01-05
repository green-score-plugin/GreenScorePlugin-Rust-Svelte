import { redirect } from '@sveltejs/kit';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
    const session = event.cookies.get('greenscoreweb_sessions');

    // Routes publiques
    const publicRoutes = ['/login', '/inscription', '/inscription-organisation', '/cgu', '/confidentialite'];
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
