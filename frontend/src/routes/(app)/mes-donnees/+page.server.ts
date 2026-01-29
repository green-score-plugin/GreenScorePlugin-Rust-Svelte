import type { PageServerLoad } from './$types';
import { BACKEND_URL } from "$lib/config.ts";

function formatMonthlyData(data: Array<{ label: string; value: number }>) {
    const now = new Date();
    const result: Array<{ label: string; value: number }> = [];
    for (let i = 11; i >= 0; i--) {
        const date = new Date(now.getFullYear(), now.getMonth() - i, 1);
        const monthIndex = date.getMonth();
        const year = date.getFullYear();
        // Standard format MM/YYYY
        const monthKey = `${String(monthIndex + 1).padStart(2, '0')}/${year}`;

        const existingData = data.find(d => d.label === monthKey);

        result.push({
            label: monthKey,
            value: existingData?.value || 0
        });
    }
    return result;
}

export const load: PageServerLoad = async ({ fetch, request }) => {
    try {
        const response = await fetch(`${BACKEND_URL}/mes-donnees`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Cookie': request.headers.get('cookie') || ''
            },
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
                dailyConsumption: [],
                weeklyConsumption: [],
                monthlyConsumption: formatMonthlyData([]),
                topPollutingSites: [],
                adviceUser: '',
                adviceDev: '',
            };
        }

        return {
            myAverageDailyCarbonFootprint: result.my_average_daily_carbon_footprint,
            averageDailyCarbonFootprint: result.average_daily_carbon_footprint,
            messageAverageFootprint: result.message_average_footprint,
            totalConsumption: result.total_consumption,
            letterGreenScore: result.letter_green_score || 'A',
            envNomination: result.env_nomination || 'nominations.profile.A',
            equivalents: result.equivalents || [],
            dailyConsumption: result.daily_consumption || [],
            weeklyConsumption: result.weekly_consumption || [],
            monthlyConsumption: formatMonthlyData(result.monthly_consumption || []),
            topPollutingSites: result.top_polluting_sites || [],
            adviceUser: result.advices?.[1] || '',
            adviceDev: result.advices?.[0] || '',
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
            dailyConsumption: [],
            weeklyConsumption: [],
            monthlyConsumption: formatMonthlyData([]),
            topPollutingSites: [],
            adviceUser: '',
            adviceDev: '',
        };
    }
};
