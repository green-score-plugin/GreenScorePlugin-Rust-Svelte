<script lang="ts">
    import AuthBase from '$lib/components/AuthBase.svelte';
    import LoginForm from '$lib/components/LoginForm.svelte';
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    import { browser } from '$app/environment';
    import { t } from 'svelte-i18n';

    export let form;

    let showDeletedToast = false;

    $: if (browser && $page.url.searchParams.get('account_deleted') === 'true') {
        showDeletedToast = true;
        const newUrl = new URL($page.url);
        newUrl.searchParams.delete('account_deleted');
        window.history.replaceState({}, '', newUrl);

        setTimeout(() => {
            showDeletedToast = false;
        }, 5000);
    }
</script>

<AuthBase title={$t('auth.login.title')} subtitle={$t('auth.login.subtitle')} {form} showModeSwitcher={true}>
    <svelte:fragment slot="mode-switcher">
        <a href="/login" class="rounded-full px-4 py-1 text-white bg-lime-600">{$t('auth.login.already_have_account')}</a>
        <a href="/inscription" class="round ed-full px-4 py-1 text-[#979797]">{$t('auth.login.register_link')}</a>
    </svelte:fragment>

    <svelte:fragment slot="form">
        <LoginForm />
    </svelte:fragment>
</AuthBase>

{#if showDeletedToast}
    <div class="fixed bottom-4 right-4 bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded shadow-lg z-50 animate-bounce" role="alert">
        <strong class="font-bold">{$t('auth.login.success_deleted')}</strong>
        <span class="block sm:inline"> {$t('auth.login.account_deleted_msg')}</span>
    </div>
{/if}
