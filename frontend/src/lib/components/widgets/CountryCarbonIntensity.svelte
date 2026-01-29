<script lang="ts">
    import { t } from 'svelte-i18n';
    export let country: string | null = null;
    export let flagUrl: string | null = null;
    export let carbonIntensity: number | null = null;
    import empreinteCarboneUrl from '$lib/images/empreinte_carbone.png';
    import { tweened } from 'svelte/motion';
    import { cubicOut } from 'svelte/easing';

    const animatedValue = tweened(0, {
        duration: 2000,
        easing: cubicOut
    });

    $: if (carbonIntensity) {
        animatedValue.set(carbonIntensity);
    }
</script>

<div class="bg-white flex flex-col gap-4 p-6 rounded-lg shadow-md col-span-1 sm:col-span-2 order-2 sm:order-3 lg:col-span-4 lg:order-2">
    {#if country && carbonIntensity}
    <div class="flex items-center gap-4">
        {#if flagUrl}
        <img src="{ flagUrl }" alt="{$t('widgets.country_carbon_intensity.alt_flag', { values: { country } })}" class="w-10 h-10 rounded-full object-cover border" loading="lazy">
        {:else}
        <div class="w-10 h-10 rounded-full bg-red-300"></div>
        {/if}
        <h2 class="text-lg font-bold font-outfit text-grey-950">{$t('widgets.country_carbon_intensity.search_from', { values: { country } })}</h2>
    </div>
    <div class="flex justify-center flex-wrap items-center gap-4 h-full">
        <picture class="flex items-center justify-center mt-4 flex-1">
            <img src="{empreinteCarboneUrl}" class="w-32 h-auto min-w-[100px]" alt="Empreinte Carbone" loading="lazy">
        </picture>
        <div class="flex flex-col justify-center items-center flex-1 font-outfit">
            <p class="text-6xl font-bold text-[#92bfff] animate-counter" data-value="{carbonIntensity}">{Math.round($animatedValue)}</p>
            <p class="text-xl font-bold">gCO2eq/kWh*</p>
            <p class="text-sm text-center pt-5">{$t('widgets.country_carbon_intensity.description')}</p>
        </div>
    </div>
        <p class="text-sm text-gray-600 text-right font-outfit">{$t('widgets.country_carbon_intensity.data_source')} <a href="https://app.electricitymaps.com/map/live/fifteen_minutes">electricitymap.com</a></p>
    {:else}
    <p class="text-center text-gray-500 flex items-center justify-center h-full">{$t('widgets.common.no_data')}</p>
    {/if}
</div>