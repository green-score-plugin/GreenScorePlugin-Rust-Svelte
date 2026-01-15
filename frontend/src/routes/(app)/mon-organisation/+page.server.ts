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

        console.log(result);

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
        
        return {
            organisationData: {
                name: result.mo_infos?.name || '',
                averageDailyCarbonFootprint: result.mo_infos?.average_daily_carbon_footprint || 0,
                equivalent: result.mo_infos?.equivalent || '',
                members: result.mo_infos?.members || 0,
                totalCarbonFootprint: result.mo_infos?.total_consumption || 0,
            },
            adviceUser: result.advices?.[0] || '',
            adviceDev: result.advices?.[1] || '',
            letterGreenScore: result.letter || 'A',
            envNomination: result.anv_nomination || 'Gardien des Écosystèmes',
            equivalents: result.equivalents || []
        };

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