<script lang="ts">
    import Swiper from 'swiper';
    import { Navigation, Pagination } from 'swiper/modules';
    import 'swiper/css';
    import 'swiper/css/navigation';
    import 'swiper/css/pagination';

    import firefoxLogo from '$lib/images/firefox.png';
    import backgroundImage from '$lib/images/background.png';
    import citationImage from '$lib/images/citation.svg';
    import greenscoreImage from '$lib/images/greenscore-image.png';
    // import greenscoreLogo from '$lib/images/greenscore-logo.png';
    import homeImage1 from '$lib/images/home-image1.png';
    import homeImage2 from '$lib/images/home-image2.png';
    import homeImage3 from '$lib/images/home-image3.png';
    import homeImage4 from '$lib/images/home-image4.png';
    import homeImage5 from '$lib/images/home-image5.png';
    import homeImageMobile1 from '$lib/images/home-image-mobile1.png';
    import homeImageMobile2 from '$lib/images/home-image-mobile2.png';
    import homeImageMobile3 from '$lib/images/home-image-mobile3.png';
    // import registerImage1 from '$lib/images/register-image1.png';
    import { onMount, tick } from "svelte";
    import { t } from 'svelte-i18n';


    interface AdviceItem {
        title: string;
        advice: string;
        icon: string;
        is_dev: boolean;
    }

    export let advice: AdviceItem[] = [];

    export let isLoggedIn: boolean = false;

    let isDevMode: boolean = false;
    let showVideo: boolean = false;
    let swiper: Swiper | null = null;

    $: filteredAdvice = advice.filter(item =>
        isDevMode ? item.is_dev : !item.is_dev
    );

    async function initializeSwiper() {
        if (swiper) {
            swiper.destroy(true, true);
            swiper = null;
        }

        await tick();

        swiper = new Swiper(".advice-swiper", {
            modules: [Navigation, Pagination],
            slidesPerView: 1,
            spaceBetween: 8,
            observer: true,
            observeParents: true,
            pagination: {
                el: ".swiper-pagination",
                clickable: true,
            },
            navigation: {
                nextEl: ".custom-next-button",
                prevEl: ".custom-prev-button",
            },
            breakpoints: {
                640: { slidesPerView: 1, spaceBetween: 8 },
                1024: { slidesPerView: 2, spaceBetween: 8 },
                1280: { slidesPerView: 3, spaceBetween: 8 },
            },
            speed: 400,
            rewind: true,
        });
    }

    onMount(async () => {
        await initializeSwiper();
    });

    async function toggleDevMode() {
        isDevMode = !isDevMode;

        await tick();

        // Détruire complètement l'instance Swiper
        if (swiper) {
            swiper.destroy(true, true);
            swiper = null;
        }

        // Petite pause pour laisser le DOM se mettre à jour
        await new Promise(resolve => setTimeout(resolve, 50));

        // Réinitialiser complètement Swiper
        await initializeSwiper();
    }

</script>

<svelte:head>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.2/css/all.min.css" />
    <meta name="google-site-verification" content="dJm0vNbi9V4IoEj0eTdExvhVHDS8N_ek9uLC1qNTvyw" />
</svelte:head>

