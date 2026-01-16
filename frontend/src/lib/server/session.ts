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

export async function setSessionCookie(cookies: Cookies, source: Response | string | null | undefined) {
    if (!source) {
        console.warn('setSessionCookie: source vide');
        return;
    }

    if (typeof source === 'string') {
        cookies.set('greenscoreweb_sessions', source, {
            path: '/',
            httpOnly: true,
            sameSite: 'lax',
            maxAge: 60 * 60 // 1 heure
        });
        return;
    }

    try {
        const setCookieHeader = source.headers?.get?.('set-cookie');
        if (setCookieHeader) {
            const cookieMatch = setCookieHeader.match(/greenscoreweb_sessions=([^;]+)/);
            if (cookieMatch) {
                const sessionValue = cookieMatch[1];
                cookies.set('greenscoreweb_sessions', sessionValue, {
                    path: '/',
                    httpOnly: true,
                    sameSite: 'lax',
                    maxAge: 60 * 60
                });
                return;
            }
        }


        try {
            const cloned = (typeof (source as any).clone === 'function') ? (source as any).clone() : source;
            const maybeJson = await cloned.json().catch(() => null);
            const sessionValueFromJson = maybeJson?.token ?? maybeJson?.session ?? maybeJson?.sessionValue ?? null;
            if (typeof sessionValueFromJson === 'string') {
                cookies.set('greenscoreweb_sessions', sessionValueFromJson, {
                    path: '/',
                    httpOnly: true,
                    sameSite: 'lax',
                    maxAge: 60 * 60
                });
                return;
            }
        } catch (e) {

        }

        console.warn('setSessionCookie: aucun token trouvé ni header Set-Cookie dans la source fournie');

    } catch (err) {
        console.error('Erreur dans setSessionCookie:', err);
    }
}
