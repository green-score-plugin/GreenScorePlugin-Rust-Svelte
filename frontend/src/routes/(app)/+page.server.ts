import { BACKEND_URL } from '$lib/config';
import type { PageServerLoad } from './$types';
export const load: PageServerLoad = async ({ fetch }) => {
    try {
        const response = await fetch(`${BACKEND_URL}/advice`);

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();

        const adviceWithDevFlag = data.advice.map((item: any) => ({
            ...item,
            // isDev: false // ou une logique pour déterminer si c'est un conseil dev
        }));

        return {
            advice: adviceWithDevFlag,
            error: undefined
        };
    } catch (error) {
        console.error('Error fetching advice:', error);
        return {
            advice: [],
            error: 'Failed to load advice'
        };
    }
};