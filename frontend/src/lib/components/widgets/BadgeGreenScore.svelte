<script lang="ts">
    export let letterGreenScore: string | null = 'A';
    export let envNomination: string | null = null;

    $: mobileUrl = letterGreenScore ? `/images/badges/mobile/${letterGreenScore}.png` : null;
    $: desktopUrl = letterGreenScore ? `/images/badges/desktop/${letterGreenScore}.svg` : null;
</script>

<div class="flex flex-col bg-gradient-to-tr from-white via-blue-100 to-green-100 p-6 rounded-lg shadow-md col-span-1 order-3 sm:order-2 lg:order-3 lg:col-span-4 text-grey-950 font-outfit">
    <div class="flex flex-col items-center justify-between h-full text-center">
        <h2 class="text-lg font-bold row-span-1">Badge d'impact environnemental</h2>

        {#if letterGreenScore}
            <div class="flex-1 flex items-center justify-center w-full relative">
                <picture class="w-full h-full flex items-center justify-center">
                    <source media="(max-width: 768px)" srcset={mobileUrl} />
                    <img
                            src={desktopUrl}
                            alt={`Badge Impact Environnemental ${letterGreenScore}`}
                            class="w-[80%] h-auto object-contain"
                            loading="lazy"
                    />
                </picture>
            </div>
        {:else}
            <p class="text-center text-gray-500 flex items-center justify-center h-full">pas de données</p>
        {/if}

        {#if envNomination}
            <p class="text-lg font-bold">{envNomination}</p>
        {/if}
    </div>
</div>
