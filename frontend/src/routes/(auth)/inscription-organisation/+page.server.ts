import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { BACKEND_URL } from '$lib/config';

export const actions = {
    default: async ({ request, fetch }) => {
        const data = await request.formData();
        const organisationName = data.get('organisationName');
        const siret = data.get('siret');
        const email = data.get('email');
        const password = data.get('password');

        console.log('organisationName', organisationName);
        console.log('siret', siret);
        console.log('email', email);
        console.log('password', password);

        if(!organisationName || !email || !password) {
            return fail(400, { message: "Tous les champs sont requis" })
        }

        try{
            const response = await fetch(`${BACKEND_URL}/inscription-organisation`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                credentials: 'include',
                body: JSON.stringify({
                    orga_name: organisationName,
                    siret : siret || null,
                    email,
                    password })
            });

            const result = await response.json();

            if(result.success) {
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
