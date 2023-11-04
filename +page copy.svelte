<script lang="ts">
	import { createActor } from './src/declarations/backend';
	import { all } from '$lib/global';
	import { NFID } from '@nfid/embed';
	import type { Identity } from '@dfinity/agent';
	import { onMount } from 'svelte';

	let keyInput = '';
	let ageInput = '';
	let nameInput = '';
	let disabled = false;
	let result = '';
	let output = '';
	let indexInput: bigint;

	const canisterId = import.meta.env.VITE_BACKEND_CANISTER_ID;
	const host = import.meta.env.VITE_HOST;
	const actor = createActor(canisterId, { agentOptions: { host } });

	const handleOnSubmit = async () => {
		disabled = true;
		try {
			const key = BigInt(keyInput);
			const userProfile = { age: Number(ageInput), name: nameInput };
			console.log(userProfile);
			await actor.insert(key, Denizen);
			const retrievedValue = await actor.get(key);
			console.log(retrievedValue[0]);
			result = retrievedValue[0]
				? `${retrievedValue[0].name}, ${retrievedValue[0].age}`
				: 'Key not found';
		} catch (err: unknown) {
			console.error(err);
		}
		disabled = false;
	};

	const getValue = async () => {
		const index = BigInt(indexInput); // Parse index to bigint
		const retrievedValue = await actor.get(index);
		output = retrievedValue[0] ? `${retrievedValue[0].name}}` : 'Key not found';
	};

	type NFIDConfig = {
		origin?: string; // default is "https://nfid.one"
		application?: {
			name?: string;
			logo?: string;
		};
	};

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

	<form on:submit|preventDefault={handleOnSubmit}>
		<label for="key">Enter key: &nbsp;</label>
		<input id="key" alt="Key" type="text" bind:value={keyInput} {disabled} />

		<label for="age">Enter age: &nbsp;</label>
		<input id="age" alt="Age" type="number" bind:value={ageInput} {disabled} />

		<label for="name">Enter name: &nbsp;</label>
		<input id="name" alt="Name" type="text" bind:value={nameInput} {disabled} />

		<button type="submit">Submit</button>
	</form>

	<section id="result">
		{result}
	</section>

	<form on:submit|preventDefault={() => getValue()}>
		<label for="index">Enter index: &nbsp;</label>
		<input id="index" alt="Index" type="text" bind:value={indexInput} {disabled} />
		<button type="submit">ask</button>
	</form>
	{output}
</main>

<style lang="scss">
	// img {
	// 	max-width: 60vw;
	// 	max-height: 25vw;
	// 	display: block;
	// 	margin: auto;
	// }

	form {
		display: flex;
		flex-direction: column;
		justify-content: center;
		gap: 0.5em;
		// flex-flow: row wrap;
		max-width: 40vw;
		margin: auto;
		align-items: baseline;
		font-family: sans-serif;
		font-size: 1.5rem;
	}

	button[type='submit'] {
		padding: 5px 20px;
		margin: 10px auto;
		float: right;
	}
</style>
