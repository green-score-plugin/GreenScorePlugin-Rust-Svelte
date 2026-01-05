<script lang="ts">
    import AuthBase from '$lib/components/AuthBase.svelte';
    import LoginForm from '$lib/components/LoginForm.svelte';

    export let form;
    export let title: string = 'Connexion';
    export let subtitle: string = 'Vous nous avez manqué !';
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
