<script lang="ts">
    import { enhance } from '$app/forms';
    import { page } from '$app/stores';

    export const form: { message?: string, success?: boolean } | null = null;

    let successMessage = '';
    let errorMessage = '';
    let submitted = false;
    let codeOrganisation = '';

    $: {
        if ($page.form?.actionType === 'join_orga') {
            if ($page.form?.success) {
                successMessage = $page.form.message || 'Succès';
                errorMessage = '';
                codeOrganisation = ''; // On vide le champ après succès
            } else if ($page.form?.message) {
                errorMessage = $page.form.message;
                successMessage = '';
            }
        }
    }
</script>

<form
        method="POST"
        action="?/join_orga"
        use:enhance={() => {
            submitted = true;
            errorMessage = '';
            successMessage = '';
            return async ({ update }) => {
                submitted = false;
                update();
            };
        }}
        class="flex flex-col gap-4">

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
