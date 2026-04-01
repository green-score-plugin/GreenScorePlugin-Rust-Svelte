<script lang="ts">
    import { enhance } from '$app/forms';
    import { page } from '$app/state';
    import { t } from 'svelte-i18n';
    import { fade } from 'svelte/transition';

    let { organisationId } = $props();

    let successMessage = $state('');
    let errorMessage = $state('');
    let submitted = $state(false);

    const form = $derived(page.form);

    $effect(() => {
        if (form?.actionType === 'create_service') {
            if (form?.success) {
                successMessage = $t(form.message || 'success.operation_success');
                errorMessage = '';
                setTimeout(() => {
                    successMessage = '';
                }, 5000);
            } else if (form?.message) {
                errorMessage = $t(form.message);
                successMessage = '';
            }
        }
    });

    function cleanMessages() {
        successMessage = '';
        errorMessage = '';
    }
</script>

<div class="flex flex-col gap-6">

    <div class="flex flex-col gap-1">
        <h2 class="text-2xl font-bold font-outfit text-gray-900">{$t('account.service.title')}</h2>
        <p class="text-sm text-gray-500">{$t('account.service.description')}</p>
    </div>

    {#if successMessage}
        <div transition:fade class="flex items-center justify-between px-4 py-3 rounded-lg bg-green-50 border border-green-200 text-green-700 text-sm shadow-sm">
            <div class="flex items-center gap-2">
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="flex-shrink-0"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>
                <span>{successMessage}</span>
            </div>
            <button onclick={cleanMessages} class="text-green-800 hover:bg-green-100 p-1 rounded-full transition-colors" aria-label="Fermer">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
        </div>
    {/if}

    {#if errorMessage}
        <div transition:fade class="flex items-center justify-between px-4 py-3 rounded-lg bg-red-50 border border-red-200 text-red-700 text-sm shadow-sm">
            <div class="flex items-center gap-2">
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="flex-shrink-0"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="12"></line><line x1="12" y1="16" x2="12.01" y2="16"></line></svg>
                <span>{errorMessage}</span>
            </div>
            <button onclick={cleanMessages} class="text-red-800 hover:bg-red-100 p-1 rounded-full transition-colors" aria-label="Fermer">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
        </div>
    {/if}

    <div class="bg-white rounded-xl border border-gray-100 shadow-sm p-6 overflow-hidden relative transition-shadow hover:shadow-md">
        <!-- Decoration background -->
        <div class="absolute top-0 right-0 w-32 h-32 bg-gs-green-950/5 rounded-full blur-3xl -mr-16 -mt-16 pointer-events-none"></div>

        <h3 class="text-lg font-bold mb-4 font-outfit text-gray-800">{$t('account.service.create_title')}</h3>

        <form
                method="POST"
                action="?/create_service"
                use:enhance={() => {
                    submitted = true;
                    errorMessage = '';
                    successMessage = '';
                    return async ({ update }) => {
                        submitted = false;
                        await update();
                    };
                }}
                class="relative"
        >
            <input type="hidden" name="organisationId" value={organisationId} />

            <div class="flex flex-col sm:flex-row gap-4 items-end">
                <div class="w-full flex-grow">
                    <label class="block text-gray-700 text-sm font-medium mb-1.5" for="serviceName">
                        {$t('account.service.name_label')}
                    </label>
                    <input
                            id="serviceName"
                            name="serviceName"
                            type="text"
                            placeholder={$t('account.service.create_placeholder')}
                            class="w-full px-4 py-2.5 bg-gray-50 border border-gray-200 rounded-lg text-gray-900 focus:bg-white focus:outline-none focus:ring-2 focus:ring-gs-green-950/20 focus:border-gs-green-950 transition-all placeholder:text-gray-400"
                            required
                    />
                </div>

                <button
                        type="submit"
                        disabled={submitted}
                        class="w-full sm:w-auto h-[46px] px-6 rounded-lg bg-gs-green-950 text-white font-medium
                        shadow-sm hover:shadow hover:bg-gs-green-900
                        focus:ring-2 focus:ring-offset-2 focus:ring-gs-green-950
                        disabled:opacity-70 disabled:cursor-not-allowed
                        transition-all duration-200 flex items-center justify-center gap-2 min-w-[140px]"
                >
                    {#if submitted}
                         <svg class="animate-spin h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        <span>Chargement...</span>
                    {:else}
                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
                        <span>{$t('account.service.create_button')}</span>
                    {/if}
                </button>
            </div>
        </form>
    </div>
</div>
