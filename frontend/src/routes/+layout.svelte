<script lang="ts">
    import favicon from "$lib/assets/favicon.svg";
    import '../app.css';
    import { account } from '$lib/stores/account';
    import type { LayoutData } from './$types';

    let { children, data }: { children: any, data: LayoutData } = $props();

    $effect(() => {
        if (data.account) {
            account.login(data.account);
        } else {
            account.logout();
        }
    });
</script>


<svelte:head>
    <link rel="icon" href={favicon}/>
    <link rel="preconnect" href="https://fonts.googleapis.com"/>
    <link rel="preconnect" href="https://fonts.gstatic.com"/>
    <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;500;600;700;800;900&display=swap" rel="stylesheet"/>
</svelte:head>

{@render children()}