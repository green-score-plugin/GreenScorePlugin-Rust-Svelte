<script lang="ts">
    import { enhance } from '$app/forms';
    import { t } from 'svelte-i18n';

    let organisationName = '', siret = '';
    let loading = false, submitted = false;

    $: errors = {
        organisationName: !organisationName.trim()
    };

    $: isFormValid = organisationName.trim() !== ''

    function handleSubmit(event: SubmitEvent) {
        submitted = true;

        if (!isFormValid) {
            event.preventDefault();
            loading = false;
            return;
        }
    }
</script>

<form method="POST" action="?/create_organization" on:submit={handleSubmit} use:enhance={() => {
    loading = true;
    return async ({ update }) => {
        await update();
        loading = false;
    };
}} class="flex flex-col gap-3 max-w-full overflow-hidden">

    <!-- Nom Organisation -->
    <div class="w-full text-grey-700 font-outfit font-semibold text-xs">
        <label for="organisationName" class="block mb-1">{$t('auth.register.org_name')}</label>
        <input bind:value={organisationName} id="organisation_name" type="text" name="organisationName"
               class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none {submitted && errors.organisationName ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder="Mon Organisation">
        {#if submitted && errors.organisationName} <span class="text-red-500 text-xs mt-0.5 block">{$t('auth.register.org_name_required')}</span> {/if}
    </div>

    <!-- SIRET (Optionnel) -->
    <div class="w-full text-grey-700 font-outfit font-semibold text-xs">
        <label for="siret" class="block mb-1">{$t('auth.register.siret')}</label>
        <input bind:value={siret} id="siret" type="text" name="siret"
               class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none border-grey-200" placeholder="123 456 789 00012">
    </div>

    <!-- Bouton -->
    <button type="submit" disabled={loading} class="w-full rounded-lg bg-gs-green-950 hover:bg-gs-green-800 transition-all duration-300 px-3 py-2 text-sm font-semibold font-outfit text-white cursor-pointer disabled:opacity-50 flex items-center justify-center gap-2">
        {#if loading} <span class="animate-spin h-4 w-4 border-2 border-white/20 border-t-white rounded-full"></span> {/if}
        {$t('auth.register.user_title')}
    </button>
</form>
