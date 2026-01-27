import { register, init, locale } from 'svelte-i18n';
import { browser } from '$app/environment';

register('fr', () => import('./fr.json'));
register('en', () => import('./en.json'));

init({
    fallbackLocale: 'fr'
});

if (browser) {
    const saved = localStorage.getItem('lang');
    if (saved) {
        locale.set(saved);
    }

    locale.subscribe((value) => {
        if (value) {
            localStorage.setItem('lang', value);
        }
    });
}
