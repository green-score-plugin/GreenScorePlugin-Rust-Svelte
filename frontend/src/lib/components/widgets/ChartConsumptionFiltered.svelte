<script lang="ts">
    import { t } from 'svelte-i18n';
    import { onMount } from 'svelte';
    import Chart from 'chart.js/auto';

    export let consumptionData: Array<{ label: string; value: number }> = [];
    export let selectedPeriod: 'daily' | 'weekly' | 'monthly' = 'monthly';

    let canvas: HTMLCanvasElement;
    let chartInstance: Chart | null = null;

    onMount(() => {
        if (consumptionData.length > 0) {
            updateChart();
        }
    });

    $: if (canvas && consumptionData) {
        updateChart();
    }

    function updateChart() {
        const ctx = canvas?.getContext('2d');
        if (!ctx) return;

        if (chartInstance) {
            chartInstance.destroy();
            chartInstance = null;
        }

        if (consumptionData.length === 0) return;

        chartInstance = new Chart(ctx, {
            type: 'bar',
            data: {
                labels: consumptionData.map(d => d.label),
                datasets: [{
                    data: consumptionData.map(d => d.value),
                    backgroundColor: '#9333EA',
                    borderRadius: 4,
                    barThickness: 20
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        display: false
                    },
                    tooltip: {
                        callbacks: {
                            label: (context) => {
                                const value = context.parsed.y ?? 0;
                                return `${value.toFixed(2)} gCO2e`;
                            }
                        }
                    }
                },
                scales: {
                    y: {
                        beginAtZero: true,
                        grid: {
                            color: '#e5e7eb'
                        },
                        ticks: {
                            font: {
                                size: 12
                            }
                        }
                    },
                    x: {
                        grid: {
                            display: false
                        },
                        ticks: {
                            font: {
                                size: 11
                            },
                            maxRotation: 45,
                            minRotation: 45
                        }
                    }
                }
            }
        });
    }

    $: periods = [
        { value: 'daily', label: $t('widgets.common.period.labels.daily') },
        { value: 'weekly', label: $t('widgets.common.period.labels.weekly') },
        { value: 'monthly', label: $t('widgets.common.period.labels.monthly') }
    ];
</script>

<div class="bg-white rounded-lg shadow p-6 lg:col-span-4 col-span-1 sm:col-span-2 order-2 sm:order-3 lg:order-2">
    <div class="flex justify-between items-center mb-1">
        <h2 class="text-lg font-semibold text-gray-900">{$t('widgets.chart_consumption.title')}</h2>
        <select
                bind:value={selectedPeriod}
                class="px-3 py-1 border border-gray-300 rounded-lg bg-white text-xs focus:outline-none focus:ring-2 focus:ring-purple-500"
        >
            {#each periods as period}
                <option value={period.value}>{period.label}</option>
            {/each}
        </select>
    </div>
    <p class="text-xs text-gray-600 mb-1">(en gCO2e)</p>
    <p class="text-xs text-blue-600 mb-2">{$t(`widgets.common.period.messages.${selectedPeriod}`)}</p>

    {#if consumptionData.length > 0}
        <div class="h-40">
            <canvas bind:this={canvas}></canvas>
        </div>
    {:else}
        <div class="h-40 flex items-center justify-center text-gray-500">
            {$t('widgets.chart_consumption.no_data_period')}
        </div>
    {/if}
</div>