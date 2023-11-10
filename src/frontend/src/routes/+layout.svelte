<script lang="ts">
	import Spinner from '$lib/components/Spinner.svelte';
	import { onMount } from 'svelte';
	import { generatorCanisterId } from '$lib/constants';

	async function init() {
		return new Promise((resolve) => {
			setTimeout(resolve, 1000);
		});
	}

	onMount(() => {
  		console.log('helo', generatorCanisterId);
	});

</script>

{#await init()}
	<Spinner />
{:then _}
	<slot />
{/await}

<style lang="scss" global>
	@import '../lib/styles/global.scss';
</style>
