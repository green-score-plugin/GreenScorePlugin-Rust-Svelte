import type { PageServerLoad } from './$types';
import { BACKEND_URL } from "$lib/config.ts";

export const load: PageServerLoad = async ({ fetch }) => {
    try {
        const response = await fetch(`${BACKEND_URL}/mon-organisation`, {
            method: 'GET',
            headers: {'Content-Type': 'application/json'},
            credentials: 'include'
        });
        const result = await response.json();

        if (!result.success) {
            return {
                organisationData: null,
                adviceUser: '',
                adviceDev: '',
                letterGreenScore: '',
                envNomination: '',
                equivalents: []
            };
        }

    } catch (error) {
        console.error('Erreur lors de la récupération des données de l\'organisation :', error);
        return {
            organisationData: null,
            adviceUser: '',
            adviceDev: '',
            letterGreenScore: '',
            envNomination: '',
            equivalents: []
        };
    }
}