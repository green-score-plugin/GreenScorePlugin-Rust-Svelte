import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { BACKEND_URL } from '$lib/config';
import { setSessionCookie } from '$lib/server/session';

export const actions = {
    default: async ({ request, fetch, cookies }) => {
        const data = await request.formData();
        const organisationName = data.get('organisationName');
        const siret = data.get('siret');

        if(!organisationName) {
            return fail(400, { message: "errors.champs" })
        }


        try{
            const response = await fetch(`${BACKEND_URL}/auth/inscription-organisation`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                credentials: 'include',
                body: JSON.stringify({
                    orga_name: organisationName,
                    siret : siret || null,
                })
            });

            const result = await response.json();

            if(result.success) {
                await setSessionCookie(cookies, response);
                const code = result.account?.code || result.user_full?.organisation?.[0]?.code;
                redirect(303,`/inscription-organisation/${code}`);
            }

            return fail(400, { message: result.message || 'Erreur de connexion' });
        }catch (error) {
            if (error && typeof error === 'object' && ('status' in error || 'location' in error)) {
                throw error;
            }
            return fail(500, { message: 'errors.server_error' });
        }
    }
} satisfies Actions;
