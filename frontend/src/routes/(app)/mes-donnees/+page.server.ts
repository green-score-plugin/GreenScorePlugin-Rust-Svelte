import type { PageServerLoad } from './$types';
import { BACKEND_URL } from "$lib/config.ts";
import { ELECTRICITY_MAP_API_KEY } from '$env/static/private';

export const load: PageServerLoad = async ({ fetch }) => {
    try {
        const response = await fetch(`${BACKEND_URL}/mes-donnees`, {
            method: 'GET',
            headers: {'Content-Type': 'application/json'},
            credentials: 'include'
        });
        const result = await response.json();

        if (!result.success) {
            return {
                myAverageDailyCarbonFootprint: null,
                averageDailyCarbonFootprint: null,
                messageAverageFootprint: null
            };
        }

        return {
            myAverageDailyCarbonFootprint: result.my_average_daily_carbon_footprint,
            averageDailyCarbonFootprint: result.average_daily_carbon_footprint,
            messageAverageFootprint: result.message_average_footprint
        };

    } catch (error) {
        console.error('Erreur lors de la récupération des données :', error);
        return {
            myAverageDailyCarbonFootprint: null,
            averageDailyCarbonFootprint: null,
            messageAverageFootprint: null
        };
    }
};