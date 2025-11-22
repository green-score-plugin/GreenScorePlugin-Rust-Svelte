import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { BACKEND_URL } from '$lib/config';

export const actions = {
    default: async ({ request, fetch }) => {
        const data = await request.formData();
        const email = data.get('email');
        const password = data.get('password');


    }
} satisfies Actions;
