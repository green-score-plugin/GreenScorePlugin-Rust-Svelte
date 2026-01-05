<script lang="ts">

	// import Header from '$lib/Header.svelte';
    import Footer from '$lib/Footer.svelte';

	let { children } = $props();
</script>

<Header />
<main class="flex-grow">
{@render children()}
</main>
<Footer />
