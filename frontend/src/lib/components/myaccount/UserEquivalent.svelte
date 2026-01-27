<script lang="ts">

    import { page } from "$app/state";

    interface EquivalentResponse {
        id: number;
        name: string;
        icon_thumbnail: string;
        is_selected: boolean;
    }

    let account = $derived(page);
    let accountEquivalents = $derived(page.data.accountEquivalents as EquivalentResponse[] || []);

    $inspect(accountEquivalents);
</script>

<form action=""
      method="POST"
      class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4">


    {#each accountEquivalents as accountEquivalent}
        <div class="relative flex items-center space-x-3 rounded-lg border border-gray-300 bg-white px-6 py-5 shadow-sm focus-within:ring-2  focus-within:ring-offset-2 hover:border-gray-400">
            <div class="flex-shrink-0">
                <img class="h-10 w-10 object-cover" src="/images/equivalents/{accountEquivalent.icon_thumbnail}" alt="{accountEquivalent.name}">
            </div>
            <div class="min-w-0 flex-1">
                <label for="equivalent-{accountEquivalent.id}" class="focus:outline-none">
                    <span class="absolute inset-0" aria-hidden="true"></span>
                    <span class="text-sm font-medium text-gray-900">{accountEquivalent.name}</span>
                </label>
            </div>
            <div class="flex h-5 items-center">
                 <input id="equivalent-{accountEquivalent.id}"
                        name="equivalents"
                        value={accountEquivalent.id}
                        checked={accountEquivalent.is_selected}
                        type="checkbox"
                        class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500" />
            </div>
        </div>
    {/each}

    <button type="button" class="mt-4 inline-flex justify-center rounded-md border border-transparent bg-gs-green-950 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-gs-green-800 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
        Enregistrer les modifications
    </button>

</form>
