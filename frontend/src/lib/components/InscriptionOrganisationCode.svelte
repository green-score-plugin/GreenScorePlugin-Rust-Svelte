<script lang="ts">
    import clipboard from '$lib/images/clipboard.svg';

    export let code: string;

    function copyToClipboard() {
        navigator.clipboard.writeText(code);
        const checkIcon = document.getElementById('check-icon');
        if (checkIcon) {
            checkIcon.classList.remove('invisible');
            setTimeout(() => checkIcon.classList.add('invisible'), 2000);
        }
    }
</script>

<div class="flex flex-col gap-10">
    <div class="flex flex-col gap-1 font-outfit">
        <h2 class="text-2xl font-extrabold text-grey-950">Invitez des membres !</h2>
        <p class="text-grey-500 text-sm">Envoyez ce code aux membres de votre organisation. Ils peuvent le saisir dans la section : mon compte - mon organisation - rejoindre une organisation</p>
    </div>
    <div class="flex rounded-2xl border border-grey-100 items-center w-full">
        <div class="border-r border-grey-100 w-full items-center justify-center py-4">
            <p id="code-text" class="text-grey-950 text-center text-3xl font-outfit font-semibold tracking-widest">{code}</p>
        </div>
        <div class="px-4 py-4 flex flex-col items-center justify-center relative">
            <button type="button" onclick={copyToClipboard}
                class="cursor-pointer transition-transform duration-200 hover:scale-110 bg-transparent border-0 p-0"
                aria-label="Copier le code"
            >
                <img src={clipboard} alt="clipboard" class="scale-105"/>
            </button>

            <i class="fa-solid fa-check text-gs-green-950 absolute text-2xl animate-pulse invisible" id="check-icon"></i>
        </div>
    </div>

    <a href="/" class="w-full h-fit rounded-lg bg-gs-green-950 px-1 py-2 font-semibold font-outfit text-center text-white">Terminer</a>
</div>
