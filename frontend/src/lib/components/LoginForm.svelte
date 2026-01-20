<script lang="ts">
    import validator from 'validator';
    import { enhance } from '$app/forms';

    let email = '';
    let password = '';
    let loading = false;
    let submitted = false;

    const passwordRegex = /^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$%^&*-]).{8,}$/;

    $: emailValid = !submitted || validator.isEmail(email);
    $: passwordValid = !submitted || passwordRegex.test(password);

    $: isFormValid = validator.isEmail(email) && passwordRegex.test(password);
</script>

<form method="POST" use:enhance={({ cancel }) => {
    submitted = true;

    if (!isFormValid) {
        loading = false;
        cancel();
        return;
    }

    loading = true;
    return async ({ update }) => {
        await update();
        loading = false;
    };
}} class="flex flex-col gap-4">
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

    <div class="w-full text-grey-700 font-outfit font-semibold text-sm">
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
            <div class="text-red-500 text-sm mt-1">
                <p class="font-semibold mb-1">Le mot de passe doit contenir :</p>
                <ul class="list-disc list-inside space-y-0.5 ml-1">
                    <li>Au moins 8 caractères</li>
                    <li>Une majuscule</li>
                    <li>Une minuscule</li>
                    <li>Un chiffre</li>
                    <li>Un caractère spécial (#?!@$%^&*-)</li>
                </ul>
            </div>
        {/if}
    </div>

    <button type="submit" disabled={loading} class="w-full h-fit rounded-lg bg-gs-green-950 hover:bg-gs-green-800 hover:transition-all hover:duration-300 px-1 py-2 font-semibold font-outfit text-white hover:cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2">
        {#if loading}
            <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" aria-hidden="true">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Connexion...
        {:else}
            Connexion
        {/if}
    </button>
</form>
