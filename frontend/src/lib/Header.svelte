
<script>
    import { onMount } from 'svelte';

    let user = null;
    let profileMenuOpen = false;
    let mobileMenuOpen = false;

    function toggleProfileMenu() {
        profileMenuOpen = !profileMenuOpen;
    }

    function toggleMobileMenu() {
        mobileMenuOpen = !mobileMenuOpen;
    }
</script>


<header class="flex w-full bg-white py-5 border-b border-grey-100 items-center h-24 relative">
    <div class="flex items-center w-full justify-between px-8">
        <!-- Logo -->
        <a class="cursor-pointer" href="/">
            <img width="55" src="/images/greenscore-logo.png" alt="Logo" />
        </a>

        <!-- Menu desktop -->
        <div class="hidden lg:flex gap-8 items-center">
            <ul class="flex font-outfit text-grey-950 items-center gap-8 text-lg font-medium">
                <li><a href="/">Accueil</a></li>
                <li><a href="/mes-donnees">Mes données</a></li>
                <li><a href="/organisation">Mon organisation</a></li>
                <li><a href="/derniere-page">Dernière page consultée</a></li>

                {#if user}
                    <li class="relative">
                        <button on:click={toggleProfileMenu} class="focus:outline-none">
                            <svg width="34" height="35" viewBox="0 0 34 35" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <rect x="0.5" y="1" width="33" height="33" rx="16.5" stroke="#030712" />
                                <path d="M27 26.5C27 20.6665 23.3636 16.5001 17 16.5" stroke="#233430" />
                                <path d="M7 26.5C7 20.6665 10.6364 16.5001 17 16.5" stroke="#233430" />
                                <circle cx="17" cy="11.5" r="4.5" stroke="#233430" />
                            </svg>
                        </button>

                        {#if profileMenuOpen}
                            <div class="absolute right-0 mt-2 w-48 bg-white rounded-lg shadow-lg py-2 z-20">
                                {#if user.role === 'ROLE_ORGANISATION'}
                                    <a href="/gerer-organisation" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Gérer mon organisation</a>
                                {:else}
                                    <a href="/mon-compte" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Gérer mon compte</a>
                                {/if}
                                <a href="/logout" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Se déconnecter</a>
                            </div>
                        {/if}
                    </li>
                {:else}
                    <li><a href="/login">Se connecter</a></li>
                {/if}
            </ul>
        </div>

        <!-- Menu burger -->
        <div class="lg:hidden flex items-center">
            <button on:click={toggleMobileMenu} class="focus:outline-none">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 7.5h16.5m-16.5 4.5h16.5m-16.5 4.5h16.5" />
                </svg>
            </button>
        </div>
    </div>

    <!-- Mobile menu -->
    {#if mobileMenuOpen}
        <div class="absolute top-24 left-0 w-full bg-white py-5 z-10 lg:hidden">
            <ul class="flex flex-col font-outfit font-medium text-grey-950 items-center gap-4 text-lg">
                <li><a href="/">Informations</a></li>
                <li><a href="/mes-donnees">Mes données</a></li>
                <li><a href="/organisation">Mon organisation</a></li>
                <li><a href="/derniere-page">Dernière page consultée</a></li>

                {#if user}
                    <li><a href="/mon-compte" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Gérer mon compte</a></li>
                    <li><a href="/logout" class="block px-4 py-2 text-gray-800 hover:bg-gray-100">Se déconnecter</a></li>
                {:else}
                    <li><a href="/login">Se connecter</a></li>
                {/if}
            </ul>
        </div>
    {/if}
</header>
