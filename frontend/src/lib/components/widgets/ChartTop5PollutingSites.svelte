<script lang="ts">
    import { onMount } from 'svelte';
    import Chart from 'chart.js/auto';

    export let topPollutingSites: Array<{ url_domain: string; total_footprint: number }> = [];

    let canvas: HTMLCanvasElement;
    let chartInstance: Chart | null = null;

    onMount(() => {
        if (canvas && topPollutingSites.length > 0) {
            const ctx = canvas.getContext('2d');
            if (!ctx) return;

            chartInstance = new Chart(ctx, {
                type: 'bar',
                data: {
                    labels: topPollutingSites.map(site => site.url_domain),
                    datasets: [{
                        label: 'Empreinte carbone (gCO2e)',
                        data: topPollutingSites.map(site => site.total_footprint),
                        backgroundColor: [
                            '#9333EA', // Violet
                            '#EC4899', // Rose
                            '#F59E0B', // Ambre
                            '#10B981', // Vert
                            '#3B82F6'  // Bleu
                        ],
                        borderRadius: 4,
                        maxBarThickness: 60
                    }]
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    plugins: {
                        legend: { display: false },
                        tooltip: {
                            callbacks: {
                                label: (context) => `${context.parsed.y.toFixed(2)} gCO2e`
                            }
                        }
                    },
                    scales: {
                        y: {
                            beginAtZero: true,
                            ticks: {
                                callback: (value) => `${value} g`
                            }
                        },
                        x: {
                            ticks: {
                                maxRotation: 45,
                                minRotation: 45
                            }
                        }
                    }
                }
            });
        }

        return () => {
            if (chartInstance) {
                chartInstance.destroy();
            }
        };
    });
</script>

<div class="bg-white rounded-lg shadow p-6 lg:col-span-6 col-span-1 sm:col-span-2 flex-wrap order-7 lg:order-7 flex flex-col min-h-[280px]">
    <h2 class="text-lg font-semibold text-gray-900 mb-4">Top 5 des sites les plus polluants</h2>

    {#if topPollutingSites.length > 0}
        <div class="flex-1 min-h-[200px]">
            <canvas bind:this={canvas}></canvas>
        </div>
    {:else}
        <div class="flex-1 flex items-center justify-center text-gray-500">
            Aucune donnée disponible
        </div>
    {/if}
</div>
