<script lang="ts">
    import { enhance } from '$app/forms';

    export let activePage = "my_info";

    let showDeleteModal = false;
    let submitted = false;
</script>

<div class="flex flex-col gap-2 p-2">
    <button
            type="button"
            class="flex items-center gap-x-2 px-8 py-4 rounded w-full cursor-pointer
               {activePage === 'my_info' ? 'bg-gs-green-950 text-white' : 'bg-gray-100 hover:bg-gray-200 text-black'}"
            on:click={() => activePage = 'my_info'}
    >
        <svg width="24" height="24" viewBox="0 0 25 24" fill="none">
            <circle cx="12.5" cy="12" r="11.5" stroke={activePage === 'my_info' ? 'white' : 'black'} />
            <circle cx="12.5" cy="9" r="2.5" stroke={activePage === 'my_info' ? 'white' : 'black'} />
            <path d="M18.5 18C18.5 14.4999 16.3182 12.0001 12.5 12" stroke={activePage === 'my_info' ? 'white' : 'black'} />
            <path d="M6.5 18C6.5 14.4999 8.68182 12.0001 12.5 12" stroke={activePage === 'my_info' ? 'white' : 'black'} />
        </svg>
        Mon compte
    </button>

    <button
            type="button"
            class="flex items-center gap-x-2 px-8 py-4 rounded w-full cursor-pointer
               {activePage === 'organisation' ? 'bg-gs-green-950 text-white' : 'bg-gray-100 hover:bg-gray-200 text-black'}"
            on:click={() => activePage = 'organisation'}
    >
        <svg width="23" height="26" viewBox="0 0 23 26" fill="none" xmlns="http://www.w3.org/2000/svg">
        <mask id="path-1-inside-1_180_1116" fill="white">
            <path d="M14.7969 6.59302H20.8549C21.9595 6.59302 22.8549 7.48845 22.8549 8.59302V23.6393C22.8549 24.7438 21.9595 25.6393 20.8549 25.6393H14.7969V6.59302Z"/>
        </mask>
        <path d="M14.7969 6.59302H20.8549C21.9595 6.59302 22.8549 7.48845 22.8549 8.59302V23.6393C22.8549 24.7438 21.9595 25.6393 20.8549 25.6393H14.7969V6.59302Z" fill="white"/>
        <path d="M14.7969 5.59302H20.8549C22.5118 5.59302 23.8549 6.93616 23.8549 8.59302H21.8549C21.8549 8.04073 21.4072 7.59302 20.8549 7.59302H14.7969V5.59302ZM23.8549 23.6393C23.8549 25.2961 22.5118 26.6393 20.8549 26.6393H14.7969V24.6393H20.8549C21.4072 24.6393 21.8549 24.1915 21.8549 23.6393H23.8549ZM14.7969 25.6393V6.59302V25.6393ZM20.8549 5.59302C22.5118 5.59302 23.8549 6.93616 23.8549 8.59302V23.6393C23.8549 25.2961 22.5118 26.6393 20.8549 26.6393V24.6393C21.4072 24.6393 21.8549 24.1915 21.8549 23.6393V8.59302C21.8549 8.04073 21.4072 7.59302 20.8549 7.59302V5.59302Z" fill="black" mask="url(#path-1-inside-1_180_1116)"/>
        <path d="M0.645508 2C0.645508 1.17157 1.31708 0.5 2.14551 0.5H13.529C14.3574 0.5 15.029 1.17157 15.029 2V25.1392H2.14551C1.31708 25.1392 0.645508 24.4676 0.645508 23.6392V2Z" fill="white" stroke="black"/>
        <rect x="3.57617" y="20.2788" width="2.66274" height="2.66274" rx="0.5" fill="white" stroke="black"/>
        <rect x="3.57617" y="14.4185" width="2.66274" height="2.66274" rx="0.5" fill="white" stroke="black"/>
        <rect x="3.57617" y="8.55811" width="2.66274" height="2.66274" rx="0.5" fill="white" stroke="black"/>
        <rect x="3.57617" y="2.69775" width="2.66274" height="2.66274" rx="0.5" fill="white" stroke="black"/>
        <rect x="9.43652" y="20.2788" width="2.66274" height="2.66274" rx="0.5" fill="white" stroke="black"/>
        <rect x="9.43652" y="14.4185" width="2.66274" height="2.66274" rx="0.5" fill="white" stroke="black"/>
        <rect x="9.43652" y="8.55811" width="2.66274" height="2.66274" rx="0.5" fill="white" stroke="black"/>
        <rect x="17.4941" y="8.55811" width="2.66274" height="2.66274" rx="0.5" fill="white" stroke="black"/>
        <rect x="17.4941" y="14.4185" width="2.66274" height="2.66274" rx="0.5" fill="white" stroke="black"/>
        <rect x="17.4941" y="20.2788" width="2.66274" height="2.66274" rx="0.5" fill="white" stroke="black"/>
        <rect x="9.43652" y="2.69775" width="2.66274" height="2.66274" rx="0.5" fill="white" stroke="black"/>
        </svg>
        Mon organisation
    </button>


    <button
            type="button"
            class="flex items-center gap-x-2 px-8 py-4 text-red-600 hover:bg-red-50 transition-colors w-full justify-center cursor-pointer"
            on:click={() => showDeleteModal = true}
    >
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2M10 11v6M14 11v6"/>
        </svg>
        <span class="text-base whitespace-nowrap">Supprimer le compte</span>
    </button>
</div>

{#if showDeleteModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70">
        <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4 shadow-lg max-h-[90vh] overflow-auto">
            <h2 class="text-xl font-semibold mb-4">Confirmer la suppression</h2>
            <p class="text-gray-600 mb-6">
                Êtes-vous sûr de vouloir supprimer votre compte utilisateur ?
            </p>
            <div class="flex justify-end gap-4">
                <button
                        type="button"
                        class="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg cursor-pointer transition"
                        on:click={() => showDeleteModal = false}
                >
                    Annuler
                </button>

                <form
                        method="POST"
                        action="?/supprimer"
                        use:enhance={() => {
                            submitted = true;
                        }}
                        class="flex flex-col gap-4">

                <button
                        type="submit"
                        class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 cursor-pointer transition"
                >
                    Supprimer
                </button>
                </form>
            </div>
        </div>
    </div>
{/if}

