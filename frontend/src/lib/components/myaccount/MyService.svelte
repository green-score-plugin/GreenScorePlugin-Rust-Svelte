<script lang="ts">
    import { t } from 'svelte-i18n';
    import { enhance } from '$app/forms';
    import type { SubmitFunction } from '@sveltejs/kit';

    let { services = [], organisationId, members = [], currentUserId } = $props();
    let scrollContainer: HTMLDivElement | undefined = $state();

    let showDeleteModal = $state(false);
    let serviceToDelete: any = $state(null);
    let deleteErrorMessage = $state('');

    let showAssignModal = $state(false);
    let serviceToAssign: any = $state(null);
    let assignErrorMessage = $state('');
    let assignSuccessMessage = $state('');

    const handleDelete: SubmitFunction = () => {
        return async ({ result, update }) => {
            if (result.type === 'success') {
                showDeleteModal = false;
                serviceToDelete = null;
                await update();
            } else if (result.type === 'failure') {
                 // @ts-ignore
                deleteErrorMessage = $t(result.data?.message || 'errors.unknown_error');
            } else {
                 await update();
            }
        };
    };

    const handleAssign: SubmitFunction = () => {
        return async ({ result, update }) => {
            if (result.type === 'success') {
                showAssignModal = false;
                serviceToAssign = null;
                assignSuccessMessage = $t('success.user_assigned');
                // Auto hide success message
                setTimeout(() => assignSuccessMessage = '', 4000);
                await update();
            } else if (result.type === 'failure') {
                // @ts-ignore
                assignErrorMessage = $t(result.data?.message || 'errors.unknown_error');
            } else {
                await update();
            }
        };
    };

    let availableMembers = $derived(
        members.filter((m: any) =>
            (!serviceToAssign || m.service_id !== serviceToAssign?.id) &&
            m.id !== currentUserId
        )
            .sort((a: any, b: any) => a.nom.localeCompare(b.nom))
    );

    function stopPropagation(e: Event) {
        e.stopPropagation();
    }
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

    <!-- Success Message Display -->
    {#if assignSuccessMessage}
        <div class="mb-4 flex items-center justify-between px-4 py-3 rounded-lg bg-green-50 border border-green-200 text-green-700 text-sm shadow-sm">
            <span>{assignSuccessMessage}</span>
            <button onclick={() => assignSuccessMessage = ''} class="text-green-800 hover:bg-green-100 p-1 rounded-full" aria-label="Fermer">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
        </div>
    {/if}

    <!-- Error Message Display -->
    {#if deleteErrorMessage}
        <div class="mb-4 flex items-center justify-between px-4 py-3 rounded-lg bg-red-50 border border-red-200 text-red-700 text-sm shadow-sm">
            <span>{deleteErrorMessage}</span>
            <button onclick={() => deleteErrorMessage = ''} class="text-red-800 hover:bg-red-100 p-1 rounded-full" aria-label="Fermer">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
        </div>
    {/if}
    {#if assignErrorMessage}
        <div class="mb-4 flex items-center justify-between px-4 py-3 rounded-lg bg-red-50 border border-red-200 text-red-700 text-sm shadow-sm">
            <span>{assignErrorMessage}</span>
            <button onclick={() => assignErrorMessage = ''} class="text-red-800 hover:bg-red-100 p-1 rounded-full" aria-label="Fermer">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
        </div>
    {/if}

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
                    <tbody>
                    {#each services as service (service.id)}
                        <tr class="group hover:bg-gray-50/80 transition-colors">
                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 flex justify-between items-center group">
                                <div class="flex items-center gap-3">
                                    <div class="w-8 h-8 rounded-full bg-gray-100 text-gray-600 flex items-center justify-center text-xs font-bold uppercase">
                                        {service.nom.substring(0, 2)}
                                    </div>
                                    {service.nom}
                                </div>
                                <div class="flex items-center gap-2 opacity-0 group-hover:opacity-100 focus-within:opacity-100 transition-opacity">
                                    <button
                                        class="text-gray-400 hover:text-green-600 hover:bg-green-50 p-2 rounded-full transition-all duration-200 cursor-pointer"
                                        title={$t('account.service.assign_member') || "Assigner un membre"}
                                        aria-label={$t('account.service.assign_member') || "Assigner un membre"}
                                        onclick={() => { serviceToAssign = service; showAssignModal = true; assignErrorMessage = ''; }}
                                    >
                                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="8.5" cy="7" r="4"></circle><line x1="20" y1="8" x2="20" y2="14"></line><line x1="23" y1="11" x2="17" y2="11"></line></svg>
                                    </button>
                                    <button
                                        class="text-gray-400 hover:text-red-600 hover:bg-red-50 p-2 rounded-full transition-all duration-200 cursor-pointer"
                                        title={$t('account.service.delete_service') || "Supprimer le service"}
                                        aria-label={$t('account.service.delete_service') || "Supprimer le service"}
                                        onclick={() => { serviceToDelete = service; showDeleteModal = true; deleteErrorMessage = ''; }}
                                    >
                                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
                                    </button>
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

{#if showAssignModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70">
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <div class="bg-white rounded-xl p-6 max-w-sm w-full mx-4 shadow-xl" onclick={stopPropagation}>
            <div class="text-center mb-6">
                 <div class="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4 text-green-600">
                    <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="8.5" cy="7" r="4"></circle><line x1="20" y1="8" x2="20" y2="14"></line><line x1="23" y1="11" x2="17" y2="11"></line></svg>
                </div>
                <h3 class="text-xl font-bold text-gray-900 mb-2">{$t('account.service.assign_modal.title') || "Assigner un membre"}</h3>
                <p class="text-gray-600 text-sm">
                    {$t('account.service.assign_modal.description_prefix') || "Sélectionnez un membre à ajouter au service"} <span class="font-bold">"{serviceToAssign?.nom}"</span>.
                </p>
            </div>

            <form
                    method="POST"
                    action="?/assign_user_service"
                    use:enhance={handleAssign}
                    class="flex flex-col gap-4"
            >
                <input type="hidden" name="serviceId" value={serviceToAssign?.id} />
                <input type="hidden" name="organisationId" value={organisationId} />

                <div class="flex flex-col gap-1 text-left">
                    <label for="userId" class="text-sm font-medium text-gray-700 cursor-pointer">{$t('account.service.assign_modal.member_label') || "Membre"}</label>
                    <div class="relative hover:opacity-80 transition-opacity">
                        <select
                            name="userId"
                            id="userId"
                            required
                            class="cursor-pointer appearance-none bg-none w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-gs-green-950 focus:border-gs-green-950 sm:text-sm rounded-lg border bg-white hover:border-gray-400 transition-colors"
                        >
                            <option value="" disabled selected>{$t('account.service.assign_modal.select_placeholder') || "Choisir un membre..."}</option>
                            {#each availableMembers as member}
                                <option value={member.id}>
                                    {member.prenom} {member.nom}
                                    {member.service_id ? ($t('account.service.assign_modal.move_suffix') || '(Déplacer)') : ''}
                                </option>
                            {/each}
                        </select>
                         <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                             <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
                         </div>
                    </div>
                </div>

                <div class="flex gap-3 justify-center mt-4">
                    <button
                            type="button"
                            onclick={() => { showAssignModal = false; serviceToAssign = null; }}
                            class="px-5 py-2.5 rounded-lg border border-gray-200 text-gray-700 font-medium hover:bg-gray-50 transition-colors w-full cursor-pointer"
                    >
                        {$t('common.cancel') || "Annuler"}
                    </button>
                    <button
                            type="submit"
                            class="px-5 py-2.5 rounded-lg bg-gs-green-950 text-white font-medium hover:bg-gs-green-800 shadow-sm transition-colors w-full cursor-pointer"
                    >
                        {$t('common.validate') || "Valider"}
                    </button>
                </div>
            </form>
        </div>
    </div>
{/if}

{#if showDeleteModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70">
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <div class="bg-white rounded-lg p-6 max-w-sm w-full mx-4 shadow-lg text-center" onclick={stopPropagation}>
            <svg class="w-16 h-16 text-yellow-500 mx-auto mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
            </svg>
            <h2 class="text-xl font-bold mb-2">{$t('common.warning') || "Attention"}</h2>
            <p class="text-gray-600 mb-6">
                {$t('account.service.delete_modal.confirmation_prefix') || "Voulez-vous vraiment supprimer le service"} <span class="font-bold">"{serviceToDelete?.nom}"</span> ?
                <br><br>
                {$t('account.service.delete_modal.warning_detail') || "Cette action supprimera également le lien entre ce service et les utilisateurs associés."}
            </p>

            <div class="flex justify-center gap-4">
                <button
                    onclick={() => { showDeleteModal = false; serviceToDelete = null; }}
                    class="px-4 py-2 rounded-lg bg-gray-200 text-gray-800 hover:bg-gray-300 font-semibold cursor-pointer"
                >
                    {$t('common.cancel') || "Annuler"}
                </button>

                <form
                    method="POST"
                    action="?/delete_service"
                    use:enhance={handleDelete}
                >
                    <input type="hidden" name="serviceId" value={serviceToDelete?.id} />
                    <input type="hidden" name="organisationId" value={organisationId} />
                    <button
                        type="submit"
                        class="px-4 py-2 rounded-lg bg-red-600 text-white hover:bg-red-700 font-semibold cursor-pointer"
                    >
                        {$t('common.confirm') || "Confirmer"}
                    </button>
                </form>
            </div>
        </div>
    </div>
{/if}
