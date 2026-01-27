<script lang="ts">
    import { t } from 'svelte-i18n';
    export let myAverageDailyCarbonFootprint: number | null = null;
    export let averageDailyCarbonFootprint: number | null = null;
    export let messageAverageFootprint: string | null = null;
</script>

<div class="h-full bg-white flex flex-col justify-between gap-2 p-6 rounded-lg shadow-md col-span-1 order-1 lg:col-span-4 text-grey-950">
    {#if myAverageDailyCarbonFootprint === null && averageDailyCarbonFootprint === null}
        <p class="text-center text-gray-500 flex items-center justify-center h-full">{$t('widgets.common.no_data')}</p>
    {:else}
        <h2 class="text-lg font-bold text-center font-outfit">{$t('widgets.common.average_daily_footprint.title')}</h2>
        <div class="flex justify-center gap-5">
            {#if myAverageDailyCarbonFootprint !== null}
                <div class="flex flex-col items-center">
                    <div class="relative text-center">
                        <svg class="w-20 h-20 rotate-[-90deg]" viewBox="0 0 36 36">
                            <circle cx="18" cy="18" r="15.9155" class="text-gradient-purple" fill="none" stroke="currentColor" stroke-width="2" stroke-dasharray="100 0" stroke-linecap="round"></circle>
                        </svg>
                        <p class="absolute inset-0 flex items-center justify-center text-3xl font-bold text-gradient-purple" data-value="{myAverageDailyCarbonFootprint}">{myAverageDailyCarbonFootprint}</p>
                    </div>
                    <p class="text-sm text-gray-500">{$t('widgets.common.average_daily_footprint.you')}</p>
                </div>
            {/if}
            {#if averageDailyCarbonFootprint !== null}
                <div class="flex flex-col items-center">
                    <div class="relative text-center">
                        <svg class="w-20 h-20 rotate-[-90deg]" viewBox="0 0 36 36">
                            <circle cx="18" cy="18" r="15.9155" class="text-gradient-purple" fill="none" stroke="currentColor" stroke-width="2" stroke-dasharray="100 0" stroke-linecap="round"></circle>
                        </svg>
                        <p class="absolute inset-0 flex items-center justify-center text-3xl font-bold text-gradient-purple" data-value="{averageDailyCarbonFootprint}">{averageDailyCarbonFootprint}</p>
                    </div>
                    <p class="text-sm text-gray-500">{$t('widgets.common.average_daily_footprint.other_users')}</p>
                </div>
            {/if}
        </div>

        {#if myAverageDailyCarbonFootprint !== null && averageDailyCarbonFootprint !== null && messageAverageFootprint !== null}
            <p class="text-sm text-center {myAverageDailyCarbonFootprint > averageDailyCarbonFootprint ? 'text-red-700' : 'text-green-700'} font-medium mt-2">
                {messageAverageFootprint}
            </p>
        {/if}
    {/if}
</div>
