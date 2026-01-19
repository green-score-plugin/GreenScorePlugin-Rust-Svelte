<script lang="ts">
    import validator from 'validator';
    import { enhance } from '$app/forms';

    let organisationName = '';
    let siret = '';
    let email = '';
    let password = '';
    let confirmPassword = '';
    let agreeTerms = false;

    let loading = false;
    let submitted = false;

    $: emailValid = !submitted || validator.isEmail(email);
    $: passwordValid = !submitted || password.length >= 8;
    $: confirmPasswordValid = !submitted || (confirmPassword.trim() !== '' && confirmPassword === password);
    $: organisationNameValid = !submitted || organisationName.trim() !== '';
    $: agreeTermsValid = !submitted || agreeTerms;

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

    <div class="w-full text-grey-700 font-outfit font-semibold text-xs">
        <label for="organisationName" class="block mb-1">Nom de l'organisation</label>
        <input
                bind:value={organisationName}
                id="organisationName"
                type="text"
                name="organisationName"
                class="px-3 py-1.5 text-sm border rounded-lg text-grey-700 w-full focus:outline-none {!organisationNameValid ? 'border-red-700 bg-red-50' : 'border-grey-200'}"
                placeholder="Mon Organisation"
        >
        {#if !organisationNameValid}
            <span class="text-red-500 text-xs mt-0.5 block">Veuillez entrer un nom.</span>
        {/if}
    </div>

    <div class="w-full text-grey-700 font-outfit font-semibold text-xs">
        <label for="siret" class="block mb-1">Numéro SIRET (optionnel)</label>
        <input
                bind:value={siret}
                id="siret"
                type="text"
                name="siret"
                class="px-3 py-1.5 text-sm border rounded-lg text-grey-700 w-full focus:outline-none border-grey-200"
                placeholder="123 456 789 00012"
        >
    </div>

    <div class="w-full text-grey-700 font-outfit font-semibold text-xs">
        <label for="inputEmail" class="block mb-1">Email</label>
        <input
                bind:value={email}
                type="email"
                name="email"
                id="inputEmail"
                class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none {!emailValid ? 'border-red-700 bg-red-50' : 'border-grey-200'}"
                placeholder="john.doe@example.com"
        >
        {#if !emailValid}
            <span class="text-red-500 text-xs mt-0.5 block">Email invalide</span>
        {/if}
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <div class="text-grey-700 font-outfit font-semibold text-xs">
            <label for="inputPassword" class="block mb-1">Mot de passe</label>
            <input
                    bind:value={password}
                    type="password"
                    name="password"
                    id="inputPassword"
                    class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none {!passwordValid ? 'border-red-700 bg-red-50' : 'border-grey-200'}"
                    placeholder="••••••••"
            >
            {#if !passwordValid}
                <span class="text-red-500 text-xs mt-0.5 block">Le mot de passe doit contenir au moins 8 caractères</span>
            {/if}
        </div>

        <div class="text-grey-700 font-outfit font-semibold text-xs">
            <label for="confirmPassword" class="block mb-1">Confirmation</label>
            <input
                    bind:value={confirmPassword}
                    id="confirmPassword"
                    type="password"
                    name="confirmPassword"
                    class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none {!confirmPasswordValid ? 'border-red-700 bg-red-50' : 'border-grey-200'}"
                    placeholder="••••••••"
            >
            {#if !confirmPasswordValid}
                <span class="text-red-500 text-xs mt-0.5 block">
                    {confirmPassword.trim() === '' ? 'Veuillez confirmer votre mot de passe.' : 'Les mots de passe ne correspondent pas.'}
                </span>
            {/if}
        </div>
    </div>

    <div class="text-grey-700 font-semibold text-xs font-outfit">
        <div class="flex gap-2 items-start">
            <input
                    bind:checked={agreeTerms}
                    id="agreeTerms"
                    type="checkbox"
                    name="agreeTerms"
                    class="w-3.5 h-3.5 mt-0.5 rounded border-gray-300 text-gs-green-950 focus:ring-0 focus:ring-offset-0 accent-gs-green-950 cursor-pointer flex-shrink-0"
            >
            <label for="agreeTerms" class="leading-tight text-xs">
                En vous inscrivant sur GreenScoreWeb, vous acceptez nos conditions générales d'utilisation.
            </label>
        </div>
        {#if !agreeTermsValid}
            <span class="text-red-500 text-xs mt-0.5 block ml-5">Vous devez accepter nos conditions générales d'utilisation</span>
        {/if}
    </div>

    <button
            type="submit"
            disabled={loading}
            class="w-full rounded-lg bg-gs-green-950 hover:bg-gs-green-800 transition-all duration-300 px-3 py-2 text-sm font-semibold font-outfit text-white cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
    >
        {#if loading}
            <svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" aria-hidden="true">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Inscription...
        {:else}
            Inscription
        {/if}
    </button>

    <a href="/inscription" class="text-grey-950 font-outfit font-semibold text-xs text-center hover:underline">
        Je suis un utilisateur
    </a>
</form>
