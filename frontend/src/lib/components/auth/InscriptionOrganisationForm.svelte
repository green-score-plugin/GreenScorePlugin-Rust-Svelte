<script lang="ts">
    import { enhance } from '$app/forms';
    import { t } from 'svelte-i18n';

    let organisationName = '', siret = '';
    let loading = false;
    let submitted = false;

    function formatSiret(value: string) {
        const raw = value.replace(/\D/g, '').substring(0, 14);
        const parts = [];
        if (raw.length > 0) parts.push(raw.substring(0, 3));
        if (raw.length > 3) parts.push(raw.substring(3, 6));
        if (raw.length > 6) parts.push(raw.substring(6, 9));
        if (raw.length > 9) parts.push(raw.substring(9, 14));
        return parts.join(' ');
    }

    function handleSiretInput(event: Event) {
        const input = event.target as HTMLInputElement;
        const formatted = formatSiret(input.value);
        siret = formatted;
        input.value = formatted;
    }

    $: cleanSiret = siret.replace(/\s/g, '');

    $: isSiretValid = cleanSiret === '' || /^\d{14}$/.test(cleanSiret);

    $: errors = {
        organisationName: !organisationName.trim(),
        siret: !isSiretValid
    };

</script>

<form method="POST" action="?/create_organization" use:enhance={({ formData, cancel }) => {
    submitted = true;

    const currentName = formData.get('organisationName')?.toString().trim() || '';
    const currentSiretValues = formData.get('siret')?.toString().replace(/\s/g, '') || '';

    const nameValid = currentName.length > 0;
    const siretValid = currentSiretValues === '' || /^\d{14}$/.test(currentSiretValues);

    if (!nameValid || !siretValid) {
        cancel();
        return;
    }

    if (currentSiretValues) {
        formData.set('siret', currentSiretValues);
    } else {
        formData.delete('siret');
    }

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
        <input value={siret} on:input={handleSiretInput} id="siret" type="text" name="siret"
               class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none {submitted && errors.siret ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder="123 456 789 00012">
        {#if submitted && errors.siret} <span class="text-red-500 text-xs mt-0.5 block">{$t('errors.validation_siret_format')}</span> {/if}
    </div>

    <!-- Bouton -->
    <button type="submit" disabled={loading} class="w-full rounded-lg bg-gs-green-950 hover:bg-gs-green-800 transition-all duration-300 px-3 py-2 text-sm font-semibold font-outfit text-white cursor-pointer disabled:opacity-50 flex items-center justify-center gap-2">
        {#if loading} <span class="animate-spin h-4 w-4 border-2 border-white/20 border-t-white rounded-full"></span> {/if}
        {$t('auth.register.user_title')}
    </button>
</form>
