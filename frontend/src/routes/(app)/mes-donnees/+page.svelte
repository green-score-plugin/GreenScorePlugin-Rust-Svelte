<script lang="ts">
    import UserAverageDailyCarbonFootprint from '$lib/components/widgets/UserAverageDailyCarbonFootprint.svelte';
    import TotalConsumption from '$lib/components/widgets/TotalConsumption.svelte';
    import ChartConsumptionFiltered from '$lib/components/widgets/ChartConsumptionFiltered.svelte';
    import BadgeGreenScore from '$lib/components/widgets/BadgeGreenScore.svelte';
    import Equivalent from '$lib/components/widgets/Equivalent.svelte';
    // import ChartTop5PollutingSites from '$lib/widgets/ChartTop5PollutingSites.svelte';
    // import Advice from '$lib/widgets/Advice.svelte';
    // import Top from '$lib/dashboards/Top.svelte';

    import type { PageData } from './$types';
    let myAverageDailyCarbonFootprint: number;
    let averageDailyCarbonFootprint: number;
    let messageAverageFootprint: string;
    let totalConsumption: number;

    export let totalConsuUnit = 'gCO2eq';
    export let title : string = 'Mes données';
    export let description : string = 'Voici une analyse détaillée de vos données de navigation : ';
    export let noDatas : boolean = false;
    export let envNomination: string = 'Maître des Forêts';
    export let letterGreenScore: string = 'A';
    export let equivalent1: { name: string; value: number; icon: string } = { name: '', value: 0, icon: '' };
    export let equivalent2: { name: string; value: number; icon: string } = { name: '', value: 0, icon: '' };

    export let dailyConsumption: Array<{ label: string; value: number }> = [];
    export let weeklyConsumption: Array<{ label: string; value: number }> = [];
    export let monthlyConsumption: Array<{ label: string; value: number }> = [];
    // Choix de la période (par défaut : mensuel)
    let selectedPeriod: 'daily' | 'weekly' | 'monthly' = 'monthly';
    export let data: PageData;
    $: ({
        myAverageDailyCarbonFootprint,
        averageDailyCarbonFootprint,
        messageAverageFootprint,
        totalConsumption,
        letterGreenScore,
        envNomination,
        equivalents,
        dailyConsumption,
        weeklyConsumption,
        monthlyConsumption
    } = data);
    $: equivalent1 = equivalents?.[0] || { name: '', value: 0, icon: '' };
    $: equivalent2 = equivalents?.[1] || { name: '', value: 0, icon: '' };
    let carbonFootprint = data.totalConsumption;
    console.log("equivalent1 : ", equivalent1);
    console.log("equivalent2 : ", equivalent2);


    $: consumptionData = selectedPeriod === 'daily' ? dailyConsumption
        : selectedPeriod === 'weekly' ? weeklyConsumption
            : monthlyConsumption;
</script>

<svelte:head>
    <title>{title} | GreenScore Web</title>
</svelte:head>

<div class="w-full bg-green-bg min-h-screen">
    <!-- Top -->
<!--    <Top {title} {description} link={null} />-->

    {#if !noDatas}
        <!-- Main Content Grid -->
        <div class="grid grid-cols-1 gap-6 p-10 sm:grid-cols-2 lg:grid-cols-12">
            <UserAverageDailyCarbonFootprint
                    {myAverageDailyCarbonFootprint}
                    {averageDailyCarbonFootprint}
                    {messageAverageFootprint}
            />
            <ChartConsumptionFiltered {consumptionData} bind:selectedPeriod />
            <BadgeGreenScore {letterGreenScore} {envNomination} />
            <Equivalent equivalent={equivalent1} order={1} />
            <TotalConsumption
                    {carbonFootprint}
                    {totalConsuUnit}
                    label="Total de votre consommation depuis la création du compte :"
            />
            <Equivalent equivalent={equivalent2} order={2} />
<!--            <ChartTop5PollutingSites {usersIdsCharts} />-->
<!--            <Advice type="lambda" {advice} />-->
<!--            <Advice type="dev" advice={adviceDev} />-->
        </div>
    {:else}
        <div class="w-full h-screen flex items-center justify-center font-outfit">
            <p class="text-2xl text-gray-600 font-medium">
                Vous n'avez pas encore utilisé notre plugin afin d'analyser votre consommation !
            </p>
        </div>
    {/if}
</div>
