<script lang="ts">
    export let advice: string | null = null;
    export let type: 'dev' | 'nav' | null = null;
    import { t } from 'svelte-i18n';
</script>

<div class="h-full {type === 'dev' ? 'bg-[#92BFFF]' : 'bg-[#94E9B8]'} p-6 rounded-lg shadow-md col-span-1 lg:col-span-3 flex flex-col justify-between order-8 lg:order-8 font-outfit">
    {#if advice && type}
        <div class="flex gap-3 items-start">
            <div class="bg-white w-8 h-8 min-w-8 min-h-8 flex items-center justify-center rounded-lg {type === 'dev' ? 'text-[#263346]' : 'text-gs-green-800'}">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path fill="currentColor" d="M15.333 9.5A3.5 3.5 0 0 0 8.8 7.75a1 1 0 0 0 1.733 1a1.5 1.5 0 0 1 1.3-.75a1.5 1.5 0 1 1 0 3h-.003a1 1 0 0 0-.19.039a1 1 0 0 0-.198.04a1 1 0 0 0-.155.105a1 1 0 0 0-.162.11a1 1 0 0 0-.117.174a1 1 0 0 0-.097.144a1 1 0 0 0-.043.212a1 1 0 0 0-.035.176v1l.002.011v.491a1 1 0 0 0 1 .998h.003a1 1 0 0 0 .998-1.002l-.002-.662A3.49 3.49 0 0 0 15.333 9.5m-4.203 6.79a1 1 0 0 0 .7 1.71a1.04 1.04 0 0 0 .71-.29a1.015 1.015 0 0 0 0-1.42a1.034 1.034 0 0 0-1.41 0"/></svg>
            </div>
            <h2 class="text-lg font-bold text-white">
                {type === 'dev' ? $t('dashboard.widgets.advice.title_dev') : $t('dashboard.widgets.advice.title_nav')}
            </h2>
        </div>
        <p class="text-sm mt-2 text-white">
            {$t(advice)}
        </p>
        <div class="flex w-full justify-end">
            <a class="{type === 'dev' ? 'bg-[#455a79]' : 'bg-[#4b7d60]'} text-white px-4 py-2 mt-4 rounded-lg hover:cursor-pointer hover:scale-105 transition-all ease-in-out duration-300" href="/#advice">{$t('dashboard.widgets.advice.more_info')}</a>
        </div>
    {:else}
        <p class="text-center text-gray-500 flex items-center justify-center h-full">{$t('dashboard.widgets.advice.no_data')}</p>
    {/if}
</div>
