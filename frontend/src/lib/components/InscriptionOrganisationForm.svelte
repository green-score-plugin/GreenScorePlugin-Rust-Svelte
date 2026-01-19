<script lang="ts">
    import validator from 'validator';
    import { enhance } from '$app/forms';

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

    $: hasErrors = Object.values(errors).some(Boolean);
</script>

<form method="POST" use:enhance={({ cancel }) => {
    submitted = true;
    if (hasErrors) cancel();
    else {
        loading = true;
        return async ({ update }) => { await update(); loading = false; };
    }
}} class="flex flex-col gap-3 max-w-full overflow-hidden">

    <!-- Nom Organisation -->
    <div class="w-full text-grey-700 font-outfit font-semibold text-xs">
        <label for="organisationName" class="block mb-1">Nom de l'organisation</label>
        <input bind:value={organisationName} id="organisationName" type="text" name="organisationName"
               class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none {submitted && errors.organisationName ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder="Mon Organisation">
        {#if submitted && errors.organisationName} <span class="text-red-500 text-xs mt-0.5 block">Nom requis.</span> {/if}
    </div>

    <!-- SIRET (Optionnel) -->
    <div class="w-full text-grey-700 font-outfit font-semibold text-xs">
        <label for="siret" class="block mb-1">Numéro SIRET (optionnel)</label>
        <input bind:value={siret} id="siret" type="text" name="siret"
               class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none border-grey-200" placeholder="123 456 789 00012">
    </div>

    <!-- Email -->
    <div class="w-full text-grey-700 font-outfit font-semibold text-xs">
        <label for="inputEmail" class="block mb-1">Email</label>
        <input bind:value={email} type="email" name="email" id="inputEmail"
               class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none {submitted && errors.email ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder="john.doe@example.com">
        {#if submitted && errors.email} <span class="text-red-500 text-xs mt-0.5 block">Email invalide.</span> {/if}
    </div>

    <!-- Mots de passe -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <div class="text-grey-700 font-outfit font-semibold text-xs">
            <label for="inputPassword" class="block mb-1">Mot de passe</label>
            <input bind:value={password} type="password" name="password" id="inputPassword"
                   class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none {submitted && errors.password ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder="••••••••">
            {#if submitted && errors.password}
                <div class="text-red-500 text-xs mt-1">
                    <p class="font-semibold mb-0.5">Le mot de passe doit contenir :</p>
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
        <div class="text-grey-700 font-outfit font-semibold text-xs">
            <label for="confirmPassword" class="block mb-1">Confirmation</label>
            <input bind:value={confirmPassword} id="confirmPassword" type="password" name="confirmPassword"
                   class="px-3 py-1.5 text-sm border rounded-lg w-full focus:outline-none {submitted && errors.confirmPassword ? 'border-red-700 bg-red-50' : 'border-grey-200'}" placeholder="••••••••">
            {#if submitted && errors.confirmPassword} <span class="text-red-500 text-xs mt-0.5 block">Les mots de passe ne correspondent pas.</span> {/if}
        </div>
    </div>

    <!-- CGU -->
    <div class="text-grey-700 font-semibold text-xs font-outfit">
        <div class="flex gap-2 items-start">
            <input bind:checked={agreeTerms} id="agreeTerms" type="checkbox" name="agreeTerms" class="w-3.5 h-3.5 mt-0.5 rounded border-gray-300 accent-gs-green-950 cursor-pointer flex-shrink-0">
            <label for="agreeTerms" class="leading-tight text-xs">En vous inscrivant sur GreenScoreWeb, vous acceptez nos conditions générales d'utilisation.</label>
        </div>
        {#if submitted && errors.agreeTerms} <span class="text-red-500 text-xs mt-0.5 block ml-5">Merci d'accepter les CGU.</span> {/if}
    </div>

    <!-- Bouton -->
    <button type="submit" disabled={loading} class="w-full rounded-lg bg-gs-green-950 hover:bg-gs-green-800 transition-all duration-300 px-3 py-2 text-sm font-semibold font-outfit text-white cursor-pointer disabled:opacity-50 flex items-center justify-center gap-2">
        {#if loading} <span class="animate-spin h-4 w-4 border-2 border-white/20 border-t-white rounded-full"></span> {/if}
        Inscription
    </button>

    <a href="/inscription" class="text-grey-950 font-outfit font-semibold text-xs text-center hover:underline">Je suis un utilisateur</a>
</form>
