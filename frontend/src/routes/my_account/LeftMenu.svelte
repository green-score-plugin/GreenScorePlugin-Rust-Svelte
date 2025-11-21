<script>
    let showModal = false;
</script>

<div class="max-w-sm mx-auto flex flex-col gap-2 p-0">

    <!-- Mon compte -->
    <a href="/my_account" class="flex justify-center items-center gap-x-2 px-8 py-4  bg-gs-green-950 text-white rounded w-full">
        <svg width="24" height="24" viewBox="0 0 25 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <circle cx="12.5" cy="12" r="11.5" stroke="white"/>
            <circle cx="12.5" cy="9" r="2.5" stroke="white"/>
            <path d="M18.5 18C18.5 14.4999 16.3182 12.0001 12.5 12" stroke="white"/>
            <path d="M6.5 18C6.5 14.4999 8.68182 12.0001 12.5 12" stroke="white"/>
        </svg>
        <span class="text-sm font-medium">Mon compte</span>
    </a>

    <!-- Mon organisation -->
    <a href="/my_account/organisation" class="flex justify-center items-center gap-x-2 px-8 py-4 bg-gray-100 hover:bg-gray-200 rounded w-full">
        <svg width="24" height="24" viewBox="0 0 23 26" fill="none" xmlns="http://www.w3.org/2000/svg">
            <rect x="0.5" y="0.5" width="22" height="25" fill="white" stroke="black"/>
            <rect x="3" y="3" width="2" height="2" fill="black"/>
            <rect x="3" y="7" width="2" height="2" fill="black"/>
        </svg>
        <span class="text-sm font-medium">Mon organisation</span>
    </a>

    <!-- Bouton Supprimer le compte -->
    <button
            on:click={() => showModal = true}
            class="flex items-center gap-x-2 px-8 py-4 text-red-600 hover:bg-red-50 transition-colors w-full justify-center rounded"
    >
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2M10 11v6M14 11v6"/>
        </svg>
        <span class="text-sm font-medium">Supprimer le compte</span>
    </button>

    <!-- Modal -->
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

</div>
