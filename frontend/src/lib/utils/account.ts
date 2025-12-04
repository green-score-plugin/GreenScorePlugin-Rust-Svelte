import type { Account } from '$lib/types/account';

export function isUser(account: Account | null): account is Account & { role: 'user' } {
    return account?.role === 'user';
}

export function isOrganisation(account: Account | null): account is Account & { role: 'organisation' } {
    return account?.role === 'organisation';
}

export function getDisplayName(account: Account | null): string {
    if (!account) return 'Invité';
    if (isUser(account)) return `${account.prenom} ${account.nom}`;
    if (isOrganisation(account)) return account.nom;
    return 'Compte';
}