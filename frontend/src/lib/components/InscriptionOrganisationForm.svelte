<script lang="ts">
    import validator from 'validator';
    import { enhance } from '$app/forms';
    import { t } from 'svelte-i18n';

    let organisationName = '', siret = '', email = '', password = '', confirmPassword = '';
    let agreeTerms = false, loading = false, submitted = false;

    const passwordRegex = /^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$%^&*-]).{8,}$/;

    $: errors = {
        organisationName: !organisationName.trim(),
        email: !validator.isEmail(email),
        password: !passwordRegex.test(password),
        confirmPassword: confirmPassword.trim() === '' || confirmPassword !== password,
        agreeTerms: !agreeTerms
    };

    $: isFormValid =
        validator.isEmail(email) &&
        password.length >= 8 &&
        confirmPassword.trim() !== '' &&
        confirmPassword === password &&
        organisationName.trim() !== '' &&
        agreeTerms;

    function handleSubmit(event: SubmitEvent) {
        submitted = true;

        if (!isFormValid) {
            event.preventDefault();
            loading = false;
            return;
        }
    }
</script>

<form method="POST" on:submit={handleSubmit} use:enhance={() => {
    loading = true;
    return async ({ update }) => {
        await update();
        loading = false;
    };
}} class="flex flex-col gap-3 max-w-full overflow-hidden">

    <!-- Nom Organisation -->
    <div class="w-full text-grey-700 font-outfit font-semibold text-xs">
        <label for="organisationName" class="block mb-1">{$t('auth.register.org_name')}</label>
        <input bind:value={organisationName} id="organisationName" type="text" name="organisationName"
               class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none {submitted && errors.organisationName ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder="Mon Organisation">
        {#if submitted && errors.organisationName} <span class="text-red-500 text-xs mt-0.5 block">{$t('auth.register.org_name_required')}</span> {/if}
    </div>

    <!-- SIRET (Optionnel) -->
    <div class="w-full text-grey-700 font-outfit font-semibold text-xs">
        <label for="siret" class="block mb-1">{$t('auth.register.siret')}</label>
        <input bind:value={siret} id="siret" type="text" name="siret"
               class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none border-grey-200" placeholder="123 456 789 00012">
    </div>

    <!-- Email -->
    <div class="w-full text-grey-700 font-outfit font-semibold text-xs">
        <label for="inputEmail" class="block mb-1">{$t('auth.login.email_label')}</label>
        <input bind:value={email} type="email" name="email" id="inputEmail"
               class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none {submitted && errors.email ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder={$t('auth.login.email_placeholder')}>
        {#if submitted && errors.email} <span class="text-red-500 text-xs mt-0.5 block">{$t('auth.login.invalid_email')}</span> {/if}
    </div>

    <!-- Mots de passe -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <div class="text-grey-700 font-outfit font-semibold text-xs">
            <label for="inputPassword" class="block mb-1">{$t('auth.login.password_label')}</label>
            <input bind:value={password} type="password" name="password" id="inputPassword"
                   class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none {submitted && errors.password ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder={$t('auth.login.password_placeholder')}>
            {#if submitted && errors.password}
                <div class="text-red-500 text-xs mt-1">
                    <ul class="list-disc list-inside space-y-0.5 ml-1">
                        <li>{$t('auth.login.req_length')}</li>
                        <li>{$t('auth.login.req_uppercase')}</li>
                        <li>{$t('auth.login.req_lowercase')}</li>
                        <li>{$t('auth.login.req_number')}</li>
                        <li>{$t('auth.login.req_special')}</li>
                    </ul>
                </div>
            {/if}
        </div>
        <div class="text-grey-700 font-outfit font-semibold text-xs">
            <label for="confirmPassword" class="block mb-1">{$t('auth.register.confirm_password')}</label>
            <input bind:value={confirmPassword} id="confirmPassword" type="password" name="confirmPassword"
                   class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none {submitted && errors.confirmPassword ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder={$t('auth.login.password_placeholder')}>
            {#if submitted && errors.confirmPassword} <span class="text-red-500 text-xs mt-0.5 block">{$t('auth.register.passwords_mismatch')}</span> {/if}
        </div>
    </div>

    <!-- CGU -->
    <div class="text-grey-700 font-semibold text-xs font-outfit">
        <div class="flex gap-2 items-start">
            <input bind:checked={agreeTerms} id="agreeTerms" type="checkbox" name="agreeTerms" class="w-3.5 h-3.5 mt-0.5 rounded border-gray-300 accent-gs-green-950 cursor-pointer flex-shrink-0">
            <label for="agreeTerms" class="leading-tight text-xs">{$t('auth.register.agree_terms')}</label>
        </div>
        {#if submitted && errors.agreeTerms} <span class="text-red-500 text-xs mt-0.5 block ml-5">{$t('auth.register.accept_terms_error')}</span> {/if}
    </div>

    <!-- Bouton -->
    <button type="submit" disabled={loading} class="w-full rounded-lg bg-gs-green-950 hover:bg-gs-green-800 transition-all duration-300 px-3 py-2 text-sm font-semibold font-outfit text-white cursor-pointer disabled:opacity-50 flex items-center justify-center gap-2">
        {#if loading} <span class="animate-spin h-4 w-4 border-2 border-white/20 border-t-white rounded-full"></span> {/if}
        {$t('auth.register.user_title')}
    </button>

    <a href="/inscription" class="text-grey-950 font-outfit font-semibold text-xs text-center hover:underline">{$t('auth.register.switch_to_user')}</a>
</form>
