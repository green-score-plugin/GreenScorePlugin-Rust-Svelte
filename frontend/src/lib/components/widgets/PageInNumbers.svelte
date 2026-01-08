<script lang="ts">
    export let pageSize: number | null = null;
    export let pageSizeUnit: string | null = null;
    export let loadingTime: number | null = null;
    export let queriesQuantity: number | null = null;

    import { tweened } from 'svelte/motion';
    import { cubicOut } from 'svelte/easing';

    const animatedPageSize = tweened(0, {
        duration: 2000,
        easing: cubicOut
    });

    const animatedLoadingTime = tweened(0, {
        duration: 2000,
        easing: cubicOut
    });

    const animatedQueriesQuantity = tweened(0, {
        duration: 2000,
        easing: cubicOut
    });

    $: if (pageSize) {
        animatedPageSize.set(pageSize);
    }

    $: if (loadingTime) {
        animatedLoadingTime.set(loadingTime);
    }

    $: if (queriesQuantity) {
        animatedQueriesQuantity.set(queriesQuantity);
    }
</script>

<div class="bg-white grid grid-row-3 p-6 rounded-lg shadow-md col-span-1 sm:col-span-2 lg:col-span-6 flex-col flex-wrap min-h-[300px] order-7 lg:order-7">
    <h2 class="text-lg font-bold mb-4 row-span-1 font-outfit">La page en quelques chiffres</h2>
    <div class="flex flex-col sm:flex-row sm:flex-wrap justify-center w-full h-fit row-span-2 gap-3">
        {#if !pageSize && !loadingTime && !queriesQuantity}
            <p class="text-center text-gray-500 flex items-center justify-center h-full">pas de données</p>
        {:else }
            {#if pageSize && pageSizeUnit}
                <div class="flex flex-col flex-1 h-fit items-center">
                    <div class="relative text-center">
                        <svg class="w-28 h-28 rotate-[-90deg]" viewBox="0 0 36 36">
                            <circle cx="18" cy="18" r="15.9155" class="text-[#8458A9]" fill="none" stroke="currentColor" stroke-width="2" stroke-dasharray="100 0" stroke-linecap="round"></circle>
                        </svg>
                        <p class="absolute inset-0 flex items-center justify-center text-3xl font-bold text-[#8458A9]">{Math.round($animatedPageSize)}{pageSizeUnit}</p>
                    </div>
                    <p class="text-sm text-gray-500 text-center">Taille</p>
                </div>
            {/if}
            {#if loadingTime}
                <div class="flex flex-col flex-1 h-fit items-center">
                    <div class="relative text-center">
                        <svg class="w-28 h-28 rotate-[-90deg]" viewBox="0 0 36 36">
                            <circle cx="18" cy="18" r="15.9155" class="text-[#8458A9]" fill="none" stroke="currentColor" stroke-width="2" stroke-dasharray="100 0" stroke-linecap="round"></circle>
                        </svg>
                        <p class="absolute inset-0 flex items-center justify-center text-3xl font-bold text-[#8458A9]">{$animatedLoadingTime.toFixed(1)} s</p>
                    </div>
                    <p class="text-sm text-gray-500 text-center">Temps de chargement</p>
                </div>
            {/if}
            {#if queriesQuantity}
                <div class="flex flex-col flex-1 h-fit items-center">
                    <div class="relative text-center">
                        <svg class="w-28 h-28 rotate-[-90deg]" viewBox="0 0 36 36">
                            <circle cx="18" cy="18" r="15.9155" class="text-[#8458A9]" fill="none" stroke="currentColor" stroke-width="2" stroke-dasharray="100 0" stroke-linecap="round"></circle>
                        </svg>
                        <p class="absolute inset-0 flex items-center justify-center text-3xl font-bold text-[#8458A9]">{Math.round($animatedQueriesQuantity)}</p>
                    </div>
                    <p class="text-sm text-gray-500 text-center">Nombre de requêtes</p>
                </div>
            {/if}
        {/if}
    </div>
</div>
