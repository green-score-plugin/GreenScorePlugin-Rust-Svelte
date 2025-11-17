<script lang="ts">
    import imageFond from '$lib/images/register-image1.png';
    import logo from '$lib/images/greenscore-logo.png';
</script>

<div class="flex items-center w-screen h-screen">
    <img src="{imageFond}" alt="GreenScore" class="hidden xl:flex w-1/2 h-full object-cover">
    <div class="w-full xl:w-1/2 h-full px-6 md:px-[120px] py-24 flex flex-col items-center justify-center">
        <div class="flex flex-col w-full gap-10">
            <div class="flex gap-2 items-center">
                <img width="55px" height="auto" src="{logo}" alt="GreenScore Logo">
                <h1 class="text-3xl font-outfit font-extrabold text-gs-green-950">GreenScore Web</h1>
            </div>
        </div>
    </div>
</div>