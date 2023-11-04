<script lang="ts">
	import { createActor } from '../../../declarations/backend';
	import { all } from '$lib/global';
	import { NFID } from '@nfid/embed';
	import type { Identity } from '@dfinity/agent';
	import { onMount } from 'svelte';

	const loginNFID = async () => {
		const config = {
			application: {
				name: 'oversyn',
				logo: '/logo_skull.png'
			}
		};
		const nfid = await NFID.init(config);

		if (nfid.isAuthenticated) {
			all.isAuth = true;
		} else {
			all.delegIdentity = await nfid.getDelegation({
				targets: [import.meta.env.VITE_BACKEND_CANISTER_ID],
				derivationOrigin: `https://${import.meta.env.VITE_BACKEND_CANISTER_ID}.ic0.app`,
				maxTimeToLive: BigInt(8) * BigInt(3_600_000_000_000)
			});
			if (all.delegIdentity) {
				all.isAuth = true;
			} else console.error('Failed to obtain delegation identity');
		}
	};
</script>

<main>
	<br />
	<br />

	<button on:click={loginNFID}>Authenticate with NFID</button>

	<br />
	<br />
</main>

<style lang="scss">
	button[type='submit'] {
		padding: 5px 20px;
		margin: 10px auto;
		float: right;
	}
</style>