<div class="min-h-screen bg-gs-green-950 text-white font-['Ovo']">
    <!-- Hero Section -->
    <div  style="background-image: url({backgroundImage});" class="bg-cover flex flex-col items-center pt-20 pb-0 px-4 sm:px-6 lg:px-8 h-[calc(100vh-96px)] overflow-hidden md:justify-between">
        <div class="max-w-4xl w-full text-center md:w-[703px]">
            <h1 class="text-4xl text-gs-green-950 mb-4 font-outfit font-extrabold sm:text-5xl">
                {$t('home.hero.title')}
            </h1>

            <p class="text-xl text-gs-green-950 mb-8 font-outfit font-regular">
                {$t('home.hero.subtitle')}
            </p>

            <div class="hidden lg:block">
                <a href="https://addons.mozilla.org/fr/firefox/addon/greenscoreplugin/" class="inline-flex items-center bg-white text-gs-green-950 font-outfit font-regular p-3 rounded-full mx-auto shadow-md shadow-black">
                    <img src="{firefoxLogo}" alt="Firefox" class="mr-2 h-6 w-6">
                    {$t('home.hero.add_to_firefox')}
                </a>
            </div>

            <div class="mt-8 lg:hidden flex justify-center">
                <a href="https://addons.mozilla.org/fr/firefox/addon/greenscoreplugin/" class="inline-flex items-center bg-white text-gs-green-950 font-outfit font-regular p-3 rounded-full mx-auto shadow-md shadow-black">
                    <img src="{firefoxLogo}" alt="Firefox" class="mr-2 h-6 w-6">
                    {$t('home.hero.add_to_firefox')}
                </a>
            </div>
        </div>

        <!-- Images Gallery -->
        <div class="w-full mt-12 md:mt-18">
            <!-- Mobile Gallery -->
            <div class="flex justify-between space-x-2 md:hidden">
                <div class="w-[106px] h-[300px] rounded-2xl overflow-hidden mt-3">
                    <img src="{homeImageMobile1}" alt="Interface mobile 1" class="w-full h-full">
                </div>
                <div class="w-[106px] h-[300px] rounded-2xl overflow-hidden">
                    <img src="{homeImageMobile2}" alt="Interface mobile 2" class="w-full h-full">
                </div>
                <div class="w-[106px] h-[300px] rounded-2xl overflow-hidden mt-3">
                    <img src="{homeImageMobile3}" alt="Interface mobile 3" class="w-full h-full">
                </div>
            </div>

            <!-- Desktop Gallery -->
            <div class="hidden md:flex flex-wrap justify-center md:justify-between lg:justify-between gap-2 md:gap-3 lg:gap-4 max-w-7xl mx-auto px-2 mb-0">
                <div class="hidden lg:block">
                    <div class="overflow-hidden rounded-2xl w-[180px] lg:w-[230px] h-[414px]">
                        <img src="{homeImage1}" alt="Interface application 1" class="w-full h-full object-fill">
                    </div>
                </div>

                <div class="hidden md:block pt-8">
                    <div class="overflow-hidden rounded-2xl w-[180px] lg:w-[230px] h-[414px]">
                        <img src="{homeImage2}" alt="Interface application 2" class="w-full h-full object-fill">
                    </div>
                </div>

                <div class="block">
                    <div class="overflow-hidden rounded-2xl w-[140px] md:w-[180px] lg:w-[230px] h-[270px] md:h-[414px]">
                        <img src="{homeImage3}" alt="Interface application 3" class="w-full h-full object-fill">
                    </div>
                </div>

                <div class="block pt-8">
                    <div class="overflow-hidden rounded-2xl w-[140px] md:w-[180px] lg:w-[230px] h-[270px] md:h-[414px]">
                        <img src="{homeImage4}" alt="Interface application 4" class="w-full h-full object-fill">
                    </div>
                </div>

                <div class="block mb-0 md:hidden lg:block">
                    <div class="overflow-hidden rounded-2xl w-[140px] md:w-[180px] lg:w-[230px] h-[270px] md:h-[414px]">
                        <img src="{homeImage5}" alt="Interface application 5" class="w-full h-full object-fill">
                    </div>
                </div>
            </div>
        </div>
    </div>

    <!-- Video Section -->
    <div class="container mx-auto px-4 md:px-16 py-12">
        <div class="grid lg:grid-cols-2 gap-8">
            <div class="bg-white rounded-lg overflow-hidden block">
                <div class="text-black font-outfit font-regular px-4 py-2 flex items-center gap-2">
                    <div class="w-4 h-4 bg-[#6D874B] rounded-full"></div>
                    <span class="text-xs font-bold">{$t('home.video.tag')}</span>
                </div>
                <div class="h-[200px] lg:h-[400px] flex items-center justify-center overflow-hidden bg-black relative">
                    {#if showVideo}
                        <iframe
                                width="100%"
                                height="100%"
                                src="https://www.youtube-nocookie.com/embed/lAYfNt7web8?autoplay=1&mute=1&controls=0&loop=1&playlist=lAYfNt7web8&modestbranding=1&showinfo=0&rel=0&iv_load_policy=3"
                                title="GreenScore Web Vidéo"
                                style="border:0;"
                                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                                referrerpolicy="strict-origin-when-cross-origin"
                                allowfullscreen
                                class="w-full h-full"
                        ></iframe>
                    {:else}
                        <button
                                class="w-full h-full relative group cursor-pointer focus:outline-none"
                                on:click={() => showVideo = true}
                                aria-label="Lancer la vidéo"
                        >
                            <img
                                    src="https://img.youtube.com/vi/lAYfNt7web8/hqdefault.jpg"
                                    alt="Aperçu vidéo"
                                    class="w-full h-full object-cover opacity-90 group-hover:opacity-100 transition-opacity"
                            />
                            <div class="absolute inset-0 flex items-center justify-center">
                                <div class="w-16 h-16 bg-[#6D874B] rounded-full flex items-center justify-center shadow-lg group-hover:scale-110 transition-transform">
                                    <i class="fa-solid fa-play text-white text-2xl ml-1"></i>
                                </div>
                            </div>
                        </button>
                    {/if}
                </div>
            </div>

            <div class="flex flex-col text-center justify-center">
                <h1 class="text-3xl md:text-4xl mb-6">{$t('home.video.title')}</h1>
                <p class="text-lg md:text-xl mb-6 px-4 md:px-2 font-outfit font-extralight">
                    {$t('home.video.description')}
                    <span class="hidden md:inline">{$t('home.video.description_more')}</span>
                </p>
                {#if !isLoggedIn}
                    <div class="hidden lg:block">
                        <a href="/login" class="inline-flex items-center bg-white text-gs-green-950 font-outfit font-regular py-3 px-5 rounded-full mx-auto shadow-md shadow-black">
                            <span class="mr-4">{$t('home.video.register')}</span>
                            <span class="bg-gray-200 px-3 py-1.5 rounded-full flex items-center justify-center">
                <i class="fa-solid fa-arrow-right"></i>
              </span>
                        </a>
                    </div>
                {/if}
            </div>
        </div>
    </div>

    <!-- Statistics Section -->
    <div class="bg-white text-gs-green-950 py-16 relative">
        <div class="absolute inset-0 -z-0 hidden xl:block overflow-hidden items-center justify-center">
            <img src="{citationImage}" class="w-10/12 mx-auto h-full object-cover md:object-contain" alt="Background">
        </div>

        <div class="container mx-auto text-center relative z-1">
            <h2 class="text-3xl md:text-5xl font-regular mb-12 relative px-4 md:px-[40px] font-outfit">
                {$t('home.stats.title')}
            </h2>

            <div class="flex flex-col md:flex-row justify-center items-center gap-8 mb-12">
                <div class="text-center">
                    <div class="flex flex-row items-baseline justify-center">
                        <h3 class="text-6xl md:text-8xl text-gs-green-950 mr-2">{$t('home.stats.users_count')}</h3>
                        <p class="text-2xl md:text-3xl">{$t('home.stats.users_label')}</p>
                    </div>
                    <p class="max-w-70 text-[13px]">{$t('home.stats.users_desc')}</p>
                </div>

                <div class="hidden md:block w-12 border-t border-dashed border-gray-500"></div>

                <div class="text-center">
                    <div class="flex flex-row items-baseline justify-center">
                        <h3 class="text-6xl md:text-8xl text-gs-green-950 mr-2">{$t('home.stats.devices_count')}</h3>
                        <p class="text-2xl md:text-3xl">{$t('home.stats.devices_label')}</p>
                    </div>
                    <p class="max-w-56 text-[13px]">{$t('home.stats.devices_desc')}</p>
                </div>

                <div class="hidden md:block w-12 border-t border-dashed border-gray-500"></div>

                <div class="text-center">
                    <div class="flex flex-row items-baseline justify-center">
                        <h3 class="text-6xl md:text-8xl text-gs-green-950 mr-2">{$t('home.stats.images_count')}</h3>
                        <p class="text-2xl md:text-3xl">{$t('home.stats.images_label')}</p>
                    </div>
                    <p class="max-w-56 text-[13px]">{$t('home.stats.images_desc')}</p>
                </div>
            </div>

            <div class="flex flex-col md:flex-row justify-center items-center gap-8">
                <div class="text-center">
                    <div class="flex flex-row items-baseline justify-center">
                        <h3 class="text-6xl md:text-8xl text-gs-green-950 mr-2">{$t('home.stats.video_count')}</h3>
                        <p class="text-2xl md:text-3xl">{$t('home.stats.video_label')}</p>
                    </div>
                    <p class="max-w-70 text-[13px]">{$t('home.stats.video_desc')}</p>
                </div>

                <div class="hidden md:block w-12 border-t border-dashed border-gray-500"></div>

                <div class="text-center">
                    <div class="flex flex-row items-baseline justify-center">
                        <h3 class="text-6xl md:text-8xl text-gs-green-950 mr-2">{$t('home.stats.reduction_count')}</h3>
                        <p class="text-2xl md:text-3xl">{$t('home.stats.reduction_label')}</p>
                    </div>
                    <p class="max-w-64 text-[13px]">{$t('home.stats.reduction_desc')}</p>
                </div>
            </div>
        </div>
    </div>

    <!-- Advice Section -->
    <section id="advice" class="bg-[#F5F7FF] text-gs-green-950 py-12">
        <div class="container mx-auto px-4 md:px-16">
            <div class="flex justify-center items-center mb-8">
                <h2 class="text-3xl font-ovo font-regular text-center mr-4">
                    {isDevMode ? $t('home.advice.title_dev') : $t('home.advice.title_gen')}
                </h2>
                <button
                        on:click={toggleDevMode}
                        class="bg-black text-white text-xs rounded-full px-4 py-2 flex items-center hover:cursor-pointer hover:scale-105 transition-all ease_in_out duration-200"
                >
                    {#if isDevMode}
            <span class="mr-2">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5 inline">
                <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6.75A3.75 3.75 0 1 1 12 3a3.75 3.75 0 0 1 3.75 3.75ZM19.5 21v-1.5a6 6 0 0 0-6-6h-3a6 6 0 0 0-6 6V21"/>
              </svg>
            </span>
                        {$t('home.advice.for_users')}
                    {:else}
                        <span class="mr-2">&lt;/&gt;</span>
                        {$t('home.advice.for_devs')}
                    {/if}
                </button>
            </div>

            <div class="relative px-4 md:px-16 flex flex-col justify-center w-full max-w-6xl mx-auto">
                {#if filteredAdvice.length > 0}
                    <div class="swiper advice-swiper w-full !pt-6 !pb-14 !px-4 -mx-4">
                        <div class="swiper-wrapper">
                            {#each filteredAdvice as item}
                                <div class="swiper-slide !h-auto">
                                    <div class="bg-white h-full w-full rounded-xl shadow-md p-6 flex flex-col sm:flex-row items-center justify-center text-center sm:text-left gap-4 transition-transform hover:scale-[1.02]">
                                        <div class="bg-[#F5F7FF] p-3 rounded-lg w-20 h-20 flex-shrink-0 flex items-center justify-center">
                                            <i class="{item.icon} text-[#6D874B] text-3xl"></i>
                                        </div>
                                        <div>
                                            <h3 class="font-outfit font-semibold text-lg mb-2 text-gray-900">{item.title}</h3>
                                            <p class="text-sm leading-relaxed text-gray-600">{item.advice}</p>
                                        </div>
                                    </div>
                                </div>
                            {/each}
                        </div>

                        <!-- Pagination inside swiper container for proper positioning relative to slides -->
                        <div class="swiper-pagination !bottom-0 [&_.swiper-pagination-bullet-active]:!bg-[#6D874B] [&_.swiper-pagination-bullet]:!bg-gray-300 [&_.swiper-pagination-bullet]:!opacity-100"></div>
                    </div>

                    <!-- Navigation buttons outside swiper container -->
                    <button class="custom-prev-button absolute left-0 md:-left-6 top-[calc(50%-1rem)] -translate-y-1/2 z-10 w-10 h-10 md:w-12 md:h-12 flex items-center justify-center bg-white text-[#6D874B] rounded-full shadow-lg hover:bg-[#6D874B] hover:text-white transition-all duration-300 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed" aria-label="Conseil précédent">
                        <i class="fa-solid fa-chevron-left text-sm md:text-base"></i>
                    </button>
                    <button class="custom-next-button absolute right-0 md:-right-6 top-[calc(50%-1rem)] -translate-y-1/2 z-10 w-10 h-10 md:w-12 md:h-12 flex items-center justify-center bg-white text-[#6D874B] rounded-full shadow-lg hover:bg-[#6D874B] hover:text-white transition-all duration-300 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed" aria-label="Conseil suivant">
                        <i class="fa-solid fa-chevron-right text-sm md:text-base"></i>
                    </button>
                {:else}
                    <div class="text-center py-8">
                        <p class="text-gray-500">{$t('home.advice.no_advice')}</p>
                    </div>
                {/if}
            </div>
        </div>
    </section>

    <!-- GreenScore Explanation -->
    <div id="greenscore" class="flex flex-col md:flex-row mx-auto gap-6 bg-white w-full font-outfit">
        <div class="w-full p-8 md:w-2/3 flex flex-col space-y-8 md:p-16">
            <div>
                <h1 class="text-4xl font-regular text-gray-800 text-center mb-4">{$t('home.greenscore.title')}</h1>
                <p class="text-gray-700">
                    {$t('home.greenscore.description')}
                </p>
            </div>

            <div>
                <h2 class="text-2xl font-bold text-gray-800 mb-4">{$t('home.greenscore.calc_title')}</h2>
                <p class="text-gray-700 mb-6">
                    {$t('home.greenscore.calc_desc')}
                </p>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="bg-gray-100 rounded-lg p-4 flex items-center gap-3">
                        <i class="fa-solid fa-leaf fa-2xl text-[#72A52D]"></i>
                        <p class="text-gray-700">{$t('home.greenscore.calc_items.resources')}</p>
                    </div>

                    <div class="bg-gray-100 rounded-lg p-4 flex items-center gap-3">
                        <i class="fa-solid fa-database fa-2xl text-[#72A52D]"></i>
                        <p class="text-gray-700">{$t('home.greenscore.calc_items.transfer')}</p>
                    </div>

                    <div class="bg-gray-100 rounded-lg p-4 flex items-center gap-3">
                        <i class="fa-solid fa-server fa-2xl text-[#72A52D]"></i>
                        <p class="text-gray-700">{$t('home.greenscore.calc_items.requests')}</p>
                    </div>

                    <div class="bg-gray-100 rounded-lg p-4 flex items-center gap-3">
                        <i class="fa-solid fa-clock fa-2xl text-[#72A52D]"></i>
                        <p class="text-gray-700">{$t('home.greenscore.calc_items.loading_time')}</p>
                    </div>
                </div>

                <p class="mt-6 text-gray-700">
                    {@html $t('home.greenscore.calc_conclusion')}
                </p>

                <div class="mt-4 bg-green-50 border-l-4 border-[#72A52D] p-4 rounded">
                    <p class="text-gray-700">
                        <span class="font-semibold">À noter :</span> Ce calcul fournit une <span class="font-semibold">estimation</span> et non une mesure exacte. Toutes les données nécessaires pour un calcul précis de l'empreinte carbone ne sont pas disponibles. Néanmoins, ces résultats permettent de <span class="font-semibold">comparer efficacement</span> la consommation énergétique entre différentes pages web et d'identifier les axes d'amélioration.
                    </p>
                </div>
            </div>

            <div>
                <h2 class="text-4xl font-regular text-gray-800 text-center mb-4">{$t('home.greenscore.scale.title')}</h2>
                <p class="text-gray-700 mb-6">
                    {$t('home.greenscore.scale.description')}
                </p>

                <!-- Mobile Grid -->
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 text-center sm:hidden">
                    <div class="bg-green-500 text-gray-800 rounded-lg p-4">
                        <h3 class="font-bold">A (≤ 0,25 gCO<sub>2</sub>e)</h3>
                        <p>{$t('home.greenscore.scale.grades.A')}</p>
                    </div>

                    <div class="bg-green-400 text-gray-800 rounded-lg p-4">
                        <h3 class="font-bold">B (≤ 0,50 gCO<sub>2</sub>e)</h3>
                        <p>{$t('home.greenscore.scale.grades.B')}</p>
                    </div>

                    <div class="bg-yellow-400 text-gray-800 rounded-lg p-4">
                        <h3 class="font-bold">C (≤ 0,75 gCO<sub>2</sub>e)</h3>
                        <p>{$t('home.greenscore.scale.grades.C')}</p>
                    </div>

                    <div class="bg-orange-400 text-gray-800 rounded-lg p-4">
                        <h3 class="font-bold">D (≤ 1,00 gCO<sub>2</sub>e)</h3>
                        <p>{$t('home.greenscore.scale.grades.D')}</p>
                    </div>

                    <div class="bg-orange-600 text-gray-800 rounded-lg p-4">
                        <h3 class="font-bold">E (≤ 1,25 gCO<sub>2</sub>e)</h3>
                        <p>{$t('home.greenscore.scale.grades.E')}</p>
                    </div>

                    <div class="bg-red-500 text-gray-800 rounded-lg p-4">
                        <h3 class="font-bold">F (≤ 1,50 gCO<sub>2</sub>e)</h3>
                        <p>{$t('home.greenscore.scale.grades.F')}</p>
                    </div>

                    <div class="bg-red-700 text-white rounded-lg p-4">
                        <h3 class="font-bold">G (≤ 1,75 gCO<sub>2</sub>e)</h3>
                        <p>{$t('home.greenscore.scale.grades.G')}</p>
                    </div>
                </div>

                <!-- Desktop Scale -->
                <div class="hidden sm:block">
                    <div class="relative w-full mb-24">
                        <div class="flex justify-between text-sm text-gray-600 px-2">
                            <span>{$t('home.greenscore.scale.labels.low')}</span>
                            <span>{$t('home.greenscore.scale.labels.critical')}</span>
                        </div>
                        <div class="flex w-full h-16">
                            <div class="relative flex-1 group cursor-pointer">
                                <div class="absolute inset-0 bg-green-500">
                                    <div class="h-full flex items-center justify-center">
                                        <span class="font-bold text-white text-xl">A</span>
                                    </div>
                                </div>
                                <div class="absolute pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-200 bg-white rounded-lg shadow-lg p-3 ml-2 w-48 z-20" style="top: 100%; margin-top: 8px;">
                                    <h3 class="font-bold text-gray-800">A (≤ 0,25 gCO<sub>2</sub>e)</h3>
                                    <p class="text-gray-700">{$t('home.greenscore.scale.grades.A')}</p>
                                </div>
                            </div>

                            <div class="relative flex-1 group cursor-pointer -ml-4">
                                <div class="absolute inset-0 bg-green-400">
                                    <div class="h-full flex items-center justify-center">
                                        <span class="font-bold text-white text-xl">B</span>
                                    </div>
                                </div>
                                <div class="absolute pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-200 bg-white rounded-lg shadow-lg p-3 ml-2 w-48 z-20" style="top: 100%; margin-top: 8px;">
                                    <h3 class="font-bold text-gray-800">B (≤ 0,50 gCO<sub>2</sub>e)</h3>
                                    <p class="text-gray-700">{$t('home.greenscore.scale.grades.B')}</p>
                                </div>
                            </div>

                            <div class="relative flex-1 group cursor-pointer -ml-4">
                                <div class="absolute inset-0 bg-yellow-400">
                                    <div class="h-full flex items-center justify-center">
                                        <span class="font-bold text-white text-xl">C</span>
                                    </div>
                                </div>
                                <div class="absolute pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-200 bg-white rounded-lg shadow-lg p-3 ml-2 w-48 z-20" style="top: 100%; margin-top: 8px;">
                                    <h3 class="font-bold text-gray-800">C (≤ 0,75 gCO<sub>2</sub>e)</h3>
                                    <p class="text-gray-700">{$t('home.greenscore.scale.grades.C')}</p>
                                </div>
                            </div>

                            <div class="relative flex-1 group cursor-pointer -ml-4">
                                <div class="absolute inset-0 bg-orange-400">
                                    <div class="h-full flex items-center justify-center">
                                        <span class="font-bold text-white text-xl">D</span>
                                    </div>
                                </div>
                                <div class="absolute pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-200 bg-white rounded-lg shadow-lg p-3 ml-2 w-48 z-20" style="top: 100%; margin-top: 8px;">
                                    <h3 class="font-bold text-gray-800">D (≤ 1,00 gCO<sub>2</sub>e)</h3>
                                    <p class="text-gray-700">{$t('home.greenscore.scale.grades.D')}</p>
                                </div>
                            </div>

                            <div class="relative flex-1 group cursor-pointer -ml-4">
                                <div class="absolute inset-0 bg-orange-600">
                                    <div class="h-full flex items-center justify-center">
                                        <span class="font-bold text-white text-xl">E</span>
                                    </div>
                                </div>
                                <div class="absolute pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-200 bg-white rounded-lg shadow-lg p-3 ml-2 w-48 z-20" style="top: 100%; margin-top: 8px;">
                                    <h3 class="font-bold text-gray-800">E (≤ 1,25 gCO<sub>2</sub>e)</h3>
                                    <p class="text-gray-700">{$t('home.greenscore.scale.grades.E')}</p>
                                </div>
                            </div>

                            <div class="relative flex-1 group cursor-pointer -ml-4">
                                <div class="absolute inset-0 bg-red-500">
                                    <div class="h-full flex items-center justify-center">
                                        <span class="font-bold text-white text-xl">F</span>
                                    </div>
                                </div>
                                <div class="absolute pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-200 bg-white rounded-lg shadow-lg p-3 ml-2 w-48 z-20" style="top: 100%; margin-top: 8px;">
                                    <h3 class="font-bold text-gray-800">F (≤ 1,50 gCO<sub>2</sub>e)</h3>
                                    <p class="text-gray-700">{$t('home.greenscore.scale.grades.F')}</p>
                                </div>
                            </div>

                            <div class="relative flex-1 group cursor-pointer -ml-4">
                                <div class="absolute inset-0 bg-red-700 [clip-path:polygon(0%_0%,85%_0%,100%_50%,85%_100%,0%_100%)]">
                                    <div class="h-full flex items-center justify-center">
                                        <span class="font-bold text-white text-xl">G</span>
                                    </div>
                                </div>
                                <div class="absolute pointer-events-none opacity-0 right-3 md:right-auto group-hover:opacity-100 transition-opacity duration-200 bg-white rounded-lg shadow-lg p-3 ml-0 w-48 z-20" style="top: 100%; margin-top: 8px;">
                                    <h3 class="font-bold text-gray-800">G (≤ 1,75 gCO<sub>2</sub>e)</h3>
                                    <p class="text-gray-700">{$t('home.greenscore.scale.grades.G')}</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div class="hidden w-full md:w-1/3 md:block">
            <div class="h-full w-full overflow-hidden">
                <img src="{greenscoreImage}" alt="Plantes vertes grimpant sur un tronc d'arbre" class="w-full h-full object-cover" />
            </div>
        </div>
    </div>
</div>

