<script lang="ts">
    import AuthBase from '$lib/components/AuthBase.svelte';
    import InscriptionOrganisationCode from "$lib/components/InscriptionOrganisationCode.svelte";

    export let subtitle : string = '';
    export let data;
</script>

<AuthBase title="Inscription" {subtitle} showModeSwitcher={false}>

    <svelte:fragment slot="form">
        <InscriptionOrganisationCode code={data.code} />
    </svelte:fragment>
</AuthBase>