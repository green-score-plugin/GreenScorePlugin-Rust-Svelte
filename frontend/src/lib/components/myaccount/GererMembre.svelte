<script lang="ts">
    import {page} from "$app/state";
    import CodeClipboard from "$lib/components/CodeClipboard.svelte";
    import type { Organisation } from "$lib/types/account";

    let user = $derived(page.data.user as Organisation);
    let members = $derived(page.data.members);

    let query = $state('');

    let filteredMembers = $derived(
        Array.isArray(members)
            ? members.filter(m =>
                `${m.nom} ${m.prenom} ${m.email}`.toLowerCase().includes(query.trim().toLowerCase())
            )
            : []
    );

</script>

<div class="flex flex-col gap-4" data-controller="copy-clipboard">
    <h1 class="font-outfit text-2xl font-semibold">Membres</h1>
    {#if members.length > 0}
        <div data-controller="members-search" data-members-search-url-value="{''}">
            <div class="flex gap-4 items-center justify-between w-full font-outfit mb-4">
                <div class="flex w-full">
                    <input
                            type="search"
                            data-members-search-target="input"
                            name="search"
                            placeholder="Rechercher un membre"
                            bind:value={query}
                            class="w-full px-3 py-2 border border-grey-200 rounded-lg focus:outline-none focus:ring-gs-green-950"
                    >
                </div>
            </div>

            {#each filteredMembers as member}
                <div class="flex py-3 items-center justify-between w-full border-b border-grey-200 last:border-b-0 font-outfit">
                    <div class="flex flex-col">
                        <p class="text-sm font-medium text-gray-950">{ member.nom } { member.prenom }</p>
                        <p class="text-xs text-gray-500">{ member.email }</p>
                    </div>
                    <button
                            data-action="click->member-delete#openModal"
                            data-member-delete-target="deleteButton"
                            data-member-id="{ member.id }"
                            class="hover:scale-110 transition-transform duration-200 text-red-600"
                            aria-label="Delete"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="2em" height="2em" viewBox="0 0 24 24">
                            <path fill="currentColor" d="M18 12.998H6a1 1 0 0 1 0-2h12a1 1 0 0 1 0 2"/>
                        </svg>
                    </button>
                </div>
            {/each}
        </div>
    {:else }
        <div class="flex flex-col gap-4">
            <p class="text-sm text-gray-500">Vous n'avez pas encore ajouté de membres à votre organisation. Envoyez leur le code suivant afin qu'ils puissent rejoindre votre organisation.</p>
            <CodeClipboard code={user.code} />
        </div>
    {/if}
</div>