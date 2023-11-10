<script lang="ts">
    import Spinner from '$lib/components/Spinner.svelte';
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { dS } from '$lib/stores';
    import { syncAuth } from '$lib/stores';

    let isLoading = true;

    onMount(async () => {
        await syncAuth();
        if (!$dS.identity && location.pathname !== '/login') {
            goto('/login');
        }
        isLoading = false;
    });
</script>

{#if isLoading}
    <Spinner />
{:else}
    <slot />
{/if}


<style lang="scss" global>
	@import '../lib/styles/global.scss';
</style>
