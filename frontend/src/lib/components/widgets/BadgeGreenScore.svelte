<script lang="ts">
    export let letterGreenScore: string | null = 'A';
    export let envNomination: string | null = null;
    import { t } from 'svelte-i18n';

    $: validLetter = letterGreenScore && letterGreenScore !== 'N/A' ? letterGreenScore : null;
    $: mobileUrl = validLetter ? `/images/badges/mobile/${validLetter}.png` : null;
    $: desktopUrl = validLetter ? `/images/badges/desktop/${validLetter}.svg` : null;
</script>

<div class="h-full flex flex-col bg-gradient-to-tr from-white via-blue-100 to-green-100 p-6 rounded-lg shadow-md col-span-1 order-3 sm:order-2 lg:order-3 lg:col-span-4 text-grey-950 font-outfit">
    <div class="flex flex-col items-center justify-between h-full text-center">
        <h2 class="text-lg font-bold row-span-1">{$t('dashboard.widgets.badge.title')}</h2>

        {#if validLetter}
            <div class="flex-1 flex items-center justify-center w-full relative">
                <picture class="w-full h-full flex items-center justify-center">
                    <source media="(max-width: 768px)" srcset={mobileUrl} />
                    <img
                            src={desktopUrl}
                            alt={`Badge Impact Environnemental ${validLetter}`}
                            class="w-[80%] h-auto object-contain"
                            loading="lazy"
                    />
                </picture>
            </div>
        {:else}
            <p class="text-center text-gray-500 flex items-center justify-center h-full">{$t('dashboard.widgets.badge.no_data')}</p>
        {/if}

        {#if envNomination && envNomination !== 'N/A'}
            <p class="text-lg font-bold">{$t(envNomination)}</p>
        {/if}
    </div>
</div>
