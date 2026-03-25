import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { BACKEND_URL } from '$lib/config';
import { setSessionCookie } from '$lib/server/session';

interface LoginAttempt {
    count: number;
    firstAttemptTime: number;
}

const loginAttempts = new Map<string, LoginAttempt>();
const MAX_ATTEMPTS = 5;
const WINDOW_MS = 15 * 60 * 1000; // 15 minutes

function cleanupAttempts() {
    const now = Date.now();
    for (const [ip, attempt] of loginAttempts.entries()) {
        if (now - attempt.firstAttemptTime > WINDOW_MS) {
            loginAttempts.delete(ip);
        }
    }
}

export const actions = {
    default: async ({ request, fetch, cookies, getClientAddress }) => {
        const clientIp = getClientAddress();
        const now = Date.now();

        if (Math.random() < 0.1) cleanupAttempts();

        let attempt = loginAttempts.get(clientIp);

        if (attempt && now - attempt.firstAttemptTime > WINDOW_MS) {
            loginAttempts.delete(clientIp);
            attempt = undefined;
        }

        if (attempt && attempt.count >= MAX_ATTEMPTS) {
            return fail(429, { message: 'errors.too_many_attempts' });
        }

        const data = await request.formData();
        const email = data.get('email');
        const password = data.get('password');

        if (!email || !password) {
            return fail(400, { message: 'errors.match_user' });
        }

        try {
            const response = await fetch(`${BACKEND_URL}/auth/login`, {
                method: 'POST',
                credentials: 'include',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ email, password })
            });

            const result = await response.json();


            if (result.success) {
                loginAttempts.delete(clientIp);
                setSessionCookie(cookies, response);
                return redirect(303, '/');
            }

            if (!attempt) {
                loginAttempts.set(clientIp, { count: 1, firstAttemptTime: now });
            } else {
                attempt.count++;
            }

            return fail(400, { message: result.message || 'Erreur de connexion' });
        } catch (error) {
            if (error && typeof error === 'object' && ('status' in error || 'location' in error)) {
                throw error;
            }
            return fail(500, { message: 'errors.server_error' });
        }
    }
} satisfies Actions;
