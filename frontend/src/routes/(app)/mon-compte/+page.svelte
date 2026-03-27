<script lang="ts">
    import { goto } from '$app/navigation';
    import LeftMenu from '$lib/components/myaccount/LeftMenu.svelte';
    import MyInfo from '$lib/components/myaccount/MyInfo.svelte';
    import GererOrganisation from '$lib/components/myaccount/GererOrganisation.svelte';
    import CreateService from '$lib/components/myaccount/CreateService.svelte';
    import MyService from '$lib/components/myaccount/MyService.svelte';
    import GererMembre from '$lib/components/myaccount/GererMembre.svelte';
    import salutation from '$lib/images/salutation.png';
    import {page} from "$app/state";
    import { t } from 'svelte-i18n';
    import UserEquivalent from "$lib/components/myaccount/UserEquivalent.svelte";
    import type {User, Organisation, UserFull} from "$lib/types/account.ts";
    import MyInfoOrganisation from "$lib/components/myaccount/MyInfoOrganisation.svelte";

    let activePage = $state(page.url.searchParams.get('tab') ?? "my_info");
    let activeOrgTab = $state('details');

    let userFull = $derived((page.form?.updatedUser as UserFull | undefined) ?? (page.data.userFull as UserFull));
    let user = $derived(userFull.user as User);
    let service = $derived(userFull.service)
    let services = $derived((page.form?.updatedServices as any[]) ?? (page.data.services as any[]));

    let orgIdParam = $derived(page.url.searchParams.get('orgId'));
    let actionParam = $derived(page.url.searchParams.get('action'));

    let selectedOrgId = $derived.by(() => {
        if (orgIdParam) return parseInt(orgIdParam);
        if (userFull.organisation?.length > 0) return userFull.organisation[0].id;
        return null;
    });

    let selectedOrganisation = $derived(
        userFull.organisation?.find(o => o.id === selectedOrgId)
    );

    let isCreatingOrg = $derived(actionParam === 'new' || userFull.organisation?.length === 0);

    $inspect(userFull);
    $inspect(activePage);

    function handleOrgChange(event: Event) {
        const select = event.target as HTMLSelectElement;
        const val = select.value;
        if (val === 'new') {
            goto(`?tab=organisation&action=new`); // Fixed: use backticks for template literal
        } else {
             goto(`?tab=organisation&orgId=${val}`); // Fixed: use backticks
        }
    }
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

                {#if userFull.organisation?.length > 0}
                    <div class="mb-6 flex items-center justify-between">
                         <label class="flex items-center gap-2 w-full sm:w-auto">
                            <span class="font-semibold text-gray-700 whitespace-nowrap">Organisation:</span>
                            <select
                                class="px-3 py-2 border border-grey-200 rounded-lg text-grey-700 bg-white focus:outline-none focus:ring-1 focus:ring-gs-green-950 cursor-pointer text-sm font-medium w-full sm:w-auto"
                                onchange={handleOrgChange}
                            >
                                {#each userFull.organisation as org}
                                    <option value={org.id} selected={!isCreatingOrg && selectedOrgId === org.id}>
                                        {org.nom} {org.est_admin ? '(Admin)' : ''}
                                    </option>
                                {/each}
                                <option value="new" selected={isCreatingOrg}>+ Rejoindre / Créer</option>
                            </select>
                        </label>
                    </div>
                {/if}

                {#if isCreatingOrg}
                     <GererOrganisation organisation={undefined} />
                {:else if selectedOrganisation}
                    {#if selectedOrganisation.est_admin === true}
                        <!-- Tabs Navigation -->
                        <div class="border-b border-gray-200 mb-6">
                            <nav class="-mb-px flex gap-8" aria-label="Tabs">
                                <button
                                        onclick={() => activeOrgTab = 'details'}
                                        class="{activeOrgTab === 'details'
                                            ? 'border-gs-green-950 text-gs-green-950'
                                            : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}
                                            whitespace-nowrap pb-4 px-1 border-b-2 font-medium text-sm transition-colors font-outfit cursor-pointer"
                                >
                                    {$t('account.menu.my_organization')}
                                </button>
                                <button
                                        onclick={() => activeOrgTab = 'services'}
                                        class="{activeOrgTab === 'services'
                                            ? 'border-gs-green-950 text-gs-green-950'
                                            : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}
                                            whitespace-nowrap pb-4 px-1 border-b-2 font-medium text-sm transition-colors font-outfit cursor-pointer"
                                >
                                    {$t('account.menu.services')}
                                </button>
                            </nav>
                        </div>

                        {#if activeOrgTab === 'details'}
                            <div class="flex flex-col gap-6 animate-in fade-in duration-300">
                                <MyInfoOrganisation organisation={selectedOrganisation} />
                                <GererMembre organisation={selectedOrganisation} />
                            </div>
                        {:else}
                            <div class="flex flex-col gap-6 animate-in fade-in duration-300">
                                <CreateService organisationId={selectedOrganisation.id} />
                                <MyService {services} organisationId={selectedOrganisation.id} members={page.data.members} currentUserId={user.id} />
                            </div>
                        {/if}
                    {:else}
                        <GererOrganisation organisation={selectedOrganisation} />
                    {/if}
                {:else}
                    <p>Organisation introuvable.</p>
                {/if}
            {/if}
        </div>
    </div>
</div>
