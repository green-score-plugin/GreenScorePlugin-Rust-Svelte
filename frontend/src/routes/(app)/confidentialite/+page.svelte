<script lang="ts">
    import { t } from 'svelte-i18n';
</script>

<svelte:head>
    <title>{$t('legal.privacy_policy.title')}</title>
</svelte:head>

<div class="container mx-auto p-6 md:p-12 font-outfit">
    <h1 class="text-4xl font-bold text-center mb-8 text-gray-800">{$t('legal.privacy_policy.title')}</h1>

    <div class="prose max-w-none mb-8 text-gray-700">
        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.privacy_policy.s1.title')}</h2>
            <p class="mb-2">{$t('legal.privacy_policy.s1.content')}</p>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{@html $t('legal.privacy_policy.s1.li1')}</li>
                <li>{$t('legal.privacy_policy.s1.li2')}</li>
                <li>{$t('legal.privacy_policy.s1.li3')} <a href="mailto:greenscoreweb@gmail.com" class="text-blue-600 hover:text-blue-800">greenscoreweb@gmail.com</a></li>
            </ul>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.privacy_policy.s2.title')}</h2>
            <p><strong>{$t('legal.privacy_policy.s2.s2_1_title')}</strong></p>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.privacy_policy.s2.s2_1_li1')}</li>
                <li>{$t('legal.privacy_policy.s2.s2_1_li2')}</li>
            </ul>

            <p><strong>{$t('legal.privacy_policy.s2.s2_2_title')}</strong></p>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.privacy_policy.s2.s2_2_li1')}</li>
                <li>{$t('legal.privacy_policy.s2.s2_2_li2')}</li>
                <li>{$t('legal.privacy_policy.s2.s2_2_li3')}</li>
                <li>{$t('legal.privacy_policy.s2.s2_2_li4')}</li>
                <li>{$t('legal.privacy_policy.s2.s2_2_li5')}</li>
            </ul>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.privacy_policy.s3.title')}</h2>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.privacy_policy.s3.li1')}</li>
                <li>{$t('legal.privacy_policy.s3.li2')}</li>
                <li>{$t('legal.privacy_policy.s3.li3')}</li>
                <li>{$t('legal.privacy_policy.s3.li4')}</li>
                <li>{$t('legal.privacy_policy.s3.li5')}</li>
            </ul>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.privacy_policy.s4.title')}</h2>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.privacy_policy.s4.li1')}</li>
                <li>{$t('legal.privacy_policy.s4.li2')}</li>
                <li>{$t('legal.privacy_policy.s4.li3')}</li>
            </ul>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.privacy_policy.s5.title')}</h2>
            <p class="mb-4">{$t('legal.privacy_policy.s5.intro')}</p>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.privacy_policy.s5.li1')}</li>
                <li>{$t('legal.privacy_policy.s5.li2')}</li>
            </ul>
            <p class="mt-4">{$t('legal.privacy_policy.s5.outro')}</p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.privacy_policy.s6.title')}</h2>
            <p class="mb-4">{$t('legal.privacy_policy.s6.intro')}</p>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.privacy_policy.s6.li1')}</li>
                <li>{$t('legal.privacy_policy.s6.li2')}</li>
                <li>{$t('legal.privacy_policy.s6.li3')}</li>
            </ul>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.privacy_policy.s7.title')}</h2>
            <p class="mb-4">{$t('legal.privacy_policy.s7.p1')}</p>
            <p class="mb-4"><strong>{$t('legal.privacy_policy.s7.p2_strong')}</strong></p>
            <p>{$t('legal.privacy_policy.s7.p2')}</p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.privacy_policy.s8.title')}</h2>
            <p class="mb-4">{$t('legal.privacy_policy.s8.intro')}</p>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.privacy_policy.s8.li1')}</li>
                <li>{$t('legal.privacy_policy.s8.li2')}</li>
                <li>{$t('legal.privacy_policy.s8.li3')}</li>
                <li>{$t('legal.privacy_policy.s8.li4')}</li>
                <li>{$t('legal.privacy_policy.s8.li5')}</li>
                <li>{$t('legal.privacy_policy.s8.li6')}</li>
            </ul>
            <p class="mt-4">{$t('legal.privacy_policy.s8.outro')}</p>
            <p><a href="mailto:greenscoreweb@gmail.com" class="text-blue-600 hover:text-blue-800">greenscoreweb@gmail.com</a></p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.privacy_policy.s9.title')}</h2>
            <p>{$t('legal.privacy_policy.s9.content')}</p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.privacy_policy.s10.title')}</h2>
            <p>{$t('legal.privacy_policy.s10.content')}</p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.privacy_policy.s11.title')}</h2>
            <p>{$t('legal.privacy_policy.s11.intro')}</p>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.privacy_policy.s11.li1')}</li>
                <li>{$t('legal.privacy_policy.s11.li2')} <a href="mailto:greenscoreweb@gmail.com" class="text-blue-600 hover:text-blue-800">greenscoreweb@gmail.com</a></li>
            </ul>
        </section>
    </div>
</div>