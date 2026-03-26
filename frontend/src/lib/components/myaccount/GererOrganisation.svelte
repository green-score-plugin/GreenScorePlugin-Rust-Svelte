<script lang="ts">
    import { enhance } from '$app/forms';
    import CodeClipboard from "$lib/components/CodeClipboard.svelte";
    import { t } from 'svelte-i18n';
    import InscriptionOrganisationForm from "$lib/components/auth/InscriptionOrganisationForm.svelte";

    interface ActionData {
        success?: boolean;
        message?: string;
    }

    let successMessage = $state('');
    let errorMessage = $state('');
    let submitted = $state(false);
    let codeOrganisation = $state('');

    let hasLeftOrga = $state(false);
    let showConfirmModal = $state(false);
    let showChangeModal = $state(false);
    let showInscriptionOrga = $state(false);

    let { organisation } = $props();

    const handleLeaveOrga = () => {
        return async ({ result, update }: { result: any, update: any }) => {
            if (result.type === 'success' || result.type === 'failure') {
                const data = result.data as ActionData;
                if (data) {
                    if (data.success) {
                        showConfirmModal = false;
                        hasLeftOrga = true;
                        successMessage = $t(data.message || 'success.operation_success');
                        errorMessage = '';
                    } else if (data.message) {
                        errorMessage = $t(data.message);
                        successMessage = '';
                        showConfirmModal = false;
                    }
                }
            }
            await update();
        };
    };

    const handleChangeOrga = () => {
        submitted = true;
        return async ({ result, update }: { result: any, update: any }) => {
            if (result.type === 'success' || result.type === 'failure') {
                const data = result.data as ActionData;
                if (data) {
                    if (data.success) {
                        codeOrganisation = '';
                        showChangeModal = false;
                        hasLeftOrga = false;
                        successMessage = $t(data.message || 'success.operation_success');
                        errorMessage = '';
                    } else if (data.message) {
                        errorMessage = $t(data.message);
                        successMessage = '';
                        showChangeModal = false;
                    }
                }
            }
            await update();
            submitted = false;
        };
    };

    const handleJoinOrga = () => {
        submitted = true;
        errorMessage = '';
        successMessage = '';

        return async ({ result, update }: { result: any, update: any }) => {
            if (result.type === 'success' || result.type === 'failure') {
                const data = result.data as ActionData;
                if (data) {
                    if (data.success) {
                        codeOrganisation = '';
                        hasLeftOrga = false;
                        successMessage = $t(data.message || 'success.operation_success');
                        errorMessage = '';
                    } else if (data.message) {
                        errorMessage = $t(data.message);
                        successMessage = '';
                    }
                }
            }
            await update();
            submitted = false;
        };
    };

</script>

