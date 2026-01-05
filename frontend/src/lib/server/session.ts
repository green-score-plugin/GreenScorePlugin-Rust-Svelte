import { BACKEND_URL } from '$lib/config';
import type { Account } from '$lib/types/account';
import type { Cookies } from '@sveltejs/kit';

interface CacheEntry {
    account: Account;
    expires: number;
}

const cache = new Map<string, CacheEntry>();

function getCacheTTL(account: Account): number {
    return account.role === 'user' ? 5 * 60 * 1000 : 2 * 60 * 1000;
}

export async function getAccount(sessionCookie: string | undefined): Promise<Account | null> {
    if (!sessionCookie) return null;

    const cached = cache.get(sessionCookie);
    if (cached && Date.now() < cached.expires) {
        return cached.account;
    }

    try {
        const response = await fetch(`${BACKEND_URL}/get-account`, {
            method: 'POST',
            headers: { cookie: `greenscoreweb_sessions=${sessionCookie}` }
        });

        if (response.ok) {
            const result = await response.json();
            if (result.success && result.account) {
                cache.set(sessionCookie, {
                    account: result.account,
                    expires: Date.now() + getCacheTTL(result.account)
                });
                return result.account;
            }
        }
    } catch (_) {
        console.error('Erreur session');
    }

    return null;
}

export function invalidateCache(sessionCookie: string) {
    cache.delete(sessionCookie);
}

export function setSessionCookie(cookies: Cookies, response: Response) {
    const setCookieHeader = response.headers.get('set-cookie');
    if (setCookieHeader) {
        const cookieMatch = setCookieHeader.match(/greenscoreweb_sessions=([^;]+)/);
        if (cookieMatch) {
            const sessionValue = cookieMatch[1];
            cookies.set('greenscoreweb_sessions', sessionValue, {
                path: '/',
                httpOnly: true,
                sameSite: 'lax',
                maxAge: 60 * 60 // 1 heure
            });
        }
    }
}
