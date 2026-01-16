<script lang="ts">
    import clipboard from '$lib/images/clipboard.svg';
    import { copyToClipboard} from "$lib/utils/clipboard.ts";

    let { code }: { code?: string } = $props();
    let copied = $state(false);
    async function handleCopy() {
        if (!code) return;

        await copyToClipboard(code);
        copied = true;
        setTimeout(() => {
            copied = false;
        }, 2000);
    }
</script>

<div class="flex rounded-2xl border border-grey-100 items-center w-full">
    <div class="border-r border-grey-100 w-full items-center justify-center py-4">
        <p id="code-text" class="text-grey-950 text-center text-3xl font-outfit font-semibold tracking-widest">{code}</p>
    </div>
    <div class="px-4 py-4 flex flex-col items-center justify-center relative">
        <button type="button" onclick={handleCopy}
                class="cursor-pointer transition-transform duration-200 hover:scale-110 bg-transparent border-0 p-0"
                aria-label="Copier le code"
        >
            <img src={clipboard} alt="clipboard" class="scale-105"/>
        </button>

        <i class="fa-solid fa-check text-gs-green-950 absolute text-2xl animate-pulse invisible" id="check-icon"></i>
    </div>
</div>