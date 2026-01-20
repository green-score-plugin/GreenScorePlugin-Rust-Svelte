<script lang="ts">
    import validator from 'validator';
    import { enhance } from '$app/forms';

    let firstname = '', lastname = '', email = '', password = '', confirmPassword = '';
    let agreeTerms = false, loading = false, submitted = false;

    const passwordRegex = /^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$%^&*-]).{8,}$/;

    $: errors = {
        firstname: !firstname.trim(),
        lastname: !lastname.trim(),
        email: !validator.isEmail(email),
        password: !passwordRegex.test(password),
        confirmPassword: confirmPassword.trim() === '' || confirmPassword !== password,
        agreeTerms: !agreeTerms
    };

    $: hasErrors = Object.values(errors).some(Boolean);
</script>


<form method="POST" use:enhance={({cancel}) => {
    submitted = true;

    if (hasErrors) {
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

    <!-- Prénom / Nom -->
    <div class="flex flex-col sm:flex-row gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full">
            <label for="firstname">Prénom</label>
            <input bind:value={firstname} id="firstname" name="firstname" type="text"
                   class="px-4 py-2 border rounded-lg w-full focus:outline-none {submitted && errors.firstname ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder="John">
            {#if submitted && errors.firstname} <span class="text-red-500 text-sm">Prénom requis.</span> {/if}
        </div>
        <div class="w-full">
            <label for="lastname">Nom</label>
            <input bind:value={lastname} id="lastname" name="lastname" type="text"
                   class="px-4 py-2 border rounded-lg w-full focus:outline-none {submitted && errors.lastname ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder="Doe">
            {#if submitted && errors.lastname} <span class="text-red-500 text-sm">Nom requis.</span> {/if}
        </div>
    </div>

    <!-- Email -->
    <div class="w-full text-grey-700 font-outfit font-semibold text-sm">
        <label for="inputEmail">Email</label>
        <input bind:value={email} type="email" name="email" id="inputEmail"
               class="px-4 py-2 border rounded-lg w-full focus:outline-none {submitted && errors.email ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder="john.doe@example.com">
        {#if submitted && errors.email} <span class="text-red-500 text-sm">Email invalide.</span> {/if}
    </div>

    <!-- Mots de passe -->
    <div class="flex flex-col sm:flex-row gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full">
            <label for="inputPassword">Mot de passe</label>
            <input bind:value={password} type="password" name="password" id="inputPassword"
                   class="px-4 py-2 border rounded-lg w-full focus:outline-none {submitted && errors.password ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder="••••••••">
            {#if submitted && errors.password}
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
        <div class="w-full">
            <label for="confirmPassword">Confirmation</label>
            <input bind:value={confirmPassword} id="confirmPassword" type="password" name="confirmPassword"
                   class="px-4 py-2 border rounded-lg w-full focus:outline-none {submitted && errors.confirmPassword ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder="••••••••">
            {#if submitted && errors.confirmPassword} <span class="text-red-500 text-sm">Les mots de passe ne correspondent pas.</span> {/if}
        </div>
    </div>

    <!-- CGU -->
    <div class="flex flex-col gap-2 text-grey-700 font-semibold text-sm font-outfit">
        <div class="flex gap-2 items-start">
            <input bind:checked={agreeTerms} id="agreeTerms" type="checkbox" name="agreeTerms" class="mt-1 accent-gs-green-950 cursor-pointer">
            <label for="agreeTerms">En vous inscrivant sur GreenScoreWeb, vous acceptez nos conditions générales d'utilisation.</label>
        </div>
        {#if submitted && errors.agreeTerms} <span class="text-red-500 text-sm">Merci d'accepter les CGU.</span> {/if}
    </div>

    <!-- Bouton -->
    <button type="submit" disabled={loading} class="w-full rounded-lg bg-gs-green-950 hover:bg-gs-green-800 p-2 font-semibold font-outfit text-white disabled:opacity-50 flex justify-center gap-2 transition-colors">
        {#if loading} <span class="animate-spin h-5 w-5 border-2 border-white/20 border-t-white rounded-full"></span> {/if}
        Inscription
    </button>

    <a href="/inscription-organisation" class="text-grey-950 font-outfit font-semibold text-sm text-center">Je suis une organisation</a>
</form>
