import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { BACKEND_URL } from '$lib/config.ts';
import { invalidateCache } from '$lib/server/session.ts';

export const load: PageServerLoad = async ({ url, cookies, fetch }) => {
    const sessionCookie = cookies.get('greenscoreweb_sessions');

    if (sessionCookie) {
        invalidateCache(sessionCookie);
        try {
            await fetch(`${BACKEND_URL}/auth/logout`, {
                method: 'POST',
                headers: { cookie: `greenscoreweb_sessions=${sessionCookie}` }
            });
        } catch {
            console.error('Erreur lors de la déconnexion');
        }

        cookies.delete('greenscoreweb_sessions', { path: '/' });
    }

    const accDeleted = url.searchParams.get('account_deleted');
    if (accDeleted) {
        throw redirect(303, '/login?account_deleted=true');
    }

    throw redirect(303, '/login');
};

