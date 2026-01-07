import type { PageServerLoad } from './$types';
import { BACKEND_URL } from "$lib/config.ts";

export const load: PageServerLoad = async ({ fetch }) => {
    let adviceDev: string = '';
    try {
        const response = await fetch(`${BACKEND_URL}/advice?is_dev=true`, {
            method: 'GET',
            headers: {'Content-Type': 'application/json'},
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
            headers: {'Content-Type': 'application/json'},
            credentials: 'include'
        });
        const result = await response.json();
        if (result.success) {
            adviceUser = result.advice;
        }
    } catch (error) {
        console.error('Erreur lors de la récupération du conseil pour les utilisateurs :', error);
    }

    let link: string = '';
    try {
        const response = await fetch(`${BACKEND_URL}/last-link`, {
            method: 'GET',
            headers: {'Content-Type': 'application/json'},
            credentials: 'include'
        });
        const result = await response.json();
        console.log(result);
        if (result.success) {
            link = result.link;
        }
    } catch (error) {
        console.error('Erreur lors de la récupération du dernier lien consulté :', error);
    }

    let pageData: any = null;
    try {
        const response = await fetch(`${BACKEND_URL}/derniere-page-consultee`, {
            method: 'GET',
            headers: {'Content-Type': 'application/json'},
            credentials: 'include'
        });
        const result = await response.json();
        console.log(result);
        if (result.success) {
            return {
                pageData: {
                    link: result.lpc_infos?.link || null,
                    letterGreenScore: 'A', // À calculer selon vos critères
                    country: result.lpc_infos?.country || 'Inconnu',
                    pageSize: result.lpc_infos?.data_transferred ? Math.round(result.lpc_infos.data_transferred / 1024) : 0,
                    loadingTime: result.lpc_infos?.loading_time || 0,
                    queriesQuantity: result.lpc_infos?.queries_quantity || 0
                },
                adviceUser: result.advices?.[1] || '',
                adviceDev: result.advices?.[0] || ''
            };
        }
    } catch (error) {
        console.error('Erreur lors de la récupération des données :', error);
    }
}