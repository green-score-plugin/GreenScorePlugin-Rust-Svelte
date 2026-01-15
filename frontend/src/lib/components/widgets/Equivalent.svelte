<script lang="ts">
    export let equivalent: { name: string; value: number; icon: string | null } | null = null;
    export let order: number = 1;

    const modules = import.meta.glob('/src/lib/images/equivalents/*.{png,jpg,jpeg,webp,svg}', { eager: true }) as Record<string, any>;
    const iconMap: Record<string, string> = {};
    for (const p in modules) {
        const name = p.split('/').pop()!;
        iconMap[name] = modules[p].default ?? modules[p];
    }

    $: iconUrl = equivalent?.icon ? iconMap[equivalent.icon] ?? null : null;

    $: orderClasses = order === 1
        ? 'order-4 sm:order-5 lg:order-4'
        : 'order-5 sm:order-6 lg:order-6';
</script>

<div class="h-full bg-white flex flex-col items-center justify-center p-6 rounded-lg shadow-md col-span-1 lg:col-span-3 text-grey-950 {orderClasses}">
    {#if equivalent && equivalent.name && equivalent.value}
        <div class="flex justify-center gap-2">
            <picture class="bg-[#94e9b8] w-8 h-8 flex items-center justify-center rounded-full">
                <img src="/images/equivalent.png" alt="Equivalent" class="w-4 h-4 rounded-full">
            </picture>
            <h2 class="text-lg font-bold font-outfit">Équivalent</h2>
        </div>
        {#if equivalent.icon}
            <picture class="w-24 h-24 flex items-center justify-center mt-4">
                <img
                        src="{iconUrl}"
                        alt={equivalent.name}
                        class="w-full h-auto object-contain"
                        loading="lazy"
                >
            </picture>
        {/if}
        <p class="text-center mt-2 text-lg font-medium font-outfit">
            <span class="animate-counter">{equivalent.value}</span>
            {equivalent.name}
        </p>
    {:else}
        <p class="text-center text-gray-500 flex items-center justify-center h-full">pas de données</p>
    {/if}
</div>
