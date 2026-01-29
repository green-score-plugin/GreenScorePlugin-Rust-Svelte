<script lang="ts">
    import { page } from "$app/state";
    import CodeClipboard from "$lib/components/CodeClipboard.svelte";
    import type { Organisation } from "$lib/types/account";
    import { enhance } from '$app/forms';
    import { t } from 'svelte-i18n';


    let organisation = $derived(page.data.user as Organisation);
    let form = $derived(page.form);
    let message = $derived(form?.actionType === 'update_orga' ? form.message : null);
    let isSuccess = $derived(form?.actionType === 'update_orga' && form?.success);

</script>

<form action="?/modifier_orga"
      method="POST"
      class="flex flex-col gap-4"
      use:enhance={() => {
          return async ({ update }) => {
              await update();
          };
      }}
>

    <h1 class="text-2xl font-bold py-2">{$t('account.info.title')}</h1>

    {#if message}
        <div class={`px-4 py-3 rounded-lg border text-sm ${
            isSuccess
                ? 'bg-green-50 border-green-200 text-green-700'
                : 'bg-red-50 border-red-200 text-red-700'
        }`}>
            {$t(message)}
        </div>
    {/if}

    <div class="flex gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full flex flex-col">
            <label for="organisationName">{$t('account.organization.name_label')}</label>
            <input
                    id="organisationName"
                    name="organisationName"
                    type="text"
                    bind:value={organisation.nom}
                    class="px-4 py-2 border border-grey-200 rounded-lg w-full focus:outline-none"
                    placeholder="Nom de l'organisation"
            />
        </div>
    </div>

    <div class="flex gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full flex flex-col">
            <label for="siret">{$t('auth.register.siret')}</label>
            <input
                    id="siret"
                    name="siret"
                    type="text"
                    bind:value={organisation.siret}
                    class="px-4 py-2 border border-grey-200 rounded-lg w-full focus:outline-none"
                    placeholder="Votre SIRET"
            />
        </div>
    </div>


    <CodeClipboard code={organisation.code} />

    <button
            type="submit"
            class="mt-4 w-full h-fit rounded-lg bg-gs-green-950 px-1 py-2 font-semibold font-outfit text-white
               cursor-pointer hover:bg-gs-green-800 active:bg-gs-green-700
               transition-colors duration-150 ease-in-out disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
    >
            {$t('account.info.button_validate')}
    </button>

</form>