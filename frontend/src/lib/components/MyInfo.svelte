<script lang="ts">
    import { page } from '$app/stores';
    import { enhance } from '$app/forms';

    let submitted = false;
    let loading = false;

    let user = { prenom: '', nom: '', email: '' };
    let password = '';
    let passwordConfirm = '';

    let successMessage = '';
    let errorMessage = '';

    $: {
        if ($page.data.user) {
            user = { ...$page.data.user };
        }
    }

    $: {
        if ($page.form?.success) {
            successMessage = $page.form.message || 'Informations mises à jour avec succès';
            errorMessage = '';
            password = '';
            passwordConfirm = '';
            submitted = false;
        } else if ($page.form?.message) {
            errorMessage = $page.form.message;
            successMessage = '';
        }
    }
    $: passwordValid = !submitted || (password === passwordConfirm && (password.length === 0 || password.length >= 8));
</script>

<form
        method="POST"
        action="?/modifier"
        use:enhance={() => {
        submitted = true;
        if (!passwordValid) return async () => {};
        loading = true;
        successMessage = '';
        errorMessage = '';

        return async ({ update }) => {
            await update();
            loading = false;
        };
    }}
        class="flex flex-col gap-4"
>
    <h1 class="text-2xl font-bold py-2">Mes informations</h1>

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

    <div class="flex gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full flex flex-col">
            <label for="firstName">Prénom</label>
            <input
                    id="firstName"
                    name="prenom"
                    type="text"
                    bind:value={user.prenom}
                    class="px-4 py-2 border border-grey-200 rounded-lg w-full focus:outline-none"
                    placeholder="Votre prénom"
                    autocomplete="off"
            />
        </div>
    </div>

    <div class="flex gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full flex flex-col">
            <label for="lastName">Nom</label>
            <input
                    id="lastName"
                    name="nom"
                    type="text"
                    bind:value={user.nom}
                    class="px-4 py-2 border border-grey-200 rounded-lg w-full focus:outline-none"
                    placeholder="Votre nom"
                    autocomplete="off"
            />
        </div>
    </div>

    <div class="flex gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full flex flex-col">
            <label for="email">Email</label>
            <input
                    id="email"
                    name="email"
                    type="email"
                    bind:value={user.email}
                    class="px-4 py-2 border border-grey-200 rounded-lg w-full focus:outline-none"
                    placeholder="Votre email"
                    autocomplete="off"
            />
        </div>
    </div>

    <div class="flex flex-col md:flex-row gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full flex flex-col">
            <label for="password">Mot de passe</label>
            <input
                    id="password"
                    name="password"
                    type="password"
                    bind:value={password}
                    class="px-4 py-2 border border-grey-200 rounded-lg w-full focus:outline-none"
                    placeholder="Votre mot de passe"
            />
        </div>

        <div class="w-full flex flex-col">
            <label for="passwordConfirm">Confirmation</label>
            <input
                    id="passwordConfirm"
                    type="password"
                    bind:value={passwordConfirm}
                    class="px-4 py-2 border border-grey-200 rounded-lg w-full focus:outline-none"
                    placeholder="Confirmez votre mot de passe"
            />
        </div>
    </div>

    {#if !passwordValid}
        <span class="text-red-500 text-sm">
            Les mots de passe doivent être identiques et contenir au moins 8 caractères
        </span>
    {/if}

    <button
            type="submit"
            disabled={loading || !passwordValid}
            class="mt-4 w-full h-fit rounded-lg bg-gs-green-950 px-1 py-2 font-semibold font-outfit text-white
               cursor-pointer hover:bg-gs-green-800 active:bg-gs-green-700
               transition-colors duration-150 ease-in-out disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
    >
        {#if loading}
            <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" aria-hidden="true">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Enregistrement...
        {:else}
            Valider
        {/if}
    </button>
</form>