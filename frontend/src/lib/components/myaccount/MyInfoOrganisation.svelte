<script lang="ts">
    import { page } from "$app/state";
    import CodeClipboard from "$lib/components/CodeClipboard.svelte";
    import type { Organisation } from "$lib/types/account";
    import { enhance } from '$app/forms';
    import { t } from 'svelte-i18n';

    let { organisation } = $props();
    let form = $derived(page.form);
    let message = $derived(form?.actionType === 'update_orga' ? form.message : null);
    let isSuccess = $derived(form?.actionType === 'update_orga' && form?.success);

    let siret = $state('');
    let submitted = $state(false);


    $effect(() => {
        siret = organisation?.siret ?? '';
    });

    function formatSiret(value: string) {
        const raw = value.replace(/\D/g, '').substring(0, 14);
        const parts = [];
        if (raw.length > 0) parts.push(raw.substring(0, 3));
        if (raw.length > 3) parts.push(raw.substring(3, 6));
        if (raw.length > 6) parts.push(raw.substring(6, 9));
        if (raw.length > 9) parts.push(raw.substring(9, 14));
        return parts.join(' ');
    }

    function handleSiretInput(event: Event) {
        const input = event.target as HTMLInputElement;
        const formatted = formatSiret(input.value);
        siret = formatted;
        input.value = formatted;
        if (submitted) submitted = false;
    }

    let cleanSiret = $derived(siret.replace(/\s/g, ''));
    let isSiretValid = $derived(cleanSiret === '' || /^\d{14}$/.test(cleanSiret));
    let errors = $derived({ siret: !isSiretValid });

</script>

<form action="?/modifier_orga"
      method="POST"
      class="flex flex-col gap-4"
      use:enhance={({ formData, cancel }) => {
          submitted = true;

          const currentSiretValues = formData.get('siret')?.toString().replace(/\s/g, '') || '';
          const siretValid = currentSiretValues === '' || /^\d{14}$/.test(currentSiretValues);

          if (!siretValid) {
              cancel();
              return;
          }

          if (currentSiretValues) {
              formData.set('siret', currentSiretValues);
          } else {
              formData.delete('siret');
          }

          return async ({ update }) => {
              await update();
          };
      }}
>

    <h1 class="text-2xl font-bold py-2">{$t('account.info.title')}</h1>
    <input type="hidden" name="id" value={organisation?.id} />

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
                    value={siret}
                    oninput={handleSiretInput}
                    id="siret"
                    name="siret"
                    type="text"
                    class="px-4 py-2 border border-grey-200 rounded-lg w-full focus:outline-none {submitted && errors.siret ? 'border-red-700 bg-red-50' : 'border-grey-200'}"
                    placeholder="Votre SIRET"
            />
            {#if submitted && errors.siret} <span class="text-red-500 text-xs mt-0.5 block">{$t('errors.validation_siret_format')}</span> {/if}
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