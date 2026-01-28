<script lang="ts">
    import { t } from 'svelte-i18n';
</script>

<footer class="bg-gs-green-950 text-white py-6 font-outfit">
    <div class="container mx-auto text-center">
        <p class="text-lg mb-4">{$t('footer.copyright')}</p>
        <div class="space-x-4">
            <a href="/confidentialite" class="hover:text-blue-400">{$t('footer.privacy_policy')}</a>
            <span>|</span>
            <a href="/cgu" class="hover:text-blue-400">{$t('footer.terms_of_use')}</a>
        </div>
        <p class="mt-4">{$t('footer.contact_us')} <a href="mailto:greenscoreweb@gmail.com" class="hover:text-blue-400">greenscoreweb@gmail.com</a></p>
    </div>
</footer>