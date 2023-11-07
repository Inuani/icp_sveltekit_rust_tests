<script lang="ts">
	import Spinner from '$lib/components/Spinner.svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { dS } from '$lib/stores';
	import { syncAuth } from '$lib/stores';

	// async function init() {
	// 	return new Promise((resolve) => {
	// 		setTimeout(resolve, 1000);
	// 	});
	// }

	// onMount(async () => {

	// });
</script>

{#await syncAuth()}
	<Spinner />
{:then _}
	<slot />
{/await}

<style lang="scss" global>
	@import '../lib/styles/global.scss';
</style>
