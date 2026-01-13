<script lang="ts">
    import {page} from "$app/state";
    import CodeClipboard from "$lib/components/CodeClipboard.svelte";
    import type { Organisation } from "$lib/types/account";

    let user = $derived(page.data.user as Organisation);

</script>

<div class="flex flex-col gap-4" data-controller="copy-clipboard">
    <h1 class="font-outfit text-2xl font-semibold">Membres</h1>
    <div class="flex flex-col gap-4">
        <p class="text-sm text-gray-500">Vous n'avez pas encore ajouté de membres à votre organisation. Envoyez leur le code suivant afin qu'ils puissent rejoindre votre organisation.</p>
        <CodeClipboard code={user.code} />
    </div>
</div>