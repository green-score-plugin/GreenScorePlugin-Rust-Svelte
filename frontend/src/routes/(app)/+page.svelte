<script lang="ts">
    import type { PageData } from './$types';
    import Home from '$lib/components/Home.svelte';
    export let data: PageData;
</script>

<Home advice={data.advice}/>