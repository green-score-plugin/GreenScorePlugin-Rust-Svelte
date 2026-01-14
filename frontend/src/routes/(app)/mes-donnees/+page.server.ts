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
                messageAverageFootprint: null,
                totalConsumption: null,
                letterGreenScore: null,
                envNomination: null,
                equivalents: [],
                usersIdsCharts: []
            };
        }

        return {
            myAverageDailyCarbonFootprint: result.my_average_daily_carbon_footprint,
            averageDailyCarbonFootprint: result.average_daily_carbon_footprint,
            messageAverageFootprint: result.message_average_footprint,
            totalConsumption: result.total_consumption,
            letterGreenScore: result.letter_green_score || 'A',
            envNomination: result.env_nomination || 'Maître des Forêts',
            equivalents: result.equivalents || [],
            usersIdsCharts: result.users_ids_charts || []
        };

    } catch (error) {
        console.error('Erreur lors de la récupération des données :', error);
        return {
            myAverageDailyCarbonFootprint: null,
            averageDailyCarbonFootprint: null,
            messageAverageFootprint: null,
            totalConsumption: null,
            letterGreenScore: null,
            envNomination: null,
            equivalents: [],
            usersIdsCharts: []
        };
    }
};