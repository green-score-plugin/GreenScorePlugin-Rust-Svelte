<script lang="ts">
    import { page } from "$app/state";
    import CodeClipboard from "$lib/components/CodeClipboard.svelte";
    import type { Organisation } from "$lib/types/account";
    import { enhance } from '$app/forms';
    import type { SubmitFunction } from '@sveltejs/kit';
    import { t } from 'svelte-i18n';

    let showDeleteModal = $state(false);
    let deletingMemberId: number | null = $state(null);

    let user = $derived(page.data.user as Organisation);
    let members = $state(page.data.members || []);

    $effect(() => {
        members = page.data.members || [];
    });

    let query = $state('');
    let limit = $state(6);
    let scrollContainer: HTMLElement | undefined = $state();


    const handleDeleteResult: SubmitFunction = () => {
        return async ({ result }) => {
            if (result.type === 'success') {
                members = members.filter((m: any) => m.id !== deletingMemberId);
                showDeleteModal = false;
                deletingMemberId = null;
                // Add a toast or notification if needed, using result.data.message
            }
        };
    };

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
                                onclick={() => { showDeleteModal = true; deletingMemberId = member.id; }}
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

{#if showDeleteModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70">
        <div class="bg-white rounded-lg p-6 max-w-sm w-full mx-4 shadow-lg text-center">
            <svg class="w-16 h-16 text-yellow-500 mx-auto mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
            </svg>
            <h2 class="text-xl font-bold mb-2">{$t('account.modals.delete_member_title')}</h2>
            <p class="text-gray-600 mb-6">{$t('account.modals.delete_member_desc')}</p>
            <div class="flex justify-center gap-4">
                <button
                        onclick={() => { showDeleteModal = false; deletingMemberId = null; }}
                        class="px-4 py-2 rounded-lg bg-gray-200 text-gray-800 hover:bg-gray-300 font-semibold cursor-pointer"
                >
                    {$t('account.modals.cancel')}
                </button>
                <form
                        use:enhance={handleDeleteResult}
                        method="POST"
                        action="?/supprimer_membre"
                >
                    <input type="hidden" name="deleteMemberId" value={deletingMemberId} />
                    <button
                            type="submit"
                            class="px-4 py-2 rounded-lg bg-red-600 text-white hover:bg-red-700 font-semibold cursor-pointer"
                        >
                        {$t('account.modals.delete')}
                    </button>
                </form>
            </div>
        </div>
    </div>
{/if}
