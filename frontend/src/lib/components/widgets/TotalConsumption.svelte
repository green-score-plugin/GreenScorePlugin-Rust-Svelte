<script lang="ts">
    import { t } from 'svelte-i18n';
    export let label: string | null = null;
    export let carbonFootprint: number | null = null;
    export let totalConsuUnit: string | null = null;
</script>

<div class="h-full min-h-[250px] grid grid-row-3 bg-gradient-to-tl from-purple-600 to-zinc-600 text-white p-6 rounded-lg shadow-md col-span-1 sm:col-span-2 order-5 sm:order-4 lg:col-span-6 text-center lg:order-5">
    <h2 class="font-bold font-outfit row-span-1 flex justify-start">{label ?? $t('widgets.common.total_consumption')}</h2>
    {#if carbonFootprint && totalConsuUnit}
    <p class="text-6xl font-extrabold font-outfit mt-4 row-span-2 flex flex-wrap justify-center"><span class="animate-counter">{carbonFootprint}</span>&nbsp;{totalConsuUnit}</p>
    {:else}
    <p class="text-center text-gray-500 flex items-center justify-center h-full">{$t('widgets.common.no_data')}</p>
    {/if}
</div>