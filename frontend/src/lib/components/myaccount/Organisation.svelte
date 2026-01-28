<script lang="ts">
    import { enhance } from '$app/forms';
    import { page } from '$app/stores';
    import { invalidateAll } from '$app/navigation';
    import CodeClipboard from "$lib/components/CodeClipboard.svelte";
    import { t } from 'svelte-i18n';

    export const form: { message?: string, success?: boolean } | null = null;

    let successMessage = '';
    let errorMessage = '';
    let submitted = false;
    let codeOrganisation = '';

    let hasLeftOrga = false;
    let showConfirmModal = false;
    let showChangeModal = false;

    $: user = $page.data.user;
    $: hasOrga = (!!user?.id_orga && !hasLeftOrga);

    $: orgaDetails = $page.data.organisation || null;

    $: {
        if ($page.form?.actionType === 'join_orga' || $page.form?.actionType === 'leave_orga' || $page.form?.actionType === 'change_orga') {
            if ($page.form?.success) {
                successMessage = $t($page.form.message || 'success.operation_success');
                errorMessage = '';
                codeOrganisation = '';

                if ($page.form.actionType === 'leave_orga') {
                    hasLeftOrga = true;
                    orgaDetails = null;
                } else if ($page.form.actionType === 'join_orga' || $page.form.actionType === 'change_orga') {
                    hasLeftOrga = false;
                }

                invalidateAll();
            } else if ($page.form?.message) {
                errorMessage = $t($page.form.message);
                successMessage = '';
            }
        }
    }
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

    {#if hasOrga}
        <div class="flex flex-col gap-6">
                <div class="flex flex-col gap-1">
                    <p class="text-xs font-semibold uppercase text-grey-500">{$t('account.organization.name_label')}</p>
                    <div class="text-xl font-bold text-gs-green-950">{orgaDetails?.name}</div>
                </div>

                <div class="w-full flex flex-col gap-2">
                    <label for="codeDisplay" class="text-sm font-semibold text-grey-700">{$t('account.organization.code_label')}</label>
                    <CodeClipboard code={orgaDetails?.code || $t('account.organization.code_unknown')} />
                </div>

                <div class="flex gap-3 pt-2">
                    <button
                            type="button"
                            on:click={() => showConfirmModal = true}
                            class="flex-1 px-4 py-2 rounded-lg bg-red-600 text-white hover:bg-red-700 font-semibold transition-colors cursor-pointer"
                    >
                        {$t('account.organization.leave_button')}
                    </button>

                    <button
                            type="button"
                            on:click={() => showChangeModal = true}
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
                            {$t('account.modals.leave_org_confirm_desc', { values: { orgName: orgaDetails?.name || 'Inconnue' } })}
                        </p>
                        <div class="flex justify-end gap-4">
                            <button
                                    type="button"
                                    on:click={() => showConfirmModal = false}
                                    class="px-4 py-2 rounded-lg bg-red-600 text-white hover:bg-red-700 cursor-pointer transition"
                            >
                                {$t('account.modals.cancel')}
                            </button>

                            <form
                                    action="?/leave_orga"
                                    method="POST"
                                    use:enhance={() => {
                                    showConfirmModal = false;
                                    return async ({ update }) => {
                                        await update();
                                    };
                                }}
                            >
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
                                use:enhance={() => {
                                    submitted = true;
                                    return async ({ update }) => {
                                        await update();
                                        submitted = false;
                                        showChangeModal = false;
                                    };
                                }}
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
                                        on:click={() => {
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
        <form
                method="POST"
                action="?/join_orga"
                use:enhance={() => {
                    submitted = true;
                    errorMessage = '';
                    successMessage = '';
                    return async ({ update }) => {
                        await update();
                        submitted = false;
                    };
                }}
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
        </form>
    {/if}
</div>
