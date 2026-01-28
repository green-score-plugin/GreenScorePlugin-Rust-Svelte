import { register, init, locale, getLocaleFromNavigator } from 'svelte-i18n';
import { browser } from '$app/environment';

register('fr', () => import('./fr.json'));
register('en', () => import('./en.json'));

const defaultLocale = 'fr';
const detectedLocale = browser ? getLocaleFromNavigator() : defaultLocale;
const browserLocale = detectedLocale?.startsWith('en') ? 'en' : defaultLocale;

const initialLocale = browser
    ? (localStorage.getItem('lang') || browserLocale)
    : defaultLocale;

init({
    fallbackLocale: defaultLocale,
    initialLocale: initialLocale
});

if (browser) {

    locale.subscribe((value) => {
        if (value) {
            localStorage.setItem('lang', value);
        }
    });
}
