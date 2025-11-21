<script lang="ts">
    import imageFond from '$lib/images/register-image1.png';
    import logo from '$lib/images/greenscore-logo.png';
    import { BACKEND_URL } from '$lib/config';
    import {error} from "@sveltejs/kit";

    export let mode: 'login' | 'register';


    let email = '';
    let password = '';
    let loading = false;
    let erreurMessage = '';

    async function handleSubmit(event: Event): Promise<void> {
        event.preventDefault();

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
                    <input bind:value={email} type="email" name="email" id="inputEmail" class="form-control px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none" autocomplete="email" required autofocus>
                </div>

                <div class="w-full text-grey-700 font-outfit font-semibold text-sm sm:flex-row">
                    <label for="inputPassword">Mot de passe</label>
                    <input bind:value={password} type="password" name="password" id="inputPassword" class="form-control px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none" autocomplete="current-password" required>
                </div>

                <input type="hidden" name="_csrf_token" value="">

                <button type="submit" class="w-full h-fit rounded-lg bg-gs-green-950 hover:bg-gs-green-800 hover:transition-all hover:duration-300 px-1 py-2 font-semibold font-outfit text-white hover:cursor-pointer">Connexion</button>

            </form>
        </div>
    </div>
</div>