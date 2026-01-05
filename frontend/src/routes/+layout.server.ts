import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals }) => {
    console.log('Layout Server Load - User:', locals.user);
    return {
        user: locals.user
    };
};