<script>
    import MyInfo from './MyInfo.svelte';
    import LeftMenu from './LeftMenu.svelte';
    import salutation from '$lib/images/salutation.png';
</script>



<div  class="xl:px-52 flex flex-col">
    <div class="px-4 lg:px-16 py-8 flex justify-center lg:justify-start items-center gap-x-4">
        <img class="w-[54px] h-auto" src={salutation} alt="Salutation">
        <h1 class="text-2xl font-bold">Bonjour !</h1>
    </div>
    <div class="flex flex-col lg:flex-row px-4 lg:px-16 gap-8 lg:gap-16 mb-2">
        <div class="flex-grid h-fit min-w-60 shadow-lg bg-white items-center">
            <LeftMenu />
        </div>
        <div class="shadow-lg bg-white py-4 px-6">
            <MyInfo />
        </div>
        <div class="shadow-lg bg-white py-4 px-6">
        </div>
    </div>
</div>