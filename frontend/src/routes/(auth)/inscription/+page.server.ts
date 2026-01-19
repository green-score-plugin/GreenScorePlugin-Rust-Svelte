import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { BACKEND_URL } from '$lib/config';
import { setSessionCookie } from '$lib/server/session';

export const actions = {
    default: async ({ request, fetch, cookies }) => {
        const data = await request.formData();
        const firstname = data.get('firstname');
        const lastname = data.get('lastname');
        const email = data.get('email');
        const password = data.get('password');
        const confirmPassword = data.get('confirmPassword');
        const agreeTerms = data.get('agreeTerms');

        if(!firstname || !lastname || !email || !password) {
            return fail(400, { message: "Tous les champs sont requis" })
        }

        if(password !== confirmPassword) {
            return fail(400, { message: "Les mots de passe ne correspondent pas" })
        }

        if(agreeTerms !== 'on') {
            return fail(400, { message: "Vous devez accepter les conditions générales d'utilisation" })
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
                setSessionCookie(cookies, response);
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
