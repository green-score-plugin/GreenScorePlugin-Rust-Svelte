<script lang="ts">
    import { enhance } from '$app/forms';
    import { page } from '$app/stores';
    import { invalidateAll } from '$app/navigation';


    let successMessage = '';
    let errorMessage = '';
    let submitted = false;
    let codeOrganisation = '';

    let hasLeftOrga = false;
    let showConfirmModal = false;
    let showChangeModal = false;

    $: user = $page.data.user;
    $: hasOrga = (!!user?.id_orga && !hasLeftOrga);

    $: orgaDetails = $page.data.organisation || null;

    $: {
        if ($page.form?.actionType === 'join_orga' || $page.form?.actionType === 'leave_orga' || $page.form?.actionType === 'change_orga') {
            if ($page.form?.success) {
                successMessage = $page.form.message || 'Opération réussie';
                errorMessage = '';
                codeOrganisation = '';

                if ($page.form.actionType === 'leave_orga') {
                    hasLeftOrga = true;
                    orgaDetails = null;
                } else if ($page.form.actionType === 'join_orga' || $page.form.actionType === 'change_orga') {
                    hasLeftOrga = false;
                }

                invalidateAll();
            } else if ($page.form?.message) {
                errorMessage = $page.form.message;
                successMessage = '';
            }
        }
    }

    function copyCode() {
        if (orgaDetails?.code) {
            navigator.clipboard.writeText(orgaDetails.code);
            successMessage = "Code copié dans le presse-papier !";
            setTimeout(() => successMessage = '', 3000);
        }
    }
</script>

