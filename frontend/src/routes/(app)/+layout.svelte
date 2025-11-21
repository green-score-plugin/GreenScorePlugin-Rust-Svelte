<script lang="ts">

	import Header from '$lib/Header.svelte';

	let { children } = $props();
</script>

<Header />

{@render children()}
