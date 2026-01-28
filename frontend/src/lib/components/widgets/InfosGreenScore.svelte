<script lang="ts">
    import { t } from 'svelte-i18n';
    export let APP_URL: string = 'http://localhost:5173/';
    export let letterGreenScore: string | null = null;
</script>

<div class="bg-white flex flex-col justify-between gap-4 p-6 rounded-lg shadow-lg col-span-1 order-1 lg:col-span-4 text-gray-900 font-outfit">
    <h2 class="text-lg font-bold mb-4">{$t('widgets.infos_greenscore.title')}</h2>

    {#if letterGreenScore}
        <div class="flex flex-col items-center w-full gap-4">
            <!-- Bouton interactif avec couleur dynamique -->
            <button on:click={() => window.location.href = `${APP_URL}#greenscore`}
                    type="button"
                    class="text-2xl flex items-center font-bold hover:scale-105 gap-4 transition-transform duration-200 rounded-lg px-4 py-2 shadow-md cursor-pointer text-white {
             letterGreenScore === 'A' ? 'bg-[#2e9b43]' :
             letterGreenScore === 'B' ? 'bg-[#347851]' :
             letterGreenScore === 'C' ? 'bg-[#cadd00]' :
             letterGreenScore === 'D' ? 'bg-[#f7ed00]' :
             letterGreenScore === 'E' ? 'bg-[#ffce00]' :
             letterGreenScore === 'F' ? 'bg-[#fb9929]' :
             letterGreenScore === 'G' ? 'bg-[#f01c16]' :
             'bg-gray-300'
           }">
                <span class="text-xl font-medium text-white">{$t('widgets.infos_greenscore.greenscore')}</span>
                <span class="w-10 h-10 flex items-center justify-center rounded-full bg-white text-2xl font-bold {
              letterGreenScore === 'A' ? 'text-[#2e9b43]' :
              letterGreenScore === 'B' ? 'text-[#347851]' :
              letterGreenScore === 'C' ? 'text-[#cadd00]' :
              letterGreenScore === 'D' ? 'text-[#f7ed00]' :
              letterGreenScore === 'E' ? 'text-[#ffce00]' :
              letterGreenScore === 'F' ? 'text-[#fb9929]' :
              letterGreenScore === 'G' ? 'text-[#f01c16]' :
              'text-gray-800'
            }">
                    {letterGreenScore}
                </span>
            </button>

            <!-- Texte contextuel -->
            <p class="text-center text-lg font-medium w-4/5">
                {#if letterGreenScore === 'A'}
                    {$t('widgets.infos_greenscore.score.A')}
                {:else if letterGreenScore === 'B'}
                    {$t('widgets.infos_greenscore.score.B')}
                {:else if letterGreenScore === 'C'}
                    {$t('widgets.infos_greenscore.score.C')}
                {:else if letterGreenScore === 'D'}
                    {$t('widgets.infos_greenscore.score.D')}
                {:else if letterGreenScore === 'E'}
                    {$t('widgets.infos_greenscore.score.E')}
                {:else if letterGreenScore === 'F'}
                    {$t('widgets.infos_greenscore.score.F')}
                {:else if letterGreenScore === 'G'}
                    {$t('widgets.infos_greenscore.score.G')}
                {:else}
                    {$t('widgets.infos_greenscore.score.unknown')}
                {/if}
            </p>

        </div>

        <p class="text-sm mt-4 text-gray-600">
            {$t('widgets.infos_greenscore.description')}
        </p>
    {:else}
        <p class="text-center text-gray-500 flex items-center justify-center h-full">{$t('widgets.common.no_data')}</p>
    {/if}
</div>