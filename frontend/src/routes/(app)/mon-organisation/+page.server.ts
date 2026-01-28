import type { PageServerLoad } from './$types';
import { BACKEND_URL } from "$lib/config.ts";
import fr from '$lib/i18n/fr.json';
import en from '$lib/i18n/en.json';

function formatMonthlyData(data: Array<{ label: string; value: number }>, locale: string) {
    const monthNames = locale === 'en' ? en.widgets.common.period.months : fr.widgets.common.period.months;
    const now = new Date();
    const result: Array<{ label: string; value: number }> = [];
    // Créer les 12 derniers mois
    for (let i = 11; i >= 0; i--) {
        const date = new Date(now.getFullYear(), now.getMonth() - i, 1);
        const monthIndex = date.getMonth();
        const year = date.getFullYear();
        const formattedLabel = `${monthNames[monthIndex]} ${year}`;

        // Chercher si des données existent pour ce mois
        const monthKey = `${String(monthIndex + 1).padStart(2, '0')}/${year}`;
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
        if (cookieHeader.includes('lang=en') || request.headers.get('accept-language')?.startsWith('en')) {
             locale = 'en';
        }

        const response = await fetch(`${BACKEND_URL}/mon-organisation`, {
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
                organisationData: null,
                adviceUser: '',
                adviceDev: '',
                letterGreenScore: '',
                envNomination: '',
                equivalents: [],
                dailyConsumption: [],
                weeklyConsumption: [],
                monthlyConsumption: formatMonthlyData([], locale),
                topPollutingSites: [],
            };
        }
        
        return {
            organisationData: {
                name: result.mo_infos?.name || '',
                averageDailyCarbonFootprint: result.mo_infos?.average_daily_carbon_footprint || 0,
                equivalent: result.mo_infos?.equivalent || '',
                totalCarbonFootprint: result.mo_infos?.total_consumption || 0,
            },
            adviceUser: result.advices?.[0] || '',
            adviceDev: result.advices?.[1] || '',
            letterGreenScore: result.letter || 'A',
            envNomination: result.env_nomination || 'nominations.profile.A',
            equivalents: result.equivalents || [],
            dailyConsumption: result.daily_consumption || [],
            weeklyConsumption: result.weekly_consumption || [],
            monthlyConsumption: formatMonthlyData(result.monthly_consumption || [], locale),
            topPollutingSites: result.top_polluting_sites || [],
        };

    } catch (error) {
        console.error('Erreur lors de la récupération des données de l\'organisation :', error);
        return {
            organisationData: null,
            adviceUser: '',
            adviceDev: '',
            letterGreenScore: '',
            envNomination: '',
            equivalents: [],
            dailyConsumption: [],
            weeklyConsumption: [],
            monthlyConsumption: formatMonthlyData([], 'fr'),
            topPollutingSites: [],
        };
    }
}