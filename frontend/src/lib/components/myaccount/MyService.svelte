<script lang="ts">
    import { t } from 'svelte-i18n';

    let { services = [] } = $props();
    let scrollContainer: HTMLDivElement | undefined = $state();
</script>

<div class="mt-8 pt-6 border-t border-gray-100">
    <h3 class="text-xl font-bold mb-4 text-gray-800">{$t('account.service.list_title') || "Liste des services de l'organisation"}</h3>

    {#if !services || services.length === 0}
        <div class="p-4 bg-gray-50 rounded-lg text-gray-500 italic text-center">
            {$t('account.service.no_services') || "Aucun service trouvé pour cette organisation."}
        </div>
    {:else}
        <div class="border border-gray-200 rounded-lg shadow-sm overflow-hidden">
            <div
                    bind:this={scrollContainer}
                    class="max-h-[300px] overflow-y-auto pr-2 custom-scrollbar"
            >
                <table class="min-w-full divide-y divide-gray-200">
                    <thead class="bg-gray-50 sticky top-0 z-10 shadow-sm">
                    <tr>
                        <th scope="col" class="px-6 py-3 text-left text-xs font-semibold text-gray-500 uppercase tracking-wider">
                            {$t('account.service.name') || 'Nom du service'}
                        </th>
                    </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-gray-200">
                    {#each services as service (service.id)}
                        <tr class="hover:bg-gray-50 transition-colors">
                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                                {service.nom}
                            </td>
                        </tr>
                    {/each}
                    </tbody>
                </table>
            </div>
        </div>
        <div class="mt-2 text-xs text-gray-400 text-right">
            Total: {services.length} services
        </div>
    {/if}
</div>
