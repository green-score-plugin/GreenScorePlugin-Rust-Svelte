<script lang="ts">
    import CodeClipboard from "$lib/components/CodeClipboard.svelte";
    import { t } from 'svelte-i18n';

    export let code: string;

</script>

<div class="flex flex-col gap-10">
    <div class="flex flex-col gap-1 font-outfit">
        <h2 class="text-2xl font-extrabold text-grey-950">{$t('account.organization.invite_title')}</h2>
        <p class="text-grey-500 text-sm">{$t('account.organization.invite_desc')}</p>
    </div>
    <CodeClipboard code={code} />

    <a href="/" class="w-full h-fit rounded-lg bg-gs-green-950 px-1 py-2 font-semibold font-outfit text-center text-white">{$t('account.organization.finish_button')}</a>
</div>
