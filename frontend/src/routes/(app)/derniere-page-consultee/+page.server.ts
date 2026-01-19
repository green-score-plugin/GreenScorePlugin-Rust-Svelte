import type { PageServerLoad } from './$types';
import { BACKEND_URL, ELECTRICITY_MAP_API_KEY } from "$lib/config.ts";

export const load: PageServerLoad = async ({ fetch, request, url }) => {
    try {
        let backendUrl = `${BACKEND_URL}/derniere-page-consultee`;

        if (url.searchParams.toString()) {
            backendUrl += `?${url.searchParams.toString()}`;
        }

        const response = await fetch(backendUrl, {
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
                pageData: null,
                adviceUser: '',
                adviceDev: '',
                letterGreenScore: '',
                envNomination: '',
                equivalents: []
            };
        }

        // Récupérer le code ISO et le drapeau
        let flagUrl = 'https://flagcdn.com/fr.svg';
        let carbonIntensity = 0;
        const country = result.lpc_infos?.country;
        let countryCode;

        if (country && country !== 'Inconnu') {
            try {
                const countryResponse = await fetch(`https://restcountries.com/v3.1/name/${country}?fullText=true`);
                const countries = await countryResponse.json();

                if (countries && countries[0]) {
                    countryCode = countries[0].cca2.toLowerCase();
                    flagUrl = `/images/flags/${countryCode}.svg`;
                }
            } catch (error) {
                console.error('Erreur lors de la récupération du drapeau :', error);
            }

            try {
                const carbonIntensityResponse = await fetch(`https://api.electricitymap.org/v3/carbon-intensity/latest?zone=${countryCode.toUpperCase()}`, {
                    method: 'GET',
                    headers: {
                        'Content-Type': 'application/json',
                        'auth-token': ELECTRICITY_MAP_API_KEY
                    },
                    credentials: 'include'
                });
                const carbonData = await carbonIntensityResponse.json();

                if (carbonData && carbonData.carbonIntensity) {
                    carbonIntensity = carbonData.carbonIntensity;
                }
            } catch (error) {
                console.error('Erreur lors de la récupération de l\'intensité carbone :', error);
            }
        }

        return {
            pageData: {
                link: result.lpc_infos?.link || null,
                letterGreenScore: 'A',
                country: country || 'Inconnu',
                carbonFootprint: result.lpc_infos?.carbon_footprint || 0,
                pageSize: result.lpc_infos?.data_transferred ? Math.round(result.lpc_infos.data_transferred / 1024) : 0,
                loadingTime: result.lpc_infos?.loading_time || 0,
                queriesQuantity: result.lpc_infos?.queries_quantity || 0,
                flagUrl,
                carbonIntensity
            },
            adviceUser: result.advices?.[1] || '',
            adviceDev: result.advices?.[0] || '',
            letterGreenScore: result.letter || 'A',
            envNomination: result.env_nomination || 'Maître des Forêts',
            equivalents: result.equivalents || []
        };
    } catch (error) {
        console.error('Erreur lors de la récupération des données :', error);
        return {
            pageData: null,
            adviceUser: '',
            adviceDev: '',
            letterGreenScore: '',
            envNomination: '',
            equivalents: []
        };
    }
}
