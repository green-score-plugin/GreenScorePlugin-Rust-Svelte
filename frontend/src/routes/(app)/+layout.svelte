<script lang="ts">
    import Header from '$lib/Header.svelte';
    import Footer from '$lib/Footer.svelte';

    let { children } = $props();
</script>

<div class="flex flex-col min-h-screen">
    <Header />
    <main class="flex-grow w-full">
        {@render children()}
    </main>
    <Footer />
</div>