<div class="flex flex-col gap-4">

    <h2 class="text-2xl font-bold py-2">{$t('account.organization.title')}</h2>

    {#if successMessage}
        <div class="px-4 py-3 rounded-lg bg-green-50 border border-green-200 text-green-700 text-sm">
            {successMessage}
        </div>
    {/if}

    {#if errorMessage}
        <div class="px-4 py-3 rounded-lg bg-red-50 border border-red-200 text-red-700 text-sm">
            {errorMessage}
        </div>
    {/if}

    {#if organisation != null}
        <div class="flex flex-col gap-6">
                <div class="flex flex-col gap-1">
                    <p class="text-xs font-semibold uppercase text-grey-500">{$t('account.organization.name_label')}</p>
                    <div class="text-xl font-bold text-gs-green-950">{organisation.nom}</div>
                </div>

                <div class="w-full flex flex-col gap-2">
                    <label for="codeDisplay" class="text-sm font-semibold text-grey-700">{$t('account.organization.code_label')}</label>
                    <CodeClipboard code={organisation.code || $t('account.organization.code_unknown')} />
                </div>

                <div class="flex gap-3 pt-2">
                    <button
                            type="button"
                            onclick={() => showConfirmModal = true}
                            class="flex-1 px-4 py-2 rounded-lg bg-red-600 text-white hover:bg-red-700 font-semibold transition-colors cursor-pointer"
                    >
                        {$t('account.organization.leave_button')}
                    </button>

                    <button
                            type="button"
                            onclick={() => showChangeModal = true}
                            class="flex-1 px-4 py-2 rounded-lg bg-gs-green-950 text-white hover:bg-gs-green-800 font-semibold transition-colors cursor-pointer"
                    >
                        {$t('account.organization.change_button')}
                    </button>
                </div>
            </div>

            {#if showConfirmModal}
                <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70">
                    <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4 shadow-lg max-h-[90vh] overflow-auto">
                        <h2 class="text-xl font-semibold mb-4">{$t('account.modals.leave_org_confirm_title')}</h2>
                        <p class="text-gray-600 mb-6">
                            {$t('account.modals.leave_org_confirm_desc', { values: { orgName: organisation.nom || 'Inconnue' } })}
                        </p>
                        <div class="flex justify-end gap-4">
                            <button
                                    type="button"
                                    onclick={() => showConfirmModal = false}
                                    class="px-4 py-2 rounded-lg bg-red-600 text-white hover:bg-red-700 cursor-pointer transition"
                            >
                                {$t('account.modals.cancel')}
                            </button>

                            <form
                                    action="?/leave_orga"
                                    method="POST"
                                    use:enhance={handleLeaveOrga}
                            >
                                <input type="hidden" name="organisationId" value={organisation.id} />
                                <button
                                        type="submit"
                                        class="px-4 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700 cursor-pointer transition"
                                >
                                    {$t('account.modals.confirm_leave')}
                                </button>
                            </form>
                        </div>
                    </div>
                </div>
            {/if}

            {#if showChangeModal}
                <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70">
                    <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4 shadow-lg max-h-[90vh] overflow-auto">
                        <h2 class="text-xl font-semibold mb-4">{$t('account.modals.change_org_title')}</h2>
                        <form
                                action="?/change_orga"
                                method="POST"
                                use:enhance={handleChangeOrga}
                                class="flex flex-col gap-4"
                        >
                            <div class="flex flex-col gap-2">
                                <label for="newCodeOrganisation" class="text-sm font-semibold text-grey-700">{$t('account.organization.code_label')}</label>
                                <input
                                        id="newCodeOrganisation"
                                        name="codeOrganisation"
                                        type="text"
                                        bind:value={codeOrganisation}
                                        placeholder={$t('account.modals.new_code_placeholder')}
                                        class="px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none"
                                />
                                <p class="text-xs text-gray-500">{$t('account.modals.new_code_help')}</p>
                            </div>
                            <div class="flex justify-end gap-4">
                                <button
                                        type="button"
                                        onclick={() => {
                                            showChangeModal = false;
                                            codeOrganisation = '';
                                        }}
                                        class="px-4 py-2 rounded-lg bg-red-600 text-white hover:bg-red-700 cursor-pointer transition"
                                >
                                    {$t('account.modals.cancel')}
                                </button>
                                <button
                                        type="submit"
                                        disabled={submitted}
                                        class="px-4 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700 cursor-pointer transition disabled:opacity-50"
                                >
                                    {#if submitted}{$t('account.organization.join_loading')}{:else}{$t('account.modals.confirm')}{/if}
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            {/if}

    {:else}
        {#if showInscriptionOrga}
            <InscriptionOrganisationForm />
        {:else}
            <form
                    method="POST"
                    action="?/join_orga"
                    use:enhance={handleJoinOrga}
                    class="flex flex-col gap-4">

                <div class="flex gap-4 w-full text-grey-700 font-outfit font-semibold text-sm sm:flex-row">
                    <div class="w-full flex flex-col">
                        <label for="codeOrganisation">{$t('account.organization.code_label')}</label>
                        <input
                                id="codeOrganisation"
                                name="codeOrganisation"
                                type="text"
                                bind:value={codeOrganisation}
                                placeholder={$t('account.organization.code_placeholder')}
                                class="px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none"
                        />
                        <p class="text-xs text-gray-500 mt-1">{$t('account.organization.code_help')}</p>
                    </div>
                </div>

                <button
                        type="submit"
                        disabled={submitted}
                        class="w-full h-fit rounded-lg bg-gs-green-950 px-1 py-2 font-semibold font-outfit text-white
                        cursor-pointer
                        hover:bg-gs-green-800
                        active:bg-gs-green-700
                        transition-colors duration-150 ease-in-out disabled:opacity-50"
                >
                    {#if submitted}{$t('account.organization.join_loading')}{:else}{$t('account.organization.join_button')}{/if}
                </button>

                <div class="flex items-center">
                    <hr class="grow border-grey-200" />
                    <span class="text-xs mx-2 text-grey-500 font-semibold">{$t('account.organization.or')}</span>
                    <hr class="grow border-grey-200" />
                </div>

                <button
                    type="button"
                    class="w-full h-fit rounded-lg bg-blue-600 px-1 py-2 font-semibold font-outfit text-white
                    cursor-pointer
                    hover:bg-blue-700
                    active:bg-blue-800
                    transition-colors duration-150 ease-in-out"
                    onclick={() => { showInscriptionOrga = true; }}
                >
                    {$t('account.organization.create_button')}
                </button>
            </form>
        {/if}
    {/if}
</div>
