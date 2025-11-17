
<script>
    import { onMount } from 'svelte';

    let user = 10;
    let profileMenuOpen = false;
    let mobileMenuOpen = false;

    function toggleProfileMenu() {
        profileMenuOpen = !profileMenuOpen;
    }

    function toggleMobileMenu() {
        mobileMenuOpen = !mobileMenuOpen;
    }
</script>
