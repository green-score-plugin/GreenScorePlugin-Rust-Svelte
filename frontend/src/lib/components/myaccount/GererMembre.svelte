<script lang="ts">
    import { page } from "$app/state";
    import CodeClipboard from "$lib/components/CodeClipboard.svelte";
    import type { Organisation } from "$lib/types/account";

    let user = $derived(page.data.user as Organisation);
    let members = $derived(page.data.members || []);

    let query = $state('');
    let limit = $state(6);
    let scrollContainer: HTMLElement | undefined = $state();

    let filteredMembers = $derived(
        Array.isArray(members)
            ? members.filter(m =>
                `${m.nom} ${m.prenom} ${m.email}`.toLowerCase().includes(query.trim().toLowerCase())
            )
            : []
    );

    let visibleMembers = $derived(filteredMembers.slice(0, limit));

    function loadMore() {
        if (limit < filteredMembers.length) {
            limit += 6;
        }
    }

    $effect(() => {
        query;
        limit = 6;
    });

    function infiniteScroll(node: HTMLElement) {
        const observer = new IntersectionObserver((entries) => {
            if (entries[0].isIntersecting) {
                loadMore();
            }
        }, {
            root: scrollContainer,
            rootMargin: '50px'
        });

        observer.observe(node);
        return { destroy: () => observer.disconnect() };
    }
</script>

<div class="flex flex-col gap-4">
    <h1 class="font-outfit text-2xl font-semibold">Membres</h1>

    {#if members.length > 0}
        <div class="w-full font-outfit">
            <div class="mb-4">
                <input
                        type="search"
                        placeholder="Rechercher un membre"
                        bind:value={query}
                        class="w-full px-3 py-2 border border-grey-200 rounded-lg focus:outline-none focus:ring-1 focus:ring-gs-green-950"
                >
            </div>

            <div
                    bind:this={scrollContainer}
                    class="max-h-[300px] overflow-y-auto pr-2 custom-scrollbar"
            >
                {#each visibleMembers as member (member.id || member.email)}
                    <div class="flex py-3 items-center justify-between w-full border-b border-grey-200 last:border-b-0">
                        <div class="flex flex-col">
                            <p class="text-sm font-medium text-gray-950">{ member.nom } { member.prenom }</p>
                            <p class="text-xs text-gray-500">{ member.email }</p>
                        </div>
                        <button
                                class="hover:scale-110 transition-transform duration-200 text-red-600 p-1 cursor-pointer"
                                aria-label="Delete"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="1.5em" height="1.5em" viewBox="0 0 24 24">
                                <path fill="currentColor" d="M18 12.998H6a1 1 0 0 1 0-2h12a1 1 0 0 1 0 2"/>
                            </svg>
                        </button>
                    </div>
                {/each}

                {#if limit < filteredMembers.length}
                    <div use:infiniteScroll class="py-6 flex justify-center items-center">
                        <div class="w-6 h-6 border-2 border-gs-green-950 border-t-transparent rounded-full animate-spin"></div>
                    </div>
                {/if}
            </div>
        </div>
    {:else}
        <div class="flex flex-col gap-4">
            <p class="text-sm text-gray-500">Vous n'avez pas encore ajouté de membres...</p>
            <CodeClipboard code={user.code} />
        </div>
    {/if}
</div>
