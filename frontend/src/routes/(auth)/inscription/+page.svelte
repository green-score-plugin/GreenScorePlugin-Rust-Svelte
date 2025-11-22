<script lang="ts">
    import AuthBase from '$lib/components/AuthBase.svelte';
    import InscriptionForm from "$lib/components/InscriptionForm.svelte";

    export let form;
    export let subtitle : string = 'Prêts à agir ?';
</script>

<AuthBase title="Inscription" {subtitle} {form} showModeSwitcher={true}>
    <svelte:fragment slot="mode-switcher">
        <a href="/login" class="rounded-full px-4 py-1 text-[#979797]">J'ai déjà un compte</a>
        <a href="/inscription" class="rounded-full px-4 py-1 text-white bg-lime-600">Inscription</a>
    </svelte:fragment>

    <svelte:fragment slot="form">
        <InscriptionForm />
    </svelte:fragment>
</AuthBase>