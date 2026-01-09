<script lang="ts">
    import OrganizationAverageDailyCarbonFootprint from "$lib/components/widgets/OrganizationAverageDailyCarbonFootprint.svelte";
    export let title : string = 'Mon Organisation';
    export let description : string = 'Toutes les données sur les membres de : ORGANISATION A RÉCUPÉRER';
    export let noDatas : boolean = false;


    export let averageFootprint: number = 10;
    export let equivalentAverage: { name: string; value: number; icon: string } = { name: 'km en voiture', value: 5, icon: 'car.png' };

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
        <!--{% include 'widgets/_organisation_average_daily_carbon_footprint.html.twig' with {averageFootprint: averageFootprint, equivalentAverage: equivalentAverage} %}-->
        <!--{% include 'widgets/_chart_consumption_filtered.html.twig' with {usersIdsCharts: usersIdsCharts} %}-->
        <!--{% include 'widgets/_badge_green_score.html.twig' with {letterGreenScore: letterGreenScore, envNomination: envNomination} %}-->
        <!--{% include 'widgets/_equivalent.html.twig' with {equivalent: equivalent1, order: 1} %}-->
        <!--{% include 'widgets/_total_consumption.html.twig' with {totalConsu: totalConsu, totalConsuUnit: totalConsuUnit, label:'Total de la consommation de votre organisation depuis la création du compte :'} %}-->
        <!--{% include 'widgets/_equivalent.html.twig' with {equivalent: equivalent2, order: 2} %}-->
        <!--{% include 'widgets/_chart_top5_polluting_sites.html.twig' with {usersIdsCharts: usersIdsCharts} %}-->
        <!--{% include 'widgets/_advice.html.twig' with {type: "lambda", advice: advice} %}-->
        <!--{% include 'widgets/_advice.html.twig' with {type: "dev", advice: adviceDev} %}-->

        <OrganizationAverageDailyCarbonFootprint {averageFootprint} {equivalentAverage}/>
    </div>
    {:else}
    <div class="w-full h-screen flex items-center justify-center font-outfit">
        <p class="text-2xl text-gray-600 font-medium">Votre organisation n'a pas de données à analyser ou vous n'êtes pas relié à une organisation</p>
    </div>
    {/if}
</div>