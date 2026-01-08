<script lang="ts">
    // import Top from './dashboards/Top.svelte';
    import InfosGreenScore from '$lib/components/widgets/InfosGreenScore.svelte';
    import CountryCarbonIntensity from '$lib/components/widgets/CountryCarbonIntensity.svelte';
    import BadgeGreenScore from '$lib/components/widgets/BadgeGreenScore.svelte';
    import Equivalent from '$lib/components/widgets/Equivalent.svelte';
    import TotalConsumption from '$lib/components/widgets/TotalConsumption.svelte';
    import PageInNumbers from '$lib/components/widgets/PageInNumbers.svelte';
    import Advice from '$lib/components/widgets/Advice.svelte';

    export let title : string = 'Dernière page consultée';
    export let description : string = 'Voici une analyse détaillée de votre dernière page consultée : ';
    export let noDatas : boolean = false;
    export let link : string;

    export let letterGreenScore : string;
    export let country : string = 'France';
    export let carbonIntensity : number;
    export let flagUrl : string;
    export let envNomination : string;
    export let equivalent1Value: number = 5;
    export let equivalent1Name: string = 'km en voiture thermique';
    export let equivalent1Icon: string | null = 'co2.svg';

    $: equivalent1 = {
        name: equivalent1Name,
        value: equivalent1Value,
        icon: equivalent1Icon
    };
    export let totalConsu : number = 13;
    export let totalConsuUnit = 'gCO2eq';
    export let equivalent2Value: number = 12;
    export let equivalent2Name: string = 'litres d\'eau potable';
    export let equivalent2Icon: string | null = 'eau.svg';

    $: equivalent2 = {
        name: equivalent2Name,
        value: equivalent2Value,
        icon: equivalent2Icon
    };
    export let pageSize: number;
    export let loadingTime: number;
    export let queriesQuantity: number;
    export let pageSizeUnit: string = 'Ko';
    export let adviceUser: string;
    export let adviceDev: string;
    export let equivalents: Array<[string, string, string]> = [];

    export let carbonFootprint: number;

    import type { PageData } from './$types';
    export let data: PageData;

    console.log(data.equivalents[0]);
    $: if (data.pageData) {
        letterGreenScore = data.pageData.letterGreenScore || 'A';
        link = data.pageData.link;
        country = data.pageData.country || 'France';
        carbonFootprint = data.pageData.carbonFootprint || 0;
        pageSize = data.pageData.pageSize || 0;
        loadingTime = data.pageData.loadingTime || 0;
        queriesQuantity = data.pageData.queriesQuantity || 0;
        flagUrl = data.pageData.flagUrl || 'https://flagcdn.com/fr.svg';
        adviceUser = data.adviceUser || "Baissez la luminosité de vos écrans.";
        adviceDev = data.adviceDev || "Optimisez vos requêtes SQL.";
        carbonIntensity = data.pageData.carbonIntensity || 0;
        letterGreenScore = data.letterGreenScore || 'A';
        envNomination = data.envNomination || 'Maître des Forêts';
        equivalent1 = data.equivalents[0]
        equivalent2 = data.equivalents[1]
    }
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
            <BadgeGreenScore {letterGreenScore} {envNomination} />
            <Equivalent equivalent={equivalent1} order={1} />
            <TotalConsumption {carbonFootprint} {totalConsuUnit} label="Emission carbone de la page :" />
            <Equivalent equivalent={equivalent2} order={2} />
            <PageInNumbers {pageSize} {loadingTime} {queriesQuantity} {pageSizeUnit} />
            <Advice type="nav" advice={adviceUser} />
            <Advice type="dev" advice={adviceDev} />
        </div>
    {:else}
        <div class="w-full h-screen flex items-center justify-center font-outfit">
            <p class="text-2xl text-gray-600 font-medium">
                Vous n'avez pas encore analysé de pages web avec notre plugin
            </p>
        </div>
    {/if}
</div>