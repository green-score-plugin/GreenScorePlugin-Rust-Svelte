<script lang="ts">
    import AuthBase from '$lib/components/AuthBase.svelte';
    import LoginForm from '$lib/components/LoginForm.svelte';
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    import { browser } from '$app/environment';

    export let form;
    export let title: string = 'Connexion';
    export let subtitle: string = 'Vous nous avez manqué !';

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

<AuthBase {title} {subtitle} {form} showModeSwitcher={true}>
    <svelte:fragment slot="mode-switcher">
        <a href="/login" class="rounded-full px-4 py-1 text-white bg-lime-600">J'ai déjà un compte</a>
        <a href="/inscription" class="round ed-full px-4 py-1 text-[#979797]">Inscription</a>
    </svelte:fragment>

    <svelte:fragment slot="form">
        <LoginForm />
    </svelte:fragment>
</AuthBase>

{#if showDeletedToast}
    <div class="fixed bottom-4 right-4 bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded shadow-lg z-50 animate-bounce" role="alert">
        <strong class="font-bold">Succès !</strong>
        <span class="block sm:inline"> Votre compte a été supprimé avec succès.</span>
    </div>
{/if}
