<script lang="ts">
    import { page } from '$app/stores';
    import { enhance } from '$app/forms';
    import { t } from 'svelte-i18n';

    let submitted = false;
    let loading = false;
    let submittedPassword = '';
    let submittedPasswordConfirm = '';

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
        if ($page.form?.actionType === 'update_info') {
            if ($page.form?.success) {
                successMessage = $t($page.form.message || 'auth.register.success_register');
                errorMessage = '';
                password = '';
                passwordConfirm = '';
                submitted = false;
            } else if ($page.form?.message) {
                errorMessage = $t($page.form.message);
                successMessage = '';
            }
        }
    }
    const passwordRegex = /^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$%^&*-]).{8,}$/

    $: passwordValid = (password === '' && passwordConfirm === '') ||
        (password === passwordConfirm && passwordRegex.test(password));

    $: showPasswordError = submitted && !passwordValid;

    const noAutofill = { autocomplete: 'nop' } as any;

    function handlePasswordInput() {
        if (submitted && (password !== submittedPassword || passwordConfirm !== submittedPasswordConfirm)) {
            submitted = false;
        }
    }
</script>

<form
        method="POST"
        action="?/modifier"
        autocomplete="off"
        use:enhance={({ cancel }) => {
            if (!passwordValid) {
                submittedPassword = password;
                submittedPasswordConfirm = passwordConfirm;
                submitted = true;
                cancel();
                return;
            }

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
    <h1 class="text-2xl font-bold py-2">{$t('account.info.title')}</h1>

    <div style="opacity: 0; position: absolute; top: 0; left: 0; height: 0; width: 0; z-index: -1;">
        <input type="text" name="fake_email_prevent_autofill" tabindex="-1" />
        <input type="password" name="fake_password_prevent_autofill" tabindex="-1" />
    </div>

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

    {#if showPasswordError}
        <div class="px-4 py-3 rounded-lg bg-red-50 border border-red-200 text-red-700 text-sm">
            <p class="font-semibold mb-2">{$t('auth.login.password_requirements')}</p>
            <ul class="list-disc list-inside space-y-1 ml-1">
                <li>{$t('auth.login.req_length')}</li>
                <li>{$t('auth.login.req_uppercase')}</li>
                <li>{$t('auth.login.req_lowercase')}</li>
                <li>{$t('auth.login.req_number')}</li>
                <li>{$t('auth.login.req_special')}</li>
            </ul>
        </div>
    {/if}


    <div class="flex gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full flex flex-col">
            <label for="firstName">{$t('auth.register.firstname')}</label>
            <input
                    id="firstName"
                    name="prenom"
                    type="text"
                    bind:value={user.prenom}
                    class="px-4 py-2 border border-grey-200 rounded-lg w-full focus:outline-none"
                    placeholder={$t ('account.info.first_name_label')}
                    {...noAutofill}
            />
        </div>
    </div>

    <div class="flex gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full flex flex-col">
            <label for="lastName">{$t('auth.register.lastname')}</label>
            <input
                    id="lastName"
                    name="nom"
                    type="text"
                    bind:value={user.nom}
                    class="px-4 py-2 border border-grey-200 rounded-lg w-full focus:outline-none"
                    placeholder={$t ('account.info.last_name_label')}
                    {...noAutofill}
            />
        </div>
    </div>

    <div class="flex gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full flex flex-col">
            <label for="email">{$t('auth.login.email_label')}</label>
            <input
                    id="email"
                    name="email"
                    type="email"
                    bind:value={user.email}
                    class="px-4 py-2 border border-grey-200 rounded-lg w-full focus:outline-none"
                    placeholder={$t ('account.info.email_label')}
                    {...noAutofill}
            />
        </div>
    </div>

    <div class="flex flex-col md:flex-row gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full flex flex-col">
            <label for="password">{$t('auth.login.password_label')}</label>
            <input
                    id="password"
                    name="password"
                    type="password"
                    bind:value={password}
                    on:input={handlePasswordInput}
                    class="px-4 py-2 border border-grey-200 rounded-lg w-full focus:outline-none"
                    placeholder={$t ('account.info.password_label')}
                    autocomplete="new-password"
            />
        </div>

        <div class="w-full flex flex-col">
            <label for="passwordConfirm">{$t('auth.register.confirm_password')}</label>
            <input
                    id="passwordConfirm"
                    type="password"
                    bind:value={passwordConfirm}
                    on:input={handlePasswordInput}
                    class="px-4 py-2 border border-grey-200 rounded-lg w-full focus:outline-none"
                    placeholder={$t ('account.info.confirm_password_label')}
                    autocomplete="new-password"
            />
        </div>
    </div>


    <button
            type="submit"
            disabled={loading}
            class="mt-4 w-full h-fit rounded-lg bg-gs-green-950 px-4 py-2 font-semibold font-outfit text-white
               cursor-pointer hover:bg-gs-green-800 active:bg-gs-green-700
               transition-colors duration-150 ease-in-out disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
    >
        {#if loading}
            <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" aria-hidden="true">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            {$t('account.info.button_saving')}
        {:else}
            {$t('account.info.button_validate')}
        {/if}
    </button>
</form>