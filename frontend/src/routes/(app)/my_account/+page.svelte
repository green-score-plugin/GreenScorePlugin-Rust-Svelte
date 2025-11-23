<script>
    import LeftMenu from './LeftMenu.svelte';
    import MyInfo from './MyInfo.svelte';
    import Organisation from './Organisation.svelte';
    import salutation from '$lib/images/salutation.png';

    // Variable pour savoir quel composant afficher
    let activePage = 'my_info';

    // Modal supprimer compte
    let showModal = false;
</script>

<div class="xl:px-52 flex flex-col">
    <!-- Header -->
    <div class="px-4 lg:px-16 py-8 flex justify-center lg:justify-start items-center gap-x-4">
        <img class="w-[54px] h-auto" src={salutation} alt="Salutation">
        <h1 class="text-2xl font-bold">Bonjour !</h1>
    </div>

    <!-- Layout principal -->
    <div class="flex flex-col lg:flex-row px-4 lg:px-16 gap-8 lg:gap-16 mb-2">

        <!-- Menu gauche -->
        <div class="flex-grid h-fit min-w-60 shadow-lg bg-white items-center">
            <LeftMenu bind:activePage />
        </div>

        <!-- Contenu central -->
        <div class="flex-1 shadow-lg bg-white py-4 px-6">
            {#if activePage === 'my_info'}
                <MyInfo />
            {:else if activePage === 'organisation'}
                <Organisation />
            {/if}
        </div>
    </div>
</div>

<!-- Modal Supprimer compte -->
{#if showModal}
    <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
        <div class="bg-white p-4 rounded shadow-lg w-80">
            <h2 class="text-lg font-bold mb-2">Supprimer le compte</h2>
            <p class="text-sm mb-4">Es-tu sûr de vouloir supprimer votre compte ?</p>
            <div class="flex justify-end gap-2">
                <button class="px-4 py-2 bg-gray-200 rounded" on:click={() => showModal = false}>Annuler</button>
                <button class="px-4 py-2 bg-red-600 text-white rounded" on:click={() => {
                    console.log('Compte supprimé');
                    showModal = false;
                }}>Supprimer</button>
            </div>
        </div>
    </div>
{/if}
