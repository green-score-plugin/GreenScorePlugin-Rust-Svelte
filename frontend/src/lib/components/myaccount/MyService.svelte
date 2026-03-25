<script lang="ts">
    import { t } from 'svelte-i18n';

    let { services = [] } = $props();
    let scrollContainer: HTMLDivElement | undefined = $state();
</script>

<div class="mt-8 pt-8 border-t border-gray-100">
    <div class="flex items-center justify-between mb-6">
        <h3 class="text-xl font-bold font-outfit text-gray-900">
            {$t('account.service.list_title') || "Liste des services"}
        </h3>
        <span class="px-2.5 py-0.5 rounded-full bg-gray-100 text-gray-600 text-xs font-medium border border-gray-200">
            {services?.length || 0}
        </span>
    </div>

    {#if !services || services.length === 0}
        <div class="flex flex-col items-center justify-center p-8 bg-gray-50 rounded-xl border border-dashed border-gray-200 text-center">
            <div class="bg-white p-3 rounded-full mb-3 shadow-sm text-gray-400">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect><line x1="8" y1="21" x2="16" y2="21"></line><line x1="12" y1="17" x2="12" y2="21"></line></svg>
            </div>
            <p class="text-gray-500 text-sm">
                {$t('account.service.no_services') || "Aucun service trouvé pour cette organisation."}
            </p>
        </div>
    {:else}
        <div class="border border-gray-200 rounded-xl shadow-sm overflow-hidden bg-white">
            <div
                    bind:this={scrollContainer}
                    class="max-h-[400px] overflow-y-auto custom-scrollbar"
            >
                <table class="min-w-full divide-y divide-gray-100">
                    <thead class="bg-gray-50 sticky top-0 z-10">
                    <tr>
                        <th scope="col" class="px-6 py-3.5 text-left text-xs font-semibold text-gray-500 uppercase tracking-wider">
                            {$t('account.service.name') || 'Nom du service'}
                        </th>
                    </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-gray-100">
                    {#each services as service (service.id)}
                        <tr class="group hover:bg-gray-50/80 transition-colors">
                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                                <div class="flex items-center gap-3">
                                    <div class="w-8 h-8 rounded-full bg-gray-100 text-gray-600 flex items-center justify-center text-xs font-bold uppercase">
                                        {service.nom.substring(0, 2)}
                                    </div>
                                    {service.nom}
                                </div>
                            </td>
                        </tr>
                    {/each}
                    </tbody>
                </table>
            </div>
        </div>
    {/if}
</div>
