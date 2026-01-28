<script lang="ts">
    import InfosGreenScore from '$lib/components/widgets/InfosGreenScore.svelte';
    import CountryCarbonIntensity from '$lib/components/widgets/CountryCarbonIntensity.svelte';
    import BadgeGreenScore from '$lib/components/widgets/BadgeGreenScore.svelte';
    import Equivalent from '$lib/components/widgets/Equivalent.svelte';
    import TotalConsumption from '$lib/components/widgets/TotalConsumption.svelte';
    import PageInNumbers from '$lib/components/widgets/PageInNumbers.svelte';
    import Advice from '$lib/components/widgets/Advice.svelte';
    import { t } from 'svelte-i18n';

    export let noDatas : boolean = false;
    export let letterGreenScore: string = 'A';
    export let link: string | null = null;
    export let country: string = 'France';
    export let carbonFootprint: number = 0;
    export let totalConsuUnit: string = 'gCO2eq';
    export let pageSize: number = 0;
    export let pageSizeUnit: string = 'Ko';
    export let loadingTime: number = 0;
    export let queriesQuantity: number = 0;
    export let flagUrl: string = 'https://flagcdn.com/fr.svg';
    export let adviceUser: string = "Baissez la luminosité de vos écrans.";
    export let adviceDev: string = "Optimisez vos requêtes SQL.";
    export let carbonIntensity: number = 0;
    export let envNomination: string = 'nominations.page.A';
    export let equivalent1: { name: string; value: number; icon: string } = { name: '', value: 0, icon: '' };
    export let equivalent2: { name: string; value: number; icon: string } = { name: '', value: 0, icon: '' };

    import type { PageData } from './$types';
    export let data: PageData;

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
        envNomination = data.envNomination || 'nominations.page.A';
        equivalent1 = data.equivalents[0]
        equivalent2 = data.equivalents[1]
    }
</script>

<svelte:head>
    <title>{$t('dashboard.last_page.title')} | GreenScore Web</title>
</svelte:head>

<div class="w-full bg-green-bg min-h-screen">
    <div class="w-full text-center px-10 lg:px-0 pt-10 font-outfit flex items-center justify-center flex-col text-grey-950">
        <h1 class="text-4xl font-bold">{$t('dashboard.last_page.title')}</h1>
        {#if !noDatas}
        <p class="text-base w-fit">
            {$t('dashboard.last_page.description')}
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
            <TotalConsumption {carbonFootprint} {totalConsuUnit} label={$t('dashboard.last_page.carbon_emission')} />
            <Equivalent equivalent={equivalent2} order={2} />
            <PageInNumbers {pageSize} {loadingTime} {queriesQuantity} {pageSizeUnit} />
            <Advice type="nav" advice={adviceUser} />
            <Advice type="dev" advice={adviceDev} />
        </div>
    {:else}
        <div class="w-full h-screen flex items-center justify-center font-outfit">
            <p class="text-2xl text-gray-600 font-medium">
                {$t('dashboard.last_page.no_data')}
            </p>
        </div>
    {/if}
</div>