<script lang="ts">
    import OrganizationAverageDailyCarbonFootprint from "$lib/components/widgets/OrganizationAverageDailyCarbonFootprint.svelte";
    import ChartConsumptionFiltered from "$lib/components/widgets/ChartConsumptionFiltered.svelte";
    import BadgeGreenScore from "$lib/components/widgets/BadgeGreenScore.svelte";
    import Equivalent from "$lib/components/widgets/Equivalent.svelte";
    import TotalConsumption from "$lib/components/widgets/TotalConsumption.svelte";
    import ChartTop5PollutingSites from "$lib/components/widgets/ChartTop5PollutingSites.svelte";
    import Advice from "$lib/components/widgets/Advice.svelte";
    import type { PageData } from './$types';
    import { t } from 'svelte-i18n';
    import { goto } from '$app/navigation';

    export let data: PageData;

    export let totalConsuUnit = 'gCO2eq';
    export let noDatas: boolean = false;

    let selectedPeriod: 'daily' | 'weekly' | 'monthly' = 'monthly';

    if (data.organisationData === null) {
        noDatas = true;
    } else {
        noDatas = false;
    }

    $: description = $t('dashboard.my_organization.description') + (data.organisationData?.name || '');
    $: averageFootprint = data.organisationData?.averageDailyCarbonFootprint;
    $: equivalentAverage = data.organisationData?.equivalent;
    $: carbonFootprint = (data.organisationData?.totalCarbonFootprint ?? 0).toFixed(2);
    $: letterGreenScore = data.letterGreenScore || 'A';
    $: envNomination = data.envNomination || 'nominations.profile.A';
    $: equivalent1 = data.equivalents?.[0] || { name: '', value: 0, icon: '' };
    $: equivalent2 = data.equivalents?.[1] || { name: '', value: 0, icon: '' };
    $: advice = data.adviceUser || '';
    $: adviceDev = data.adviceDev || '';
    $: dailyConsumption = data.dailyConsumption || [];
    $: weeklyConsumption = data.weeklyConsumption || [];
    $: monthlyConsumption = data.monthlyConsumption || [];
    $: topPollutingSites = data.topPollutingSites || [];

    $: consumptionData = selectedPeriod === 'daily' ? dailyConsumption
        : selectedPeriod === 'weekly' ? weeklyConsumption
            : monthlyConsumption;

    $: isAdmin = data.isAdmin;
    $: services = data.services || [];
    $: userOrgs = data.userFull?.organisation || [];

    let selectedOrgId = data.currentOrgId || '';
    let selectedServiceId = data.currentServiceId || '';

    $: {
        if (!selectedOrgId && userOrgs.length > 0) {
            selectedOrgId = userOrgs[0].id.toString();
        }
    }

    function handleFilterChange() {
        let qs = new URLSearchParams();
        if (selectedOrgId) qs.set('org_id', selectedOrgId);
        if (selectedServiceId) qs.set('service_id', selectedServiceId);
        goto(`?${qs.toString()}`);
    }
</script>

<svelte:head>
    <title>{$t('dashboard.my_organization.title')} | GreenScore Web</title>
</svelte:head>

<div class="w-full bg-green-bg min-h-screen">
    <div class="w-full text-center px-10 lg:px-0 pt-10 font-outfit flex items-center justify-center flex-col text-grey-950">
        <h1 class="text-4xl font-bold">{$t('dashboard.my_organization.title')}</h1>
        {#if !noDatas}
            <p class="text-base w-fit">
            { description }
            </p>
        {/if}
        <div class="mt-4 flex flex-col md:flex-row gap-4 items-center justify-center">
            {#if userOrgs.length > 1}
                <div class="flex items-center gap-2">
                    <label for="org-select" class="font-medium text-sm">{$t('auth.register.org_name')}:</label>
                    <select id="org-select" class="block w-full py-2 px-3 border border-gray-300 bg-white rounded-md shadow-sm focus:outline-none sm:text-sm" bind:value={selectedOrgId} on:change={() => { selectedServiceId = ''; handleFilterChange(); }}>
                        {#each userOrgs as org}
                            <option value={org.id.toString()}>{org.nom}</option>
                        {/each}
                    </select>
                </div>
            {/if}
            {#if isAdmin}
                    <div class="flex items-center gap-2">
                        <label for="service-select" class="font-medium text-sm">Service:</label>
                        <select id="service-select" class="block w-full py-2 px-3 border border-gray-300 bg-white rounded-md shadow-sm focus:outline-none sm:text-sm" bind:value={selectedServiceId} on:change={handleFilterChange}>
                            <option value="">Tous les services</option>
                            {#each services as svc}
                                <option value={svc.id.toString()}>{svc.nom}</option>
                            {/each}
                        </select>
                    </div>
            {/if}
        </div>
    </div>

    {#if !noDatas}
    <div class="grid grid-cols-1 gap-6 p-10 sm:grid-cols-2 lg:grid-cols-12">
        <OrganizationAverageDailyCarbonFootprint {averageFootprint} {equivalentAverage}/>
        <ChartConsumptionFiltered {consumptionData} bind:selectedPeriod/>
        <BadgeGreenScore {letterGreenScore} {envNomination}/>
        <Equivalent equivalent={equivalent1} order={1} />
        <TotalConsumption {carbonFootprint} {totalConsuUnit} label={$t('dashboard.my_organization.total_consumption')} />
        <Equivalent equivalent={equivalent2} order={2} />
        <ChartTop5PollutingSites {topPollutingSites} />
        <Advice type="nav" advice={advice}/>
        <Advice type="dev" advice={adviceDev} />
    </div>
    {:else}
    <div class="w-full h-screen flex items-center justify-center font-outfit">
        <p class="text-2xl text-gray-600 font-medium">{$t('dashboard.my_organization.no_data')}</p>
    </div>
    {/if}
</div>