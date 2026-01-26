<script lang="ts">

    import { page } from "$app/state";

    let account = $derived(page);
    let accountEquivalents = $derived(page.data.accountEquivalents || []);

    $inspect(accountEquivalents);
</script>

<form action=""
      method="POST"
      class="flex flex-col gap-4">

</form>
