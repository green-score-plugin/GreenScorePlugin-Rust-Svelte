<script lang="ts">
    import { page } from "$app/state";
    import CodeClipboard from "$lib/components/CodeClipboard.svelte";
    import { enhance } from '$app/forms';
    import type { SubmitFunction } from '@sveltejs/kit';
    import { t } from 'svelte-i18n';
    import type {User} from "$lib/types/account.ts";

    let showDeleteModal = $state(false);
    let deletingMemberId: number | null = $state(null);

    let showUnassignModal = $state(false);
    let unassignMemberId: number | null = $state(null);
    let unassignServiceMemberName: string | null = $state(null);

    let { organisation } = $props();
    let user = $derived(page.data.userFull.user as User);
    let members = $state(page.data.members || []);
    let deleteErrorMessage: string | null = $state(null);
    let unassignErrorMessage: string | null = $state(null);

    $effect(() => {
        members = page.data.members || [];
    });

    let query = $state('');
    let limit = $state(6);
    let scrollContainer: HTMLElement | undefined = $state();


    const handleDeleteResult: SubmitFunction = () => {
        deleteErrorMessage = null;
        return async ({ result }) => {
            if (result.type === 'success') {
                members = members.filter((m: any) => m.id !== deletingMemberId);
                showDeleteModal = false;
                deletingMemberId = null;
            } else if (result.type === 'failure') {
                // @ts-ignore
                deleteErrorMessage = result.data?.message ? $t(result.data.message) : $t('errors.unknown_error');
            } else {
                 deleteErrorMessage = $t('errors.unknown_error');
            }
        };
    };

    const handleUnassignResult: SubmitFunction = () => {
        unassignErrorMessage = null;
        return async ({ result }) => {
            if (result.type === 'success') {
                members = members.map((m: any) =>
                    m.id === unassignMemberId
                        ? { ...m, service_id: null, service_name: null }
                        : m
                );
                showUnassignModal = false;
                unassignMemberId = null;
            } else if (result.type === 'failure') {
                // @ts-ignore
                unassignErrorMessage = result.data?.message ? $t(result.data.message) : $t('errors.unknown_error');
            } else {
                 unassignErrorMessage = $t('errors.unknown_error');
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
                            <p class="text-sm font-medium text-gray-950 flex items-center gap-2">
                                { member.nom } { member.prenom }
                                {#if member.service_name}
                                    {#if user.id !== member.id}
                                        <button
                                            type="button"
                                            class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-blue-100 text-blue-800 transition-colors duration-200 cursor-pointer hover:bg-red-100 hover:text-red-800"
                                            title="Cliquer pour retirer du service"
                                            aria-label="Retirer du service"
                                            onclick={() => {
                                                showUnassignModal = true;
                                                unassignMemberId = member.id;
                                                unassignServiceMemberName = `${member.prenom} ${member.nom}`;
                                            }}
                                        >
                                            {member.service_name}
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-3 h-3 ml-1.5 opacity-60 hover:opacity-100 transition-opacity">
                                                <path d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z" />
                                            </svg>
                                        </button>
                                    {:else}
                                        <span
                                            class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-blue-100 text-blue-800 transition-colors duration-200"
                                            title="admin"
                                        >
                                            Admin
                                        </span>
                                    {/if}
                                {:else if user.id === member.id}
                                    <span
                                        class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-blue-100 text-blue-800 transition-colors duration-200"
                                        title="admin"
                                    >
                                        Admin
                                    </span>
                                {/if}
                            </p>
                            <p class="text-xs text-gray-500">{ member.email }</p>
                        </div>
                        {#if user.id !== member.id}
                            <button
                                    class="hover:scale-110 transition-transform duration-200 text-red-600 p-1 cursor-pointer"
                                    aria-label="Delete"
                                    onclick={() => { showDeleteModal = true; deletingMemberId = member.id; }}
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" width="1.5em" height="1.5em" viewBox="0 0 24 24">
                                    <path fill="currentColor" d="M18 12.998H6a1 1 0 0 1 0-2h12a1 1 0 0 1 0 2"/>
                                </svg>
                            </button>
                        {/if}
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
            <CodeClipboard code={organisation.code} />
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
            {#if deleteErrorMessage}
                <p class="text-red-500 mb-4 font-medium text-sm">{deleteErrorMessage}</p>
            {/if}
            <div class="flex justify-center gap-4">
                <button
                        onclick={() => { showDeleteModal = false; deletingMemberId = null; deleteErrorMessage = null; }}
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
                    <input type="hidden" name="organisationId" value={organisation.id} />
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

{#if showUnassignModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70">
        <div class="bg-white rounded-lg p-6 max-w-sm w-full mx-4 shadow-lg text-center">
             <div class="w-16 h-16 bg-blue-100 rounded-full flex items-center justify-center mx-auto mb-4 text-blue-600">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="8.5" cy="7" r="4"></circle><line x1="23" y1="11" x2="17" y2="11"></line></svg>
            </div>
            <h2 class="text-xl font-bold mb-2">{$t('account.service.unassign_modal.title') || "Retirer du service"}</h2>
            <p class="text-gray-600 mb-6">
                {$t('account.service.unassign_modal.description_prefix') || "Voulez-vous vraiment retirer"} <span class="font-bold">{unassignServiceMemberName}</span> {$t('account.service.unassign_modal.description_suffix') || "de son service ?"}.
            </p>
            {#if unassignErrorMessage}
                <p class="text-red-500 mb-4 font-medium text-sm">{unassignErrorMessage}</p>
            {/if}
            <div class="flex justify-center gap-4">
                <button
                        onclick={() => { showUnassignModal = false; unassignMemberId = null; unassignErrorMessage = null; }}
                        class="px-4 py-2 rounded-lg bg-gray-200 text-gray-800 hover:bg-gray-300 font-semibold cursor-pointer"
                >
                    {$t('account.modals.cancel')}
                </button>
                <form
                        use:enhance={handleUnassignResult}
                        method="POST"
                        action="?/unassign_user_service"
                >
                    <input type="hidden" name="userId" value={unassignMemberId} />
                    <input type="hidden" name="organisationId" value={organisation.id} />
                    <button
                            type="submit"
                            class="px-4 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700 font-semibold cursor-pointer"
                        >
                        {$t('account.service.unassign_modal.confirm_button') || "Retirer"}
                    </button>
                </form>
            </div>
        </div>
    </div>
{/if}
