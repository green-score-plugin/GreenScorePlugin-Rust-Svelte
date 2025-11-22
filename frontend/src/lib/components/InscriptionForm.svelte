<script lang="ts">
    import validator from 'validator';
    import { enhance } from '$app/forms';

    let firstname = '';
    let lastname = '';
    let email = '';
    let password = '';
    let confirmPassword = '';
    let agreeTerms = false;

    let loading = false;
    let emailValid = true;
    let passwordValid = true;
    let confirmPasswordValid = true;

    $: firstname;
    $: lastname;
    $: email && (emailValid = validator.isEmail(email));
    $: password && (passwordValid = password.length >= 8);
    $: confirmPassword && (confirmPasswordValid = confirmPassword === password);
    $: agreeTerms;

</script>

<form method="POST" use:enhance={() => {
    loading = true;
    return async ({ update }) => {
        await update();
        loading = false;
    };
}} class="flex flex-col gap-4">


    <div class="flex flex-col md:flex-row gap-4 w-full text-grey-700 font-outfit font-semibold text-sm sm:flex-row">
        <div class="w-full">
            <label for="firstname">Prénom</label>
            <input
                bind:value={firstname}
                id="firstname" type="text" name="firstname"
                class="px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none"
                placeholder="John"
            >
        </div>
        <div class="w-full">
            <label for="lastname">Nom</label>
            <input
                bind:value={lastname}
                id="lastname" type="text" name="lastname"
                class="px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none"
                placeholder="Doe"
            >
        </div>
    </div>


    <div class="w-full text-grey-700 font-outfit font-semibold text-sm">
        <label for="inputEmail">Email</label>
        <input
                bind:value={email}
                type="email"
                name="email"
                id="inputEmail"
                class="form-control px-4 py-2 border rounded-lg w-full focus:outline-none {!emailValid ? 'border-red-700 bg-red-50' : 'border-grey-200'}"
                placeholder="john.doe@example.com"
        >
        {#if !emailValid}
            <span class="text-red-500 text-sm">Email invalide</span>
        {/if}
    </div>

    <div class="flex flex-col md:flex-row gap-4 w-full text-grey-700 font-outfit font-semibold text-sm sm:flex-row">
        <div class="w-full">
            <label for="inputPassword">Mot de passe</label>
            <input
                    bind:value={password}
                    type="password"
                    name="password"
                    id="inputPassword"
                    class="form-control px-4 py-2 border rounded-lg w-full focus:outline-none {!passwordValid ? 'border-red-700 bg-red-50' : 'border-grey-200'}"
                    placeholder="••••••••"
            >
            {#if !passwordValid}
                <span class="text-red-500 text-sm">Le mot de passe doit contenir au moins 8 caractères</span>
            {/if}
        </div>
        <div class="w-full">
            <label for="confirmPassword">Confirmation du mot de passe</label>
            <input
                bind:value={confirmPassword}
                id="confirmPassword" type="text" name="confirmPassword"
                class="px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none"
                placeholder="••••••••"
            >
        </div>
    </div>

    <div class="flex flex-col gap-2 text-grey-700 font-semibold text-sm font-outfit">
        <div class="flex gap-2 items-start">
            <input
                bind:checked={agreeTerms}
                id="agreeTerms" type="checkbox" name="agreeTerms"
                class="w-4 h-4 mt-1 rounded border-gray-300 text-gs-green-950 focus:ring-0 focus:ring-offset-0 accent-gs-green-950 cursor-pointer"
            >
            <label for="agreeTerms">
                En vous inscrivant sur GreenScoreWeb, vous acceptez nos conditions générales d’utilisation.
            </label>
        </div>
    </div>

    <button type="submit" disabled={loading} class="w-full h-fit rounded-lg bg-gs-green-950 hover:bg-gs-green-800 hover:transition-all hover:duration-300 px-1 py-2 font-semibold font-outfit text-white hover:cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2">
        {#if loading}
            <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" aria-hidden="true">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Inscription...
        {:else}
            Inscription
        {/if}
    </button>

    <a href="/inscription-organisation" class="text-grey-950 font-outfit font-semibold text-sm text-center">
        Je suis une organisation
    </a>
</form>
