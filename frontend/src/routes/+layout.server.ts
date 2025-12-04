import type { LayoutServerLoad } from './$types';
import { getAccount } from '$lib/server/session';

export const load: LayoutServerLoad = async ({ cookies }) => {
    const session = cookies.get('greenscoreweb_sessions');
    const account = await getAccount(session);
    return {account};
};