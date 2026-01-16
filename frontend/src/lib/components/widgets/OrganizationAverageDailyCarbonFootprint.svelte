<script lang="ts">
    import { tweened } from 'svelte/motion';
    import { cubicOut } from 'svelte/easing';
    import equivalent from '$lib/images/equivalent.png';
    export let averageFootprint: number | null;
    export let equivalentAverage: { name: string; value: number; icon: string } | null;

    const animatedAverageFootprint = tweened(0, {
        duration: 2000,
        easing: cubicOut
    });

    $: if (averageFootprint) {
        animatedAverageFootprint.set(averageFootprint);
    }

    const modules = import.meta.glob('/src/lib/images/equivalents/*.{png,jpg,jpeg,webp,svg}', { eager: true }) as Record<string, any>;
    const iconMap: Record<string, string> = {};
    for (const p in modules) {
        const name = p.split('/').pop()!;
        iconMap[name] = modules[p].default ?? modules[p];
    }

    $: iconUrl = equivalentAverage?.icon ? iconMap[equivalentAverage.icon] ?? null : null;
</script>

<div class="bg-white flex flex-col justify-around gap-3 p-6 rounded-lg shadow-md col-span-1 order-1 lg:col-span-4 text-grey-950">
    {#if averageFootprint}
    <h2 class="text-lg font-bold text-center">Moyenne de l'empreinte carbone sur un jour (en gCO2e)</h2>
    <div class="flex justify-center gap-5 items-center">
        <div class="flex flex-col items-center flex-1 flex-grow">
            <div class="relative text-center">
                <svg class="w-20 h-20 rotate-[-90deg]" viewBox="0 0 36 36">
                    <circle cx="18" cy="18" r="15.9155" class="text-[#8458A9]" fill="none" stroke="currentColor" stroke-width="2" stroke-dasharray="100 0" stroke-linecap="round"></circle>
                </svg>
                <p class="absolute inset-0 flex items-center justify-center text-3xl font-bold text-gradient-purple animate-counter">{$animatedAverageFootprint.toFixed(1)}</p>
            </div>
        </div>
        {#if equivalentAverage && equivalentAverage.name && equivalentAverage.value}
        <picture class="bg-green-300 w-8 h-8 flex items-center justify-center rounded-full">
            <img src="{equivalent}" alt="Equivalent" class="w-4 h-4">
        </picture>
        <div class="flex flex-col justify-center flex-1">
            {#if equivalentAverage.icon}
            <div class="flex justify-center items-center w-full mt-4">
                <picture class="w-12 h-12 flex items-center justify-center">
                    <img src="{iconUrl}" alt="{equivalentAverage.name}" class="w-full h-auto object-contain" loading="lazy">
                </picture>
            </div>
            {/if}
            <p class="text-center mt-2 text-lg font-medium"><span class="animate-counter">{equivalentAverage.value}</span> {equivalentAverage.name}</p>
        </div>
        {:else}
        <p class="text-center text-gray-500 flex items-center justify-center h-full">pas de données</p>
        {/if}
    </div>
    {:else}
    <p class="text-center text-gray-500 flex items-center justify-center h-full">pas de données</p>
    {/if}
</div>