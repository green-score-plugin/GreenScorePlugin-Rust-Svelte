<script lang="ts">
    import CodeClipboard from "$lib/components/CodeClipboard.svelte";

    export let code: string;

</script>

<div class="flex flex-col gap-10">
    <div class="flex flex-col gap-1 font-outfit">
        <h2 class="text-2xl font-extrabold text-grey-950">Invitez des membres !</h2>
        <p class="text-grey-500 text-sm">Envoyez ce code aux membres de votre organisation. Ils peuvent le saisir dans la section : mon compte - mon organisation - rejoindre une organisation</p>
    </div>
    <CodeClipboard code={code} />

    <a href="/" class="w-full h-fit rounded-lg bg-gs-green-950 px-1 py-2 font-semibold font-outfit text-center text-white">Terminer</a>
</div>
