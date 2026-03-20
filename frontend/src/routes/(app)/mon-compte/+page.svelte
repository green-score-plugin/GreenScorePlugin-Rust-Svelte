<script lang="ts">
    import LeftMenu from '$lib/components/myaccount/LeftMenu.svelte';
    import MyInfo from '$lib/components/myaccount/MyInfo.svelte';
    import GererOrganisation from '$lib/components/myaccount/GererOrganisation.svelte';
    import GererMembre from '$lib/components/myaccount/GererMembre.svelte';
    import salutation from '$lib/images/salutation.png';
    import {page} from "$app/state";
    import { t } from 'svelte-i18n';
    import UserEquivalent from "$lib/components/myaccount/UserEquivalent.svelte";
    import type {User, Organisation, UserFull} from "$lib/types/account.ts";

    let activePage = $state("my_info");

    let userFull = $derived((page.form?.updatedUser as UserFull | undefined) ?? (page.data.userFull as UserFull));
    let user = $derived(userFull.user as User);
    let organisation = $derived(userFull.organisation as Organisation)

    $inspect(userFull);
    $inspect(activePage);
</script>

<svelte:head>
    <title>{$t('header.manage_account')} | GreenScore Web</title>
</svelte:head>

<div class="xl:px-52 flex flex-col h-full">
    <div class="px-4 lg:px-16 py-8 flex justify-center lg:justify-start items-center gap-x-4">
        <img class="w-[54px] h-auto" src={salutation} alt="Salutation">
        <h1 class="text-2xl font-bold">{$t('hello')} {user.prenom}!</h1>
    </div>

    <div class="flex flex-col lg:flex-row px-4 lg:px-16 gap-8 lg:gap-16 mb-2">

        <div class="flex-grid h-fit min-w-60 shadow-lg bg-white items-center">
            <LeftMenu bind:activePage/>
        </div>
        <div class="flex-1 shadow-lg bg-white py-4 px-6">
            {#if activePage === 'my_info'}
                <MyInfo />
            {:else if activePage === 'user_equivalent'}
                <UserEquivalent />
            {:else if activePage === 'organisation'}
                {#if user.est_admin === true }
                    <GererMembre />
                {:else}
                    <GererOrganisation />
                {/if}
            {/if}
        </div>
    </div>
    <div class="px-4 lg:px-16 pb-8 flex flex-col gap-4">
        <details class="bg-white p-4 rounded shadow-lg">
            <summary class="cursor-pointer font-bold text-gray-700">Voir toutes les infos utilisateur (Debug)</summary>
            <pre class="mt-4 p-4 bg-gray-100 rounded overflow-x-auto text-sm">{JSON.stringify(userFull, null, 2)}</pre>
        </details>

        <details class="bg-white p-4 rounded shadow-lg">
            <summary class="cursor-pointer font-bold text-gray-700">Voir toutes les infos organisation (Debug)</summary>
            <pre class="mt-4 p-4 bg-gray-100 rounded overflow-x-auto text-sm">{JSON.stringify(organisation, null, 2)}</pre>
        </details>
        <details class="bg-white p-4 rounded shadow-lg">
            <summary class="cursor-pointer font-bold text-gray-700">Voir toutes les infos organisation (Debug)</summary>
            <pre class="mt-4 p-4 bg-gray-100 rounded overflow-x-auto text-sm">{JSON.stringify(activePage, null, 2)}</pre>
        </details>
    </div>
</div>
