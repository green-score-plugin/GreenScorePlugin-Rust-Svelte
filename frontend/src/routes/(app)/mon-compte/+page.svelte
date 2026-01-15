<script lang="ts">
    import LeftMenu from '$lib/components/myaccount/LeftMenu.svelte';
    import MyInfo from '$lib/components/myaccount/MyInfo.svelte';
    import MyInfoOrganisation from "$lib/components/myaccount/MyInfoOrganisation.svelte";
    import Organisation from '$lib/components/myaccount/Organisation.svelte';
    import GererMembre from '$lib/components/myaccount/GererMembre.svelte';
    import salutation from '$lib/images/salutation.png';
    import {page} from "$app/state";

    let activePage = $state("my_info");
    let user = $derived(page.data.user);

</script>

<div class="xl:px-52 flex flex-col h-full">
    <div class="px-4 lg:px-16 py-8 flex justify-center lg:justify-start items-center gap-x-4">
        <img class="w-[54px] h-auto" src={salutation} alt="Salutation">
        <h1 class="text-2xl font-bold">Bonjour {user.nom}!</h1>
    </div>

    <div class="flex flex-col lg:flex-row px-4 lg:px-16 gap-8 lg:gap-16 mb-2">

        <div class="flex-grid h-fit min-w-60 shadow-lg bg-white items-center">
            <LeftMenu bind:activePage/>
        </div>
        <div class="flex-1 shadow-lg bg-white py-4 px-6">
            {#if activePage === 'my_info'}
                {#if user.role === 'user' }
                    <MyInfo />
                {:else if user.role === 'organisation'}
                    <MyInfoOrganisation />
                {/if}
            {:else if activePage === 'organisation'}
                {#if user.role === 'user' }
                    <Organisation />
                {:else if user.role === 'organisation'}
                    <GererMembre />
                {/if}
            {/if}
        </div>
    </div>
</div>

