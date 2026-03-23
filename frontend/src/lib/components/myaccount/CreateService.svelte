<script lang="ts">
    import { enhance } from '$app/forms';
    import { page } from '$app/state';
    import { t } from 'svelte-i18n';

    let successMessage = $state('');
    let errorMessage = $state('');
    let submitted = $state(false);

    const form = $derived(page.form);

    $effect(() => {
        if (form?.actionType === 'create_service') {
            if (form?.success) {
                successMessage = $t(form.message || 'success.operation_success');
                errorMessage = '';
            } else if (form?.message) {
                errorMessage = $t(form.message);
                successMessage = '';
            }
        }
    });
</script>

<div class="flex flex-col gap-4">

    <h2 class="text-2xl font-bold py-2">{$t('account.service.title')}</h2>

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

    <div class="flex flex-col gap-4">

        <div class="border rounded-lg p-6 bg-white shadow-sm">
            <h3 class="text-lg font-bold mb-4">{$t('account.service.create_title')}</h3>

            <form
                    method="POST"
                    action="?/create_service"
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

                <div class="flex flex-col gap-2">
                    <label for="serviceName" class="text-sm font-semibold text-gray-700">{$t('account.service.name_label')}</label>
                    <input
                            id="serviceName"
                            name="serviceName"
                            type="text"
                            placeholder={$t('account.service.create_placeholder')}
                            class="px-4 py-2 border border-gray-200 rounded-lg text-gray-700 w-full focus:outline-none focus:ring-2 focus:ring-gs-green-950/20 focus:border-gs-green-950"
                            required
                    />
                </div>

                <button
                        type="submit"
                        disabled={submitted}
                        class="w-full sm:w-auto px-6 py-2 rounded-lg bg-gs-green-950 font-semibold text-white
                        cursor-pointer
                        hover:bg-gs-green-800
                        disabled:opacity-50 transition-colors"
                >
                    {#if submitted}Chargement...{:else}{$t('account.service.create_button')}{/if}
                </button>
            </form>
        </div>
    </div>
</div>
