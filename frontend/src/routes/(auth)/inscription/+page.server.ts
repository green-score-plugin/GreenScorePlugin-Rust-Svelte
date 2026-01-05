import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { BACKEND_URL } from '$lib/config';

export const actions = {
    default: async ({ request, fetch, cookies }) => {
        const data = await request.formData();
        const firstname = data.get('firstname');
        const lastname = data.get('lastname');
        const email = data.get('email');
        const password = data.get('password');

        if(!firstname || !lastname || !email || !password) {
            return fail(400, { message: "Tous les champs sont requis" })
        }

        try{
            const response = await fetch(`${BACKEND_URL}/inscription`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                credentials: 'include',
                body: JSON.stringify({ email, password, firstname, lastname })
            });

            const result = await response.json();

            if(result.success) {
                const setCookieHeader = response.headers.get('set-cookie');
                if (setCookieHeader) {
                    const cookieMatch = setCookieHeader.match(/greenscoreweb_sessions=([^;]+)/);
                    if (cookieMatch) {
                        const sessionValue = cookieMatch[1];
                        cookies.set('greenscoreweb_sessions', sessionValue, {
                            path: '/',
                            httpOnly: true,
                            sameSite: 'lax',
                            maxAge: 60 * 60 // 1 heure
                        });
                    }
                }
                redirect(303,'/');
            }

            return fail(400, { message: result.message || 'Erreur de connexion' });
        }catch (error) {
            if (error && typeof error === 'object' && ('status' in error || 'location' in error)) {
                throw error;
            }
            return fail(500, { message: 'Erreur serveur' });
        }
    }
} satisfies Actions;
