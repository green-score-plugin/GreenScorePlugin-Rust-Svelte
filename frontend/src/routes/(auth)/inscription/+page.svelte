<script lang="ts">
    import AuthBase from '$lib/components/AuthBase.svelte';
    import InscriptionForm from "$lib/components/InscriptionForm.svelte";
    import { t } from 'svelte-i18n';

    export let form;
</script>

<AuthBase title={$t('auth.register.user_title')} subtitle={$t('auth.register.subtitle')} {form} showModeSwitcher={true}>
    <svelte:fragment slot="mode-switcher">
        <a href="/login" class="rounded-full px-4 py-1 text-[#979797]">{$t('auth.login.already_have_account')}</a>
        <a href="/inscription" class="rounded-full px-4 py-1 text-white bg-lime-600">{$t('auth.login.register_link')}</a>
    </svelte:fragment>

    <svelte:fragment slot="form">
        <InscriptionForm />
    </svelte:fragment>
</AuthBase>