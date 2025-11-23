<script lang="ts">
    import imageFond from '$lib/images/register-image1.png';
    import logo from '$lib/images/greenscore-logo.png';

    export let title: string;
    export let subtitle: string;
    export let form: { message?: string } | null = null;
    export let showModeSwitcher = false;
</script>

<title>{title}</title>

<div class="flex items-center w-screen h-screen">
    <img src="{imageFond}" alt="GreenScore" class="hidden xl:flex w-1/2 h-full object-cover">
    <div class="w-full xl:w-1/2 h-full px-6 md:px-[120px] py-24 flex flex-col items-center justify-center">
        <div class="flex flex-col w-full gap-10">
            <div class="flex gap-2 items-center">
                <img width="55px" height="auto" src="{logo}" alt="GreenScore Logo">
                <h1 class="text-3xl font-outfit font-extrabold text-gs-green-950">GreenScore Web</h1>
            </div>

            <div class="flex flex-col gap-4 font-outfit">
                <h2 class="text-2xl font-extrabold text-grey-950">{subtitle}</h2>

                {#if showModeSwitcher}
                    <div class="flex gap-[10px] px-1 border border-grey-200 rounded-full py-1 w-fit font-medium">
                        <slot name="mode-switcher"></slot>
                    </div>
                {/if}
            </div>

            {#if form?.message}
                <div class="w-full bg-red-50 text-red-700 text-sm font-outfit font-medium border border-red-700 rounded-lg px-6 py-6">
                    {form.message}
                </div>
            {/if}

            <slot name="form"></slot>
        </div>
    </div>
</div>
