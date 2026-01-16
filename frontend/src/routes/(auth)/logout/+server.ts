import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from '../../../../.svelte-kit/types/src/routes';
import { BACKEND_URL } from '$lib/config.ts';
import { invalidateCache } from '$lib/server/session.ts';

export const GET: RequestHandler = async ({ url, cookies, fetch }) => {
    const sessionCookie = cookies.get('greenscoreweb_sessions');

    // ... code ...
    if (sessionCookie) {
        invalidateCache(sessionCookie);
        try {
            await fetch(`${BACKEND_URL}/logout`, {
                method: 'POST',
                headers: { cookie: `greenscoreweb_sessions=${sessionCookie}` }
            });
        } catch (e) {

        }

        cookies.delete('greenscoreweb_sessions', { path: '/' });
    }

    const accDeleted = url.searchParams.get('account_deleted');
    if (accDeleted) {
        throw redirect(303, '/login?account_deleted=true');
    }

    throw redirect(303, '/login');
};

