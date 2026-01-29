<script lang="ts">
    import UserAverageDailyCarbonFootprint from '$lib/components/widgets/UserAverageDailyCarbonFootprint.svelte';
    import TotalConsumption from '$lib/components/widgets/TotalConsumption.svelte';
    import ChartConsumptionFiltered from '$lib/components/widgets/ChartConsumptionFiltered.svelte';
    import BadgeGreenScore from '$lib/components/widgets/BadgeGreenScore.svelte';
    import Equivalent from '$lib/components/widgets/Equivalent.svelte';
    import ChartTop5PollutingSites from '$lib/components/widgets/ChartTop5PollutingSites.svelte';
    import Advice from "$lib/components/widgets/Advice.svelte";
    import type {PageData} from './$types';
    import { t } from 'svelte-i18n';

    export let data: PageData;

    export let totalConsuUnit = 'gCO2eq';
    // export let title: string = 'Mes données';
    // export let description: string = 'Voici une analyse détaillée de vos données de navigation : ';
    export let noDatas: boolean = false;

    let selectedPeriod: 'daily' | 'weekly' | 'monthly' = 'monthly';

    $: myAverageDailyCarbonFootprint = data.myAverageDailyCarbonFootprint || 0;
    $: averageDailyCarbonFootprint = data.averageDailyCarbonFootprint || 0;
    $: messageAverageFootprint = data.messageAverageFootprint || '';
    $: letterGreenScore = data.letterGreenScore || 'A';
    $: envNomination = data.envNomination || 'nominations.profile.A';
    $: dailyConsumption = data.dailyConsumption || [];
    $: weeklyConsumption = data.weeklyConsumption || [];
    $: monthlyConsumption = data.monthlyConsumption || [];
    $: topPollutingSites = data.topPollutingSites || [];
    $: adviceUser = data.adviceUser || '';
    $: adviceDev = data.adviceDev || '';

    $: equivalent1 = data.equivalents?.[0] || {name: '', value: 0, icon: ''};
    $: equivalent2 = data.equivalents?.[1] || {name: '', value: 0, icon: ''};
    $: carbonFootprint = data.totalConsumption;

    $: consumptionData = selectedPeriod === 'daily' ? dailyConsumption
        : selectedPeriod === 'weekly' ? weeklyConsumption
            : monthlyConsumption;
</script>

<svelte:head>
    <title>{$t('dashboard.my_data.title')} | GreenScore Web</title>
</svelte:head>

<div class="w-full bg-green-bg min-h-screen">
    <div class="w-full text-center px-10 lg:px-0 pt-10 font-outfit flex items-center justify-center flex-col text-grey-950">
        <h1 class="text-4xl font-bold">{$t('dashboard.my_data.title')}</h1>
        {#if !noDatas}
            <p class="text-base w-fit">
                {$t('dashboard.my_data.description')}
            </p>
        {/if}
    </div>
    {#if !noDatas}
        <div class="p-10 space-y-6">
            <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
                <div class="lg:col-span-1 h-full">
                    <UserAverageDailyCarbonFootprint
                            {myAverageDailyCarbonFootprint}
                            {averageDailyCarbonFootprint}
                            {messageAverageFootprint}
                    />
                </div>
                <div class="lg:col-span-2 h-full">
                    <ChartConsumptionFiltered {consumptionData} bind:selectedPeriod/>
                </div>

                <div class="lg:col-span-1 h-full">
                    <BadgeGreenScore {letterGreenScore} {envNomination}/>
                </div>
            </div>
            <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
                <div class="lg:col-span-1 h-full">
                    <Equivalent equivalent={equivalent1} order={1}/>
                </div>

                <div class="lg:col-span-2 h-full">
                    <TotalConsumption
                            {carbonFootprint}
                            {totalConsuUnit}
                            label={$t('dashboard.my_data.total_consumption')}
                    />
                </div>

                <div class="lg:col-span-1 h-full">
                    <Equivalent equivalent={equivalent2} order={2}/>
                </div>
            </div>
            <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
                <div class="lg:col-span-2 h-full">
                    <ChartTop5PollutingSites {topPollutingSites}/>
                </div>

                <div class="lg:col-span-1 h-full">
                    <Advice type="nav" advice={adviceUser}/>
                </div>

                <div class="lg:col-span-1 h-full">
                    <Advice type="dev" advice={adviceDev}/>
                </div>
            </div>
        </div>
    {:else}
        <div class="w-full h-screen flex items-center justify-center font-outfit">
            <p class="text-2xl text-gray-600 font-medium">
                {$t('dashboard.my_data.no_data')}
            </p>
        </div>
    {/if}
</div>

