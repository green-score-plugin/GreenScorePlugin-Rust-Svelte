import { writable } from 'svelte/store';
import type { Account } from '$lib/types/account';

function createAccountStore() {
    const { subscribe, set } = writable<Account | null>(null);

    return {
        subscribe,
        login: (account: Account) => set(account),
        logout: () => set(null)
    };
}

export const account = createAccountStore();