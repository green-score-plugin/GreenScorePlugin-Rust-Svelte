<script>
    let codeOrganisation = '';

    const handleSubmit = (event) => {
        event.preventDefault();
        console.log('Code organisation soumis :', codeOrganisation);
        // Ici tu peux appeler ton backend
    };
</script>

<h2 class="text-2xl font-bold py-2">Organisation</h2>

<form on:submit={handleSubmit} class="flex flex-col gap-4">
    <!-- Code organisation -->
    <div class="flex gap-4 w-full text-grey-700 font-outfit font-semibold text-sm sm:flex-row">
        <div class="w-full flex flex-col">
            <label for="codeOrganisation">Code Organisation</label>
            <input
                    id="codeOrganisation"
                    type="text"
                    bind:value={codeOrganisation}
                    placeholder="Entrez le code organisation"
                    class="px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none"
            />
            <!-- Erreur exemple -->
            <!-- <div class="text-red-700 text-sm">Code invalide</div> -->
        </div>
    </div>

    <button
            type="submit"
            class="w-full h-fit rounded-lg bg-gs-green-950 px-1 py-2 font-semibold font-outfit text-white
               cursor-pointer
               hover:bg-gs-green-800
               active:bg-gs-green-700
               transition-colors duration-150 ease-in-out"
    >
        Rejoindre
    </button>
</form>
