<script lang="ts">
    // import Top from './dashboards/Top.svelte';
    import InfosGreenScore from '$lib/components/widgets/InfosGreenScore.svelte';
    import CountryCarbonIntensity from '$lib/components/widgets/CountryCarbonIntensity.svelte';
    // import BadgeGreenScore from './widgets/BadgeGreenScore.svelte';
    // import Equivalent from './widgets/Equivalent.svelte';
    // import TotalConsumption from './widgets/TotalConsumption.svelte';
    // import PageInNumbers from './widgets/PageInNumbers.svelte';
    // import Advice from './widgets/Advice.svelte';

    export let title : string = 'Dernière page consultée';
    export let description : string = 'Voici une analyse détaillée de votre dernière page consultée : ';
    export let link : string = 'https://exemple.com';
    export let noDatas : boolean = false;
    export let letterGreenScore : string = 'G';
    export let country : string = 'France';
    export let carbonIntensity = 56;
    export let flagUrl : string = 'https://flagcdn.com/fr.svg';
    // export let envNomination;
    // export let equivalent1;
    // export let totalConsu;
    // export let totalConsuUnit;
    // export let equivalent2;
    // export let pageSize;
    // export let loadingTime;
    // export let queriesQuantity;
    // export let pageSizeUnit;
    // export let advice;
    // export let adviceDev;
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
            {#if link}
            <span class="text-base ml-1 w-52 truncate inline-block align-bottom text-left">
                    { link }
                </span>
            {/if}
        </p>
        {/if}
    </div>

    {#if !noDatas}
        <!-- Main Content Grid -->
        <div class="grid grid-cols-1 gap-6 p-10 sm:grid-cols-2 lg:grid-cols-12">
            <InfosGreenScore {letterGreenScore} />
            <CountryCarbonIntensity {country} {carbonIntensity} {flagUrl} />
<!--            <BadgeGreenScore {letterGreenScore} {envNomination} />-->
<!--            <Equivalent equivalent={equivalent1} order={1} />-->
<!--            <TotalConsumption {totalConsu} {totalConsuUnit} label="Emission carbone de la page :" />-->
<!--            <Equivalent equivalent={equivalent2} order={2} />-->
<!--            <PageInNumbers {pageSize} {loadingTime} {queriesQuantity} {pageSizeUnit} />-->
<!--            <Advice type="lambda" {advice} />-->
<!--            <Advice type="dev" advice={adviceDev} />-->
        </div>
    {:else}
        <div class="w-full h-screen flex items-center justify-center font-outfit">
            <p class="text-2xl text-gray-600 font-medium">
                Vous n'avez pas encore analysé de pages web avec notre plugin
            </p>
        </div>
    {/if}
</div>