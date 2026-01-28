import type { PageServerLoad } from './$types';
import { BACKEND_URL } from "$lib/config.ts";
import fr from '$lib/i18n/fr.json';
import en from '$lib/i18n/en.json';

function formatMonthlyData(data: Array<{ label: string; value: number }>, locale: string) {
    const monthNames = locale === 'en' ? en.widgets.common.period.months : fr.widgets.common.period.months;
    const now = new Date();
    const result: Array<{ label: string; value: number }> = [];
    for (let i = 11; i >= 0; i--) {
        const date = new Date(now.getFullYear(), now.getMonth() - i, 1);
        const monthIndex = date.getMonth();
        const year = date.getFullYear();
        const monthKey = `${String(monthIndex + 1).padStart(2, '0')}/${year}`;

        const formattedLabel = `${monthNames[monthIndex]} ${year}`;
        const existingData = data.find(d => d.label === monthKey);

        result.push({
            label: formattedLabel,
            value: existingData?.value || 0
        });
    }
    return result;
}
export const load: PageServerLoad = async ({ fetch, request }) => {
    try {
        const cookieHeader = request.headers.get('cookie') || '';
        let locale = 'fr';
        // Simple check for lang cookie or similar if used.
        // The user's i18n/index.ts uses localStorage which is not sent.
        // Assuming the app might set a cookie or we default to fr.
        // If we want to support 'en', we rely on client sending something or just default to 'fr'.
        // But if the user wants "anglais, fr", we can try to parse Accept-Language if no cookie.
        // For now, let's look for a "lang" cookie which is common, even if not explicitly set in index.ts shown.
        if (cookieHeader.includes('lang=en') || request.headers.get('accept-language')?.startsWith('en')) {
             locale = 'en';
        }

        const response = await fetch(`${BACKEND_URL}/mes-donnees`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Cookie': cookieHeader
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
                monthlyConsumption: formatMonthlyData([], locale),
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
            monthlyConsumption: formatMonthlyData(result.monthly_consumption || [], locale),
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
            monthlyConsumption: formatMonthlyData([], 'fr'),
            topPollutingSites: [],
            adviceUser: '',
            adviceDev: '',
        };
    }
};
