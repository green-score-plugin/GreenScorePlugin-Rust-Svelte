<script lang="ts">
    export let advice: string | null = null;
    export let type: 'dev' | 'nav' | null = null;
</script>

<div class="{type === 'dev' ? 'bg-[#92BFFF]' : 'bg-[#94E9B8]'} p-6 rounded-lg shadow-md col-span-1 lg:col-span-3 flex flex-col justify-between order-8 lg:order-8 font-outfit">
    {#if advice && type}
        <div class="flex gap-3 items-start">
            <div class="bg-white w-8 h-8 min-w-8 min-h-8 flex items-center justify-center rounded-lg">
                <i class="fa-solid fa-circle-question {type === 'dev' ? 'text-[#263346]' : 'text-gs-green-800'}"></i>
            </div>
            <h2 class="text-lg font-bold text-white">
                Des conseils pour {type === 'dev' ? 'le développement' : 'la navigation'}&nbsp;?
            </h2>
        </div>
        <p class="text-sm mt-2 text-white">
            {advice}
        </p>
        <div class="flex w-full justify-end">
            <button class="{type === 'dev' ? 'bg-[#455a79]' : 'bg-[#4b7d60]'} text-white px-4 py-2 mt-4 rounded-lg">En savoir plus</button>
        </div>
    {:else}
        <p class="text-center text-gray-500 flex items-center justify-center h-full">pas de données</p>
    {/if}
</div>