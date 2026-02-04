<script lang="ts">
    import favicon from "$lib/images/greenscore-logo.png";
    import '../app.css';
    import '$lib/i18n';
    import { waitLocale } from 'svelte-i18n';

    let { children } = $props();
</script>

<svelte:head>
    <link rel="icon" href={favicon}/>
    <link rel="preconnect" href="https://fonts.googleapis.com"/>
    <link rel="preconnect" href="https://fonts.gstatic.com"/>
    <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;500;600;700;800;900&display=swap" rel="stylesheet"/>
</svelte:head>

{#await waitLocale() then}
    {@render children()}
{/await}