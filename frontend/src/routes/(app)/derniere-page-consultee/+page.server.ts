import type { PageServerLoad } from './$types';
import { BACKEND_URL } from "$lib/config.ts";

export const load: PageServerLoad = async ({ fetch }) => {
    let adviceDev: string = '';
    try {
        const response = await fetch(`${BACKEND_URL}/advice?is_dev=true`, {
            method: 'GET',
            headers: { 'Content-Type': 'application/json' },
            credentials: 'include'
        });
        const result = await response.json();
        if (result.success) {
            adviceDev = result.advice;
        }
    } catch (error) {
        console.error('Erreur lors de la récupération du conseil pour les développeurs :', error);
    }

    let adviceUser: string = '';
    try {
        const response = await fetch(`${BACKEND_URL}/advice?is_dev=false`, {
            method: 'GET',
            headers: { 'Content-Type': 'application/json' },
            credentials: 'include'
        });
        const result = await response.json();
        if (result.success) {
            adviceUser = result.advice;
        }
    } catch (error) {
        console.error('Erreur lors de la récupération du conseil pour les utilisateurs :', error);
    }


    return {
        adviceDev,
        adviceUser
    }
}