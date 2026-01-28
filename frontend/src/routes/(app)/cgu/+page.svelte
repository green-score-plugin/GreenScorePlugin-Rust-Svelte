<script lang="ts">
    import { t } from 'svelte-i18n';
</script>

<svelte:head>
    <title>{$t('legal.cgu.title')}</title>
</svelte:head>

<div class="container mx-auto p-6 md:p-12 font-outfit">
    <h1 class="text-4xl font-bold text-center mb-8 text-gray-800">
        {$t('legal.cgu.title')}
    </h1>

    <div class="prose max-w-none mb-8 text-gray-700">

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.cgu.s1.title')}</h2>
            <p class="text-lg mb-4">{$t('legal.cgu.s1.intro')}</p>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.cgu.s1.li1')}</li>
                <li>{$t('legal.cgu.s1.li2')}</li>
            </ul>
            <p class="mt-4">
                {$t('legal.cgu.s1.outro')}
            </p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.cgu.s2.title')}</h2>
            <p class="text-lg mb-4">
                {$t('legal.cgu.s2.content')}
            </p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.cgu.s3.title')}</h2>
            <h3 class="text-2xl font-semibold text-gray-800 mb-4">{$t('legal.cgu.s3.s3_1_title')}</h3>
            <p class="text-lg mb-4">
                {$t('legal.cgu.s3.s3_1_content')}
            </p>

            <h3 class="text-2xl font-semibold text-gray-800 mb-4">{$t('legal.cgu.s3.s3_2_title')}</h3>
            <p class="text-lg mb-4">
                {$t('legal.cgu.s3.s3_2_content')}
            </p>

            <h3 class="text-2xl font-semibold text-gray-800 mb-4">{$t('legal.cgu.s3.s3_3_title')}</h3>
            <p class="text-lg mb-4">{$t('legal.cgu.s3.s3_3_intro')}</p>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.cgu.s3.s3_3_li1')}</li>
                <li>{$t('legal.cgu.s3.s3_3_li2')}</li>
            </ul>
            <p class="mt-4">
                {$t('legal.cgu.s3.s3_3_outro')}
            </p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.cgu.s4.title')}</h2>
            <p class="text-lg mb-4">{$t('legal.cgu.s4.intro')}</p>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.cgu.s4.li1')}</li>
                <li>{$t('legal.cgu.s4.li2')}</li>
                <li>{$t('legal.cgu.s4.li3')}</li>
            </ul>
            <p class="mt-4">
                {$t('legal.cgu.s4.outro')}
            </p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.cgu.s5.title')}</h2>
            <p class="text-lg mb-4">
                {$t('legal.cgu.s5.intro')}
            </p>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.cgu.s5.li1')}</li>
                <li>{$t('legal.cgu.s5.li2')}</li>
            </ul>
            <p class="mt-4">{$t('legal.cgu.s5.forbidden_intro')}</p>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.cgu.s5.forbidden_li1')}</li>
                <li>
                    {$t('legal.cgu.s5.forbidden_li2')}
                </li>
            </ul>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.cgu.s6.title')}</h2>
            <p class="text-lg mb-4">
                {$t('legal.cgu.s6.p1')}
            </p>
            <p class="text-lg mb-4">
                {$t('legal.cgu.s6.p2')}
            </p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.cgu.s7.title')}</h2>
            <p class="text-lg mb-4">
                {$t('legal.cgu.s7.intro')}
            </p>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.cgu.s7.li1')}</li>
                <li>{$t('legal.cgu.s7.li2')}</li>
            </ul>
            <p class="mt-4">
                {$t('legal.cgu.s7.outro')}
            </p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">
                {$t('legal.cgu.s8.title')}
            </h2>
            <p class="text-lg mb-4">
                {$t('legal.cgu.s8.content')} <a href="/confidentialite" class="text-blue-600 hover:text-blue-800"
            >{$t('legal.cgu.s8.link')}</a
            >{$t('legal.cgu.s8.content_end')}
            </p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.cgu.s9.title')}</h2>
            <p class="text-lg mb-4">
                {$t('legal.cgu.s9.content')}
            </p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.cgu.s10.title')}</h2>
            <p class="text-lg mb-4">
                {$t('legal.cgu.s10.p1')}
            </p>
            <p class="mt-4">
                {$t('legal.cgu.s10.p2')}
            </p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">
                {$t('legal.cgu.s11.title')}
            </h2>
            <p class="text-lg mb-4">{$t('legal.cgu.s11.p1')}</p>
            <p class="text-lg">
                {$t('legal.cgu.s11.p2')}
            </p>
        </section>

        <section class="mt-8">
            <h2 class="text-3xl font-semibold text-gray-800 mb-4">{$t('legal.cgu.s12.title')}</h2>
            <p class="text-lg mb-4">
                {$t('legal.cgu.s12.intro')}
            </p>
            <ul class="list-disc pl-6 space-y-2 text-lg">
                <li>{$t('legal.cgu.s12.address')}</li>
                <li>
                    {$t('legal.cgu.s12.email')} <a
                        href="mailto:greenscoreweb@gmail.com"
                        class="text-blue-600 hover:text-blue-800">greenscoreweb@gmail.com</a
                >
                </li>
            </ul>
        </section>
    </div>
</div>

