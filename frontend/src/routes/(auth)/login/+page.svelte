<script lang="ts">
    import AuthBase from '$lib/components/AuthBase.svelte';
    import type { ActionData } from './$types';

    export let form: ActionData;
</script>

<AuthBase mode="login" {form} />