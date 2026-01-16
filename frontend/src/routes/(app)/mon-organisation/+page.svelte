<script lang="ts">
    import OrganizationAverageDailyCarbonFootprint from "$lib/components/widgets/OrganizationAverageDailyCarbonFootprint.svelte";
    import ChartConsumptionFiltered from "$lib/components/widgets/ChartConsumptionFiltered.svelte";
    import BadgeGreenScore from "$lib/components/widgets/BadgeGreenScore.svelte";
    import Equivalent from "$lib/components/widgets/Equivalent.svelte";
    import TotalConsumption from "$lib/components/widgets/TotalConsumption.svelte";
    import ChartTop5PollutingSites from "$lib/components/widgets/ChartTop5PollutingSites.svelte";
    import Advice from "$lib/components/widgets/Advice.svelte";
    import type { PageData } from './$types';

    export let data: PageData;

    export let totalConsuUnit = 'gCO2eq';
    export let title: string = 'Mon Organisation';
    export let description: string = 'Toutes les données sur les membres de : ';
    export let noDatas: boolean = false;
    export let label: string = 'Total de la consommation de votre organisation depuis la création du compte :';

    let selectedPeriod: 'daily' | 'weekly' | 'monthly' = 'monthly';

    $: description = 'Toutes les données sur les membres de : ' + data.organisationData!.name;
    $: averageFootprint = data.organisationData!.averageDailyCarbonFootprint;
    $: equivalentAverage = data.organisationData!.equivalent;
    $: carbonFootprint = (data.organisationData!.totalCarbonFootprint ?? 0).toFixed(2);
    $: letterGreenScore = data.letterGreenScore || 'A';
    $: envNomination = data.envNomination || 'Maître des Forêts';
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
</script>

<svelte:head>
    <title>{title} | GreenScore Web</title>
</svelte:head>

<div class="w-full bg-green-bg min-h-screen">
    <div class="w-full text-center px-10 lg:px-0 pt-10 font-outfit flex items-center justify-center flex-col text-grey-950">
        <h1 class="text-4xl font-bold">{title}</h1>
        {#if !noDatas}
            <p class="text-base w-fit">
            { description }
            </p>
        {/if}
    </div>

    {#if !noDatas}
    <div class="grid grid-cols-1 gap-6 p-10 sm:grid-cols-2 lg:grid-cols-12">
        <OrganizationAverageDailyCarbonFootprint {averageFootprint} {equivalentAverage}/>
        <ChartConsumptionFiltered {consumptionData} bind:selectedPeriod/>
        <BadgeGreenScore {letterGreenScore} {envNomination}/>
        <Equivalent equivalent={equivalent1} order={1} />
        <TotalConsumption {carbonFootprint} {totalConsuUnit} {label} />
        <Equivalent equivalent={equivalent2} order={2} />
        <ChartTop5PollutingSites {topPollutingSites} />
        <Advice type="nav" advice={advice}/>
        <Advice type="dev" advice={adviceDev} />
    </div>
    {:else}
    <div class="w-full h-screen flex items-center justify-center font-outfit">
        <p class="text-2xl text-gray-600 font-medium">Votre organisation n'a pas de données à analyser ou vous n'êtes pas relié à une organisation</p>
    </div>
    {/if}
</div>