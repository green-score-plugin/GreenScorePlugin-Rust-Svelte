<script lang="ts">
    import imageFond from '$lib/images/register-image1.png';
    import logo from '$lib/images/greenscore-logo.png';
    import { t, locale } from 'svelte-i18n';

    export let title: string;
    export let subtitle: string;
    export let form: { message?: string } | null = null;
    export let showModeSwitcher = false;
</script>

<title>{title} | GreenScore Web</title>

<div class="flex items-center w-screen h-screen">
    <img src="{imageFond}" alt="GreenScore" class="hidden xl:flex w-1/2 h-full object-cover">
    <div class="relative w-full xl:w-1/2 h-full px-6 md:px-[120px] py-24 flex flex-col items-center justify-center">
        <div class="absolute top-5 right-5 flex gap-3">
            <button
                    onclick={() => locale.set('en')}
                    class="transition-all duration-300 hover:scale-110 cursor-pointer {$locale === 'en' ? 'opacity-100 scale-110 grayscale-0' : 'opacity-50 grayscale hover:grayscale-0 hover:opacity-100'}"
                    title="English"
            >
                <img src="/images/flags/gb.svg" alt="English" class="w-6 h-auto object-cover rounded shadow-sm" />
            </button>
            <button
                    onclick={() => locale.set('fr')}
                    class="transition-all duration-300 hover:scale-110 cursor-pointer {$locale === 'fr' ? 'opacity-100 scale-110 grayscale-0' : 'opacity-50 grayscale hover:grayscale-0 hover:opacity-100'}"
                    title="Français"
            >
                <img src="/images/flags/fr.svg" alt="Français" class="w-6 h-auto object-cover rounded shadow-sm" />
            </button>
        </div>

        <div class="flex flex-col w-full gap-10">
            <div class="flex gap-2 items-center">
                <img width="55px" height="auto" src="{logo}" alt="GreenScore Logo">
                <h1 class="text-3xl font-outfit font-extrabold text-gs-green-950">GreenScore Web</h1>
            </div>

            <div class="flex flex-col gap-4 font-outfit">
                <h2 class="text-2xl font-extrabold text-grey-950">{subtitle}</h2>

                {#if showModeSwitcher}
                    <div class="flex gap-[10px] px-1 border border-grey-200 rounded-full py-1 w-fit font-medium">
                        <slot name="mode-switcher"></slot>
                    </div>
                {/if}
            </div>

            {#if form?.message}
                <div class="w-full bg-red-50 text-red-700 text-sm font-outfit font-medium border border-red-700 rounded-lg px-6 py-6">
                    {$t(form.message)}
                </div>
            {/if}

            <slot name="form"></slot>
        </div>
    </div>
</div>
