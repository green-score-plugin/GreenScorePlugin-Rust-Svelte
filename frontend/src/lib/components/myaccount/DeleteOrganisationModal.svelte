<script lang="ts">
    import { t } from 'svelte-i18n';

    let { isOpen, onClose, onConfirm, organisation } = $props();
</script>

{#if isOpen}
    <div
            class="fixed flex inset-0 bg-black/50 items-center justify-center z-[1000]"
            role="dialog"
            aria-modal="true"
    >
        <div class="bg-white rounded-lg p-6 max-w-md mx-auto shadow-xl animate-in fade-in zoom-in duration-200">
            <h2 class="text-xl font-semibold mb-4">{$t('account.delete_org.title', { default: 'Supprimer l\'organisation' })}</h2>

            <p class="text-gray-600 mb-6">
                {$t('account.delete_org.confirmation', {values: { orgName: organisation?.nom }, default: `Êtes-vous sûr de vouloir supprimer l'organisation ${organisation?.nom} ?` })}
                <br>
                <span class="text-red-500 font-bold text-sm block mt-2">
                    {$t('account.delete_org.warning', { default: 'Cette action est irréversible et supprimera toutes les données associées.' })}
                </span>
            </p>

            <div class="flex justify-end gap-4">
                <button
                        onclick={onClose}
                        class="hover:cursor-pointer px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg transition-colors font-medium border border-gray-200"
                >
                    {$t('common.cancel', { default: 'Annuler' })}
                </button>

                <button
                        onclick={onConfirm}
                        class="hover:cursor-pointer px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors font-medium shadow-sm"
                >
                    {$t('account.modals.delete')}
                </button>
            </div>
        </div>
    </div>
{/if}
