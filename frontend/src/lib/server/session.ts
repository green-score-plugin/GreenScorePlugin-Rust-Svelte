import { BACKEND_URL } from '$lib/config';
import type { Account } from '$lib/types/account';

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
    } catch (error) {
        console.error('Erreur session:', error);
    }

    return null;
}

export function invalidateCache(sessionCookie: string) {
    cache.delete(sessionCookie);
}