<div class="flex flex-col gap-4">

    <h2 class="text-2xl font-bold py-2">Organisation</h2>

    {#if successMessage}
        <div class="px-4 py-3 rounded-lg bg-green-50 border border-green-200 text-green-700 text-sm">
            {successMessage}
        </div>
    {/if}

    {#if errorMessage}
        <div class="px-4 py-3 rounded-lg bg-red-50 border border-red-200 text-red-700 text-sm">
            {errorMessage}
        </div>
    {/if}

    {#if hasOrga}
        <div class="flex flex-col gap-6">
                <div class="flex flex-col gap-1">
                    <label class="text-xs font-semibold uppercase text-grey-500">Nom de l'organisation</label>
                    <div class="text-xl font-bold text-gs-green-950">{orgaDetails?.name}</div>
                </div>

                <div class="w-full flex flex-col gap-2">
                    <label for="codeDisplay" class="text-sm font-semibold text-grey-700">Code Organisation</label>
                    <div class="flex gap-2">
                        <input
                                id="codeDisplay"
                                type="text"
                                readonly
                                value={orgaDetails?.code || ''}
                                class="px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full bg-gray-50 cursor-not-allowed focus:outline-none"
                        />
                        <button
                                type="button"
                                on:click={copyCode}
                                class="px-4 py-2 rounded-lg border border-grey-200 hover:bg-gray-100 font-semibold text-grey-700 transition-colors"
                        >
                            Copier
                        </button>
                    </div>
                </div>

                <div class="flex gap-3 pt-2">
                    <button
                            type="button"
                            on:click={() => showConfirmModal = true}
                            class="flex-1 px-4 py-2 rounded-lg bg-red-600 text-white hover:bg-red-700 font-semibold transition-colors cursor-pointer"
                    >
                        Quitter
                    </button>

                    <button
                            type="button"
                            on:click={() => showChangeModal = true}
                            class="flex-1 px-4 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700 font-semibold transition-colors cursor-pointer"
                    >
                        Changer d'organisation
                    </button>
                </div>
            </div>

            {#if showConfirmModal}
                <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70">
                    <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4 shadow-lg max-h-[90vh] overflow-auto">
                        <h2 class="text-xl font-semibold mb-4">Confirmation</h2>
                        <p class="text-gray-600 mb-6">
                            Etes-vous sur de vouloir quitter l'organisation "{orgaDetails?.name || 'Inconnue'}" ?
                        </p>
                        <div class="flex justify-end gap-4">
                            <button
                                    type="button"
                                    on:click={() => showConfirmModal = false}
                                    class="px-4 py-2 rounded-lg bg-red-600 text-white hover:bg-red-700 cursor-pointer transition"
                            >
                                Annuler
                            </button>

                            <form
                                    action="?/leave_orga"
                                    method="POST"
                                    use:enhance={() => {
                                    showConfirmModal = false;
                                    return async ({ update }) => {
                                        await update();
                                    };
                                }}
                            >
                                <button
                                        type="submit"
                                        class="px-4 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700 cursor-pointer transition"
                                >
                                    Oui, quitter
                                </button>
                            </form>
                        </div>
                    </div>
                </div>
            {/if}

            {#if showChangeModal}
                <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70">
                    <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4 shadow-lg max-h-[90vh] overflow-auto">
                        <h2 class="text-xl font-semibold mb-4">Veuillez entrer le code qui vous a été envoyé par l'administrateur de votre organisation</h2>
                        <form
                                action="?/change_orga"
                                method="POST"
                                use:enhance={() => {
                                    submitted = true;
                                    return async ({ update }) => {
                                        await update();
                                        submitted = false;
                                        showChangeModal = false;
                                    };
                                }}
                                class="flex flex-col gap-4"
                        >
                            <div class="flex flex-col gap-2">
                                <label for="newCodeOrganisation" class="text-sm font-semibold text-grey-700">Code Organisation</label>
                                <input
                                        id="newCodeOrganisation"
                                        name="codeOrganisation"
                                        type="text"
                                        bind:value={codeOrganisation}
                                        placeholder="Entrez le nouveau code"
                                        class="px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none"
                                />
                                <p class="text-xs text-gray-500">Merci d'entrer le code à 8 caractères de la nouvelle organisation</p>
                            </div>
                            <div class="flex justify-end gap-4">
                                <button
                                        type="button"
                                        on:click={() => {
                                            showChangeModal = false;
                                            codeOrganisation = '';
                                        }}
                                        class="px-4 py-2 rounded-lg bg-gray-200 text-gray-700 hover:bg-gray-300 cursor-pointer transition"
                                >
                                    Annuler
                                </button>
                                <button
                                        type="submit"
                                        disabled={submitted}
                                        class="px-4 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700 cursor-pointer transition disabled:opacity-50"
                                >
                                    {#if submitted}Chargement...{:else}Confirmer{/if}
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            {/if}

    {:else}
        <form
                method="POST"
                action="?/join_orga"
                use:enhance={() => {
                    submitted = true;
                    errorMessage = '';
                    successMessage = '';
                    return async ({ update }) => {
                        await update();
                        submitted = false;
                    };
                }}
                class="flex flex-col gap-4">

            <div class="flex gap-4 w-full text-grey-700 font-outfit font-semibold text-sm sm:flex-row">
                <div class="w-full flex flex-col">
                    <label for="codeOrganisation">Code Organisation</label>
                    <input
                            id="codeOrganisation"
                            name="codeOrganisation"
                            type="text"
                            bind:value={codeOrganisation}
                            placeholder="Entrez le code organisation"
                            class="px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none"
                    />
                    <p class="text-xs text-gray-500 mt-1">Merci d'entrer le code à 8 caractères envoyé par votre organisation</p>
                </div>
            </div>

            <button
                    type="submit"
                    disabled={submitted}
                    class="w-full h-fit rounded-lg bg-gs-green-950 px-1 py-2 font-semibold font-outfit text-white
                    cursor-pointer
                    hover:bg-gs-green-800
                    active:bg-gs-green-700
                    transition-colors duration-150 ease-in-out disabled:opacity-50"
            >
                {#if submitted}Chargement...{:else}Rejoindre{/if}
            </button>
        </form>
    {/if}
</div>
