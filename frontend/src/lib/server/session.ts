import { BACKEND_URL } from '$lib/config';
import type { UserFull } from '$lib/types/account';
import type { Cookies } from '@sveltejs/kit';

interface CacheEntry {
    user: UserFull;
    expires: number;
}

const cache = new Map<string, CacheEntry>();

function getCacheTTL(user: UserFull): number {
    return (user.organisation && user.organisation.length > 0) ? 2 * 60 * 1000 : 5 * 60 * 1000;
}

export async function getAccount(sessionCookie: string | undefined): Promise<UserFull | null> {
    if (!sessionCookie) return null;

    const cached = cache.get(sessionCookie);
    if (cached && Date.now() < cached.expires) {
        return cached.user;
    }

    try {
        const response = await fetch(`${BACKEND_URL}/auth/get-account`, {
            method: 'POST',
            headers: { cookie: `greenscoreweb_sessions=${sessionCookie}` }
        });

        if (response.ok) {
            const result = await response.json();
            
            if (result.success && result.user_full) {
                const userFull = result.user_full;
                cache.set(sessionCookie, {
                    user: userFull,
                    expires: Date.now() + getCacheTTL(userFull)
                });
                return userFull;
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
