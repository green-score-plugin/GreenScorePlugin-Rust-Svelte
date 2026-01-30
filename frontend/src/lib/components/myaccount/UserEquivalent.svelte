<script lang="ts">

    import { page } from "$app/state";
    import { enhance } from '$app/forms';
    import { fade } from 'svelte/transition';
    import { t } from 'svelte-i18n';

    interface EquivalentResponse {
        id: number;
        name: string;
        icon_thumbnail: string;
        is_selected: boolean;
    }

    let accountEquivalents = $derived(page.data.accountEquivalents as EquivalentResponse[] || []);
    let form = $derived(page.form);

    let loading = $state(false);
    let message = $derived(form?.actionType === 'modification_equivalents' ? form.message : null);
    let isSuccess = $derived(form?.actionType === 'modification_equivalents' ? form?.success : false);

    $inspect(accountEquivalents);
</script>

<div class="flex flex-col gap-4 font-outfit">
    <h1 class="text-2xl font-semibold">Mes équivalents</h1>

    <form action="?/modification_equivalents"
          method="POST"
          use:enhance={() => {
              loading = true;
              return async ({ update }) => {
                  await update({ reset: false });
                  loading = false;
              };
          }}
          class="flex flex-col gap-4">

        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
            {#each accountEquivalents as accountEquivalent}
                <div class="relative flex items-center space-x-3 rounded-lg border border-gray-300 bg-white px-6 py-5 shadow-sm hover:border-gray-400 has-[:focus-visible]:ring-2 has-[:focus-visible]:ring-gs-green-950 has-[:checked]:border-gs-green-950 has-[:checked]:ring-1 has-[:checked]:ring-gs-green-950 has-[:checked]:bg-green-50">
                    <div class="flex-shrink-0">
                        <img class="h-10 w-10 object-cover" src="/images/equivalents/{accountEquivalent.icon_thumbnail}" alt="{accountEquivalent.name}">
                    </div>
                    <div class="min-w-0 flex-1">
                        <label for="equivalent-{accountEquivalent.id}" class="cursor-pointer">
                            <span class="absolute inset-0" aria-hidden="true"></span>
                            <span class="text-sm font-medium text-gray-900">{$t(accountEquivalent.name)}</span>
                        </label>
                    </div>
                    <div class="flex h-5 items-center">
                        <input id="equivalent-{accountEquivalent.id}"
                               name="equivalents"
                               value={accountEquivalent.id}
                               checked={accountEquivalent.is_selected}
                               type="checkbox"
                               class="h-4 w-4 rounded border-gray-300 text-gs-green-950" />
                    </div>
                </div>
            {/each}
        </div>

        <div class="flex items-center justify-end gap-3 mt-4 border-t border-gray-200 pt-4">
            {#if message}
                <div in:fade class="text-sm font-medium {isSuccess ? 'text-green-600' : 'text-red-600'}">
                    {$t(message)}
                </div>
            {/if}

            <button type="submit"
                    disabled={loading}
                    class="hover:cursor-pointer inline-flex justify-center rounded-lg border border-transparent bg-gs-green-950 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-gs-green-800 focus:outline-none focus:ring-2 focus:ring-gs-green-950 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed transition-all">

                {#if loading}
                    <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                {/if}

                {loading ? $t("account.equivalent.message.loading") : $t("account.equivalent.button")}
            </button>
        </div>

    </form>
</div>
