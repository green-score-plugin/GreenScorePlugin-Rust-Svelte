<script lang="ts">
    import imageFond from '$lib/images/register-image1.png';
    import logo from '$lib/images/greenscore-logo.png';
    import { BACKEND_URL } from '$lib/config';

    export let mode: 'login' | 'register';


    let email = '';
    let password = '';
    let loading = false;
    let erreurMessage = '';
    let emailValid = true;
    let passwordValid = true;


    const EMAIL_REGEX = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

    $: if (email) emailValid = true;
    $: if (password) passwordValid = true;

    async function handleSubmit(event: Event): Promise<void> {
        event.preventDefault();

        emailValid = (EMAIL_REGEX.test(email));
        passwordValid = (password.length >= 8);

        if(!emailValid || !passwordValid) return;

        loading = true;
        const endpoint = `${BACKEND_URL}${mode === 'register' ? '/register' : '/login'}`;

            fetch(endpoint, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                credentials: 'include',
                body: JSON.stringify({
                    email: email,
                    password: password,
                })
            })
            .then((response) => response.json())
            .then(data => {
                if(data.success) {
                    window.location.href = '/';
                } else {
                    erreurMessage = data.message;
                }
            })
            .finally(() => { loading = false; })

    }

</script>

<div class="flex items-center w-screen h-screen">
    <img src="{imageFond}" alt="GreenScore" class="hidden xl:flex w-1/2 h-full object-cover">
    <div class="w-full xl:w-1/2 h-full px-6 md:px-[120px] py-24 flex flex-col items-center justify-center">
        <div class="flex flex-col w-full gap-10">
            <div class="flex gap-2 items-center">
                <img width="55px" height="auto" src="{logo}" alt="GreenScore Logo">
                <h1 class="text-3xl font-outfit font-extrabold text-gs-green-950">GreenScore Web</h1>
            </div>
            <div class="flex flex-col gap-4 font-outfit">
                <h2 class="text-2xl font-extrabold text-grey-950">Vous nous avez manqué !</h2>
                <div class="flex gap-[10px] px-1 border border-grey-200 rounded-full py-1 w-fit font-medium">
                    <a href="/login" class=" rounded-full px-4 py-1  decoration-0 {mode === 'login' ? 'text-white bg-lime-600': 'text-[#979797]'}">
                        J'ai déjà un compte
                    </a>
                    <a href="/register" class="rounded-full px-4 py-1 text-[#979797] decoration-0">
                        Inscription
                    </a>
                </div>
            </div>

            {#if erreurMessage !== ''}
                <div class="w-full bg-red-50 text-red-700 text-sm font-outfit font-medium border border-red-700 rounded-lg px-6 py-6">
                    {erreurMessage}
                </div>
            {/if}

            <form  on:submit|preventDefault={handleSubmit} class="flex flex-col gap-4">
                <div class="w-full text-grey-700 font-outfit font-semibold text-sm sm:flex-row">
                    <label for="inputEmail">Email</label>
                    <input bind:value={email} type="email" name="email" id="inputEmail" class="form-control px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none { !emailValid ? 'border-red-700 bg-red-50' : 'border-gray-200' } " autocomplete="email">
                    {#if !emailValid}
                        <span class="text-red-500 text-sm">Email invalide</span>
                    {/if}
                </div>

                <div class="w-full text-grey-700 font-outfit font-semibold text-sm sm:flex-row">
                    <label for="inputPassword">Mot de passe</label>
                    <input bind:value={password} type="password" name="password" id="inputPassword" class="form-control px-4 py-2 border rounded-lg text-grey-700 w-full focus:outline-none { !passwordValid ? 'border-red-700 bg-red-50' : 'border-gray-200' } " autocomplete="current-password">
                    {#if !passwordValid}
                        <span class="text-red-500 text-sm">Le mot de passe doit contenir au moins 8 caractères</span>
                    {/if}
                </div>

                <input type="hidden" name="_csrf_token" value="">

                <button type="submit" disabled={loading} class="w-full h-fit rounded-lg bg-gs-green-950 hover:bg-gs-green-800 hover:transition-all hover:duration-300 px-1 py-2 font-semibold font-outfit text-white hover:cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2">
                    {#if loading}
                        <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        Connexion...
                    {:else}
                        Connexion
                    {/if}
                </button>

            </form>
        </div>
    </div>
</div>