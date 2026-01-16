import type { PageServerLoad } from './$types';
import { BACKEND_URL } from "$lib/config.ts";

function formatMonthlyData(data: Array<{ label: string; value: number }>) {
    const monthNames = ['Jan', 'Fév', 'Mar', 'Avr', 'Mai', 'Juin', 'Juil', 'Aoû', 'Sep', 'Oct', 'Nov', 'Déc'];
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
                equivalents: [],
                dailyConsumption: [],
                weeklyConsumption: [],
                monthlyConsumption: formatMonthlyData([]),
                topPoulltingSites: [],
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
            equivalents: result.equivalents || [],
            dailyConsumption: result.daily_consumption || [],
            weeklyConsumption: result.weekly_consumption || [],
            monthlyConsumption: formatMonthlyData(result.monthly_consumption) || formatMonthlyData([]),
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
            monthlyConsumption: formatMonthlyData([]),
            topPoulltingSites: [],
        };
    }
}